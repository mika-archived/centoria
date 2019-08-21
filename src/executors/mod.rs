use std::process::ExitStatus;

use clap::ArgMatches;

mod alias;

pub use alias::Alias;

#[typetag::serde(tag = "runas")]
pub trait Executor {
    fn can_execute(&self) -> bool;
    fn execute(&self, args: &ArgMatches) -> Result<ExitStatus, failure::Error>;
}
