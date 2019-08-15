use std::env;
use std::path::PathBuf;

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
