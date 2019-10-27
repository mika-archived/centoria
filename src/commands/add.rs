use clap::ArgMatches;
use regex::Regex;

use crate::config::Config;
use crate::executors::{Alias, Executor, Function, SubCommand};

pub fn add(args: &ArgMatches) -> Result<(), failure::Error> {
    if args.value_of("program").is_some() {
        add_subcommand(args)?;
    } else {
        add_function(args)?;
    }

    Ok(())
}

fn add_function(args: &ArgMatches) -> Result<(), failure::Error> {
    let mut cfg = Config::load()?;
    let name = args.value_of("name").unwrap();

    cfg.add(name, construct(args))?;
    cfg.save()?;

    Ok(())
}

fn add_subcommand(args: &ArgMatches) -> Result<(), failure::Error> {
    // I don't know the best practice of adding the entry to (de)serialized object
    let mut cfg = Config::load()?;
    let name = args.value_of("name").unwrap();
    let program = args.value_of("program").unwrap();
    let command = args.value_of("command").unwrap();
    let condition = args.value_of("condition");
    let description = args.value_of("description");
    let shell = args.value_of("shell");
    let executor = SubCommand::new(program, name, command, condition, description, shell);

    cfg.add(program, Box::new(executor))?;
    cfg.save()?;

    Ok(())
}

fn construct(args: &ArgMatches) -> Box<dyn Executor> {
    let command = args.value_of("command").unwrap();
    let condition = args.value_of("condition");
    let cwd = args.value_of("cwd");
    let description = args.value_of("description");
    let shell = args.value_of("shell");

    let regex = Regex::new(r"\{\d+(\.\.(\d+)?)?\}").unwrap();
    if regex.is_match(&command) {
        Box::new(Function::new(&command, condition, cwd, description, shell))
    } else {
        Box::new(Alias::new(&command, condition, cwd, description, shell))
    }
}
