use crate::{
    ast::{Expression, Expressions},
    parser::{data_types, statements, Statement},
    Error, Parser, Precedence, Token,
};

pub fn parse(parser: &mut Parser, precedence: Precedence) -> Result<Expression, Error> {
    let mut node: Option<Expression> = None;

    if parser.current_token_is(Token::True)? || parser.current_token_is(Token::False)? {
        node = Some(Expression::new(
            parser.get_current_token()?.get_position(),
            Expressions::Boolean(parser.current_token_is(Token::True)?),
        ));
    } else if let Some(identifier_value) = parser.get_current_token()?.get_token().get_identifier()
    {
        let identifier_position = parser.get_current_token()?.get_position();

        if parser.expect_token(Token::Colon)? {
            parser.read_next_token()?;

            let argument_data_type = data_types::parse(parser)?;
            let mut argument_value: Option<Box<Expression>> = None;

            if parser.expect_token(Token::Equal)? {
                parser.read_next_token()?;

                argument_value = Some(Box::new(parse(parser, Precedence::Lowest)?));
            }

            node = Some(Expression::new(
                identifier_position,
                Expressions::Argument {
                    name: identifier_value,
                    data_type: Box::new(argument_data_type),
                    value: argument_value,
                },
            ));
        } else {
            node = Some(Expression::new(
                identifier_position,
                Expressions::Identifier(identifier_value),
            ));
        }
    } else if parser.current_token_is(Token::If)? {
        let if_position = parser.get_current_token()?.get_position();

        if !parser.expect_token(Token::LeftParentheses)? {
            return Err(Error::new_expect_token(
                parser.get_next_token()?.get_position(),
                "(",
                &parser.get_next_token()?.get_token().to_string(),
            ));
        }

        parser.read_next_token()?;

        let if_condition = parse(parser, Precedence::Lowest)?;

        if !parser.expect_token(Token::RightParentheses)? {
            return Err(Error::new_expect_token(
                parser.get_next_token()?.get_position(),
                ")",
                &parser.get_next_token()?.get_token().to_string(),
            ));
        }

        let if_consequence = statements::parse_body(parser)?;

        let mut if_alternative: Vec<Statement> = Vec::new();

        if parser.expect_token(Token::Else)? {
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
    } else if let Some(number_value) = parser.get_current_token()?.get_token().get_number() {
        node = Some(Expression::new(
            parser.get_current_token()?.get_position(),
            Expressions::Number(number_value),
        ));
    } else if parser.current_token_is(Token::Minus)? || parser.current_token_is(Token::Not)? {
        let prefix_position = parser.get_current_token()?.get_position();
        let prefix_token = parser.get_current_token()?.get_token();

        parser.read_next_token()?;

        node = Some(Expression::new(
            prefix_position,
            Expressions::Prefix(prefix_token, Box::new(parse(parser, Precedence::Prefix)?)),
        ));
    } else if let Some(string_value) = parser.get_current_token()?.get_token().get_string() {
        node = Some(Expression::new(
            parser.get_current_token()?.get_position(),
            Expressions::String(string_value),
        ));
    }

    if node.is_none() {
        return Err(Error::new_unknown_token(
            parser.get_current_token()?.get_position(),
        ));
    }

    let mut node_out: Expression = node.unwrap();

    if parser.expect_token(Token::LeftParentheses)? {
        let mut call_arguments: Vec<Expression> = Vec::new();

        parser.read_next_token()?;

        while !parser.current_token_is(Token::RightParentheses)? {
            call_arguments.push(parse(parser, Precedence::Lowest)?);

            parser.expect_token(Token::Comma)?;

            parser.read_next_token()?;
        }

        parser.expect_token(Token::Semicolon)?;

        node_out = Expression::new(
            node_out.get_position(),
            Expressions::Call(Box::new(node_out.clone()), call_arguments),
        );
    }

    while !parser.current_token_is(Token::Semicolon)?
        && precedence < parser.get_next_precedence()?
    {
        if parser.expect_token(Token::Equal)?
            || parser.expect_token(Token::PlusEqual)?
            || parser.expect_token(Token::MinusEqual)?
            || parser.expect_token(Token::StarEqual)?
            || parser.expect_token(Token::SlashEqual)?
            || parser.expect_token(Token::PercentEqual)?
            || parser.expect_token(Token::DoubleStarEqual)?
        {
            let assignment_position = parser.get_current_token()?.get_position();
            let assignment_sign = parser.get_current_token()?.get_token();

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

        if parser.expect_token(Token::Plus)?
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
            let infix_position = parser.get_current_token()?.get_position();
            let infix_operator = parser.get_current_token()?.get_token();
            let infix_precendence = parser.get_current_precedence()?;

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

        if parser.expect_token(Token::Dot)? {
            let method_position = parser.get_current_token()?.get_position();

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
