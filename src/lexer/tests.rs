use crate::lexer::{Lexer, Token};

#[test]
fn test_lexer_basic() {
    let input = "let x = 42;";
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize();
    
    assert_eq!(tokens.len(), 5);
    assert!(matches!(tokens[0].token, Token::Let));
    assert!(matches!(tokens[1].token, Token::Identifier(ref s) if s == "x"));
    assert!(matches!(tokens[2].token, Token::Equal));
    assert!(matches!(tokens[3].token, Token::Number(42)));
    assert!(matches!(tokens[4].token, Token::Semicolon));
}

#[test]
fn test_lexer_string_literals() {
    let input = r#"let message = "Hello, world!"; let single = 'Single quotes';"#;
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize();
    
    assert!(matches!(tokens[3].token, Token::String(ref s) if s == "Hello, world!"));
    assert!(matches!(tokens[8].token, Token::SingleQuoteString(ref s) if s == "Single quotes"));
}

#[test]
fn test_lexer_regex() {
    let input = r#"let pattern = /[a-z]+/g;"#;
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize();
    
    assert!(matches!(tokens[3].token, Token::Regex(ref s) if s == "/[a-z]+/g"));
}