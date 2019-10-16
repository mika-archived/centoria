use std::process::{Command, ExitStatus};

// run command without existing aliases
pub fn safe_run(shell: &str, command: &str) -> Result<ExitStatus, failure::Error> {
    match Command::new(shell)
        .args(&["-c", &format!("command {}", command)])
        .status()
    {
        Ok(status) => Ok(status),
        Err(e) => {
            let msg = failure::err_msg(format!("function failed: {}", e));
            Err(msg)
        }
    }
}
