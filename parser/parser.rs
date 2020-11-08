pub mod data_types;
pub mod expressions;
pub mod statements;

use crate::{ast::Statement, Error, Position, Precedence, Tok, Token};

pub struct Parser {
    tokens: Vec<Tok>,
    current_position: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Tok>) -> Self {
        Self {
            tokens,

            current_position: 0,
        }
    }

    pub fn read_next_token(&mut self) -> Result<Tok, Error> {
        let current_token = self.get_current_token()?;

        self.get_next_token()?;
        self.current_position += 1;

        Ok(current_token)
    }

    fn get_token(&self, position: usize) -> Result<Tok, Error> {
        if position >= self.tokens.len() {
            Err(Error::new_unknown_position(if self.tokens.len() != 0 {
                self.tokens[self.tokens.len() - 1].get_position()
            } else {
                Position::new(0, 0, 1, 1)
            }))
        } else {
            Ok(self.tokens[position].clone())
        }
    }

    pub fn get_current_token(&self) -> Result<Tok, Error> {
        self.get_token(self.current_position)
    }

    pub fn current_token_is(&self, token: Token) -> Result<bool, Error> {
        Ok(self.get_current_token()?.get_token() == token)
    }

    pub fn get_current_precedence(&self) -> Result<Precedence, Error> {
        Ok(Precedence::from_token(
            self.get_current_token()?.get_token(),
        ))
    }

    pub fn get_next_token(&self) -> Result<Tok, Error> {
        self.get_token(self.current_position + 1)
    }

    pub fn next_token_is(&self, token: Token) -> Result<bool, Error> {
        Ok(self.get_next_token()?.get_token() == token)
    }

    pub fn get_next_precedence(&self) -> Result<Precedence, Error> {
        Ok(Precedence::from_token(self.get_next_token()?.get_token()))
    }

    pub fn expect_token(&mut self, token: Token) -> Result<bool, Error> {
        if self.next_token_is(token)? {
            self.read_next_token()?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub fn skip_eol(&mut self) -> Result<(), Error> {
        while self.current_token_is(Token::EndOfLine)? {
            self.read_next_token()?;
        }

        Ok(())
    }

    pub fn run(&mut self) -> Result<Vec<Statement>, Error> {
        let mut statements: Vec<Statement> = Vec::new();

        while self.current_position < self.tokens.len() {
            if self.current_token_is(Token::EndOfFile)? {
                self.current_position += 1;
                break;
            }

            statements.push(statements::parse(self)?);

            self.read_next_token()?;

            self.skip_eol()?;
        }

        Ok(statements)
    }
}
