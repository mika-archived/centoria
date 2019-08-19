use clap::ArgMatches;

use crate::config::Config;
use crate::function::Function;

pub fn add(args: &ArgMatches) -> Result<(), failure::Error> {
    let mut cfg = Config::load()?;
    let name = args.value_of("name").unwrap();
    let command = args.value_of("command").unwrap();
    let condition = args.value_of("condition");
    let shell = args.value_of("shell");

    cfg.add(&name, Function::new(&command, condition, shell))?;
    cfg.save()?;

    return Ok(());
}
