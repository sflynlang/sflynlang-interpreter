use crate::structures::Slang;
use crate::utils;
use clap::{App, ArgMatches};
use std::fs;
use std::path::Path;

pub fn info() -> App<'static> {
    App::new("start").about("Execute the current project.")
}

pub fn run(_matches: &ArgMatches) -> i32 {
    let current_directory = utils::get_current_directory();
    let slang_rute = format!("{}/slang.yml", current_directory);

    let slang_path = Path::new(&slang_rute);

    if !slang_path.exists() || !slang_path.is_file() {
        println!("This path is not a slang project.");
        return 1;
    }

    match fs::read_to_string(slang_rute) {
        Ok(slang_content) => match serde_yaml::from_str::<Slang>(&slang_content) {
            Ok(project_settings) => {
                let main_rute = format!("{}/{}", current_directory, project_settings.get_main());

                let main_path = Path::new(&main_rute);

                if !main_path.exists() || !main_path.is_file() {
                    println!("The `{}` path does not exist or is not a file.", main_rute);
                    return 1;
                }

                match fs::read_to_string(&main_rute) {
                    Ok(main_content) => {
                        if let Some(tokens) =
                            slang_parser::run_content(project_settings.get_main(), main_content)
                        {
                            println!("Tokens: {:?}", tokens);
                        } else {
                            return 1;
                        }
                    }
                    Err(_) => {
                        println!("Cannot read the `{}` file.", main_rute);
                        return 1;
                    }
                }
            }
            Err(_) => {
                println!("Cannot parse the `slang.yml` file.");
                return 1;
            }
        },
        Err(_) => {
            println!("Cannot read the `slang.yml` file.");
            return 1;
        }
    }

    0
}
