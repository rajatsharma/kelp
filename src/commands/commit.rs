use clap::{App, Arg, ArgMatches, SubCommand};

pub fn command() -> App<'static, 'static> {
    SubCommand::with_name("commit")
        .about("Commits staged code")
        .version("1.0")
        .author("Linus Trovalds")
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
