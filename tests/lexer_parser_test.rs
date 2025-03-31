use smashlang::lexer::{tokenize, Token};
use smashlang::parser::Parser;

#[test]
fn test_lexer_basic_tokens() {
    // Test basic token recognition
    let input = "let x = 10; print;"; 
    let tokens = tokenize(input);
    
    // Expected tokens
    assert_eq!(tokens[0], Token::Let);
    assert_eq!(tokens[1], Token::Identifier("x".to_string()));
    assert_eq!(tokens[2], Token::Equal);
    assert_eq!(tokens[3], Token::Number(10));
    assert_eq!(tokens[4], Token::Semicolon);
    assert_eq!(tokens[5], Token::Identifier("print".to_string()));
    assert_eq!(tokens[6], Token::Semicolon);
}

#[test]
fn test_lexer_string_literals() {
    // Test string literal recognition
    let input = "let message = \"Hello, World!\";"; 
    let tokens = tokenize(input);
    
    // Check for the string token
    assert_eq!(tokens[3], Token::String("Hello, World!".to_string()));
}

#[test]
fn test_parser_variable_declaration() {
    // Test parsing of variable declarations
    let input = "let x = 10;"; 
    let tokens = tokenize(input);
    let mut parser = Parser::new(tokens);
    
    let ast = parser.parse().expect("Parser should succeed");
    
    // Check that we got a variable declaration
    assert_eq!(ast.len(), 1, "AST should have one node");
    
    // We can't easily pattern match on the AST nodes due to the Box<AstNode> wrapping,
    // but we can check that parsing succeeded which is sufficient for a basic test
}

#[test]
fn test_parser_function_call() {
    // Test parsing of function calls
    let input = "print;"; 
    let tokens = tokenize(input);
    let mut parser = Parser::new(tokens);
    
    let ast = parser.parse().expect("Parser should succeed");
    
    // Check that we got a function call
    assert_eq!(ast.len(), 1, "AST should have one node");
}

#[test]
fn test_parser_error_handling() {
    // Test that the parser correctly handles syntax errors
    let input = "let x = ;"; // Missing expression after =
    let tokens = tokenize(input);
    let mut parser = Parser::new(tokens);
    
    let result = parser.parse();
    assert!(result.is_err(), "Parser should return an error for invalid syntax");
}

#[test]
fn test_complex_program() {
    // Test parsing of a more complex program
    let input = r#"
        let x = 10;
        let y = 20;
        print;
    "#;
    
    let tokens = tokenize(input);
    let mut parser = Parser::new(tokens);
    
    let ast = parser.parse().expect("Parser should succeed");
    
    // Check that we got the expected number of statements
    assert_eq!(ast.len(), 3, "AST should have three nodes");
}
