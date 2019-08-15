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

    if let Some(matches) = matches.subcommand_matches("add") {
        let name = matches.value_of("name").unwrap();
        let command = matches.value_of("command").unwrap();

        commands::add(name, command)?;
    } else if let Some(matches) = matches.subcommand_matches("remove") {
        let name = matches.value_of("name").unwrap();

        commands::remove(name)?;
    }

    return Ok(());
}
