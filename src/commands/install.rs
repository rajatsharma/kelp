use clap::{App, Arg, ArgMatches, SubCommand};
use std::fs::File;
use std::io::Write;
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
        let plugin_name: Vec<&str> = plugin.split('/').collect();

        Command::new("git")
            .current_dir(format!("{}/.kelp", home_dir))
            .arg("clone")
            .arg(&plugin_url)
            .spawn()
            .unwrap()
            .wait()
            .unwrap();

        let mut plugin_index_file =
            File::create(format!("{}/.kelp/{}/index.fish", home_dir, plugin_name[1])).unwrap();

        writeln!(
            plugin_index_file,
            "for file in ~/.kelp/{}/{{completions,functions}}/*.fish; [ -r \"$file\" ] && [ -f \"$file\" ] && source \"$file\";end",
            plugin_name[1]
        ).unwrap();

        writeln!(
            plugin_index_file,
            "source \"$HOME/.kelp/{}/init.fish\"",
            plugin_name[1]
        )
        .unwrap();
    }
}
