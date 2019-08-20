use std::process::{Command, ExitStatus};

use clap::ArgMatches;

use crate::executors::Executor;

/**
 * alias works as shell aliases
 */
#[derive(Serialize, Deserialize, Debug)]
pub struct Alias {
    command: String,

    // toml-rs does not support Option<T> serialization
    #[serde(skip_serializing_if = "Option::is_none")]
    condition: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    shell: Option<String>,
}

impl Alias {
    pub fn new(
        command: &str,
        condition: Option<&str>,
        description: Option<&str>,
        shell: Option<&str>,
    ) -> Alias {
        let condition = condition.map(|s| s.to_owned());
        let description = description.map(|s| s.to_owned());
        let shell = shell.map(|s| s.to_owned());

        return Alias {
            command: command.to_owned(),
            condition,
            description,
            shell,
        };
    }

    fn shell(&self) -> &str {
        return match &self.shell {
            Some(shell) => &shell,
            None => "sh",
        };
    }
}

#[typetag::serde(name = "alias")]
impl Executor for Alias {
    fn can_execute(&self) -> bool {
        if self.shell() != "sh" {
            match Command::new(self.shell()).arg("--version").output() {
                Ok(_) => {}
                Err(_) => return false,
            };
        }

        if let Some(condition) = &self.condition {
            #[rustfmt::skip]
            return match Command::new(self.shell()).args(&["-c", &condition]).output() {
                Ok(value) => value.status.success(),
                Err(_) => false
            };
        }

        return true;
    }

    fn execute(&self, args: &ArgMatches) -> Result<ExitStatus, failure::Error> {
        let extra: Option<Vec<&str>> = args.values_of("extra").map(|w| w.collect());

        let mut execute = self.command.to_string();
        if let Some(extra) = extra {
            execute.push_str(&format!(" {}", extra.join(" ")));
        }

        #[rustfmt::skip]
        match Command::new(self.shell()).args(&["-c", &execute.trim()]).status() {
            Ok(status) => return Ok(status),
            Err(e) => {
                let msg = failure::err_msg(format!("function failed: {}", e));
                return Err(msg);
            }
        };
    }
}
