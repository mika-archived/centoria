use crate::config::Config;

pub fn init() -> Result<(), failure::Error> {
    let config = Config::load()?;
    for entry in config.keys() {
        let export = match config.get(&entry) {
            Some(value) => value.export_as(&entry)?,
            None => return Err(failure::err_msg("invalid operation")),
        };

        println!("{}", export);
    }

    return Ok(());
}
