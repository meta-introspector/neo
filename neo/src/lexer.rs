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
            read_position: 1, // Corrected: should be 1 initially
            ch: None,
        };
        lexer.ch = lexer.input.get(0).cloned(); // Initialize ch to the first character
        lexer
    }

    fn read_char(&mut self) {
        self.position = self.read_position;
        if self.read_position >= self.input.len() {
            self.ch = None;
        } else {
            self.ch = Some(self.input[self.read_position]);
        }
        self.read_position += 1;
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

    fn read_identifier_literal(&self) -> String {
        let mut current_pos = self.position;
        while current_pos < self.input.len() {
            let c = self.input[current_pos];
            if c.is_ascii_alphanumeric() || c == '_' {
                current_pos += 1;
            } else {
                break;
            }
        }
        self.input[self.position..current_pos].iter().collect()
    }

    fn read_number_literal(&self) -> String {
        let mut current_pos = self.position;
        while current_pos < self.input.len() {
            let c = self.input[current_pos];
            if c.is_ascii_digit() {
                current_pos += 1;
            } else {
                break;
            }
        }
        self.input[self.position..current_pos].iter().collect()
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let token: Token;
        match self.ch {
            Some('=') => { self.read_char(); token = new_token(TokenType::Eq, '='); },
            Some('[') => { self.read_char(); token = new_token(TokenType::LBracket, '['); },
            Some(']') => { self.read_char(); token = new_token(TokenType::RBracket, ']'); },
            Some('(') => { self.read_char(); token = new_token(TokenType::LParen, '('); },
            Some(')') => { self.read_char(); token = new_token(TokenType::RParen, ')'); },
            Some(',') => { self.read_char(); token = new_token(TokenType::Comma, ','); },
            Some(c) => {
                if c.is_ascii_digit() { // Prioritize numbers
                    let literal = self.read_number_literal();
                    let num = literal.parse::<u32>().unwrap_or(0);
                    let consumed_len = literal.len();
                    token = Token {
                        token_type: TokenType::Number(num),
                        literal,
                    };
                    self.position += consumed_len;
                    self.read_position = self.position + 1; // Corrected
                    self.ch = self.input.get(self.position).cloned(); // Update self.ch directly
                    return token;
                } else if c.is_ascii_alphabetic() || c == '_' {
                    let literal = self.read_identifier_literal();
                    // Check for "theory" keyword followed by a number
                    if literal.starts_with("theory") && literal.len() > "theory".len() {
                        let potential_number_str = &literal["theory".len()..];
                        if potential_number_str.chars().all(|c| c.is_ascii_digit()) {
                            // It's "theory" followed by a number.
                            // Return TheoryKeyword, and adjust position for the number to be next.
                            self.position += "theory".len();
                            self.read_position = self.position + 1; // Corrected
                            self.ch = self.input.get(self.position).cloned(); // Update self.ch directly
                            return Token { token_type: TokenType::TheoryKeyword, literal: "theory".to_string() };
                        }
                    }
                    // If not "theory" followed by a number, or a general identifier
                    let consumed_len = literal.len();
                    token = if literal == "theory" { // This handles "theory" by itself
                        Token { token_type: TokenType::TheoryKeyword, literal }
                    } else {
                        Token { token_type: TokenType::Identifier(literal.clone()), literal }
                    };
                    self.position += consumed_len;
                    self.read_position = self.position + 1; // Corrected
                    self.ch = self.input.get(self.position).cloned(); // Update self.ch directly
                    return token;
                } else {
                    self.read_char(); // Advance for illegal char
                    token = new_token(TokenType::Illegal, c);
                }
            }
            None => token = Token { // Corrected: Eof literal should be empty string
                token_type: TokenType::Eof,
                literal: "".to_string(),
            },
        };
        token // Return the token
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
