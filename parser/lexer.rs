use crate::{Error, Position, Tok, Token};

#[derive(Clone, Debug)]
pub struct Lexer {
    content: String,

    current_line: usize,
    current_column: usize,
    current_position: usize,
    current_character: Option<char>,

    next_position: usize,
    next_character: Option<char>,
}

impl Lexer {
    /// Create a new lexer object and read the first two characters.
    pub fn new(content: String) -> Self {
        // Create a new mutable lexer object.
        let mut lexer = Self {
            content,

            current_line: 1,
            current_column: 1,
            current_position: 0,
            current_character: None,

            next_position: 0,
            next_character: None,
        };

        // Read the next character.
        lexer.read_next_character();

        // Read the next character.
        lexer.read_next_character();

        // Set the current position to zero.
        lexer.current_position = 0;

        // Set the current column to one.
        lexer.current_column = 1;

        // Return the lexer object.
        lexer
    }

    /// Read the next character of the content and return the current character.
    fn read_next_character(&mut self) -> Option<char> {
        // Get the current character.
        let current_character = self.current_character;

        // Replace the current character with the next character.
        self.current_character = self.next_character;

        // Get the character of the next position and replace the next character with it.
        self.next_character = self.content.chars().skip(self.next_position).next();

        // Append one to the current position.
        self.current_position += 1;

        // Append one to the next position.
        self.next_position += 1;

        // Append one to the current column.
        self.current_column += 1;

        // Return the current character copy.
        current_character
    }

    /// Ignore the whitespaces of the content.
    fn skip_whitespaces(&mut self) {
        loop {
            // Check if the current character is a whitespace.
            if self.current_character == Some(' ') || self.current_character == Some('\t') {
                // Read the next character.
                self.read_next_character();
                continue;
            }

            // End the loop.
            break;
        }
    }

    /// Check if the current character is a number.
    fn is_number_begin(&self) -> bool {
        // Check if the current character exists and is a number.
        self.current_character.is_some() && self.current_character.unwrap().is_numeric()
    }

    /// Check if the current character is a letter or an underscore.
    fn is_identifier_begin(&self) -> bool {
        // Check if the current character exists and is a letter.
        (self.current_character.is_some() && self.current_character.unwrap().is_alphabetic())
            // Check if the current character is an underscore.
            || self.current_character == Some('_')
    }

    fn read_identifier_or_keyword(&mut self) -> Token {
        // Initialize an empty value.
        let mut value = String::new();

        // Check if the current character is the begin of an identifier or a number.
        while self.is_identifier_begin() || self.is_number_begin() {
            // Append the current character to the value and read the next character.
            value.push(self.read_next_character().unwrap());
        }

        // Get the identifier or keywork token.
        Token::get_identifier_or_keyword(value)
    }

    /// Check if the current character is a single quote or a double quote.
    fn is_string_begin(&self) -> bool {
        // Check if the current character is a single quote or a double quote.
        self.current_character == Some('\'') || self.current_character == Some('"')
    }

    fn read_string(&mut self, quote: char) -> Result<Token, Error> {
        let position = Position::new(
            self.current_position,
            self.current_position,
            self.current_line,
            self.current_column,
        );

        // Initialize an empty value.
        let mut value = String::new();

        // Check if the current character is not the initial quote.
        while self.current_character != Some(quote) {
            // Check if the current character is the initial quote.
            if self.current_character == Some(quote) {
                continue;
            }

            // Check if the current character is an end of line or does not exist.
            if self.current_character == Some('\n') || self.current_character == None {
                // Return an error.
                return Err(Error::new_lexical(
                    &position,
                    "You need to close the quote.",
                ));
            }

            // Append the current character and read the next character.
            value.push(self.read_next_character().unwrap());
        }

        // Return the str token.
        Ok(Token::Str { value })
    }

