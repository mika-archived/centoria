extern crate exitfailure;
extern crate failure;

use exitfailure::ExitFailure;

fn main() -> Result<(), ExitFailure> {
    return Ok(run()?);
}

fn run() -> Result<(), failure::Error> {
    return Ok(());
}
