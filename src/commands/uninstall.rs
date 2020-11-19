use clap::{App, Arg, ArgMatches, SubCommand};
use std::fs::remove_dir_all;

const SUBCOMMAND_NAME: &'static str = "uninstall";

pub fn command() -> App<'static, 'static> {
    SubCommand::with_name(SUBCOMMAND_NAME)
        .about("Uninstall Fish plugin")
        .version("1.0")
        .author("Rajat Sharma")
        .arg(
            Arg::with_name("PLUGIN")
                .help("Plugin to uninstall eg. github_username/fish_plugin")
                .required(true)
                .index(1),
        )
}

pub fn run(matches: &ArgMatches) {
    if let Some(matches) = matches.subcommand_matches(SUBCOMMAND_NAME) {
        let home_dir = std::env::var("HOME").unwrap();
        let plugin = matches.value_of("PLUGIN").unwrap();
        let plugin_name: Vec<&str> = plugin.split('/').collect();

        remove_dir_all(format!("{}/.kelp/{}", home_dir, plugin_name[1])).unwrap();
        println!("Removed: {}", plugin_name[1]);
    }
}
