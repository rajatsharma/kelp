use clap::{App, Arg, ArgMatches, SubCommand};
use std::process::Command;

const SUBCOMMAND_NAME: &'static str = "install";

pub fn command() -> App<'static, 'static> {
    SubCommand::with_name(SUBCOMMAND_NAME)
        .about("Install Fish plugin")
        .version("1.0")
        .author("Rajat Sharma")
        .arg(
            Arg::with_name("PLUGIN")
                .help("Plugin to install eg. github_username/fish_plugin")
                .required(true)
                .index(1),
        )
}

pub fn run(matches: &ArgMatches) {
    if let Some(matches) = matches.subcommand_matches(SUBCOMMAND_NAME) {
        let home_dir = std::env::var("HOME").unwrap();
        let plugin = matches.value_of("PLUGIN").unwrap();
        let plugin_url = format!("https://github.com/{}", plugin);
        let _plugin_name: Vec<&str> = plugin.split('/').collect();

        Command::new("git")
            .current_dir(format!("{}/.kelp", home_dir))
            .arg("clone")
            .arg(&plugin_url)
            .spawn()
            .unwrap()
            .wait()
            .unwrap();
    }
}
