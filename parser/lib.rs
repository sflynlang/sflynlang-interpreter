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

pub type File = SimpleFile<String, String>;

fn run_internal(file: &File) -> Result<Vec<ast::Statement>, Error> {
    let mut lexer = Lexer::new(file.source().to_string());
    let mut parser = Parser::new(lexer.run()?);

    parser.run()
}

pub fn run(file: &File) -> Option<Vec<ast::Statement>> {
    match run_internal(file) {
        Ok(statements) => Some(statements),
        Err(error) => {
            error.show(&file);
            None
        }
    }
}
