mod console;
pub use console::*;

pub mod numbers;

use crate::{Environment, Object};
use sflynlang_parser::{
    ast::{DataType, DataTypes},
    Error, Position,
};

pub fn check_builtin(
    key: String,
    position: Position,
    arguments: Vec<DataType>,
) -> Result<DataType, Error> {
    match key.as_str() {
        "print" | "debug" => {
            if arguments.len() != 1 {
                return Err(Error::new_expect_arguments(
                    position,
                    1,
                    arguments.len(),
                ));
            }

            if arguments[0].node == DataTypes::String {
                Ok(DataType::new(position, DataTypes::Void))
            } else {
                Err(Error::new_expect_type(
                    position,
                    "string",
                    &arguments[0].node.to_string(),
                ))
            }
        }

        _ => Err(Error::new_unknown_identifier(position, key)),
    }
}

pub fn eval_builtin(
    key: String,
    position: Position,
    arguments: Vec<Object>,
    environment: &mut Environment,
) -> Result<Object, Error> {
    match key.as_str() {
        "print" => console::print(arguments, position),
        "debug" => {
            console::debug(arguments, position, environment.is_debug_mode())
        }
        _ => Err(Error::new_unknown_token(position)),
    }
}
