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
