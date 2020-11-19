use clap::{App, Arg, ArgMatches, SubCommand};

pub fn command() -> App<'static, 'static> {
    SubCommand::with_name("install")
        .about("Install Fish plugin")
        .version("1.0")
        .author("Rajat Sharma")
        .arg(
            Arg::with_name("message")
                .short("m")
                .help("commit message")
                .takes_value(true)
                .required(true),
        )
}

pub fn run(matches: &ArgMatches) {
    if let Some(matches) = matches.subcommand_matches("commit") {
        println!("Committing with: {}", matches.value_of("message").unwrap())
    }
}
