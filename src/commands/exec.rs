use clap::ArgMatches;

use crate::config::Config;

pub fn exec(args: &ArgMatches) -> Result<(), failure::Error> {
    let cfg = Config::load()?;
    let name = args.value_of("name").unwrap();
    let extra = args.values_of("extra").map(|w| w.collect());

    let function = match cfg.get(name) {
        Some(value) => value,
        None => {
            let msg = format!("function name `{}` is not exists", name);
            return Err(failure::err_msg(msg));
        }
    };

    if !function.can_execute() {
        let msg = format!("could not execute the function `{}`", name);
        return Err(failure::err_msg(msg));
    }

    function.execute(extra)?;
    return Ok(());
}
