use clap::{App, Arg, SubCommand};

pub fn build_app() -> App<'static, 'static> {
     App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .subcommand(SubCommand::with_name("init").about("initialize centoria for shell"))
        .subcommand(
            SubCommand::with_name("add")
                .about("add a function to centoria")
                .arg(
                    Arg::with_name("name")
                        .help("unique function name")
                        .required(true)
                        .index(1),
                )
                .arg(
                    Arg::with_name("command")
                        .help("command(s) to execute with this function")
                        .required(true)
                        .index(2),
                )
                .arg(
                    Arg::with_name("condition")
                        .short("c")
                        .long("condition")
                        .value_name("command")
                        .help("conditional statements for applying this function")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("description")
                        .short("d")
                        .long("description")
                        .help("description of this function")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("shell")
                        .short("s")
                        .long("shell")
                        .help("shell program that executes the function")
                        .default_value("sh")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("program")
                        .short("p")
                        .long("program")
                        .help("original command that treats this function as subcommand")
                        .takes_value(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("remove")
                .about("remove a function from centoria")
                .arg(
                    Arg::with_name("name")
                        .help("name of the function you want to remove")
                        .required(true)
                        .index(1),
                )
                .arg(
                    Arg::with_name("program")
                        .short("p")
                        .long("program")
                        .help("original command that treats this function as subcommand")
                        .takes_value(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("exec")
                .about("execute function as proxy of centoria")
                .arg(
                    Arg::with_name("name")
                        .help("name of the function to execute")
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
        )
        .subcommand(SubCommand::with_name("list").about("list all functions"))
        .subcommand(
            SubCommand::with_name("show")
                .about("show function details")
                .arg(
                    Arg::with_name("name")
                        .help("name of the function to display")
                        .required(true)
                        .index(1),
                )
                .arg(
                    Arg::with_name("subcommand")
                        .short("s")
                        .long("sub")
                        .help("name of the subcommand to display (if you pass this argument to alias or function, ignored this)")
                        .takes_value(true),
                ),
        )
}
