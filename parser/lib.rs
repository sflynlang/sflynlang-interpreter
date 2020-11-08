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

pub fn run_content(file_name: String, file_content: String) -> Option<Vec<Tok>> {
    run_file(SimpleFile::new(file_name, file_content))
}
