use crate::Token;
use std::fmt;

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
    /// Get the precedence of a token.
    /// 
    /// # Example
    /// ```rust
    /// use sflynlang_parser::{Precedence, Token};
    /// 
    /// fn main() {
    ///     println!("Precedence: {}", Precedence::from_token(Token::Equal));
    ///     // Output: Precendence: Assignment
    /// }
    /// ```
    pub fn from_token(token: Token) -> Self {
        match token {
            Token::Equal
            | Token::PlusEqual
            | Token::MinusEqual
            | Token::StarEqual
            | Token::SlashEqual
            | Token::PercentEqual
            | Token::DoubleStarEqual => Self::Assignment,

            Token::DoubleEqual
            | Token::NotEqual
            | Token::LessEqual
            | Token::GreaterEqual => Self::Equals,

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

    /// Convert the precedence to a string.
    pub fn to_string(&self) -> String {
        (match self {
            Self::Lowest => "Lowest",
            Self::Assignment => "Assignment",
            Self::Equals => "Equals",
            Self::LessGreater => "LessGreater",
            Self::Sum => "Sum",
            Self::Product => "Product",
            Self::Exponent => "Exponent",
            Self::Prefix => "Prefix",
            Self::Call => "Call",
            Self::Index => "Index",
            Self::Method => "Method",
        }).to_string()
    }
}

impl fmt::Display for Precedence {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[test]
fn test_precedence() {
    use crate::{Precedence, Token};

    macro_rules! equal_precedence {
        ($token: expr, $expected: expr) => {
            let from_token = Precedence::from_token($token);

            if from_token != $expected {
                panic!("The precedences are not equal:\nToken Precedence: {}\nExpected Precedence: {}", from_token, $expected);
            }
        };
    }

    // Lowest
    equal_precedence!(Token::Identifier(String::from("identifier")), Precedence::Lowest);
    equal_precedence!(Token::Str(String::from("'string'")), Precedence::Lowest);
    equal_precedence!(Token::Num(1.0), Precedence::Lowest);

    equal_precedence!(Token::True, Precedence::Lowest);
    equal_precedence!(Token::False, Precedence::Lowest);

    equal_precedence!(Token::Let, Precedence::Lowest);
    equal_precedence!(Token::Const, Precedence::Lowest);

    equal_precedence!(Token::Func, Precedence::Lowest);
    equal_precedence!(Token::Return, Precedence::Lowest);

    equal_precedence!(Token::If, Precedence::Lowest);
    equal_precedence!(Token::Else, Precedence::Lowest);

    equal_precedence!(Token::Boolean, Precedence::Lowest);
    equal_precedence!(Token::String, Precedence::Lowest);
    equal_precedence!(Token::Number, Precedence::Lowest);
    equal_precedence!(Token::Void, Precedence::Lowest);

    equal_precedence!(Token::Comma, Precedence::Lowest);
    equal_precedence!(Token::Colon, Precedence::Lowest);
    equal_precedence!(Token::Semicolon, Precedence::Lowest);

    equal_precedence!(Token::RightParentheses, Precedence::Lowest);

    equal_precedence!(Token::LeftBrace, Precedence::Lowest);
    equal_precedence!(Token::RightBrace, Precedence::Lowest);

    equal_precedence!(Token::RightBracket, Precedence::Lowest);

    equal_precedence!(Token::DoubleVBar, Precedence::Lowest);
    equal_precedence!(Token::DoubleAmper, Precedence::Lowest);

    equal_precedence!(Token::EndOfLine, Precedence::Lowest);
    equal_precedence!(Token::EndOfFile, Precedence::Lowest);

    // Assignment
    equal_precedence!(Token::Equal, Precedence::Assignment);
    equal_precedence!(Token::PlusEqual, Precedence::Assignment);
    equal_precedence!(Token::MinusEqual, Precedence::Assignment);
    equal_precedence!(Token::StarEqual, Precedence::Assignment);
    equal_precedence!(Token::SlashEqual, Precedence::Assignment);
    equal_precedence!(Token::PercentEqual, Precedence::Assignment);
    equal_precedence!(Token::DoubleStarEqual, Precedence::Assignment);

    // Equals
    equal_precedence!(Token::DoubleEqual, Precedence::Equals);
    equal_precedence!(Token::NotEqual, Precedence::Equals);
    equal_precedence!(Token::LessEqual, Precedence::Equals);
    equal_precedence!(Token::GreaterEqual, Precedence::Equals);

    // LessGreater
    equal_precedence!(Token::Less, Precedence::LessGreater);
    equal_precedence!(Token::Greater, Precedence::LessGreater);

    // Sum
    equal_precedence!(Token::Plus, Precedence::Sum);
    equal_precedence!(Token::Minus, Precedence::Sum);

    // Product
    equal_precedence!(Token::Star, Precedence::Product);
    equal_precedence!(Token::Slash, Precedence::Product);
    equal_precedence!(Token::Percent, Precedence::Product);

    // Exponent
    equal_precedence!(Token::DoubleStar, Precedence::Exponent);

    // Call
    equal_precedence!(Token::LeftParentheses, Precedence::Call);

    // Index
    equal_precedence!(Token::LeftBracket, Precedence::Index);

    // Method
    equal_precedence!(Token::Dot, Precedence::Method);
}
