use crate::{builtins::check_builtin, Environment};
use sflynlang_parser::{
    ast::{DataType, DataTypes, Expression, Statement},
    Error,
};

pub fn check_expression(
    expression: &Expression,
    environment: &mut Environment,
) -> Result<DataType, Error> {
    // Call
    if let Some((identifier, arguments)) = expression.node.get_call() {
        if let Some(identifier_name) = identifier.node.get_identifier() {
            if !environment.get_store().has_key(&identifier_name) {
                return Err(Error::new_unknown_identifier(
                    identifier.get_position(),
                    identifier_name,
                ));
            }

            let mut arguments_types: Vec<DataType> = Vec::new();

            for argument in arguments.iter() {
                arguments_types.push(check_expression(argument, environment)?);
            }

            if environment.get_store().is_builtin(&identifier_name) {
                return check_builtin(
                    identifier_name,
                    expression.get_position(),
                    arguments_types,
                );
            }
        }
    }
    // String
    else if let Some(_) = expression.node.get_string() {
        return Ok(DataType::new(expression.get_position(), DataTypes::String));
    }

    Err(Error::new_expect_type(
        expression.get_position(),
        "Unknown Expression",
        "Unknown",
    ))
}

pub fn check_statement(
    statement: &Statement,
    environment: &mut Environment,
) -> Result<DataType, Error> {
    // Expression
    if let Some(expression) = statement.node.get_expression() {
        return check_expression(&expression, environment);
    }

    Err(Error::new_expect_type(
        statement.get_position(),
        "Unknown Statement",
        "Unknown",
    ))
}
