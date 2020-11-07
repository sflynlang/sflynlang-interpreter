use clap::{App, Arg};

fn main() {
    let matches = App::new("slang")
        .about("Programming language.")
        .version("0.1.0")
        .author("Daniel Solarte <danielsolartech@hotmail.com>")
        .subcommand(
            App::new("init")
                .about("Initialize a new slang project.")
                .arg(
                    Arg::new("project_name")
                        .about("The name of the slang project.")
                        .index(1)
                        .required(true),
                ),
        )
        .subcommand(App::new("start").about("Execute the current project."))
        .get_matches();

    println!("Matches: {:?}\n", matches);

    slang_parser::run_test();
}
