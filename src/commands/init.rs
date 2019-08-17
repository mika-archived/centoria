use std::process::Command;

use crate::config::{self, Alias};

pub fn init() -> Result<(), failure::Error> {
    let cfg = config::load()?;

    for (key, value) in cfg.iter() {
        if is_supported(&value) {
            println!("alias {0}='{1}'", key, value.command);
        }
    }

    return Ok(());
}

fn is_supported(alias: &Alias) -> bool {
    if let Some(condition) = &alias.condition {
        return match Command::new("sh").args(&["-c", condition]).output() {
            Ok(value) => value.status.success(),
            Err(_) => false,
        };
    }

    return true;
}
