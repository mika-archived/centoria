use std::process::ExitStatus;

use clap::ArgMatches;

use crate::config::Config;

pub fn exec(args: &ArgMatches) -> Result<ExitStatus, failure::Error> {
    let cfg = Config::load()?;
    let name = args.value_of("name").unwrap();

    let executor = match cfg.get(name) {
        Some(value) => value,
        None => {
            let msg = format!("function name `{}` is not exists", name);
            return Err(failure::err_msg(msg));
        }
    };

    if !executor.can_execute() {
        let msg = format!("could not execute the function `{}`", name);
        return Err(failure::err_msg(msg));
    }

    Ok(executor.execute(args)?)
}
