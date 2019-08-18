use std::process::Command;

#[derive(Serialize, Deserialize, Debug)]
pub struct Function {
    command: String,
    condition: Option<String>,
    shell: Option<String>,
}

impl Function {
    pub fn new(command: &str, condition: Option<&str>, shell: Option<&str>) -> Function {
        let condition = condition.map(|s| s.to_string());
        let shell = shell.map(|s| s.to_string());

        return Function {
            command: command.to_string(),
            condition,
            shell,
        };
    }

    pub fn can_execute(&self) -> bool {
        if self.shell() != "sh" {
            match Command::new(self.shell()).arg("--version").status() {
                Ok(_) => {}
                Err(_) => return false,
            }
        }

        if let Some(condition) = &self.condition {
            #[rustfmt::skip]
            return match Command::new(self.shell()).args(&["-c", &condition]).output() {
                Ok(value) => value.status.success(),
                Err(_) => false,
            };
        }

        return true;
    }

    pub fn execute(&self, extra: Option<Vec<&str>>) -> Result<(), failure::Error> {
        let mut execute = self.command.to_string();
        if let Some(extra) = extra {
            execute.push_str(&format!(" {}", extra.join(" ")));
        }

        #[rustfmt::skip]
        match Command::new(self.shell()).args(&["-c", &execute.trim()]).status() {
            Ok(_) => return Ok(()),
            Err(e) => {
                let msg = failure::err_msg(format!("function failed: {}", e));
                return Err(msg);
            }
        };
    }

    fn shell(&self) -> &str {
        return match &self.shell {
            Some(shell) => &shell,
            None => "sh",
        };
    }
}
