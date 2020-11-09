use crate::structures::Sflynlang;
use crate::utils;
use clap::{App, Arg, ArgMatches};
use std::fs;
use std::path::Path;

pub fn info() -> App<'static> {
    App::new("init")
        .about("Create a new Sflynlang project.")
        .arg(
            Arg::new("project_name")
                .about("The name of the Sflynlang project.")
                .index(1)
                .required(true),
        )
}

pub fn run(matches: &ArgMatches) -> i32 {
    // Initialize the default project settings.
    let mut project_settings = Sflynlang::new();

    // Get the project name from matches.
    if let Some(project_name) = matches.value_of("project_name") {
        // Check if the project name is not a restricted name.
        if utils::restricted_names::is_windows_reserved(&project_name) {
            println!("Cannot use `{}` as the project name.", project_name);
            return 1;
        }

        // Join the current directory path with the project name.
        let project_path = utils::join_paths(&utils::get_current_directory(), project_name);

        // Split the project path by slashes.
        let project_paths: Vec<&str> = project_path.split('/').collect();

        // Get the real project name from the project path.
        let real_project_name = project_paths[project_paths.len() - 2];

        // Set the new project name.
        project_settings.set_name(real_project_name.to_string());

        // Create a new path object for the project path.
        let project_path_obj = Path::new(&project_path);

        // Check if the project path already exists.
        if project_path_obj.exists() {
            println!("A path already exists in `{}`.", project_path);
            return 1;
        }

        // Get the error when the directory is creating.
        if let Err(error) = fs::create_dir_all(project_path.clone()) {
            println!("Creating path error: {}", error);
            return 1;
        }

        // Get the slang.yml path.
        let slang_file_path = format!("{}sflynlang.yml", project_path);

        // Get the slang.yml content from the project settings.
        match serde_yaml::to_string(&project_settings) {
            Ok(slang_file_content) => {
                if let Err(_) = fs::File::create(&slang_file_path) {
                    println!("Cannot create the `sflynlang.yml` file.");
                    return 1;
                }

                if let Err(_) = fs::write(slang_file_path, slang_file_content) {
                    println!("Cannot write inside the `sflynlang.yml` file.");
                    return 1;
                }
            }
            Err(_) => {
                println!("Cannot parse the `sflynlang.yml` settings.");
                return 1;
            }
        }

        // Get the main file path.
        let main_file_path = format!("{}{}", project_path, project_settings.get_main());

        if let Err(_) = fs::File::create(&main_file_path) {
            println!("Cannot create the main file.");
            return 1;
        }

        if let Err(_) = fs::write(main_file_path, "print('Hello world!')") {
            println!("Cannot write inside the main file.");
            return 1;
        }

        println!("You've created a new Sflynlang project successfully!");
        println!("");
        println!("To join to your project use `cd {}`.", project_name);
        println!("Then, you can run the code using `sflyn start`.");
        println!("");
        println!("Happy coding!");

        return 0;
    }

    println!("Unknown error.");
    1
}
