mod commands;
use clap::{App, Arg};

use commands::install;
fn main() {
    let matches = App::new("Kelp")
        .version("1.0")
        .author("Rajat Sharma")
        .about("Plugin Manager for Fish Shell")
        .arg(Arg::with_name("debug").short("d").help("see more logs"))
        .subcommand(install::command())
        .get_matches();

    install::run(&matches);
    // Call other commands here
}
