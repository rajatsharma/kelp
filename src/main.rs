mod commands;
use clap::{App, Arg};

use commands::commit;
fn main() {
    let matches = App::new("Git")
        .version("1.0")
        .author("Linus Trovalds")
        .about("Version Control System")
        .arg(Arg::with_name("debug").short("d").help("see more logs"))
        .subcommand(commit::command())
        .get_matches();

    commit::run(&matches);
    // Call other commands here
}
