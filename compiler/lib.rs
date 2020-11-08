mod builtins;
mod environment;
mod evaluator;
mod objects;

pub use environment::{Environment, Store};
pub use objects::{Object, Objects};

use slang_parser::{ast::Statement, File};

pub fn run(statements: Vec<Statement>, debug_mode: bool, file: &File) -> i32 {
    let mut environment = Environment::new();

    environment.set_debug_mode(debug_mode);

    for statement in statements.iter() {
        if let Err(error) = evaluator::evaluate_statement(statement, &mut environment) {
            environment.add_error(error);
        }
    }

    if environment.has_errors() {
        environment.show_errors(file);
        return 1;
    }

    0
}
