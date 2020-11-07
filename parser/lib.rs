mod error;
mod lexer;
mod position;
mod token;

pub use error::Error;
pub use lexer::Lexer;
pub use position::Position;
pub use token::{Tok, Token};

use codespan_reporting::files::SimpleFile;

pub fn run_file(file: SimpleFile<String, String>) -> Option<Vec<Tok>> {
    let mut lexer = Lexer::new(file.source().to_string());

    match lexer.run() {
        Ok(tokens) => Some(tokens),
        Err(error) => {
            error.show(&file);
            None
        }
    }
}

pub fn run_test() {
    println!(
        "Tokens: {:?}",
        run_file(SimpleFile::new(
            "test.sf".to_string(),
            "print('Hello world!')".to_string(),
        ))
    );
}
