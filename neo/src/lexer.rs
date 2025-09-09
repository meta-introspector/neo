// src/lexer.rs
#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    TheoryKeyword,
    Number(u32),
    Eq,
    LBracket,
    RBracket,
    LParen,
    RParen,
    Comma,
    Identifier(String),
    Eof, // End of file
    Illegal, // For unrecognized characters
}

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
}

pub struct Lexer {
    input: Vec<char>,
    position: usize, // current position in input (points to current char)
    read_position: usize, // current reading position in input (after current char)
    ch: Option<char>, // current char under examination
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        let mut lexer = Lexer {
            input: input.chars().collect(),
            position: 0,
            read_position: 0,
            ch: None,
        };
        lexer.read_char(); // Initialize ch
        lexer
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = None;
        } else {
            self.ch = Some(self.input[self.read_position]);
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn peek_char(&self) -> Option<char> {
        if self.read_position >= self.input.len() {
            None
        } else {
            Some(self.input[self.read_position])
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.ch {
            if c.is_ascii_whitespace() {
                self.read_char();
            } else {
                break;
            }
        }
    }

    fn read_identifier(&mut self) -> String {
        let start_pos = self.position;
        while let Some(c) = self.ch {
            if c.is_ascii_alphanumeric() || c == '_' {
                self.read_char();
            } else {
                break;
            }
        }
        self.input[start_pos..self.position].iter().collect()
    }

    fn read_number(&mut self) -> String {
        let start_pos = self.position;
        while let Some(c) = self.ch {
            if c.is_ascii_digit() {
                self.read_char();
            } else {
                break;
            }
        }
        self.input[start_pos..self.position].iter().collect()
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let token = match self.ch {
            Some('=') => new_token(TokenType::Eq, '='),
            Some('[') => new_token(TokenType::LBracket, '['),
            Some(']') => new_token(TokenType::RBracket, ']'),
            Some('(') => new_token(TokenType::LParen, '('),
            Some(')') => new_token(TokenType::RParen, ')'),
            Some(',') => new_token(TokenType::Comma, ','),
            Some(c) => {
                if c.is_ascii_alphabetic() || c == '_' {
                    let literal = self.read_identifier();
                    match literal.as_str() {
                        "theory" => Token {
                            token_type: TokenType::TheoryKeyword,
                            literal,
                        },
                        _ => Token {
                            token_type: TokenType::Identifier(literal.clone()),
                            literal,
                        },
                    }
                } else if c.is_ascii_digit() {
                    let literal = self.read_number();
                    let num = literal.parse::<u32>().unwrap_or(0); // Handle potential parsing errors
                    Token {
                        token_type: TokenType::Number(num),
                        literal,
                    }
                } else {
                    new_token(TokenType::Illegal, c)
                }
            }
            None => Token {
                token_type: TokenType::Eof,
                literal: "".to_string(),
            },
        };

        self.read_char(); // Advance to the next character for the next token
        token
    }
}

fn new_token(token_type: TokenType, ch: char) -> Token {
    Token {
        token_type,
        literal: ch.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_token() {
        let input = "theory123=[concept_one, concept_two(arg1, arg2)]";
        let mut lexer = Lexer::new(input);

        let expected_tokens = vec![
            Token { token_type: TokenType::TheoryKeyword, literal: "theory".to_string() },
            Token { token_type: TokenType::Number(123), literal: "123".to_string() },
            Token { token_type: TokenType::Eq, literal: "=".to_string() },
            Token { token_type: TokenType::LBracket, literal: "[".to_string() },
            Token { token_type: TokenType::Identifier("concept_one".to_string()), literal: "concept_one".to_string() },
            Token { token_type: TokenType::Comma, literal: ",".to_string() },
            Token { token_type: TokenType::Identifier("concept_two".to_string()), literal: "concept_two".to_string() },
            Token { token_type: TokenType::LParen, literal: "(".to_string() },
            Token { token_type: TokenType::Identifier("arg1".to_string()), literal: "arg1".to_string() },
            Token { token_type: TokenType::Comma, literal: ",".to_string() },
            Token { token_type: TokenType::Identifier("arg2".to_string()), literal: "arg2".to_string() },
            Token { token_type: TokenType::RParen, literal: ")".to_string() },
            Token { token_type: TokenType::RBracket, literal: "]".to_string() },
            Token { token_type: TokenType::Eof, literal: "".to_string() },
        ];

        for expected_token in expected_tokens {
            let token = lexer.next_token();
            assert_eq!(token, expected_token);
        }
    }
}