    fn get_next_token(&mut self) -> Result<Tok, Error> {
        // Ignore the whitespaces.
        self.skip_whitespaces();

        // Initialize the position of the token.
        let mut position = Position::new(
            self.current_position,
            self.current_position,
            self.current_line,
            self.current_column,
        );

        // Initialize the token.
        let token: Token;

        // Get the current character.
        match self.current_character {
            // Check if the current character is a dot.
            Some('.') => {
                // Set the token as a dot.
                token = Token::Dot;
            }

            // Check if the current character is a comma.
            Some(',') => {
                // Set the token as a comma.
                token = Token::Comma;
            }

            // Check if the current character is a colon.
            Some(':') => {
                // Set the token as a colon.
                token = Token::Colon;
            }

            // Check if the current character is a semicolon.
            Some(';') => {
                // Set the token as a semicolon.
                token = Token::Semicolon;
            }

            // Check if the current character is an equal and get the next character.
            Some('=') => match self.next_character {
                // Check if the next character is an equal.
                Some('=') => {
                    // Read the next character.
                    self.read_next_character();

                    // Set the token as a double equal.
                    token = Token::DoubleEqual;
                }

                // Is other character
                _ => {
                    // Set the token as an equal.
                    token = Token::Equal;
                }
            },

            // Check if the current character is a plus and get the next character.
            Some('+') => match self.next_character {
                // Check if the next character is an equal.
                Some('=') => {
                    // Read the next character.
                    self.read_next_character();

                    // Set the token as a plus equal.
                    token = Token::PlusEqual;
                }

                // Is other character.
                _ => {
                    // Set the token as a plus.
                    token = Token::Plus;
                }
            },

            // Check if the current character is a minus and get the next character.
            Some('-') => match self.next_character {
                // Check if the next character is an equal.
                Some('=') => {
                    // Read the next character.
                    self.read_next_character();

                    // Set the token as a minus equal.
                    token = Token::MinusEqual;
                }

                // Is other character.
                _ => {
                    // Set the token as a minus.
                    token = Token::Minus;
                }
            },

            // Check if the current character is a star and get the next character.
            Some('*') => match self.next_character {
                // Check if the next character is an equal.
                Some('=') => {
                    // Read the next character.
                    self.read_next_character();

                    // Set the token as a star equal.
                    token = Token::StarEqual;
                }

                // Check if the next character is a star.
                Some('*') => {
                    // Read the next character.
                    self.read_next_character();

                    // Get the next character.
                    match self.next_character {
                        // Check if the next character is an equal.
                        Some('=') => {
                            // Read the next character.
                            self.read_next_character();

                            // Set the token as a double star equal.
                            token = Token::DoubleStarEqual;
                        }

                        // Is other character.
                        _ => {
                            // Set the token as a double star.
                            token = Token::DoubleStar;
                        }
                    }
                }

                // Is other character.
                _ => {
                    // Set the token as a star.
                    token = Token::Star;
                }
            },

            // Check if the current character is a slash and get the next character.
            Some('/') => match self.next_character {
                // Check if the next character is an equal.
                Some('=') => {
                    // Read the next character.
                    self.read_next_character();

                    // Set the token as a slash equal.
                    token = Token::SlashEqual;
                }

                // Is other character.
                _ => {
                    // Set the token as a slash.
                    token = Token::Slash;
                }
            },

            // Check if the current character is a percent and get the next character.
            Some('%') => match self.next_character {
                // Check if the next character is an equal.
                Some('=') => {
                    // Read the next character.
                    self.read_next_character();

                    // Set the token as a percent equal.
                    token = Token::PercentEqual;
                }

                // Is other character.
                _ => {
                    // Set the token as a percent.
                    token = Token::Percent;
                }
            },

            // Check if the current character is a left parentheses.
            Some('(') => {
                // Set the token as a left parentheses.
                token = Token::LeftParentheses;
            }

            // Check if the current character is a right parentheses.
            Some(')') => {
                // Set the token as a right parentheses.
                token = Token::RightParentheses;
            }

            // Check if the current character is a left brace.
            Some('{') => {
                // Set the token as a left brace.
                token = Token::LeftBrace;
            }

            // Check if the current character is a right brace.
            Some('}') => {
                // Set the token as a right brace.
                token = Token::RightBrace;
            }

            // Check if the current character is a left bracket.
            Some('[') => {
                // Set the token as a left bracket.
                token = Token::LeftBracket;
            }

            // Check if the current character is a right bracket.
            Some(']') => {
                // Set the token as a right bracket.
                token = Token::RightBracket;
            }

            // Check if the current character is a vertical bar and get the next character.
            Some('|') => match self.next_character {
                // Check if the next character is a vertical bar.
                Some('|') => {
                    // Read the next character.
                    self.read_next_character();

                    // Set the token as a double vertical bar.
                    token = Token::DoubleVBar;
                }

                // Is other character.
                _ => {
                    // Return an error.
                    return Err(Error::new_lexical(
                        &position,
                        "Maybe do you wish to use `||` instead of `|`?",
                    ));
                }
            },

            // Check if the current character is an amper and get the next character.
            Some('&') => match self.next_character {
                // Check if the next character is an amper.
                Some('&') => {
                    // Read the next character.
                    self.read_next_character();

                    // Set the token as a double amper.
                    token = Token::DoubleAmper;
                }

                // Is other character.
                _ => {
                    // Return an error.
                    return Err(Error::new_lexical(
                        &position,
                        "Maybe do you wish to use `&&` instead of `&`?",
                    ));
                }
            },

            // Check if the current character is an end of line.
            Some('\n') => {
                // Append one to the current line.
                self.current_line += 1;

                // Set the current column to zero.
                self.current_column = 0;

                // Set the token as an end of line.
                token = Token::EndOfLine;
            }

            // Check if the current character is none.
            None => {
                // Set the token as an end of file.
                token = Token::EndOfFile;
            }

            // Is not a sign.
            _ => {
                // Check if the current character is the begin of an identifier.
                if self.is_identifier_begin() {
                    // Set the token as the identifier or keywork token.
                    token = self.read_identifier_or_keyword();
                }
                // Check if the current character is the begin of a string.
                else if self.is_string_begin() {
                    // Set the token as the string token.
                    token = self.read_string(self.current_character.unwrap())?;
                }
                // Is not a valid character.
                else {
                    // Return an error.
                    return Err(Error::new_lexical(&position, "Unknown character."));
                }
            }
        }

        // Set the end position of the initial position as the current position.
        position.set_end_position(self.current_position);

        // Read the next character.
        self.read_next_character();

        // Return the tok object.
        Ok(Tok::new(&position, &token))
    }

    /// # Lexing
    /// Get the tokens of the file or an error.
    pub fn run(&mut self) -> Result<Vec<Tok>, Error> {
        // Initialize the tokens list.
        let mut tokens: Vec<Tok> = Vec::new();

        loop {
            // Get the next token or an error.
            let token = self.get_next_token()?;

            // Append the token to the tokens list.
            tokens.push(token.clone());

            // Check if the token is the end of file.
            if token.get_token() == Token::EndOfFile {
                break;
            }
        }

        // Return the tokens list.
        Ok(tokens)
    }
}
