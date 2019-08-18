use std::collections::BTreeMap;
use std::env;
use std::fs;
use std::path::PathBuf;

use crate::function::Function;

pub struct Config {
    entries: BTreeMap<String, Function>,
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

        let entries: BTreeMap<String, Function> = match toml::from_str(&toml_str) {
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
            let path: PathBuf = [&path.to_str().unwrap(), "centoria.toml"].iter().collect();

            return Ok(path);
        }
        return Err(failure::err_msg("could not detect configuration path."));
    }

    // instance methods
    pub fn add(&mut self, name: &str, function: Function) -> Result<(), failure::Error> {
        if self.exists(name) {
            let msg = format!("function name `{}` is already exists", name);
            return Err(failure::err_msg(msg));
        }

        self.entries.insert(name.to_string(), function);
        return Ok(());
    }

    pub fn remove(&mut self, name: &str) -> Result<(), failure::Error> {
        if !self.exists(name) {
            let msg = format!("function name `{}` is not exists", name);
            return Err(failure::err_msg(msg));
        }

        self.entries.remove(name);
        return Ok(());
    }

    pub fn get(&self, name: &str) -> Option<&Function> {
        return self.entries.get(name);
    }

    pub fn exists(&self, name: &str) -> bool {
        return match self.entries.get(name) {
            Some(_) => true,
            None => false,
        };
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
