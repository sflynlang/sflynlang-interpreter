use crate::{
    ast::{DataType, DataTypes},
    Error, Parser, Token,
};

pub fn parse(parser: &mut Parser) -> Result<DataType, Error> {
    let mut node: Option<DataType> = None;

    // Check if the current token is a boolean.
    if parser.current_token_is(Token::Boolean)? {
        node = Some(DataType::new(
            parser.get_current_token()?.get_position(),
            DataTypes::Boolean,
        ));
    }
    // Check if the current token is a left parentheses.
    else if parser.current_token_is(Token::LeftParentheses)? {
        // Get the current token position as the function position.
        let function_position = parser.get_current_token()?.get_position();

        // Read the next token.
        parser.read_next_token()?;

        // Initialize the arguments data types list.
        let mut arguments: Vec<DataType> = Vec::new();

        while !parser.current_token_is(Token::RightParentheses)? {
            // Append the data type to the arguments list.
            arguments.push(parse(parser)?);

            // Check if the next token is a comma and read the next token.
            parser.expect_token(Token::Comma)?;

            // Read the next token.
            parser.read_next_token()?;
        }

        // Check if the next token is not an equal greater.
        if !parser.expect_token(Token::EqualGreater)? {
            return Err(Error::new_expect_token(
                parser.get_next_token()?.get_position(),
                "=>",
                &parser.get_next_token()?.get_token().to_string(),
            ));
        }

        parser.read_next_token()?;

        node = Some(DataType::new(
            function_position,
            DataTypes::Function(arguments, Box::new(parse(parser)?)),
        ));
    }
    // Check if the current token is an identifier.
    else if let Some(identifier_value) =
        parser.get_current_token()?.get_token().get_identifier()
    {
        node = Some(DataType::new(
            parser.get_current_token()?.get_position(),
            DataTypes::Identifier(identifier_value),
        ));
    }
    // Check if the current token is a number.
    else if parser.current_token_is(Token::Number)? {
        node = Some(DataType::new(
            parser.get_current_token()?.get_position(),
            DataTypes::Number,
        ))
    }
    // Check if the current token is a string.
    else if parser.current_token_is(Token::String)? {
        node = Some(DataType::new(
            parser.get_current_token()?.get_position(),
            DataTypes::String,
        ));
    }
    // Check if the current token is a void.
    else if parser.current_token_is(Token::Void)? {
        node = Some(DataType::new(
            parser.get_current_token()?.get_position(),
            DataTypes::Void,
        ));
    }

    // Check if the data type node is none.
    if node.is_none() {
        // Return an error.
        return Err(Error::new_unknown_token(
            parser.get_current_token()?.get_position(),
        ));
    }

    // Get the data type.
    let mut node_out: DataType = node.unwrap();

    loop {
        // Check if the next token is a left bracket and read the next token.
        if parser.expect_token(Token::LeftBracket)? {
            // Check if the next token is not a right bracket.
            if !parser.expect_token(Token::RightBracket)? {
                return Err(Error::new_expect_token(
                    parser.get_next_token()?.get_position(),
                    "]",
                    &parser.get_next_token()?.get_token().to_string(),
                ));
            }

            // Set the array data type to the node_out.
            node_out = DataType::new(
                node_out.get_position(),
                DataTypes::Array(Box::new(node_out)),
            );

            continue;
        }

        break;
    }

    // Return the data type.
    Ok(node_out)
}
