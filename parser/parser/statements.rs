use crate::{
    ast::{DataType, DataTypes, Expression, Statement, Statements},
    parser::{data_types, expressions},
    Error, Parser, Position, Precedence, Token,
};

pub fn parse_body(parser: &mut Parser) -> Result<Vec<Statement>, Error> {
    // Check if the next token is not a left brace.
    if !parser.expect_token(Token::LeftBrace)? {
        return Err(Error::new_expect_token(
            parser.get_next_token()?.get_position(),
            "{{",
            &parser.get_next_token()?.get_token().to_string(),
        ));
    }

    // Read the next token.
    parser.read_next_token()?;

    // Ignore end of lines.
    parser.skip_eol()?;

    // Initialize the statements list.
    let mut statements: Vec<Statement> = Vec::new();

    while !parser.current_token_is(Token::RightBrace)? {
        // Parse statement:
        // Get the current statement and append it to the statements list.
        statements.push(parse(parser)?);

        // Read the next token.
        parser.read_next_token()?;

        // Ignore end of lines.
        parser.skip_eol()?;
    }

    Ok(statements)
}

pub fn parse(parser: &mut Parser) -> Result<Statement, Error> {
    // Parse functions:
    // Check if the current token is a func.
    if parser.current_token_is(Token::Func)? {
        // Get the current token position as the function position.
        let function_position = parser.get_current_token()?.get_position();

        // Read the next token.
        parser.read_next_token()?;

        // Parse expression:
        // Get the name with arguments expression (Call).
        let function_name_and_arguments =
            expressions::parse(parser, Precedence::Lowest)?;

        // Check if the function name and arguments is not a call expression.
        if function_name_and_arguments.node.get_call().is_none() {
            return Err(Error::new_expect_token(
                function_name_and_arguments.get_position(),
                "Identifier",
                &function_name_and_arguments.to_string(),
            ));
        }

        // Get the function name expression.
        let function_name: Box<Expression> =
            function_name_and_arguments.node.get_call().unwrap().0;

        // Check if the function name is not an identifier expression.
        if function_name.node.get_identifier().is_none() {
            return Err(Error::new_expect_token(
                function_name.get_position(),
                "Identifier",
                &function_name.to_string(),
            ));
        }

        // Get the function arguments expressions list.
        let function_arguments: Vec<Expression> =
            function_name_and_arguments.node.get_call().unwrap().1;

        // Check if has arguments.
        if function_arguments.len() > 0 {
            for function_argument in function_arguments.iter() {
                // Check if the argument is not an argument expression.
                if function_argument.node.get_argument().is_none() {
                    return Err(Error::new_expect_token(
                        function_argument.get_position(),
                        "Argument",
                        &function_argument.to_string(),
                    ));
                }
            }
        }

        // Initialize the function data type.
        let mut function_return_type: DataType =
            DataType::new(Position::new(0, 0, 1, 1), DataTypes::Unknown);

        // Check if the next token is a colon and read the next token.
        if parser.expect_token(Token::Colon)? {
            // Read the next token.
            parser.read_next_token()?;

            // Pase data type:
            // Get the function return data type.
            function_return_type = data_types::parse(parser)?;
        }

        // Parse body statements:
        // Get the function body between braces.
        let function_body = parse_body(parser)?;

        Ok(Statement::new(
            function_position,
            Statements::Function {
                name: function_name,
                arguments: function_arguments,
                return_type: Box::new(function_return_type),
                body: function_body,
            },
        ))
    }
    // Parse interface:
    // Check if the current token is an interface.
    else if parser.current_token_is(Token::Interface)? {
        // Get the current token position as the interface position.
        let interface_position = parser.get_current_token()?.get_position();

        if parser
            .get_next_token()?
            .get_token()
            .get_identifier()
            .is_none()
        {
            return Err(Error::new_expect_token(
                parser.get_next_token()?.get_position(),
                "Identifier",
                &parser.get_next_token()?.get_token().to_string(),
            ));
        }

        // Read the next token.
        parser.read_next_token()?;

        // Get the current token as the interface name.
        let interface_name = parser
            .get_current_token()?
            .get_token()
            .get_identifier()
            .unwrap();

        if !parser.expect_token(Token::LeftBrace)? {
            return Err(Error::new_expect_token(
                parser.get_next_token()?.get_position(),
                "{",
                &parser.get_next_token()?.get_token().to_string(),
            ));
        }

        // Read the next token.
        parser.read_next_token()?;

        // Ignore the end of lines.
        parser.skip_eol()?;

        // Initialize the methods expressions list.
        let mut methods: Vec<Expression> = Vec::new();

        while !parser.current_token_is(Token::RightBrace)? {
            // Parse expression:
            // Get the current expression.
            let method = expressions::parse(parser, Precedence::Lowest)?;

            // Check if the method is not an argument expression.
            if method.node.get_argument().is_none() {
                return Err(Error::new_expect_token(
                    method.get_position(),
                    "Argument",
                    &method.to_string(),
                ));
            }

            // Get the argument expression.
            let (_identifier, _data_type, value) =
                method.node.get_argument().unwrap();

            // Check if the method argument has a value.
            if value.is_some() {
                return Err(Error::new_lexical(
                    value.unwrap().get_position(),
                    "Cannot set a value here.",
                ));
            }

            // Append the method to the methods list.
            methods.push(method);

            // Check if the next token is a semicolon and read the next token.
            parser.expect_token(Token::Semicolon)?;

            // Read the next token.
            parser.read_next_token()?;

            // Ignore the end of lines.
            parser.skip_eol()?;
        }

        Ok(Statement::new(
            interface_position,
            Statements::Interface(interface_name, methods),
        ))
    }
    // Parse return:
    // Check if the current token is a return.
    else if parser.current_token_is(Token::Return)? {
        // Get the current token position as the return position.
        let return_position = parser.get_current_token()?.get_position();

        // Initialize the return value expression.
        let mut return_value: Option<Expression> = None;

        // Check if the next token is not a semicolon, end of line or an end of file.
        if !parser.next_token_is(Token::Semicolon)?
            && !parser.next_token_is(Token::EndOfLine)?
            && !parser.next_token_is(Token::EndOfFile)?
        {
            // Read the next token.
            parser.read_next_token()?;

            return_value =
                Some(expressions::parse(parser, Precedence::Lowest)?);
        }

        // Check if the next token is a semicolon and read the next token.
        parser.expect_token(Token::Semicolon)?;

        Ok(Statement::new(
            return_position,
            Statements::Return(return_value),
        ))
    }
    // Parse variables:
    // Check if the current token is a let or a const.
    else if parser.current_token_is(Token::Let)?
        || parser.current_token_is(Token::Const)?
    {
        // Get the current token position as the variable position.
        let variable_position = parser.get_current_token()?.get_position();

        // Check if the current token is a let.
        let variable_mutable = parser.current_token_is(Token::Let)?;

        // Read the next token.
        parser.read_next_token()?;

        let variable_data = expressions::parse(parser, Precedence::Lowest)?;

        // Check if the next token is a semicolon and read the next token.
        parser.expect_token(Token::Semicolon)?;

        if let Some((identifier, data_type, value)) =
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
        }
    }
    // Parse expression:
    // See `/parser/parser/expressions.rs` for more information.
    else {
        Ok(Statement::new(
            parser.get_current_token()?.get_position(),
            Statements::Expression(Box::new(expressions::parse(
                parser,
                Precedence::Lowest,
            )?)),
        ))
    }
}
