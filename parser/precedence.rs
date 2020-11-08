use crate::Token;

#[derive(PartialEq, PartialOrd)]
pub enum Precedence {
    Lowest = 0,
    Assignment = 1,
    Equals = 2,
    LessGreater = 3,
    Sum = 4,
    Product = 5,
    Exponent = 6,
    Prefix = 7,
    Call = 8,
    Index = 9,
    Method = 10,
}

impl Precedence {
    pub fn from_token(token: Token) -> Self {
        match token {
            Token::Equal
            | Token::PlusEqual
            | Token::MinusEqual
            | Token::StarEqual
            | Token::SlashEqual
            | Token::PercentEqual
            | Token::DoubleStarEqual => Self::Assignment,

            Token::DoubleEqual | Token::NotEqual | Token::LessEqual | Token::GreaterEqual => {
                Self::Equals
            }

            Token::Less | Token::Greater => Self::LessGreater,

            Token::Plus | Token::Minus => Self::Sum,

            Token::Star | Token::Slash | Token::Percent => Self::Product,

            Token::DoubleStar => Self::Exponent,

            Token::LeftParentheses => Self::Call,

            Token::LeftBracket => Self::Index,

            Token::Dot => Self::Method,

            _ => Self::Lowest,
        }
    }
}
