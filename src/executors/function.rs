use std::io::Write;
use std::process::{Command, ExitStatus};

use clap::ArgMatches;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

use crate::argparse::ArgParser;
use crate::executors::Executor;
use crate::fmt;
use crate::pad;
use crate::shell;

/**
 * function works as shell functions
 */
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Function {
    command: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    condition: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    cwd: Option<String>,

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
        cwd: Option<&str>,
        description: Option<&str>,
        shell: Option<&str>,
    ) -> Function {
        let condition = condition.map(|s| s.to_owned());
        let cwd = cwd.map(|s| s.to_owned());
        let description = description.map(|s| s.to_owned());
        let shell = shell.map(|s| s.to_owned());

        Function {
            command: command.to_owned(),
            condition,
            cwd,
            description,
            descriptions: None,
            shell,
        }
    }

    fn shell(&self) -> &str {
        match &self.shell {
            Some(shell) => &shell,
            None => "sh",
        }
    }

    fn format_args(&self, arg: &str) -> Result<String, ()> {
        if arg.contains(' ') {
            Ok(format!("\"{}\"", arg))
        } else {
            Ok(arg.to_owned())
        }
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

        true
    }

    fn execute(&self, args: &ArgMatches) -> Result<ExitStatus, failure::Error> {
        let extra: Vec<String> = args.values_of("extra").map_or_else(
            || vec![],
            |w| w.map(|v| self.format_args(v).unwrap()).collect(),
        );
        let show_verbose = args.is_present("verbose");

        // building
        let mut parser = ArgParser::new(&self.command, None);
        parser.parse()?;

        let execute = match parser.fill(extra) {
            Ok(value) => value,
            Err(e) => {
                let msg = format!(
                    "{}, please use `show` subcommand for checking parameters",
                    e
                );
                return Err(failure::err_msg(msg));
            }
        };

        if show_verbose {
            let mut stdout = StandardStream::stdout(ColorChoice::Always);
            let mut clrspc = ColorSpec::new();
            clrspc.set_bold(true).set_fg(Some(Color::Green));
            stdout.set_color(&clrspc)?;
            write!(&mut stdout, "Executing")?;

            clrspc.set_bold(false).set_fg(None);
            stdout.set_color(&clrspc)?;
            writeln!(&mut stdout, ": {}", execute.replace("\n", ""))?;
            stdout.flush()?;
        }

        let cwd = match &self.cwd {
            Some(value) => Some(value.to_string()),
            None => None,
        };

        shell::safe_run(self.shell(), execute.trim(), cwd)
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
            Some(value) => fmt::to_single_line(value),
            None => "No description provided".to_owned(),
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
                        description = fmt::to_single_line(w.description())
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

{description}",
            name = name,
            parameters = parameters
                .iter()
                .map(|w| format!("    {}", w))
                .collect::<Vec<String>>()
                .join("\n"),
            description = description.trim(),
            command = pad::left_pad_without_1st(&self.command, 17),
            shell = self.shell(),
        );

        Ok(())
    }

    fn export_as(&self, name: &str) -> Result<String, failure::Error> {
        Ok(format!(
            "alias {name}='cet exec {name} -- '",
            name = name.to_owned()
        ))
    }

    fn description(&self) -> &str {
        match &self.description {
            Some(value) => value,
            None => "No description provided",
        }
    }
}
