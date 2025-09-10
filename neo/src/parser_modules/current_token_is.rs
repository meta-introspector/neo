    fn current_token_is(&self, token_type: TokenType) -> bool {
        self.current_token.token_type == token_type
    }
