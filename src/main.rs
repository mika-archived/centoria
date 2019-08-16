#[macro_use]
extern crate clap;
extern crate dirs;
extern crate exitfailure;
extern crate failure;
extern crate serde;
#[macro_use]
extern crate serde_derive;

mod app;
mod commands;
mod config;

use exitfailure::ExitFailure;

fn main() -> Result<(), ExitFailure> {
    return Ok(run()?);
}

fn run() -> Result<(), failure::Error> {
    let matches = app::build_app().get_matches();

    if let Some(_) = matches.subcommand_matches("init") {
        commands::init()?;
    } else if let Some(matches) = matches.subcommand_matches("add") {
        let name = matches.value_of("name").unwrap();
        let command = matches.value_of("command").unwrap();
        let condition = matches.value_of("condition");

        commands::add(name, command, condition)?;
    } else if let Some(matches) = matches.subcommand_matches("remove") {
        let name = matches.value_of("name").unwrap();

        commands::remove(name)?;
    } else if let Some(matches) = matches.subcommand_matches("exec") {
        let name = matches.value_of("name").unwrap();
        let extras: Option<Vec<&str>> = match matches.values_of("extras") {
            Some(value) => Some(value.collect()),
            None => None
        };

        commands::exec(name, extras)?;
    }

    return Ok(());
}
