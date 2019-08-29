use std::collections::BTreeMap;
use std::process::{Command, ExitStatus};

use clap::ArgMatches;

use crate::argparse::ArgParser;
use crate::executors::Executor;
use crate::pad::right_pad;

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

        return SubCommand {
            command: program.to_owned(),
            condition,
            description: None,
            shell,
            subcommands,
        };
    }

    pub fn get(&self, name: &str) -> Option<&Function> {
        return self.subcommands.get(name);
    }

    pub fn exists(&self, name: &str) -> bool {
        return match self.subcommands.get(name) {
            Some(_) => true,
            None => false,
        };
    }

    pub fn add(&mut self, executor: SubCommand) -> Result<(), failure::Error> {
        for (key, value) in executor.subcommands.iter() {
            if self.exists(key) {
                let msg = format!("sub-function `{}` is already exists in this function", key);
                return Err(failure::err_msg(msg));
            }

            let description = value.description.as_ref().map(|s| s.to_owned());
            let descriptions = value.descriptions.as_ref().map(|w: &Vec<String>| w.clone());

            self.subcommands.insert(
                key.to_owned(),
                Function {
                    command: value.command.to_owned(),
                    description,
                    descriptions,
                },
            );
        }

        return Ok(());
    }

    pub fn remove(&mut self, name: &str) -> Result<(), failure::Error> {
        if self.exists(name) {
            self.subcommands.remove(name);
            return Ok(());
        }

        let msg = format!("sub-function `{}` is not exists in this function", name);
        return Err(failure::err_msg(msg));
    }

    pub fn has_subcommands(&self) -> bool {
        return self.subcommands.len() > 0;
    }

    fn run_command(&self, execute: &str) -> Result<ExitStatus, failure::Error> {
        #[rustfmt::skip]
        match Command::new(self.shell()).args(&["-c", execute]).status() {
            Ok(status) => return Ok(status),
            Err(e) => {
                let msg = format!("function failed because {}", e);
                return Err(failure::err_msg(msg));
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

        return true;
    }

    fn execute(&self, args: &ArgMatches) -> Result<ExitStatus, failure::Error> {
        let extra: Vec<&str> = args
            .values_of("extra")
            .map_or_else(|| vec![], |w| w.collect());

        // run original
        if extra.len() == 0 {
            return self.run_command(&self.command);
        }

        // subcommand does not assume anything other than the single command.
        if !self.exists(extra.get(0).unwrap()) {
            let mut execute = self.command.to_owned();
            execute.push_str(&format!(" {}", extra.join(" ")));

            return self.run_command(&execute);
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

        return self.run_command(&execute);
    }

    fn display(&self, args: &ArgMatches) -> Result<(), failure::Error> {
        let name = args.value_of("name").unwrap();
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
                    Some(value) => value.replace("\n", " "), // allow single-liny only in short details
                    None => "No description provided".to_owned(),
                };
                format!("{} : {}", right_pad(key, longest.len()), description)
            })
            .collect::<Vec<String>>();

        println!(
            "\
Usage (Cet)    : cet exec {name} -- <EXTRA ARGS>
Usage (Direct) : {name} <EXTRA ARGS>
Shell          : {shell}
Wrapped        : {command}

{description}

SubCommands (show details of subcommand, pass `-s <name>`):
{subcommands}
        ",
            name = name,
            description = description,
            command = self.command,
            shell = self.shell(),
            subcommands = subcommands
                .iter()
                .map(|w| format!("    {}", w))
                .collect::<Vec<String>>()
                .join("\n")
        );
        return Ok(());
    }

    fn export_as(&self, name: &str) -> Result<String, failure::Error> {
        return Ok(format!(
            "alias {name}='cet exec {name} -- '",
            name = name.to_owned()
        ));
    }
}
