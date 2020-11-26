use crate::{builtins, Environment, Object, Objects, Store};
use sflynlang_parser::{
    ast::{Expression, Statement},
    Error, Position, Token,
};
use std::collections::HashMap;

pub fn evaluate_body(body: Vec<Statement>, environment: &mut Environment) -> Result<Object, Error> {
    let object = Object::new(Position::new(0, 0, 1, 1), Objects::Unknown);

    for statement in body.iter() {
        let stmt_object = evaluate_statement(statement, environment)?;

        if let Some(return_obj) = stmt_object.get_node().get_return() {
            return Ok(*return_obj);
        }
    }

    Ok(object)
}

pub fn evaluate_expression(
    expression: &Expression,
    environment: &mut Environment,
) -> Result<Object, Error> {
    // Argument:
    if let Some((argument_name, _argument_type, argument_value)) =
        expression.node.get_argument()
    {
        // Check if the name is already in use.
        if environment.get_store().has_key_object(&argument_name) {
            return Err(Error::new_name_in_use(
                expression.get_position(),
                argument_name.clone(),
                environment
                    .get_store()
                    .get_object_with_outer(&argument_name)
                    .unwrap()
                    .get_position(),
            ));
        }

        if let Some(argument_value) = argument_value {
            let value_object = evaluate_expression(&argument_value, environment)?;

            environment.get_store().add_object(&argument_name, &value_object);

            return Ok(value_object);
        }

        let argument_object = Object::new(expression.get_position(), Objects::Unknown);

        return Ok(argument_object);
    }
    // Call:
    else if let Some((identifier, arguments)) = expression.node.get_call() {
        if let Some(identifier_name) = identifier.node.get_identifier() {
            if !environment.get_store().has_key_object(&identifier_name) {
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

            if environment.get_store().is_builtin(&identifier_name) {
                return builtins::eval_builtin(
                    identifier_name,
                    identifier.get_position(),
                    arguments_objects,
                    environment,
                );
            } else if let Some(env_object) = environment
                .get_store()
                .get_object_with_outer(&identifier_name)
            {
                if let Some((
                    function_arguments,
                    function_body,
                    _function_return_obj,
                    function_environment,
                )) = env_object.get_node().get_function()
                {
                    let min_arguments: usize = function_arguments.iter().map(|(_key, value)| value.get_node().is_unknown()).len();
                    let max_arguments: usize = function_arguments.len();

                    if arguments_objects.len() < min_arguments {
                        return Err(Error::new_expect_arguments(expression.get_position(), min_arguments, arguments_objects.len()));
                    }

                    if arguments_objects.len() > max_arguments {
                        return Err(Error::new_expect_arguments(expression.get_position(), max_arguments, arguments_objects.len()));
                    }

                    let keys: Vec<String> = function_arguments.keys().map(|arg| arg.to_string()).collect();
                    let mut index: usize = 0;

                    let mut function_environment: Environment = function_environment.clone();

                    for argument in arguments_objects.iter() {
                        function_environment.get_store().add_object(&keys[index], argument);
                        index += 1;
                    }

                    return evaluate_body(function_body, &mut function_environment);
                }
            }
        } else {
            return Err(Error::new_unknown_token(identifier.get_position()));
        }
    }
    // Identifier:
    else if let Some(identifier_name) = expression.node.get_identifier() {
        if let Some(env_obj) = environment.get_store().get_object_with_outer(&identifier_name) {
            return Ok(env_obj);
        }

        return Err(Error::new_unknown_identifier(expression.get_position(), identifier_name));
    }
    // If:
    else if let Some((condition, consequence, alternative)) = expression.node.get_if() {
        let condition_obj: Object = evaluate_expression(&condition, environment)?;

        if condition_obj.get_node().is_trusthy() {
            return evaluate_body(consequence, environment);
        } else {
            return evaluate_body(alternative, environment);
        }
    }
    // Infix:
    else if let Some((left, operator, right)) = expression.node.get_infix() {
        let left_obj = evaluate_expression(&left, environment)?;
        let right_obj = evaluate_expression(&right, environment)?;

        match operator {
            Token::Plus => {
                if let Some(left_value) = left_obj.get_node().get_string() {
                    if let Some(right_value) = right_obj.get_node().get_string() {
                        return Ok(Object::new(
                            right_obj.get_position(),
                            Objects::String(format!("{}{}", left_value, right_value)),
                        ));
                    }
                } else if let Some(left_value) = left_obj.get_node().get_number() {
                    if let Some(right_value) = right_obj.get_node().get_number() {
                        return Ok(Object::new(
                            right_obj.get_position(),
                            Objects::Number(left_value + right_value),
                        ));
                    }
                }
            }

            Token::Minus
            | Token::Star
            | Token::Slash
            | Token::Percent
            | Token::DoubleStar => {
                if let Some(left_value) = left_obj.get_node().get_number() {
                    if let Some(right_value) = right_obj.get_node().get_number() {
                        return Ok(Object::new(
                            right_obj.get_position(),
                            Objects::Number(
                                if operator == Token::Minus { left_value - right_value }
                                else if operator == Token::Star { left_value * right_value }
                                else if operator == Token::Slash { left_value / right_value }
                                else if operator == Token::Percent { left_value % right_value }
                                else if operator == Token::DoubleStar { left_value.pow(right_value.to_string().parse().unwrap()) }
                                else { 0 }
                            ),
                        ));
                    }
                }
            },

            Token::Less
            | Token::LessEqual
            | Token::Greater
            | Token::GreaterEqual => {
                if let Some(left_value) = left_obj.get_node().get_number() {
                    if let Some(right_value) = right_obj.get_node().get_number() {
                        return Ok(Object::new(
                            right_obj.get_position(),
                            Objects::Boolean(
                                if operator == Token::Less { left_value < right_value }
                                else if operator == Token::LessEqual { left_value <= right_value }
                                else if operator == Token::Greater { left_value > right_value }
                                else { left_value >= right_value }
                            ),
                        ));
                    }
                }
            }

            _ => {},
        }

        return Err(Error::new_unknown_token(expression.get_position()));
    }
    // Method:
    else if let Some((left_exp, right_exp)) = expression.node.get_method() {
        let _left_obj = evaluate_expression(&left_exp, environment)?;
        let _right_obj = evaluate_expression(&right_exp, environment)?;
    }
    // Number:
    else if let Some(value) = expression.node.get_number() {
        return Ok(Object::new(
            expression.get_position(),
            Objects::Number(value.to_string().parse().unwrap()),
        ));
    }
    // String:
    else if let Some(string_value) = expression.node.get_string() {
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
    // Expression:
    if let Some(expression) = statement.node.get_expression() {
        return evaluate_expression(&expression, environment);
    }
    // Function:
    else if let Some((
        function_name,
        function_arguments,
        _,
        function_body,
    )) = statement.node.get_function()
    {
        // Get the function name string.
        if let Some(identifier_name) = function_name.node.get_identifier() {
            // Check if the name is already in use.
            if environment.get_store().has_key_object(&identifier_name) {
                return Err(Error::new_name_in_use(
                    function_name.get_position(),
                    identifier_name.clone(),
                    environment
                        .get_store()
                        .get_object_with_outer(&identifier_name)
                        .unwrap()
                        .get_position(),
                ));
            }

            // Create a new internal storage.
            let store = Store::from_outer(&environment.get_store());

            // Create a new internal environment.
            let mut function_environment: Environment = environment.clone();

            function_environment.set_store(&store);

            let mut arguments_objects: HashMap<String, Object> = HashMap::new();

            for arg in function_arguments.iter() {
                // Evaluate expression:
                // Get the expression from the argument.
                arguments_objects
                    .insert(
                        arg.node.get_argument().unwrap().0,
                        evaluate_expression(arg, &mut function_environment)?
                    );
            }

            let function_object = Object::new(
                function_name.get_position(),
                Objects::Function {
                    arguments: arguments_objects.clone(),
                    body: function_body.clone(),
                    return_obj: Box::new(Object::new(statement.get_position(), Objects::Unknown)),
                    environment: function_environment.clone(),
                }
            );

            environment.get_store().add_object(&identifier_name, &function_object);

            return Ok(function_object);
        } else {
            return Err(Error::new_unknown_token(function_name.get_position()));
        }
    }
    // Return
    else if let Some(value) = statement.node.get_return() {
        let mut object: Object = Object::new(statement.get_position(), Objects::Void);

        if let Some(value_exp) = value {
            object = evaluate_expression(&value_exp, environment)?;
        }

        return Ok(Object::new(statement.get_position(), Objects::Return(Box::new(object))));
    }

    Err(Error::new_unknown_token(statement.get_position()))
}
