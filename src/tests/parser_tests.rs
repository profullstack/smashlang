use crate::lexer::tokenize;
use crate::parser::{Parser, AstNode};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_if_statement() {
        let input = "if (x > 0) { return true; } else { return false; }";
        let tokens = tokenize(input);
        let mut parser = Parser::new(tokens);
        
        let result = parser.parse().unwrap();
        
        // Check that we have one statement
        assert_eq!(result.len(), 1);
        
        // Check that it's an if statement
        match &result[0] {
            AstNode::If { condition, then_branch, else_branch } => {
                // Verify the condition is a binary operation
                match &**condition {
                    AstNode::BinaryOp { left, op, right } => {
                        // Check the left operand is 'x'
                        match &**left {
                            AstNode::Identifier(name) => assert_eq!(name, "x"),
                            _ => panic!("Expected identifier"),
                        }
                        
                        // Check the operator is '>'
                        assert_eq!(op, ">");
                        
                        // Check the right operand is '0'
                        match &**right {
                            AstNode::Number(n) => assert_eq!(*n, 0),
                            _ => panic!("Expected number"),
                        }
                    },
                    _ => panic!("Expected binary operation"),
                }
                
                // Verify the then branch is a block with a return statement
                match &**then_branch {
                    AstNode::Block(stmts) => {
                        assert_eq!(stmts.len(), 1);
                        match &stmts[0] {
                            AstNode::Return(expr) => {
                                match &**expr {
                                    AstNode::Boolean(b) => assert_eq!(*b, true),
                                    _ => panic!("Expected boolean"),
                                }
                            },
                            _ => panic!("Expected return statement"),
                        }
                    },
                    _ => panic!("Expected block"),
                }
                
                // Verify the else branch is a block with a return statement
                match else_branch {
                    Some(branch) => {
                        match &**branch {
                            AstNode::Block(stmts) => {
                                assert_eq!(stmts.len(), 1);
                                match &stmts[0] {
                                    AstNode::Return(expr) => {
                                        match &**expr {
                                            AstNode::Boolean(b) => assert_eq!(*b, false),
                                            _ => panic!("Expected boolean"),
                                        }
                                    },
                                    _ => panic!("Expected return statement"),
                                }
                            },
                            _ => panic!("Expected block"),
                        }
                    },
                    None => panic!("Expected else branch"),
                }
            },
            _ => panic!("Expected if statement"),
        }
    }

    #[test]
    fn test_parse_while_loop() {
        let input = "while (i < 10) { i = i + 1; }";
        let tokens = tokenize(input);
        let mut parser = Parser::new(tokens);
        
        let result = parser.parse().unwrap();
        
        // Check that we have one statement
        assert_eq!(result.len(), 1);
        
        // Check that it's a while loop
        match &result[0] {
            AstNode::While { condition, body } => {
                // Verify the condition is a binary operation
                match &**condition {
                    AstNode::BinaryOp { left, op, right } => {
                        // Check the left operand is 'i'
                        match &**left {
                            AstNode::Identifier(name) => assert_eq!(name, "i"),
                            _ => panic!("Expected identifier"),
                        }
                        
                        // Check the operator is '<'
                        assert_eq!(op, "<");
                        
                        // Check the right operand is '10'
                        match &**right {
                            AstNode::Number(n) => assert_eq!(*n, 10),
                            _ => panic!("Expected number"),
                        }
                    },
                    _ => panic!("Expected binary operation"),
                }
                
                // Verify the body is a block with an assignment
                match &**body {
                    AstNode::Block(stmts) => {
                        assert_eq!(stmts.len(), 1);
                        // Check for assignment statement
                        // This would depend on how assignments are represented in your AST
                    },
                    _ => panic!("Expected block"),
                }
            },
            _ => panic!("Expected while loop"),
        }
    }

    #[test]
    fn test_parse_for_loop() {
        let input = "for (let i = 0; i < 10; i++) { print(i); }";
        let tokens = tokenize(input);
        let mut parser = Parser::new(tokens);
        
        let result = parser.parse().unwrap();
        
        // Check that we have one statement
        assert_eq!(result.len(), 1);
        
        // Check that it's a for loop
        match &result[0] {
            AstNode::For { init, condition, update, body } => {
                // Verify the initialization is a let declaration
                match &**init.as_ref().unwrap() {
                    AstNode::LetDecl { name, value } => {
                        assert_eq!(name, "i");
                        match &**value {
                            AstNode::Number(n) => assert_eq!(*n, 0),
                            _ => panic!("Expected number"),
                        }
                    },
                    _ => panic!("Expected let declaration"),
                }
                
                // Verify the condition is a binary operation
                match &**condition.as_ref().unwrap() {
                    AstNode::BinaryOp { left, op, right } => {
                        // Check the left operand is 'i'
                        match &**left {
                            AstNode::Identifier(name) => assert_eq!(name, "i"),
                            _ => panic!("Expected identifier"),
                        }
                        
                        // Check the operator is '<'
                        assert_eq!(op, "<");
                        
                        // Check the right operand is '10'
                        match &**right {
                            AstNode::Number(n) => assert_eq!(*n, 10),
                            _ => panic!("Expected number"),
                        }
                    },
                    _ => panic!("Expected binary operation"),
                }
                
                // Verify the update is a post-increment
                match &**update.as_ref().unwrap() {
                    AstNode::PostIncrement(expr) => {
                        match &**expr {
                            AstNode::Identifier(name) => assert_eq!(name, "i"),
                            _ => panic!("Expected identifier"),
                        }
                    },
                    _ => panic!("Expected post-increment"),
                }
                
                // Verify the body is a block with a function call
                match &**body {
                    AstNode::Block(stmts) => {
                        assert_eq!(stmts.len(), 1);
                        // Check for function call
                        match &stmts[0] {
                            AstNode::FunctionCall { name, args } => {
                                assert_eq!(name, "print");
                                assert_eq!(args.len(), 1);
                                match &args[0] {
                                    AstNode::Identifier(name) => assert_eq!(name, "i"),
                                    _ => panic!("Expected identifier"),
                                }
                            },
                            _ => panic!("Expected function call"),
                        }
                    },
                    _ => panic!("Expected block"),
                }
            },
            _ => panic!("Expected for loop"),
        }
    }

    #[test]
    fn test_parse_for_in_loop() {
        let input = "for (let key in object) { print(key); }";
        let tokens = tokenize(input);
        let mut parser = Parser::new(tokens);
        
        let result = parser.parse().unwrap();
        
        // Check that we have one statement
        assert_eq!(result.len(), 1);
        
        // Check that it's a for-in loop
        match &result[0] {
            AstNode::ForIn { var_name, object, body } => {
                // Verify the variable name
                assert_eq!(var_name, "key");
                
                // Verify the object is an identifier
                match &**object {
                    AstNode::Identifier(name) => assert_eq!(name, "object"),
                    _ => panic!("Expected identifier"),
                }
                
                // Verify the body is a block with a function call
                match &**body {
                    AstNode::Block(stmts) => {
                        assert_eq!(stmts.len(), 1);
                        // Check for function call
                        match &stmts[0] {
                            AstNode::FunctionCall { name, args } => {
                                assert_eq!(name, "print");
                                assert_eq!(args.len(), 1);
                                match &args[0] {
                                    AstNode::Identifier(name) => assert_eq!(name, "key"),
                                    _ => panic!("Expected identifier"),
                                }
                            },
                            _ => panic!("Expected function call"),
                        }
                    },
                    _ => panic!("Expected block"),
                }
            },
            _ => panic!("Expected for-in loop"),
        }
    }

    #[test]
    fn test_parse_for_of_loop() {
        let input = "for (let item of array) { print(item); }";
        let tokens = tokenize(input);
        let mut parser = Parser::new(tokens);
        
        let result = parser.parse().unwrap();
        
        // Check that we have one statement
        assert_eq!(result.len(), 1);
        
        // Check that it's a for-of loop
        match &result[0] {
            AstNode::ForOf { var_name, iterable, body } => {
                // Verify the variable name
                assert_eq!(var_name, "item");
                
                // Verify the iterable is an identifier
                match &**iterable {
                    AstNode::Identifier(name) => assert_eq!(name, "array"),
                    _ => panic!("Expected identifier"),
                }
                
                // Verify the body is a block with a function call
                match &**body {
                    AstNode::Block(stmts) => {
                        assert_eq!(stmts.len(), 1);
                        // Check for function call
                        match &stmts[0] {
                            AstNode::FunctionCall { name, args } => {
                                assert_eq!(name, "print");
                                assert_eq!(args.len(), 1);
                                match &args[0] {
                                    AstNode::Identifier(name) => assert_eq!(name, "item"),
                                    _ => panic!("Expected identifier"),
                                }
                            },
                            _ => panic!("Expected function call"),
                        }
                    },
                    _ => panic!("Expected block"),
                }
            },
            _ => panic!("Expected for-of loop"),
        }
    }
}