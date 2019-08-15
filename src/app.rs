use clap::{App, Arg, SubCommand};

pub fn build_app() -> App<'static, 'static> {
  return App::new(crate_name!())
    .version(crate_version!())
    .author(crate_authors!())
    .about(crate_description!())
    .subcommand(
      SubCommand::with_name("add")
        .about("Register a alias to centoria")
        .arg(
          Arg::with_name("name")
            .help("unique alias/name")
            .required(true)
            .index(1),
        )
        .arg(
          Arg::with_name("command")
            .help("command(s) to execute with this alias")
            .required(true)
            .index(2),
        ),
    );
}
