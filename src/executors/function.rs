use std::process::{Command, ExitStatus};

use clap::ArgMatches;

use crate::argparse::ArgParser;
use crate::executors::Executor;

/**
 * function works as shell functions
 */
#[derive(Serialize, Deserialize, Debug)]
pub struct Function {
    command: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    condition: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    shell: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    descriptions: Option<Vec<String>>, // description for arguments
}

impl Function {
    pub fn new(
        command: &str,
        condition: Option<&str>,
        description: Option<&str>,
        shell: Option<&str>,
    ) -> Function {
        let condition = condition.map(|s| s.to_owned());
        let description = description.map(|s| s.to_owned());
        let shell = shell.map(|s| s.to_owned());

        return Function {
            command: command.to_owned(),
            condition,
            description,
            descriptions: None,
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

#[typetag::serde(name = "function")]
impl Executor for Function {
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
        let extra: Vec<&str> = args
            .values_of("extra")
            .map_or_else(|| vec![], |w| w.collect());

        // building
        let mut parser = ArgParser::new(&self.command, None);
        parser.parse()?;

        let execute = match parser.fill(extra) {
            Ok(value) => value,
            Err(_) => {
                let msg = "required parameter(s) is missing, please use `show` subcommand for checking parameters";
                return Err(failure::err_msg(msg));
            }
        };

        #[rustfmt::skip]
        match Command::new(self.shell()).args(&["-c", &execute.trim()]).status() {
            Ok(status) => return Ok(status),
            Err(e) => {
                let msg = failure::err_msg(format!("function failed: {}", e));
                return Err(msg);
            }
        };
    }

    fn display(&self, args: &ArgMatches) -> Result<(), failure::Error> {
        let descriptions = match &self.descriptions {
            Some(values) => Some(values.iter().map(|s| s.as_str()).collect()),
            None => None,
        };
        let mut parser = ArgParser::new(&self.command, descriptions);
        parser.parse()?;

        let name = args.value_of("name").unwrap();
        let description = match &self.description {
            Some(value) => value,
            None => "No description provided",
        };
        let parameters = match parser.arguments() {
            Some(values) => values
                .iter()
                .enumerate()
                .map(|(i, w)| {
                    format!(
                        "{index} ({opt}): {description}",
                        index = i,
                        opt = w.attribute(),
                        description = w.description()
                    )
                })
                .collect::<Vec<String>>(),
            None => vec!["No description provided".to_owned()],
        };

        println!(
            "\
Usage (Cet)    : cet exec {name} -- <EXTRA ARGS>
Usage (Direct) : {name} <EXTRA ARGS>
Execute        : {command}
Shell          : {shell}
Parameters     : 
{parameters}

{description}\
        ",
            name = name,
            parameters = parameters
                .iter()
                .map(|w| format!("    {}", w))
                .collect::<Vec<String>>()
                .join("\n"),
            description = description,
            command = self.command,
            shell = self.shell(),
        );
        return Ok(());
    }
}
