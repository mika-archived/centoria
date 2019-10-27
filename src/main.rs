#[macro_use]
extern crate clap;
#[macro_use]
extern crate downcast_rs;
#[macro_use]
extern crate serde_derive;

mod app;
mod argparse;
mod commands;
mod config;
mod executors;
mod fmt;
mod pad;
mod shell;

use std::process::exit;

use exitfailure::ExitFailure;

fn main() -> Result<(), ExitFailure> {
    Ok(run()?)
}

fn run() -> Result<(), failure::Error> {
    let matches = app::build_app().get_matches();

    match matches.subcommand() {
        ("init", Some(_)) => {
            commands::init()?;
        }
        ("add", Some(matches)) => {
            commands::add(matches)?;
        }
        ("remove", Some(matches)) => {
            commands::remove(matches)?;
        }
        ("exec", Some(matches)) => {
            let status = commands::exec(matches)?;
            if let Some(code) = status.code() {
                exit(code);
            }
        }
        ("list", Some(_)) => {
            commands::list()?;
        }
        ("show", Some(matches)) => {
            commands::show(matches)?;
        }
        _ => {
            let msg = "subcommand is required";
            return Err(failure::err_msg(msg));
        }
    };

    Ok(())
}
