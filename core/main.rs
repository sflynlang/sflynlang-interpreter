pub mod utils;

use clap::{App, Arg};
use std::{fs, path::Path};

fn main() {
    let matches = App::new("Sflynlang")
        .about("High-level programming language.")
        .version("0.1.0")
        .author("Sflynlang Contributors")
        .arg(Arg::new("FILE").index(1).required(true))
        .arg(Arg::new("debug").short('d').long("debug"))
        .get_matches();

    let mut exit_code = 0;

    if let Some(run_file) = matches.value_of("FILE") {
        let file_name =
            format!("{}/{}", utils::get_current_directory(), run_file);
        let file_path = Path::new(&file_name);

        if !file_path.exists()
            || !file_path.is_file()
            || !file_name.ends_with(".sf")
        {
            println!("This path is not a Sflynlang file.");
            exit_code = 1;
        } else if let Ok(file_content) = fs::read_to_string(file_name) {
            let file =
                sflynlang_parser::File::new(run_file.to_string(), file_content);

            if let Some(statements) = sflynlang_parser::run(&file) {
                exit_code = sflynlang_compiler::run(
                    statements,
                    matches.is_present("debug"),
                    &file,
                );
            } else {
                exit_code = 1;
            }
        } else {
            println!("Cannot read `{}` file.", run_file);
            exit_code = 1;
        }
    }

    std::process::exit(exit_code);
}
