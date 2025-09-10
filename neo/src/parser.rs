// src/parser.rs
use crate::lexer::{Lexer, Token, TokenType};
use crate::ast::{TheoryStatement, Expression};

pub struct Parser {
    lexer: Lexer,
    current_token: Token,
    peek_token: Token,
    errors: Vec<String>,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Self {
        let mut p = Parser {
            lexer,
            current_token: Token { token_type: TokenType::Eof, literal: "".to_string() },
            peek_token: Token { token_type: TokenType::Eof, literal: "".to_string() },
            errors: Vec::new(),
        };
        // Read two tokens, so current_token and peek_token are both set
        p.next_token();
        p.next_token();
        p
    }

    fn next_token(&mut self) {
        self.current_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }

    fn expect_peek(&mut self, token_type: TokenType) -> bool {
        // Changed to pass a reference
        if self.peek_token_is(&token_type) {
            self.next_token();
            true
        } else {
            self.peek_error(token_type);
            false
        }
    }

    fn current_token_is(&self, token_type: TokenType) -> bool {
        self.current_token.token_type == token_type
    }

    // Changed to take a reference
    fn peek_token_is(&self, token_type: &TokenType) -> bool {
        &self.peek_token.token_type == token_type
    }

    fn peek_error(&mut self, token_type: TokenType) {
        let msg = format!(
            "Expected next token to be {:?}, got {:?} instead",
            token_type, self.peek_token.token_type
        );
        self.errors.push(msg);
    }

    pub fn parse_theory_statement(&mut self) -> Option<TheoryStatement> {
        // Expect "theory" keyword
        if !self.current_token_is(TokenType::TheoryKeyword) {
            self.errors.push(format!(
                "Expected 'theory' keyword, got {:?} instead",
                self.current_token.token_type
            ));
            return None;
        }
        self.next_token(); // Consume "theory"

        // Expect number (ID)
        let id = if let TokenType::Number(n) = self.current_token.token_type {
            n
        } else {
            self.errors.push(format!(
                "Expected theory ID (number), got {:?} instead",
                self.current_token.token_type
            ));
            return None;
        };
        self.next_token(); // Consume number

        // Expect '='
        if !self.current_token_is(TokenType::Eq) {
            self.errors.push(format!(
                "Expected '=', got {:?} instead",
                self.current_token.token_type
            ));
            return None;
        }
        self.next_token(); // Consume '='

        // Expect '['
        if !self.current_token_is(TokenType::LBracket) {
            self.errors.push(format!(
                "Expected '[', got {:?} instead",
                self.current_token.token_type
            ));
            return None;
        }
        self.next_token(); // Consume '['

        let mut concepts = Vec::new();
        // Parse concepts until ']'
        while !self.current_token_is(TokenType::RBracket) && !self.current_token_is(TokenType::Eof) {
            if let Some(expr) = self.parse_expression() {
                concepts.push(expr);
            } else {
                return None; // Error in parsing expression
            }

            if self.current_token_is(TokenType::Comma) {
                self.next_token(); // Consume comma
            } else if self.current_token_is(TokenType::RBracket) {
                break; // Exit loop, we found the end of the list
            } else {
                self.errors.push("Expected ']'".to_string());
                return None;
            }
        }

        // Expect ']'
        if !self.current_token_is(TokenType::RBracket) {
            self.errors.push("Expected ']'".to_string());
            return None;
        }
        self.next_token(); // Consume ']'

        Some(TheoryStatement { id, concepts })
    }

    fn parse_expression(&mut self) -> Option<Expression> {
        if let TokenType::Identifier(name) = self.current_token.token_type.clone() {
            let identifier_name = name.clone();
            self.next_token(); // Consume identifier

            if self.current_token_is(TokenType::LParen) {
                // It's a function call
                self.next_token(); // Consume '('
                let mut args = Vec::new();
                while !self.current_token_is(TokenType::RParen) && !self.current_token_is(TokenType::Eof) {
                    if let Some(arg_expr) = self.parse_expression() {
                        args.push(arg_expr);
                    } else {
                        return None; // Error in parsing argument
                    }

                    if self.current_token_is(TokenType::Comma) {
                        self.next_token(); // Consume comma
                    } else if self.current_token_is(TokenType::RParen) {
                        break; // Exit loop, we found the end of the argument list
                    } else {
                        self.errors.push("Expected ')'".to_string());
                        return None;
                    }
                }
                if !self.current_token_is(TokenType::RParen) {
                    self.errors.push("Expected ')'".to_string());
                    return None;
                }
                self.next_token(); // Consume ')'
                Some(Expression::Call(identifier_name, args))
            } else {
                // It's just an identifier
                Some(Expression::Identifier(identifier_name))
            }
        } else {
            self.errors.push(format!(
                "Expected identifier, got {:?} instead",
                self.current_token.token_type
            ));
            None
        }
    }

    pub fn errors(&self) -> &[String] {
        &self.errors
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;
    use crate::ast::{TheoryStatement, Expression};

    #[test]
    fn test_parse_theory_statement() {
        let input = "theory123=[concept_one, concept_two(arg1, arg2)]";
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);

        let statement = parser.parse_theory_statement().unwrap();
        assert!(parser.errors().is_empty());

        assert_eq!(statement.id, 123);
        assert_eq!(statement.concepts.len(), 2);

        // Check first concept
        assert_eq!(statement.concepts[0], Expression::Identifier("concept_one".to_string()));

        // Check second concept (call expression)
        let expected_call = Expression::Call(
            "concept_two".to_string(),
            vec![
                Expression::Identifier("arg1".to_string()),
                Expression::Identifier("arg2".to_string()),
            ],
        );
        assert_eq!(statement.concepts[1], expected_call);
    }

    #[test]
    fn test_parse_theory_statement_single_concept() {
        let input = "theory5=[single_concept]";
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);

        let statement = parser.parse_theory_statement().unwrap();
        assert!(parser.errors().is_empty());

        assert_eq!(statement.id, 5);
        assert_eq!(statement.concepts.len(), 1);
        assert_eq!(statement.concepts[0], Expression::Identifier("single_concept".to_string()));
    }

    #[test]
    fn test_parse_theory_statement_empty_concepts() {
        let input = "theory1=[]";
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);

        let statement = parser.parse_theory_statement().unwrap();
        assert!(parser.errors().is_empty());

        assert_eq!(statement.id, 1);
        assert!(statement.concepts.is_empty());
    }

    #[test]
    fn test_parse_theory_statement_error_missing_bracket() {
        let input = "theory1=[concept";
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);

        let _ = parser.parse_theory_statement();
        assert!(!parser.errors().is_empty());
        assert!(parser.errors()[0].contains("Expected ']'"));
    }

    #[test]
    fn test_parse_theory_statement_error_missing_paren() {
        let input = "theory1=[concept(arg";
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);

        let _ = parser.parse_theory_statement();
        assert!(!parser.errors().is_empty());
        assert!(parser.errors()[0].contains("Expected ')'"));
    }
}