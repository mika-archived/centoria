use std::collections::BTreeMap;
use std::iter::FromIterator;

use crate::config::{self, Alias};

pub fn remove(name: &str) -> Result<(), failure::Error> {
  let cfg = config::load()?;
  if !check_alias_registered(name, &cfg) {
    let msg = failure::err_msg(format!("alias `{}` is not registered", name));
    return Err(msg);
  }

  let mut cfg: BTreeMap<String, Alias> = BTreeMap::from_iter(cfg.into_iter());
  cfg.remove(name);
  config::save(cfg)?;

  return Ok(());
}

fn check_alias_registered(name: &str, cfg: &BTreeMap<String, Alias>) -> bool {
  match cfg.get(name) {
    Some(_) => return true,
    None => return false,
  };
}
