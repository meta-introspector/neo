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
