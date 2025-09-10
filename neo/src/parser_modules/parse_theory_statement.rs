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
