use std::collections::BTreeMap;
use std::io::Write;
use std::process::{Command, ExitStatus};

use clap::ArgMatches;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

use crate::argparse::ArgParser;
use crate::executors::Executor;
use crate::fmt;
use crate::pad;
use crate::runner;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SubCommand {
    command: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    condition: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    shell: Option<String>,

    subcommands: BTreeMap<String, Function>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Function {
    command: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    descriptions: Option<Vec<String>>, // description for arguments
}

impl SubCommand {
    pub fn new(
        program: &str,
        name: &str,
        command: &str,
        condition: Option<&str>,
        description: Option<&str>,
        shell: Option<&str>,
    ) -> SubCommand {
        let condition = condition.map(|s| s.to_owned());
        let description = description.map(|s| s.to_owned());
        let shell = shell.map(|s| s.to_owned());
        let mut subcommands = BTreeMap::new();
        subcommands.insert(
            name.to_owned(),
            Function {
                command: command.to_owned(),
                description,
                descriptions: None,
            },
        );

        SubCommand {
            command: program.to_owned(),
            condition,
            description: None,
            shell,
            subcommands,
        }
    }

    pub fn get(&self, name: &str) -> Option<&Function> {
        self.subcommands.get(name)
    }

    pub fn exists(&self, name: &str) -> bool {
        self.subcommands.get(name).is_some()
    }

    pub fn add(&mut self, executor: SubCommand) -> Result<(), failure::Error> {
        for (key, value) in executor.subcommands.iter() {
            if self.exists(key) {
                let msg = format!("sub-function `{}` is already exists in this function", key);
                return Err(failure::err_msg(msg));
            }

            let description = value.description.as_ref().map(|s| s.to_owned());
            let descriptions = value.descriptions.as_ref().cloned();

            self.subcommands.insert(
                key.to_owned(),
                Function {
                    command: value.command.to_owned(),
                    description,
                    descriptions,
                },
            );
        }

        Ok(())
    }

    pub fn remove(&mut self, name: &str) -> Result<(), failure::Error> {
        if self.exists(name) {
            self.subcommands.remove(name);
            return Ok(());
        }

        let msg = format!("sub-function `{}` is not exists in this function", name);
        Err(failure::err_msg(msg))
    }

    pub fn has_subcommands(&self) -> bool {
        !self.subcommands.is_empty()
    }

    fn format_args(&self, arg: &str) -> Result<String, ()> {
        if arg.contains(' ') {
            Ok(format!("\"{}\"", arg))
        } else {
            Ok(arg.to_owned())
        }
    }

    fn run_command(
        &self,
        execute: &str,
        show_verbose: bool,
        dry_run: bool,
    ) -> Result<ExitStatus, failure::Error> {
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

        if dry_run {
            let mut stdout = StandardStream::stdout(ColorChoice::Always);
            let mut clrspc = ColorSpec::new();
            clrspc.set_bold(true).set_fg(Some(Color::Blue));
            stdout.set_color(&clrspc)?;
            write!(&mut stdout, "dry-run")?;

            clrspc.set_bold(false).set_fg(None);
            stdout.set_color(&clrspc)?;
            writeln!(&mut stdout, ": {}", execute.replace("\n", ""))?;
            stdout.flush()?;

            return Err(failure::err_msg("executed as a dry run"));
        }

        runner::safe_run(self.shell(), execute)
    }

    fn shell(&self) -> &str {
        match &self.shell {
            Some(shell) => &shell,
            None => "sh",
        }
    }
}

#[typetag::serde(name = "subcommand")]
impl Executor for SubCommand {
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
                Err(_) => false,
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
        let dry_run = args.is_present("dry_run");

        // run original
        if extra.is_empty() {
            return self.run_command(&self.command, show_verbose, dry_run);
        }

        // subcommand does not assume anything other than the single command.
        if !self.exists(extra.get(0).unwrap()) {
            let mut execute = self.command.to_owned();
            execute.push_str(&format!(" {}", extra.join(" ")));

            return self.run_command(&execute, show_verbose, dry_run);
        }

        // building
        let mut execute = self.command.to_owned();
        let executor = self.get(extra.get(0).unwrap()).unwrap();
        let mut parser = ArgParser::new(&executor.command, None);
        parser.parse()?;

        if parser.has_arguments()? {
            let extra = match extra.get(1..) {
                Some(value) => value.iter().map(|s| s.to_string()).collect::<Vec<String>>(),
                None => vec![],
            };
            execute.push_str(&format!(" {}", parser.fill(extra)?));
        } else {
            execute.push_str(&format!(" {}", &executor.command));

            if let Some(extra) = extra.get(1..) {
                let extra = extra.iter().map(|s| s.to_string()).collect::<Vec<String>>();
                execute.push_str(&format!(" {}", extra.join(" ")));
            }
        }

        self.run_command(&execute, show_verbose, dry_run)
    }

    fn display(&self, args: &ArgMatches) -> Result<(), failure::Error> {
        let name = args.value_of("name").unwrap();
        let sub = args.value_of("subcommand");

        if let Some(sub) = sub {
            let subcommand = self.subcommands.get(sub);
            return match subcommand {
                Some(value) => value.display(&name, &sub),
                None => {
                    let msg = format!("subcommand `{}` is not exists in this function", sub);
                    Err(failure::err_msg(msg))
                }
            };
        }

        let description = match &self.description {
            Some(value) => value,
            None => "No description provided",
        };

        let (longest, _) = self.subcommands.iter().max_by_key(|x| x.0.len()).unwrap();
        let subcommands = self
            .subcommands
            .iter()
            .map(|(key, value)| {
                let description = match &value.description {
                    Some(value) => fmt::to_single_line(value),
                    None => "No description provided".to_owned(),
                };
                format!("{} : {}", pad::right_pad(key, longest.len()), description)
            })
            .collect::<Vec<String>>();

        println!(
            "\
Usage (Cet)    : cet exec {name} -- <EXTRA ARGS>
Usage (Direct) : {name} <EXTRA ARGS>
Wrapped        : {command}
Shell          : {shell}

{description}

SubCommands (show details of subcommand, pass `-s <name>`):
{subcommands}",
            name = name,
            description = description,
            command = pad::left_pad_without_1st(&self.command, 17),
            shell = self.shell(),
            subcommands = subcommands
                .iter()
                .map(|w| format!("    {}", w))
                .collect::<Vec<String>>()
                .join("\n")
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

impl Function {
    fn display(&self, parent: &str, myself: &str) -> Result<(), failure::Error> {
        let description = match &self.description {
            Some(value) => value,
            None => "No description provided",
        };
        let descriptions = match &self.descriptions {
            Some(values) => Some(values.iter().map(|s| s.as_str()).collect()),
            None => None,
        };
        let mut parser = ArgParser::new(&self.command, descriptions);
        parser.parse()?;

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
Usage (Cet)    : cet exec {parent} -- {myself} <EXTRA ARGS>
Usage (Direct) : {parent} {myself} <EXTRA ARGS>
Wrapped        : {parent} {command}",
            parent = parent,
            myself = myself,
            command = self.command,
        );

        if !parameters.is_empty() {
            println!(
                "\
Parameters     :
{parameters}",
                parameters = parameters
                    .iter()
                    .map(|w| format!("    {}", w))
                    .collect::<Vec<String>>()
                    .join("\n"),
            );
        }

        println!("\n{description}", description = description.trim(),);

        Ok(())
    }
}
