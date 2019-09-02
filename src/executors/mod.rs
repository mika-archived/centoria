use std::process::ExitStatus;

use clap::ArgMatches;
use downcast_rs::Downcast;

mod alias;
mod function;
mod subcommand;

pub use alias::Alias;
pub use function::Function;
pub use subcommand::SubCommand;

#[typetag::serde(tag = "runas")]
pub trait Executor: Downcast {
    fn can_execute(&self) -> bool;
    fn description(&self) -> &str;
    fn display(&self, args: &ArgMatches) -> Result<(), failure::Error>;
    fn execute(&self, args: &ArgMatches) -> Result<ExitStatus, failure::Error>;
    fn export_as(&self, name: &str) -> Result<String, failure::Error>;
}

impl_downcast!(Executor);
