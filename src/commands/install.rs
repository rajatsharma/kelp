use anyhow::Result;
use clap::{App, Arg, ArgMatches, SubCommand};
use std::path::Path;
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

pub fn run(matches: &ArgMatches) -> Result<()> {
    if let Some(matches) = matches.subcommand_matches(SUBCOMMAND_NAME) {
        let home_dir = std::env::var("HOME")?;
        let plugin = matches.value_of("PLUGIN").unwrap();
        let plugin_url = format!("https://github.com/{}", plugin);
        let plugin_name: Vec<&str> = plugin.split('/').collect();

        let plugin_dir = format!("{}/.kelp/{}", home_dir, plugin_name[1]);
        let init_file = format!("{}/init.fish", plugin_dir);

        Command::new("git")
            .current_dir(format!("{}/.kelp", home_dir))
            .arg("clone")
            .arg(&plugin_url)
            .spawn()?
            .wait()?;

        if Path::new(&init_file).exists() {
            Command::new("fish")
                .current_dir(&plugin_dir)
                .arg("-c")
                .arg("source init.fish")
                .spawn()?
                .wait()?;
        };

        Ok(())
    } else {
        Ok(())
    }
}
