use crate::{
    ast::{Expression, Expressions},
    parser::{data_types, statements, Statement},
    Error, Parser, Precedence, Token,
};

use std::collections::HashMap;

pub fn parse(
    parser: &mut Parser,
    precedence: Precedence,
) -> Result<Expression, Error> {
    let mut node: Option<Expression> = None;

    // Parse array:
    // Check if the current token is a left bracket.
    if parser.current_token_is(Token::LeftBracket)? {
        // Get the current token position as the array position.
        let array_position = parser.get_current_token()?.get_position();

        // Read the next token.
        parser.read_next_token()?;

        // Initialize the array values list.
        let mut values: Vec<Expression> = Vec::new();

        while !parser.current_token_is(Token::RightBracket)? {
            // Parse expression:
            // Get the current expression and append it to the array values list.
            values.push(parse(parser, Precedence::Lowest)?);

            // Check if the next token is a comma and read the next token.
            parser.expect_token(Token::Comma)?;

            // Read the next token.
            parser.read_next_token()?;
        }

        node =
            Some(Expression::new(array_position, Expressions::Array(values)));
    }
    // Parse boolean:
    // Check if the current token is `true` or `false`.
    else if parser.current_token_is(Token::True)?
        || parser.current_token_is(Token::False)?
    {
        node = Some(Expression::new(
            parser.get_current_token()?.get_position(),
            Expressions::Boolean(parser.current_token_is(Token::True)?),
        ));
    }
    // Parse group:
    // Check if the current token is a left parentheses.
    else if parser.current_token_is(Token::LeftParentheses)? {
        // Get the current token position as the group position.
        let group_position = parser.get_current_token()?.get_position();

        // Read the next token.
        parser.read_next_token()?;

        // Parse expression:
        // Get the group expression.
        let group_exp = parse(parser, Precedence::Lowest)?;

        // Check if the next token is not a right parentheses.
        if !parser.expect_token(Token::RightParentheses)? {
            return Err(Error::new_expect_token(
                parser.get_next_token()?.get_position(),
                ")",
                &parser.get_next_token()?.get_token().to_string(),
            ));
        }

        node = Some(Expression::new(
            group_position,
            Expressions::Group(Box::new(group_exp)),
        ));
    }
    // Parse hashmap:
    // Check if the current token is a left brace.
    else if parser.current_token_is(Token::LeftBrace)? {
        // Get the current token position as the hashmap position.
        let hashmap_position = parser.get_current_token()?.get_position();

        // Read the next token.
        parser.read_next_token()?;

        // Ignore the end of lines.
        parser.skip_eol()?;

        // Initialize the hashmap data.
        let mut hashmap_data: HashMap<String, Expression> = HashMap::new();

        while !parser.current_token_is(Token::RightBrace)? {
            // Get the identifier token value.
            if let Some(identifier_value) =
                parser.get_current_token()?.get_token().get_identifier()
            {
                // Check if the next token is not a colon.
                if !parser.expect_token(Token::Colon)? {
                    return Err(Error::new_expect_token(
                        parser.get_next_token()?.get_position(),
                        ":",
                        &parser.get_next_token()?.get_token().to_string(),
                    ));
                }

                // Read the next token.
                parser.read_next_token()?;

                // Append the data to the hashmap data.
                hashmap_data.insert(
                    identifier_value,
                    parse(parser, Precedence::Lowest)?,
                );

                // Check if the next token is a comma and read the next token.
                parser.expect_token(Token::Comma)?;

                // Read the next token.
                parser.read_next_token()?;

                // Ignore the end of lines.
                parser.skip_eol()?;
            } else {
                return Err(Error::new_expect_token(
                    parser.get_current_token()?.get_position(),
                    "Identifier",
                    &parser.get_current_token()?.get_token().to_string(),
                ));
            }
        }

        node = Some(Expression::new(
            hashmap_position,
            Expressions::HashMap(hashmap_data),
        ));
    }
    // Parse argument or identifier.
    // Check if the current token is an identifier.
    else if let Some(identifier_value) =
        parser.get_current_token()?.get_token().get_identifier()
    {
        // Get the current token position as the identifier position.
        let identifier_position = parser.get_current_token()?.get_position();

        // Parse argument:
        // Check if the next token is a colon and read the next token.
        if parser.expect_token(Token::Colon)? {
            // Read the next token.
            parser.read_next_token()?;

            // Parse data type:
            // Get the argument data type.
            let argument_data_type = data_types::parse(parser)?;

            // Initialize the optional argument value.
            let mut argument_value: Option<Box<Expression>> = None;

            // Check if the next token is an equal and read the next token.
            if parser.expect_token(Token::Equal)? {
                // Read the next token.
                parser.read_next_token()?;

                // Set the argument value.
                argument_value =
                    Some(Box::new(parse(parser, Precedence::Lowest)?));
            }

            node = Some(Expression::new(
                identifier_position,
                Expressions::Argument {
                    name: identifier_value,
                    data_type: Box::new(argument_data_type),
                    value: argument_value,
                },
            ));
        }
        // Parse identifier:
        else {
            node = Some(Expression::new(
                identifier_position,
                Expressions::Identifier(identifier_value),
            ));
        }
    }
    // Parse if:
    // Check if the current token is an if.
    else if parser.current_token_is(Token::If)? {
        // Get the current token position as the if position.
        let if_position = parser.get_current_token()?.get_position();

        // Check if the next token is not a left parentheses.
        if !parser.expect_token(Token::LeftParentheses)? {
            return Err(Error::new_expect_token(
                parser.get_next_token()?.get_position(),
                "(",
                &parser.get_next_token()?.get_token().to_string(),
            ));
        }

        // Read the next token.
        parser.read_next_token()?;

        // Parse expression:
        // Get the if condition.
        let if_condition = parse(parser, Precedence::Lowest)?;

        // Check if the next token is not a right parentheses.
        if !parser.expect_token(Token::RightParentheses)? {
            return Err(Error::new_expect_token(
                parser.get_next_token()?.get_position(),
                ")",
                &parser.get_next_token()?.get_token().to_string(),
            ));
        }

        // Parse body statements:
        // Get the consequence statements between braces.
        let if_consequence = statements::parse_body(parser)?;

        // Initialize the alternative statements list.
        let mut if_alternative: Vec<Statement> = Vec::new();

        // Check if the next token is an else and read the next token.
        if parser.expect_token(Token::Else)? {
            // Parse body statements:
            // Get the alternative statements between braces.
            if_alternative = statements::parse_body(parser)?;
        }

        node = Some(Expression::new(
            if_position,
            Expressions::If {
                condition: Box::new(if_condition),
                consequence: if_consequence,
                alternative: if_alternative,
            },
        ));
    }
    // Parse number:
    // Check if the current token is a number.
    else if let Some(number_value) =
        parser.get_current_token()?.get_token().get_number()
    {
        node = Some(Expression::new(
            parser.get_current_token()?.get_position(),
            Expressions::Number(number_value),
        ));
    }
    // Parse prefix:
    // Check if the current token is a minus or a not.
    else if parser.current_token_is(Token::Minus)?
        || parser.current_token_is(Token::Not)?
    {
        // Get the current token position as the prefix position.
        let prefix_position = parser.get_current_token()?.get_position();

        // Get the current token as the prefix token.
        let prefix_token = parser.get_current_token()?.get_token();

        // Read the next token.
        parser.read_next_token()?;

        node = Some(Expression::new(
            prefix_position,
            Expressions::Prefix(
                prefix_token,
                Box::new(parse(parser, Precedence::Prefix)?),
            ),
        ));
    }
    // Parse string:
    // Check if the current token is a string.
    else if let Some(string_value) =
        parser.get_current_token()?.get_token().get_string()
    {
        node = Some(Expression::new(
            parser.get_current_token()?.get_position(),
            Expressions::String(string_value),
        ));
    }

    // Check if the node expression is none.
    if node.is_none() {
        return Err(Error::new_unknown_token(
            parser.get_current_token()?.get_position(),
        ));
    }

    let mut node_out: Expression = node.unwrap();

    // Parse call:
    // Check if the next token is a left parentheses and read the next token.
    if parser.expect_token(Token::LeftParentheses)? {
        // Initialize the call arguments expressions list.
        let mut call_arguments: Vec<Expression> = Vec::new();

        // Read the next token.
        parser.read_next_token()?;

        while !parser.current_token_is(Token::RightParentheses)? {
            // Parse expression:
            // Get the argument expresion and append it to the arguments list.
            call_arguments.push(parse(parser, Precedence::Lowest)?);

            // Check if the next token is a comma and read the next token.
            parser.expect_token(Token::Comma)?;

            // Read the next token.
            parser.read_next_token()?;
        }

        // Check if the next token is a semicolon and read the next token.
        parser.expect_token(Token::Semicolon)?;

        node_out = Expression::new(
            node_out.get_position(),
            Expressions::Call(Box::new(node_out), call_arguments),
        );
    }
    // Parse index:
    // Check if the next token is a left bracket and read the next token.
    else if parser.expect_token(Token::LeftBracket)? {
        // Read the next token.
        parser.read_next_token()?;

        // Parse expression:
        // Get the current expression as the index expression.
        let index_exp = parse(parser, Precedence::Lowest)?;

        // Check if the next token is not a right bracket.
        if !parser.expect_token(Token::RightBracket)? {
            return Err(Error::new_expect_token(
                parser.get_next_token()?.get_position(),
                "]",
                &parser.get_next_token()?.get_token().to_string(),
            ));
        }

        node_out = Expression::new(
            node_out.get_position(),
            Expressions::Index(Box::new(node_out), Box::new(index_exp)),
        );
    }

    while !parser.current_token_is(Token::Semicolon)?
        && precedence < parser.get_next_precedence()?
    {
        // Parse assignment:
        // Check if the next token is an equal, plus equal, minus equal,
        // star equal, slash equal, percent equal or a double star equal
        // and read the next token.
        if parser.expect_token(Token::Equal)?
            || parser.expect_token(Token::PlusEqual)?
            || parser.expect_token(Token::MinusEqual)?
            || parser.expect_token(Token::StarEqual)?
            || parser.expect_token(Token::SlashEqual)?
            || parser.expect_token(Token::PercentEqual)?
            || parser.expect_token(Token::DoubleStarEqual)?
        {
            // Get the current token position as the assignment position.
            let assignment_position =
                parser.get_current_token()?.get_position();

            // Get the current token as the assignment sign.
            let assignment_sign = parser.get_current_token()?.get_token();

            // Read the next token.
            parser.read_next_token()?;

            node_out = Expression::new(
                assignment_position,
                Expressions::Assignment {
                    identifier: Box::new(node_out),
                    sign: assignment_sign,
                    value: Box::new(parse(parser, Precedence::Assignment)?),
                },
            );

            continue;
        }
        // Parse infix:
        // Check if the next token is a plus, minus, star, slash, percent,
        // double star, double equal, not equal, less, less equal, greater,
        // greater equal, double vertical bar or a double amper and read the next token.
        else if parser.expect_token(Token::Plus)?
            || parser.expect_token(Token::Minus)?
            || parser.expect_token(Token::Star)?
            || parser.expect_token(Token::Slash)?
            || parser.expect_token(Token::Percent)?
            || parser.expect_token(Token::DoubleStar)?
            || parser.expect_token(Token::DoubleEqual)?
            || parser.expect_token(Token::NotEqual)?
            || parser.expect_token(Token::Less)?
            || parser.expect_token(Token::LessEqual)?
            || parser.expect_token(Token::Greater)?
            || parser.expect_token(Token::GreaterEqual)?
            || parser.expect_token(Token::DoubleVBar)?
            || parser.expect_token(Token::DoubleAmper)?
        {
            // Get the current token position as the infix position.
            let infix_position = parser.get_current_token()?.get_position();

            // Get hte current token as the infix operator.
            let infix_operator = parser.get_current_token()?.get_token();

            // Get the current precedence as the infix precedence.
            let infix_precendence = parser.get_current_precedence()?;

            // Read the next token.
            parser.read_next_token()?;

            node_out = Expression::new(
                infix_position,
                Expressions::Infix {
                    left: Box::new(node_out),
                    operator: infix_operator,
                    right: Box::new(parse(parser, infix_precendence)?),
                },
            );

            continue;
        }
        // Parse method:
        // Check if the next token is a dot and read the next token.
        else if parser.expect_token(Token::Dot)? {
            // Get the current token position as the method position.
            let method_position = parser.get_current_token()?.get_position();

            // Read the next token.
            parser.read_next_token()?;

            node_out = Expression::new(
                method_position,
                Expressions::Method(
                    Box::new(node_out),
                    Box::new(parse(parser, Precedence::Method)?),
                ),
            );

            continue;
        }

        break;
    }

    Ok(node_out)
}
