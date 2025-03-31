use crate::lexer::{tokenize, Token};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize_if_else() {
        let input = "if (x > 0) { return true; } else { return false; }";
        let tokens = tokenize(input);
        
        // Check that the tokens are correctly identified
        assert_eq!(tokens[0], Token::If);
        assert_eq!(tokens[1], Token::LParen);
        assert_eq!(tokens[2], Token::Identifier("x".to_string()));
        // ... more assertions for the rest of the tokens
        
        // Check that we have the correct number of tokens
        assert_eq!(tokens.len(), 17); // Adjust this based on the actual number of tokens
    }

    #[test]
    fn test_tokenize_while_loop() {
        let input = "while (i < 10) { i = i + 1; }";
        let tokens = tokenize(input);
        
        assert_eq!(tokens[0], Token::While);
        assert_eq!(tokens[1], Token::LParen);
        assert_eq!(tokens[2], Token::Identifier("i".to_string()));
        // ... more assertions
        
        assert_eq!(tokens.len(), 14); // Adjust this based on the actual number of tokens
    }

    #[test]
    fn test_tokenize_for_loop() {
        let input = "for (let i = 0; i < 10; i++) { print(i); }";
        let tokens = tokenize(input);
        
        assert_eq!(tokens[0], Token::For);
        assert_eq!(tokens[1], Token::LParen);
        assert_eq!(tokens[2], Token::Let);
        // ... more assertions
        
        assert_eq!(tokens.len(), 20); // Adjust this based on the actual number of tokens
    }

    #[test]
    fn test_tokenize_for_in_loop() {
        let input = "for (let key in object) { print(key); }";
        let tokens = tokenize(input);
        
        assert_eq!(tokens[0], Token::For);
        assert_eq!(tokens[1], Token::LParen);
        assert_eq!(tokens[2], Token::Let);
        assert_eq!(tokens[3], Token::Identifier("key".to_string()));
        assert_eq!(tokens[4], Token::In);
        assert_eq!(tokens[5], Token::Identifier("object".to_string()));
        // ... more assertions
        
        assert_eq!(tokens.len(), 13); // Adjust this based on the actual number of tokens
    }

    #[test]
    fn test_tokenize_for_of_loop() {
        let input = "for (let item of array) { print(item); }";
        let tokens = tokenize(input);
        
        assert_eq!(tokens[0], Token::For);
        assert_eq!(tokens[1], Token::LParen);
        assert_eq!(tokens[2], Token::Let);
        assert_eq!(tokens[3], Token::Identifier("item".to_string()));
        assert_eq!(tokens[4], Token::Of);
        assert_eq!(tokens[5], Token::Identifier("array".to_string()));
        // ... more assertions
        
        assert_eq!(tokens.len(), 13); // Adjust this based on the actual number of tokens
    }

    #[test]
    fn test_tokenize_do_while_loop() {
        let input = "do { i = i + 1; } while (i < 10);";
        let tokens = tokenize(input);
        
        assert_eq!(tokens[0], Token::Do);
        assert_eq!(tokens[1], Token::LBrace);
        // ... more assertions
        
        assert_eq!(tokens.len(), 15); // Adjust this based on the actual number of tokens
    }

    #[test]
    fn test_tokenize_switch_statement() {
        let input = "switch (value) { case 1: break; case 2: break; default: break; }";
        let tokens = tokenize(input);
        
        assert_eq!(tokens[0], Token::Switch);
        assert_eq!(tokens[1], Token::LParen);
        assert_eq!(tokens[2], Token::Identifier("value".to_string()));
        assert_eq!(tokens[3], Token::RParen);
        assert_eq!(tokens[4], Token::LBrace);
        assert_eq!(tokens[5], Token::Case);
        // ... more assertions
        
        assert_eq!(tokens.len(), 21); // Adjust this based on the actual number of tokens
    }
}