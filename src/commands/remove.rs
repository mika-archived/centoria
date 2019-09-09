use clap::ArgMatches;

use crate::config::Config;

pub fn remove(args: &ArgMatches) -> Result<(), failure::Error> {
    let mut cfg = Config::load()?;
    let name = args.value_of("name").unwrap();
    let program = args.value_of("program");

    cfg.remove(&name, program)?;
    cfg.save()?;

    Ok(())
}
