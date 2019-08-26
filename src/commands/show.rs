use clap::ArgMatches;

use crate::config::Config;

pub fn show(args: &ArgMatches) -> Result<(), failure::Error> {
    let cfg = Config::load()?;
    let name = args.value_of("name").unwrap();
    let executor = match cfg.get(&name) {
        Some(value) => value,
        None => {
            let msg = format!("function `{}` is not exists", &name);
            return Err(failure::err_msg(msg));
        }
    };
    executor.display(args)?;

    return Ok(());
}
