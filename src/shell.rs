use std::process::{Command, ExitStatus};

// run command without existing aliases
pub fn safe_run(
    shell: &str,
    command: &str,
    cwd: Option<String>,
) -> Result<ExitStatus, failure::Error> {
    let command = create_command(shell, command, cwd);

    match Command::new(shell).args(&["-c", &command]).status() {
        Ok(status) => Ok(status),
        Err(e) => {
            let msg = failure::err_msg(format!("function failed: {}", e));
            Err(msg)
        }
    }
}

fn create_command(shell: &str, command: &str, cwd: Option<String>) -> String {
    match cwd {
        Some(value) => format!(
            "cd {} {} command {}",
            value,
            chain_operator(shell).unwrap(),
            command
        ),
        None => format!("command {}", command),
    }
}

fn chain_operator(shell: &str) -> Result<String, failure::Error> {
    match shell {
        "sh" => Ok("&&".to_owned()),
        "bash" => Ok("&&".to_owned()),
        "zsh" => Ok("&&".to_owned()),
        "fish" => Ok("; and".to_owned()),
        _ => {
            let msg = failure::err_msg(format!("not supported shell: {}", shell));
            Err(msg)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{chain_operator, create_command};

    #[test]
    fn chain_operator_test() {
        assert_eq!(chain_operator("sh").unwrap(), "&&");
        assert_eq!(chain_operator("bash").unwrap(), "&&");
        assert_eq!(chain_operator("zsh").unwrap(), "&&");
        assert_eq!(chain_operator("fish").unwrap(), "; and");
        assert_eq!(
            chain_operator("ash").unwrap_or_else(|err| err.to_string()),
            "not supported shell: ash"
        );
    }

    #[test]
    fn create_command_test_for_cwd_is_none() {
        assert_eq!(create_command("bash", "ls -al", None), "command ls -al");
    }

    #[test]
    fn create_command_test_for_cwd_is_not_none() {
        assert_eq!(
            create_command("sh", "ls -al", Some("/path/to/cwd".to_owned())),
            "cd /path/to/cwd && command ls -al"
        );
        assert_eq!(
            create_command("fish", "ls -al", Some("/path/to/cwd".to_owned())),
            "cd /path/to/cwd ; and command ls -al"
        );
    }
}
