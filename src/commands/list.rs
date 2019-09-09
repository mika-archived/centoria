use crate::config::Config;
use crate::pad;

pub fn list() -> Result<(), failure::Error> {
    let config = Config::load()?;
    let entries = config.keys();

    let longest = &entries.iter().max_by_key(|w| w.len()).unwrap();
    for entry in &entries {
        let description = match config.get(&entry) {
            Some(value) => value.description(),
            None => return Err(failure::err_msg("invalid operation")),
        };

        println!(
            "{} : {}",
            pad::right_pad(&entry, longest.len()),
            description
        );
    }

    Ok(())
}
