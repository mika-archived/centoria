use std::process::Command;

use crate::config::{self, Alias};

pub fn exec(name: &str, extras: Option<Vec<&str>>) -> Result<(), failure::Error> {
  let cfg = config::load()?;
  let entry: &Alias = match cfg.get(name) {
    Some(value) => value,
    None => return Ok(()),
  };

  if !can_execute(&entry.condition) {
    eprintln!("command could not executable because condition returns fails");
    return Ok(()); // Are you sure?
  }

  return run_command(&entry.command, extras);
}

fn can_execute(condition: &Option<String>) -> bool {
  if let Some(condition) = condition {
    return match Command::new("sh").args(&["-c", &condition]).output() {
      Ok(value) => value.status.success(),
      Err(_) => false,
    };
  }

  return true;
}

fn run_command(command: &str, extras: Option<Vec<&str>>) -> Result<(), failure::Error> {
  let mut execute: String = command.to_string();
  let extras = match extras {
    Some(value) => value.join(" "),
    None => "".to_string(),
  };
  execute.push_str(" ");
  execute.push_str(&extras);

  // execute as `sh -c COMMAND+EXTRAS`
  return match Command::new("sh").args(&["-c", &execute.trim()]).status() {
    Ok(_) => Ok(()),
    Err(e) => {
      let msg = failure::err_msg(format!("command failed: {}", e));
      return Err(msg);
    }
  };
}
