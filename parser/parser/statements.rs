use crate::{
    ast::{DataType, DataTypes, Expression, Statement, Statements},
    parser::{data_types, expressions},
    Error, Parser, Position, Precedence, Token,
};

pub fn parse_body(parser: &mut Parser) -> Result<Vec<Statement>, Error> {
    if !parser.expect_token(Token::LeftBrace)? {
        return Err(Error::new_expect_token(
            parser.get_next_token()?.get_position(),
            "{{",
            &parser.get_next_token()?.get_token().to_string(),
        ));
    }

    parser.read_next_token()?;

    parser.skip_eol()?;

    let mut statements: Vec<Statement> = Vec::new();

    while !parser.current_token_is(Token::RightBrace)? {
        statements.push(parse(parser)?);

        parser.read_next_token()?;

        parser.skip_eol()?;
    }

    Ok(statements)
}

pub fn parse(parser: &mut Parser) -> Result<Statement, Error> {
    if parser.current_token_is(Token::Func)? {
        let function_position = parser.get_current_token()?.get_position();

        parser.read_next_token()?;

        let function_name_and_arguments =
            expressions::parse(parser, Precedence::Lowest)?;

        if function_name_and_arguments.node.get_call().is_none() {
            return Err(Error::new_expect_token(
                function_name_and_arguments.get_position(),
                "Identifier",
                &function_name_and_arguments.to_string(),
            ));
        }

        let function_name: Box<Expression> =
            function_name_and_arguments.node.get_call().unwrap().0;

        if function_name.node.get_identifier().is_none() {
            return Err(Error::new_expect_token(
                function_name.get_position(),
                "Identifier",
                &function_name.to_string(),
            ));
        }

        let function_arguments: Vec<Expression> =
            function_name_and_arguments.node.get_call().unwrap().1;

        if function_arguments.len() > 0 {
            for function_argument in function_arguments.iter() {
                if function_argument.node.get_argument().is_none() {
                    return Err(Error::new_expect_token(
                        function_argument.get_position(),
                        "Argument",
                        &function_argument.to_string(),
                    ));
                }
            }
        }

        let mut function_return_type: DataType =
            DataType::new(Position::new(0, 0, 1, 1), DataTypes::Unknown);

        if parser.expect_token(Token::Colon)? {
            parser.read_next_token()?;

            function_return_type = data_types::parse(parser)?;
        }

        let function_body = parse_body(parser)?;

        return Ok(Statement::new(
            function_position,
            Statements::Function {
                name: function_name,
                arguments: function_arguments,
                return_type: Box::new(function_return_type),
                body: function_body,
            },
        ));
    }

    if parser.current_token_is(Token::Return)? {
        let return_position = parser.get_current_token()?.get_position();
        let mut return_value: Option<Expression> = None;

        if !parser.next_token_is(Token::Semicolon)?
            && !parser.next_token_is(Token::EndOfLine)?
            && !parser.next_token_is(Token::EndOfFile)?
        {
            parser.read_next_token()?;

            return_value =
                Some(expressions::parse(parser, Precedence::Lowest)?);
        }

        parser.expect_token(Token::Semicolon)?;

        return Ok(Statement::new(
            return_position,
            Statements::Return(return_value),
        ));
    }

    if parser.current_token_is(Token::Let)?
        || parser.current_token_is(Token::Const)?
    {
        let variable_position = parser.get_current_token()?.get_position();
        let variable_mutable = parser.current_token_is(Token::Let)?;

        parser.read_next_token()?;

        let variable_data = expressions::parse(parser, Precedence::Lowest)?;

        parser.expect_token(Token::Semicolon)?;

        return if let Some((identifier, data_type, value)) =
            variable_data.node.get_argument()
        {
            Ok(Statement::new(
                variable_position,
                Statements::Variable {
                    is_mutable: variable_mutable,
                    name: identifier,
                    data_type: Some(*data_type),
                    value: if let Some(value) = value {
                        Some(*value)
                    } else {
                        None
                    },
                },
            ))
        } else {
            Err(Error::new_expect_token(
                variable_data.get_position(),
                "Identifier",
                &variable_data.to_string(),
            ))
        };
    }

    Ok(Statement::new(
        parser.get_current_token()?.get_position(),
        Statements::Expression(Box::new(expressions::parse(
            parser,
            Precedence::Lowest,
        )?)),
    ))
}
