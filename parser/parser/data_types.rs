use crate::{
    ast::{DataType, DataTypes},
    Error, Parser, Token,
};

pub fn parse(parser: &mut Parser) -> Result<DataType, Error> {
    let mut node: Option<DataType> = None;

    if parser.current_token_is(Token::Boolean)? {
        node = Some(DataType::new(
            parser.get_current_token()?.get_position(),
            DataTypes::Boolean,
        ));
    } else if let Some(identifier_value) =
        parser.get_current_token()?.get_token().get_identifier()
    {
        node = Some(DataType::new(
            parser.get_current_token()?.get_position(),
            DataTypes::Identifier(identifier_value),
        ));
    } else if parser.current_token_is(Token::Number)? {
        node = Some(DataType::new(
            parser.get_current_token()?.get_position(),
            DataTypes::Number,
        ))
    } else if parser.current_token_is(Token::String)? {
        node = Some(DataType::new(
            parser.get_current_token()?.get_position(),
            DataTypes::String,
        ));
    } else if parser.current_token_is(Token::Void)? {
        node = Some(DataType::new(
            parser.get_current_token()?.get_position(),
            DataTypes::Void,
        ));
    }

    if node.is_none() {
        return Err(Error::new_unknown_token(
            parser.get_current_token()?.get_position(),
        ));
    }

    Ok(node.unwrap())
}
