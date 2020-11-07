use crate::Position;

use codespan_reporting::diagnostic::{Diagnostic, Label};
use codespan_reporting::files::SimpleFile;
use codespan_reporting::term::{
    self,
    termcolor::{ColorChoice, StandardStream},
};

#[derive(Clone, Debug)]
pub enum ErrorType {
    Lexical { message: String },
}

#[derive(Clone, Debug)]
pub struct Error {
    /// Position object.
    position: Position,

    // Error type object.
    error_type: ErrorType,
}

impl Error {
    /// Create a new error object using a position and an error type.
    pub fn new(position: &Position, error_type: ErrorType) -> Self {
        Self {
            position: position.clone(),
            error_type,
        }
    }

    /// Create a new lexical error object using a position and a message.
    pub fn new_lexical(position: &Position, message: &str) -> Self {
        Self::new(
            position,
            ErrorType::Lexical {
                message: message.to_string(),
            },
        )
    }

    /// Get the position object of the error.
    pub fn get_position(&self) -> Position {
        self.position.clone()
    }

    /// Get the error type object of the error.
    pub fn get_error_type(&self) -> ErrorType {
        self.error_type.clone()
    }

    /// Convert the error to a diagnostic object.
    ///
    /// Read more about the Diagnostic object [clicking here](https://docs.rs/codespan-reporting/0.9.5/codespan_reporting/diagnostic/struct.Diagnostic.html).
    pub fn to_diagnostic(&self) -> Diagnostic<()> {
        match self.get_error_type() {
            // Get the lexical error.
            ErrorType::Lexical { message } => Diagnostic::error()
                .with_message("Lexical error")
                .with_labels(vec![
                    Label::primary((), self.get_position().get_range()).with_message(message)
                ]),
        }
    }

    /// Show the error in the console using term.
    ///
    /// Read more about the term object [clicking here](https://docs.rs/codespan-reporting/0.9.5/codespan_reporting/term/index.html).
    pub fn show(&self, file: &SimpleFile<String, String>) {
        if let Err(error) = term::emit(
            &mut StandardStream::stderr(ColorChoice::Always).lock(),
            &term::Config::default(),
            file,
            &self.to_diagnostic(),
        ) {
            println!("Term Error: {:?}", error);
        }
    }
}
