    fn peek_error(&mut self, token_type: TokenType) {
        let msg = format!(
            "Expected next token to be {:?}, got {:?} instead",
            token_type, self.peek_token.token_type
        );
        self.errors.push(msg);
    }
