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
mod function;

use exitfailure::ExitFailure;

fn main() -> Result<(), ExitFailure> {
    return Ok(run()?);
}

fn run() -> Result<(), failure::Error> {
    let matches = app::build_app().get_matches();

    if let Some(_) = matches.subcommand_matches("init") {
        commands::init()?;
    } else if let Some(matches) = matches.subcommand_matches("add") {
        commands::add(matches)?;
    } else if let Some(matches) = matches.subcommand_matches("remove") {
        commands::remove(matches)?;
    } else if let Some(matches) = matches.subcommand_matches("exec") {
        commands::exec(matches)?;
    }

    return Ok(());
}
