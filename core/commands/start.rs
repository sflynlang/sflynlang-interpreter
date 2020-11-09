use crate::structures::Sflynlang;
use crate::utils;
use clap::{App, Arg, ArgMatches};
use std::fs;
use std::path::Path;

pub fn info() -> App<'static> {
    App::new("start").about("Execute the current project.").arg(
        Arg::new("debug")
            .short('d')
            .long("debug")
            .about("Execute the current project in debug mode."),
    )
}

pub fn run(matches: &ArgMatches) -> i32 {
    let current_directory = utils::get_current_directory();
    let slang_rute = format!("{}/sflynlang.yml", current_directory);

    let slang_path = Path::new(&slang_rute);

    if !slang_path.exists() || !slang_path.is_file() {
        println!("This path is not a Sflynlang project.");
        return 1;
    }

    match fs::read_to_string(slang_rute) {
        Ok(slang_content) => match serde_yaml::from_str::<Sflynlang>(&slang_content) {
            Ok(project_settings) => {
                let main_rute = format!("{}/{}", current_directory, project_settings.get_main());

                let main_path = Path::new(&main_rute);

                if !main_path.exists() || !main_path.is_file() || !main_rute.ends_with(".sf") {
                    println!("The `{}` path does not exist or is not a file.", main_rute);
                    return 1;
                }

                match fs::read_to_string(&main_rute) {
                    Ok(main_content) => {
                        let file =
                            sflynlang_parser::File::new(project_settings.get_main(), main_content);

                        if let Some(statements) = sflynlang_parser::run(&file) {
                            sflynlang_compiler::run(statements, matches.is_present("debug"), &file)
                        } else {
                            1
                        }
                    }
                    Err(_) => {
                        println!("Cannot read the `{}` file.", main_rute);
                        1
                    }
                }
            }
            Err(_) => {
                println!("Cannot parse the `sflynlang.yml` file.");
                1
            }
        },
        Err(_) => {
            println!("Cannot read the `sflynlang.yml` file.");
            1
        }
    }
}
