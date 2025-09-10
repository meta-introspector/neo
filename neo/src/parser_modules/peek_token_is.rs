    fn peek_token_is(&self, token_type: &TokenType) -> bool {
        &self.peek_token.token_type == token_type
    }
