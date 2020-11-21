use clap::{App, Arg, ArgMatches, SubCommand};
use std::fs::remove_dir_all;
use std::path::Path;
use std::process::Command;

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
        let plugin_dir = format!("{}/.kelp/{}", home_dir, plugin_name[1]);

        let uninstall_file = format!("{}/uninstall.fish", plugin_dir);

        if Path::new(&uninstall_file).exists() {
            Command::new("fish")
                .current_dir(&plugin_dir)
                .arg("-c")
                .arg("source uninstall.fish")
                .spawn()
                .unwrap()
                .wait()
                .unwrap();
        };

        remove_dir_all(plugin_dir).unwrap();
        println!("Removed: {}", plugin_name[1]);
    }
}
