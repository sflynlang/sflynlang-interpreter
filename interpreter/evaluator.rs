use crate::{builtins, Environment, Object, Objects};
use sflynlang_parser::{
    ast::{Expression, Statement},
    Error,
};

pub fn evaluate_expression(
    expression: &Expression,
    environment: &mut Environment,
) -> Result<Object, Error> {
    if let Some((identifier, arguments)) = expression.node.get_call() {
        if let Some(identifier_name) = identifier.node.get_identifier() {
            if !environment.get_store().has_key(&identifier_name) {
                return Err(Error::new_unknown_identifier(
                    identifier.get_position(),
                    identifier_name,
                ));
            }

            let mut arguments_objects: Vec<Object> = Vec::new();

            for argument in arguments.iter() {
                arguments_objects
                    .push(evaluate_expression(argument, environment)?);
            }

            if identifier_name == "print" {
                return builtins::print(
                    arguments_objects,
                    identifier.get_position(),
                );
            } else if identifier_name == "debug" {
                return builtins::debug(
                    arguments_objects,
                    identifier.get_position(),
                    environment.is_debug_mode(),
                );
            }
        }
    } else if let Some(string_value) = expression.node.get_string() {
        return Ok(Object::new(
            expression.get_position(),
            Objects::String(string_value),
        ));
    }

    Err(Error::new_unknown_token(expression.get_position()))
}

pub fn evaluate_statement(
    statement: &Statement,
    environment: &mut Environment,
) -> Result<Object, Error> {
    if let Some(expression) = statement.node.get_expression() {
        return evaluate_expression(&expression, environment);
    }

    Err(Error::new_unknown_token(statement.get_position()))
}
