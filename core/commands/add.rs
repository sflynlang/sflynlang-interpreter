use crate::utils;
use clap::{App, Arg, ArgMatches};
use git2::*;
use std::path::Path;

pub fn info() -> App<'static> {
  App::new("add")
    .about("Add a module in your project")
    .arg(
      Arg::new("module")
        .about("Module name to add")
        .index(1)
        .required(true),
    )
    .arg(
      Arg::new("source")
        .about("The source or service from which the module will be downloaded")
        .short('s')
        .long("source")
        .takes_value(true)
        .required(false),
    )
}

pub fn run(matches: &ArgMatches) -> i32 {
  if let Some(module_name) = matches.value_of("module") {
    let module_name = &module_name.to_lowercase();
    let mut source = "https://github.com/";
    match matches.value_of("source") {
      Some("github") => source = "https://github.com/",
      Some("gitlab") => source = "https://gitlab.com/",
      Some("bitbucket") => source = "https://bitbucket.org/",
      None => println!("Using github as default source..."),
      _ => println!("The source of the module is not valid"),
    }
    let current_directory = utils::get_current_directory();
    let slang_rute = format!("{}/slang.yml", current_directory);

    let slang_path = Path::new(&slang_rute);

    if !slang_path.exists() || !slang_path.is_file() {
      println!("This path is not a slang project.");
      return 1;
    }
    let url = format!("{}{}.git", source, module_name);
    let _repo = match Repository::clone(&url, format!("./modules/{}", module_name)) {
      Ok(repo) => {
        drop(repo);
        println!("The {} module has been added successfully", module_name);
      }
      Err(e) => panic!("failed to clone: {}", e),
    };
  }

  0
}
