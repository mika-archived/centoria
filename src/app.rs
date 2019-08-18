use clap::{App, Arg, SubCommand};

pub fn build_app() -> App<'static, 'static> {
    return App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .subcommand(SubCommand::with_name("init").about("Initialize centoria for shell"))
        .subcommand(
            SubCommand::with_name("add")
                .about("add a alias to centoria")
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
                )
                .arg(
                    Arg::with_name("condition")
                        .short("c")
                        .long("condition")
                        .value_name("command")
                        .help("conditional statements for applying this alias")
                        .takes_value(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("remove")
                .about("remove a alias from centoria")
                .arg(
                    Arg::with_name("name")
                        .help("name of the alias you want to remove")
                        .required(true)
                        .index(1),
                ),
        )
        .subcommand(
            SubCommand::with_name("exec")
                .about("execute alias as proxy of centoria")
                .arg(
                    Arg::with_name("name")
                        .help("name of the alias to execute")
                        .required(true)
                        .index(1),
                )
                .arg(
                    Arg::with_name("extra")
                        .help("extra arguments of original command")
                        .multiple(true)
                        .takes_value(true)
                        .last(true),
                ),
        );
}
