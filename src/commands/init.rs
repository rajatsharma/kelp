use anyhow::Result;
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

pub fn run(matches: &ArgMatches) -> Result<()> {
    if let Some(_matches) = matches.subcommand_matches(SUBCOMMAND_NAME) {
        let home_dir = std::env::var("HOME")?;

        let mut master_index = File::create(format!("{}/.kelp/index.fish", home_dir))?;

        writeln!(
            master_index,
            "for file in ~/.kelp/*/{{conf.d,functions,completions}}/*.fish; [ -r \"$file\" ] && [ -f \"$file\" ] && source \"$file\"; end"
        )?;

        writeln!(
            master_index,
            "for file in ~/.kelp/*/init.fish; [ -r \"$file\" ] && [ -f \"$file\" ] && source \"$file\"; end;"
        )?;

        let mut fish_config_file = OpenOptions::new()
            .write(true)
            .append(true)
            .open(format!("{}/.config/fish/config.fish", home_dir))?;

        writeln!(fish_config_file, "# Added by Kelp Start ---")?;
        writeln!(fish_config_file, "source \"$HOME/.kelp/index.fish\"")?;
        writeln!(fish_config_file, "# Added by Kelp End ---")?;

        Ok(())
    } else {
        Ok(())
    }
}
