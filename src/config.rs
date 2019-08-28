use std::collections::BTreeMap;
use std::env;
use std::fs;
use std::path::PathBuf;

use crate::executors::{Executor, SubCommand};

pub struct Config {
    // value must implement Executor trait
    entries: BTreeMap<String, Box<dyn Executor>>,
}

impl Config {
    // static methods
    pub fn load() -> Result<Config, failure::Error> {
        let path = Config::find_valid_path()?;
        if !path.exists() {
            return Ok(Config {
                entries: BTreeMap::new(),
            });
        }

        let toml_str = match fs::read_to_string(&path) {
            Ok(string) => string,
            Err(e) => {
                let msg = format!("could not open the file {} because {}", path.display(), e);
                return Err(failure::err_msg(msg));
            }
        };

        let entries: BTreeMap<String, Box<dyn Executor>> = match toml::from_str(&toml_str) {
            Ok(value) => value,
            Err(e) => {
                let msg = format!("could not parse configuration file because {}", e);
                return Err(failure::err_msg(msg));
            }
        };

        return Ok(Config { entries });
    }

    fn find_valid_path() -> Result<PathBuf, failure::Error> {
        // $CENTORIA_CONFIG_PATH
        if let Ok(path) = env::var("CENTORIA_CONFIG_PATH") {
            let path = PathBuf::from(&path);

            if path.exists() {
                return Ok(path);
            }
        }

        // $SYSTEM_CONFIGURATION_DIRECTORY/centoria/centoria.toml
        if let Some(path) = dirs::config_dir() {
            let path: PathBuf = [&path.to_str().unwrap(), "centoria", "centoria.toml"]
                .iter()
                .collect();

            if path.exists() {
                return Ok(path);
            }
        }

        // $HOME/.centoria.toml
        if let Some(path) = dirs::home_dir() {
            let path: PathBuf = [&path.to_str().unwrap(), ".centoria.toml"].iter().collect();

            return Ok(path);
        }
        return Err(failure::err_msg("could not detect configuration path."));
    }

    // instance methods
    pub fn add(&mut self, name: &str, executor: Box<dyn Executor>) -> Result<(), failure::Error> {
        if executor.is::<SubCommand>() {
            return self.add_child(name, executor.downcast::<SubCommand>().ok().unwrap());
        }

        if self.exists(name) {
            let msg = format!("function `{}` is already exists", name);
            return Err(failure::err_msg(msg));
        }

        self.entries.insert(name.to_string(), executor);
        return Ok(());
    }

    fn add_child(&mut self, name: &str, executor: Box<SubCommand>) -> Result<(), failure::Error> {
        if self.exists(name) {
            let existing: &mut Box<dyn Executor> = self.entries.get_mut(name).unwrap();
            let existing: &mut SubCommand = existing.downcast_mut::<SubCommand>().unwrap();
            existing.add(*executor)?; // unboxing
        } else {
            self.entries.insert(name.to_owned(), executor);
        }

        return Ok(());
    }

    pub fn remove(&mut self, name: &str) -> Result<(), failure::Error> {
        if !self.exists(name) {
            let msg = format!("function `{}` is not exists", name);
            return Err(failure::err_msg(msg));
        }

        self.entries.remove(name);
        return Ok(());
    }

    pub fn get(&self, name: &str) -> Option<&Box<dyn Executor>> {
        return self.entries.get(name);
    }

    pub fn exists(&self, name: &str) -> bool {
        return match self.entries.get(name) {
            Some(_) => true,
            None => false,
        };
    }

    pub fn keys(&self) -> Vec<String> {
        return self.entries.keys().cloned().collect();
    }

    pub fn save(&mut self) -> Result<(), failure::Error> {
        let path = Config::find_valid_path()?;
        let toml_str = match toml::to_string_pretty(&self.entries) {
            Ok(value) => value,
            Err(e) => {
                let msg = format!("could not save configuration because {}", e);
                return Err(failure::err_msg(msg));
            }
        };

        fs::write(path, toml_str)?;

        return Ok(());
    }
}
