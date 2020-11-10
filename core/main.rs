mod commands;
pub mod structures;
pub mod utils;

use clap::App;

fn main() {
    let matches = App::new("slang")
        .about("Programming language.")
        .version("0.1.0")
        .author("Daniel Solarte <danielsolartech@hotmail.com>")
        .subcommand(commands::init::info())
        .subcommand(commands::start::info())
        .subcommand(commands::add::info())
        .get_matches();

    if let Some(ref matches) = matches.subcommand_matches("init") {
        std::process::exit(commands::init::run(matches));
    } else if let Some(ref matches) = matches.subcommand_matches("start") {
        std::process::exit(commands::start::run(matches));
    } else if let Some(ref matches) = matches.subcommand_matches("add") {
        std::process::exit(commands::add::run(matches));
    }
}
