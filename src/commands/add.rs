use clap::ArgMatches;

use crate::config::Config;
use crate::function::Function;

pub fn add(args: &ArgMatches) -> Result<(), failure::Error> {
    let mut cfg = Config::load()?;
    let name = args.value_of("name").unwrap();
    let command = args.value_of("command").unwrap();
    let condition = args.value_of("condition");
    let description = args.value_of("description");
    let shell = args.value_of("shell");

    let function = Function::new(&command, condition, description, shell);
    cfg.add(name, function)?;
    cfg.save()?;

    return Ok(());
}
