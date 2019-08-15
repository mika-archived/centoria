use std::collections::BTreeMap;
use std::env;
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug)]
pub struct Alias {
  pub command: String,
}

fn find_valid_path() -> Result<PathBuf, failure::Error> {
  if let Ok(path) = env::var("CENTORIA_CONFIG_PATH") {
    let path = PathBuf::from(&path);

    if path.exists() {
      return Ok(path);
    }
  }

  if let Ok(xdg_home) = env::var("XDG_CONFIG_HOME") {
    let mut path = PathBuf::from(&xdg_home);
    path.push("centoria");
    path.push("centoria.toml");

    if path.exists() {
      return Ok(path);
    }
  }

  if let Some(home) = dirs::home_dir() {
    let mut path = PathBuf::from(&home);
    path.push(".centoria.toml");

    return Ok(path);
  }
  return Err(failure::err_msg("could not detect configuration path."));
}

fn read() -> Result<Option<String>, failure::Error> {
  let path = find_valid_path()?;

  if path.exists() {
    match fs::read_to_string(&path) {
      Ok(value) => return Ok(Some(value)),
      Err(_) => {
        let msg = failure::err_msg(format!("could not open the file: {}", path.display()));
        return Err(msg);
      }
    }
  }

  return Ok(None);
}

pub fn load() -> Result<BTreeMap<String, Alias>, failure::Error> {
  let toml_str = read()?;
  if let Some(toml_str) = toml_str {
    match toml::from_str(&toml_str) {
      Ok(value) => return Ok(value),
      Err(e) => {
        let msg = failure::err_msg(format!("could not parse configuration file because {}", e));
        return Err(msg);
      }
    };
  }

  return Ok(BTreeMap::new());
}

pub fn save(cfg: BTreeMap<String, Alias>) -> Result<(), failure::Error> {
  let path = find_valid_path()?;
  let toml_str = match toml::to_string_pretty(&cfg) {
    Ok(value) => value,
    Err(e) => {
      let msg = failure::err_msg(format!("could not write configuration file because {}", e));
      return Err(msg);
    }
  };

  fs::write(path, toml_str)?;

  return Ok(());
}
