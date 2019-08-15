#[macro_use]
extern crate clap;
extern crate exitfailure;
extern crate failure;

use exitfailure::ExitFailure;

fn main() -> Result<(), ExitFailure> {
    return Ok(run()?);
}

fn run() -> Result<(), failure::Error> {
    let matches = app::build_app().get_matches();

    return Ok(());
}
