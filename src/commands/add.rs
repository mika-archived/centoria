use clap::ArgMatches;

use crate::config::Config;
use crate::executors::{Alias, Executor};

pub fn add(args: &ArgMatches) -> Result<(), failure::Error> {
    let mut cfg = Config::load()?;
    let name = args.value_of("name").unwrap();

    cfg.add(name, construct(args))?;
    cfg.save()?;

    return Ok(());
}

fn construct(args: &ArgMatches) -> Box<dyn Executor> {
    let command = args.value_of("command").unwrap();
    let condition = args.value_of("condition");
    let description = args.value_of("description");
    let shell = args.value_of("shell");

    return Box::new(Alias::new(&command, condition, description, shell));
}
