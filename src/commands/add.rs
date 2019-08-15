use std::collections::BTreeMap;
use std::iter::FromIterator;

use crate::config::{self, Alias};

pub fn add(name: &str, command: &str) -> Result<(), failure::Error> {
  let cfg = config::load()?;
  if check_already_registered(name, &cfg) {
    let msg = failure::err_msg(format!("alias `{}` is already registered", name));
    return Err(msg);
  }

  let mut cfg: BTreeMap<String, Alias> = BTreeMap::from_iter(cfg.into_iter());
  cfg.insert(name.to_string(), create_alias(command));
  config::save(cfg)?;
  return Ok(());
}

fn check_already_registered(name: &str, cfg: &BTreeMap<String, Alias>) -> bool {
  match cfg.get(name) {
    Some(_) => return true,
    None => return false,
  };
}

fn create_alias(command: &str) -> Alias {
  return Alias {
    command: command.to_string(),
  };
}
