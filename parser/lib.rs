pub mod ast;
mod error;
mod lexer;
pub mod parser;
mod position;
mod precedence;
mod token;

pub use error::Error;
pub use lexer::Lexer;
pub use parser::Parser;
pub use position::Position;
pub use precedence::Precedence;
pub use token::{Tok, Token};

use codespan_reporting::files::SimpleFile;

fn run_internal(file: &SimpleFile<String, String>) -> Result<Vec<ast::Statement>, Error> {
    let mut lexer = Lexer::new(file.source().to_string());
    let mut parser = Parser::new(lexer.run()?);

    parser.run()
}

pub fn run_file(file: SimpleFile<String, String>) -> Option<Vec<ast::Statement>> {
    match run_internal(&file) {
        Ok(statements) => Some(statements),
        Err(error) => {
            error.show(&file);
            None
        }
    }
}

pub fn run_content(file_name: String, file_content: String) -> Option<Vec<ast::Statement>> {
    run_file(SimpleFile::new(file_name, file_content))
}
