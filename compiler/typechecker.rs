use crate::{builtins, Environment, Store};
use sflynlang_parser::{
    ast::{DataType, DataTypes, Expression, Statement},
    Error, Position, Token,
};

fn check_body(
    body: Vec<Statement>,
    environment: &mut Environment,
) -> Result<DataType, Error> {
    let mut data_type: DataType =
        DataType::new(Position::new(0, 0, 1, 1), DataTypes::Unknown);
    let mut is_first_stmt = true;
    let mut has_main_return = false;

    for statement in body.iter() {
        if has_main_return {
            return Err(Error::new_lexical(
                statement.get_position(),
                "This will never read.",
            ));
        }

        let stmt_type = check_statement(statement, environment)?;

        if statement.node.get_return().is_some() {
            has_main_return = true;
        }

        if is_first_stmt {
            is_first_stmt = false;
            data_type = stmt_type;
            continue;
        }

        if data_type.node != stmt_type.node {
            return Err(Error::new_expect_type(
                statement.get_position(),
                &data_type.node.to_string(),
                &stmt_type.node.to_string(),
            ));
        }
    }

    Ok(data_type)
}

pub fn check_expression(
    expression: &Expression,
    environment: &mut Environment,
) -> Result<DataType, Error> {
    // Argument
    if let Some((argument_name, argument_type, argument_value)) =
        expression.node.get_argument()
    {
        // Check if the name is already in use.
        if environment.get_store().has_key_type(&argument_name) {
            return Err(Error::new_name_in_use(
                expression.get_position(),
                argument_name.clone(),
                environment
                    .get_store()
                    .get_data_type_with_outer(&argument_name)
                    .unwrap()
                    .get_position(),
            ));
        }

        if let Some(argument_value) = argument_value {
            let value_type = check_expression(&argument_value, environment)?;

            if argument_type.node != value_type.node {
                return Err(Error::new_expect_type(
                    argument_value.get_position(),
                    &argument_type.node.to_string(),
                    &value_type.node.to_string(),
                ));
            }

            let data_type = DataType::new(
                argument_type.get_position(),
                DataTypes::Option(Box::new(value_type)),
            );

            environment
                .get_store()
                .add_data_type(&argument_name, &data_type);

            return Ok(data_type);
        }

        environment
            .get_store()
            .add_data_type(&argument_name, &*argument_type);

        return Ok(*argument_type);
    }
    // Boolean
    else if let Some(_) = expression.node.get_boolean() {
        return Ok(DataType::new(
            expression.get_position(),
            DataTypes::Boolean,
        ));
    }
    // Call
    else if let Some((identifier, arguments)) = expression.node.get_call() {
        if let Some(identifier_name) = identifier.node.get_identifier() {
            if !environment.get_store().has_key_type(&identifier_name) {
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
                return builtins::check_builtin(
                    identifier_name,
                    expression.get_position(),
                    arguments_types,
                );
            } else if let Some(env_type) = environment
                .get_store()
                .get_data_type_with_outer(&identifier_name)
            {
                if let Some((_, return_type)) = env_type.node.get_function() {
                    return Ok(*return_type);
                }

                return Ok(env_type);
            }
        } else {
            return Err(Error::new_unknown_token(expression.get_position()));
        }
    }
    // Identifier
    else if let Some(identifier_name) = expression.node.get_identifier() {
        if !environment.get_store().has_key_type(&identifier_name) {
            return Err(Error::new_unknown_identifier(
                expression.get_position(),
                identifier_name,
            ));
        }

        return Ok(environment
            .get_store()
            .get_data_type_with_outer(&identifier_name)
            .unwrap());
    }
    // If
    else if let Some((condition, consequence, alternative)) =
        expression.node.get_if()
    {
        let condition_type = check_expression(&condition, environment)?;

        if !condition_type.node.is_boolean() {
            return Err(Error::new_expect_type(
                condition.get_position(),
                "boolean",
                &condition_type.node.to_string(),
            ));
        }

        let consequence_type = check_body(consequence, environment)?;

        if alternative.len() > 0 {
            let alternative_type = check_body(alternative, environment)?;

            if consequence_type.node != alternative_type.node {
                return Err(Error::new_expect_type(
                    expression.get_position(),
                    &consequence_type.node.to_string(),
                    &alternative_type.node.to_string(),
                ));
            }

            return Ok(alternative_type);
        }

        return Ok(consequence_type);
    }
    // Infix
    else if let Some((left_exp, operator, right_exp)) =
        expression.node.get_infix()
    {
        // Check expression:
        // Get the data type of the left expression.
        let left_type = check_expression(&left_exp, environment)?;

        // Check expression:
        // Get the data type of the right expression.
        let right_type = check_expression(&right_exp, environment)?;

        return match operator {
            Token::Plus => {
                if !left_type.node.is_string() && !left_type.node.is_number() {
                    Err(Error::new_expect_type(
                        left_exp.get_position(),
                        "string or number",
                        &left_type.node.to_string(),
                    ))
                } else if left_type.node != right_type.node {
                    Err(Error::new_expect_type(
                        right_exp.get_position(),
                        &left_type.node.to_string(),
                        &right_type.node.to_string(),
                    ))
                } else {
                    Ok(right_type)
                }
            }

            Token::Minus
            | Token::Star
            | Token::Slash
            | Token::DoubleStar
            | Token::Percent
            | Token::Less
            | Token::LessEqual
            | Token::Greater
            | Token::GreaterEqual => {
                if !left_type.node.is_number() {
                    Err(Error::new_expect_type(
                        left_exp.get_position(),
                        "number",
                        &left_type.node.to_string(),
                    ))
                } else if left_type.node != right_type.node {
                    Err(Error::new_expect_type(
                        right_exp.get_position(),
                        &left_type.node.to_string(),
                        &right_type.node.to_string(),
                    ))
                } else if [
                    Token::Less,
                    Token::LessEqual,
                    Token::Greater,
                    Token::GreaterEqual,
                ]
                .contains(&operator)
                {
                    Ok(DataType::new(
                        expression.get_position(),
                        DataTypes::Boolean,
                    ))
                } else {
                    Ok(right_type)
                }
            }

            _ => Err(Error::new_unknown_token(expression.get_position())),
        };
    }
    // Method
    else if let Some((left_exp, right_exp)) = expression.node.get_method() {
        let left_type: DataType = check_expression(&left_exp, environment)?;

        let method_store = Store::from_outer(environment.get_store());
        let mut method_environment = environment.clone();

        method_environment.set_store(&method_store);

        if left_type.node.is_number() {
            for (key, value) in builtins::numbers::get_hashmap_typechecker() {
                method_environment.get_store().add_data_type(&key, &value);
            }
        }

        let right_type: DataType =
            check_expression(&right_exp, &mut method_environment)?;

        return Ok(right_type);
    }
    // Number
    else if let Some(_) = expression.node.get_number() {
        return Ok(DataType::new(expression.get_position(), DataTypes::Number));
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
    // Function
    else if let Some((
        function_name,
        function_arguments,
        function_return_type,
        function_body,
    )) = statement.node.get_function()
    {
        // Get the function name string.
        if let Some(identifier_name) = function_name.node.get_identifier() {
            // Check if the name is already in use.
            if environment.get_store().has_key_type(&identifier_name) {
                return Err(Error::new_name_in_use(
                    function_name.get_position(),
                    identifier_name.clone(),
                    environment
                        .get_store()
                        .get_data_type_with_outer(&identifier_name)
                        .unwrap()
                        .get_position(),
                ));
            }

            // Create a new internal storage.
            let store = Store::from_outer(&environment.get_store());

            // Create a new internal environment.
            let mut function_environment = environment.clone();

            function_environment.set_store(&store);

            // Initialize the function arguments data types.
            let mut arguments_types: Vec<DataType> = Vec::new();

            for arg in function_arguments.iter() {
                // Check expression:
                // Append the argument data type to the function arguments data types list.
                arguments_types
                    .push(check_expression(arg, &mut function_environment)?);
            }

            let function_type: DataType = DataType::new(
                statement.get_position(),
                DataTypes::Function(arguments_types, function_return_type),
            );

            function_environment
                .get_store()
                .add_data_type(&identifier_name, &function_type);

            let function_body_type =
                check_body(function_body, &mut function_environment)?;

            if let Some((_, return_type)) = function_type.node.get_function() {
                if return_type.node != function_body_type.node {
                    return Err(Error::new_expect_type(
                        function_body_type.get_position(),
                        &return_type.node.to_string(),
                        &function_body_type.node.to_string(),
                    ));
                }
            }

            environment.get_store().add_data_type(&identifier_name, &function_type);

            return Ok(function_type);
        } else {
            return Err(Error::new_unknown_token(function_name.get_position()));
        }
    }
    // Return
    else if let Some(value_exp) = statement.node.get_return() {
        if let Some(value_exp) = value_exp {
            return check_expression(&value_exp, environment);
        }

        return Ok(DataType::new(statement.get_position(), DataTypes::Void));
    }

    Err(Error::new_expect_type(
        statement.get_position(),
        "Unknown Statement",
        "Unknown",
    ))
}
