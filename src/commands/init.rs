use clap::{App, ArgMatches, SubCommand};
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Write;

const SUBCOMMAND_NAME: &'static str = "init";

pub fn command() -> App<'static, 'static> {
    SubCommand::with_name(SUBCOMMAND_NAME)
        .about("Initialise Kelp")
        .version("1.0")
        .author("Rajat Sharma")
}

pub fn run(matches: &ArgMatches) {
    if let Some(_matches) = matches.subcommand_matches(SUBCOMMAND_NAME) {
        let home_dir = std::env::var("HOME").unwrap();

        let mut master_index = File::create(format!("{}/.kelp/index.fish", home_dir)).unwrap();

        write!(
            master_index,
            "for dir in */; [ -r \"$dir/index.fish\" ] && [ -f \"$dir/index.fish\" ] && source \"$dir/index.fish\"; end"
        )
        .unwrap();

        let mut fish_config_file = OpenOptions::new()
            .write(true)
            .append(true)
            .open(format!("{}/.config/fish/config.fish", home_dir))
            .unwrap();

        writeln!(fish_config_file, "# Added by Kelp Start ---").unwrap();
        writeln!(fish_config_file, "source \"$HOME/.kelp/index.fish\"").unwrap();
        writeln!(fish_config_file, "# Added by Kelp End ---").unwrap();
    }
}
