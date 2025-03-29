use crate::lexer::Token;
use std::collections::HashMap;
use std::fmt;

#[derive(Debug)]
pub enum AstNode {
    // Literals
    Number(i64),
    Float(f64),
    String(String),
    Boolean(bool),
    Null,
    Identifier(String),
    
    // Variable declarations
    LetDecl { name: String, value: Box<AstNode> },
    ConstDecl { name: String, value: Box<AstNode> },
    
    // Destructuring
    ArrayDestructuring { targets: Vec<DestructuringTarget>, value: Box<AstNode> },
    ObjectDestructuring { targets: Vec<DestructuringTarget>, value: Box<AstNode> },
    
    // Operators
    BinaryOp {
        left: Box<AstNode>,
        op: String,
        right: Box<AstNode>,
    },
    
    // Increment/Decrement
    PreIncrement(Box<AstNode>),  // ++x
    PostIncrement(Box<AstNode>), // x++
    PreDecrement(Box<AstNode>),  // --x
    PostDecrement(Box<AstNode>), // x--
    
    // Compound assignments
    CompoundAssignment {
        target: Box<AstNode>,
        op: String,  // +=, -=, *=, /=
        value: Box<AstNode>,
    },
    
    // Functions
    Function {
        name: String,
        params: Vec<String>,
        body: Vec<AstNode>,
    },
    FunctionCall {
        name: String,
        args: Vec<AstNode>,
    },
    
    // Data structures
    ArrayLiteral(Vec<AstNode>),
    ObjectLiteral(HashMap<String, AstNode>),
    SpreadElement(Box<AstNode>),
    
    // Control flow
    Block(Vec<AstNode>),  // Block-level scope { ... }
    Return(Box<AstNode>),
    
    // Modules
    Import(String),
}

#[derive(Debug)]
pub struct ParseError {
    message: String,
    position: usize,
}

impl ParseError {
    fn new(message: &str, position: usize) -> Self {
        ParseError {
            message: message.to_string(),
            position,
        }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Parse error at position {}: {}", self.position, self.message)
    }
}

pub type ParseResult<T> = Result<T, ParseError>;

#[derive(Debug)]
pub struct DestructuringTarget {
    pub name: String,
    pub alias: Option<String>,
    pub default_value: Option<Box<AstNode>>,
    pub is_rest: bool,
}

impl DestructuringTarget {
    pub fn new(name: String) -> Self {
        DestructuringTarget {
            name,
            alias: None,
            default_value: None,
            is_rest: false,
        }
    }
    
    pub fn with_alias(mut self, alias: String) -> Self {
        self.alias = Some(alias);
        self
    }
    
    pub fn with_default(mut self, default_value: AstNode) -> Self {
        self.default_value = Some(Box::new(default_value));
        self
    }
    
    pub fn as_rest(mut self) -> Self {
        self.is_rest = true;
        self
    }
}

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, pos: 0 }
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    fn advance(&mut self) -> Option<&Token> {
        let token = self.tokens.get(self.pos);
        self.pos += 1;
        token
    }

    fn expect(&mut self, expected: &Token) -> ParseResult<()> {
        if let Some(tok) = self.peek() {
            if tok == expected {
                self.advance();
                return Ok(());
            }
            return Err(ParseError::new(
                &format!("Expected {:?}, got {:?}", expected, tok),
                self.pos,
            ));
        }
        Err(ParseError::new("Unexpected end of input", self.pos))
    }

    pub fn parse(&mut self) -> ParseResult<Vec<AstNode>> {
        let mut nodes = Vec::new();

        while self.pos < self.tokens.len() {
            if let Some(node) = self.parse_statement()? {
                nodes.push(node);
            }
        }

        Ok(nodes)
    }

    fn parse_statement(&mut self) -> ParseResult<Option<AstNode>> {
        match self.peek() {
            Some(Token::Import) => self.parse_import(),
            Some(Token::Const) => self.parse_const(),
            Some(Token::Let) => self.parse_let(),
            Some(Token::Fn) => self.parse_function(),
            Some(Token::Return) => self.parse_return(),
            Some(Token::LBrace) => self.parse_block(),
            Some(_) => self.parse_expr().map(Some),
            None => Ok(None),
        }
    }
    
    fn parse_block(&mut self) -> ParseResult<Option<AstNode>> {
        // Consume the opening brace
        self.advance(); // {
        
        let mut statements = Vec::new();
        
        // Parse statements until we hit the closing brace
        while let Some(token) = self.peek() {
            if *token == Token::RBrace {
                break;
            }
            
            if let Some(stmt) = self.parse_statement()? {
                statements.push(stmt);
            }
        }
        
        // Consume the closing brace
        self.expect(&Token::RBrace)?;
        
        Ok(Some(AstNode::Block(statements)))
    }

    fn parse_import(&mut self) -> ParseResult<Option<AstNode>> {
        self.advance(); // consume import
        
        if let Some(Token::String(path)) = self.advance() {
            let node = AstNode::Import(path.clone());
            
            // Expect semicolon
            self.expect(&Token::Semicolon)?;
            
            Ok(Some(node))
        } else {
            Err(ParseError::new("Expected string after import", self.pos))
        }
    }

    fn parse_const(&mut self) -> ParseResult<Option<AstNode>> {
        self.advance(); // consume const
        
        // Clone the token to avoid borrowing issues
        let name_token = self.advance().cloned();
        if let Some(Token::Identifier(name)) = name_token {
            self.expect(&Token::Equal)?;
            
            let expr = self.parse_expr()?;
            
            // Expect semicolon
            self.expect(&Token::Semicolon)?;
            
            Ok(Some(AstNode::ConstDecl {
                name: name,
                value: Box::new(expr),
            }))
        } else {
            Err(ParseError::new("Expected identifier after const", self.pos))
        }
    }

    fn parse_let(&mut self) -> ParseResult<Option<AstNode>> {
        self.advance(); // consume let
        
        // Check if this is a destructuring pattern
        if let Some(Token::LBracket) = self.peek() {
            return self.parse_array_destructuring();
        } else if let Some(Token::LBrace) = self.peek() {
            return self.parse_object_destructuring();
        }
        
        // Regular variable declaration
        // Clone the token to avoid borrowing issues
        let name_token = self.advance().cloned();
        if let Some(Token::Identifier(name)) = name_token {
            self.expect(&Token::Equal)?;
            
            let expr = self.parse_expr()?;
            
            // Expect semicolon
            self.expect(&Token::Semicolon)?;
            
            Ok(Some(AstNode::LetDecl {
                name: name,
                value: Box::new(expr),
            }))
        } else {
            Err(ParseError::new("Expected identifier, array or object pattern after let", self.pos))
        }
    }
    
    fn parse_array_destructuring(&mut self) -> ParseResult<Option<AstNode>> {
        // Skip '['
        self.advance();
        
        let mut targets = Vec::new();
        
        // Parse the destructuring pattern
        while let Some(token) = self.peek() {
            if *token == Token::RBracket {
                break;
            }
            
            // Check for rest element
            if let Some(Token::Ellipsis) = self.peek() {
                self.advance();
                
                if let Some(Token::Identifier(name)) = self.peek() {
                    let name = name.clone();
                    self.advance();
                    
                    targets.push(DestructuringTarget::new(name).as_rest());
                } else {
                    return Err(ParseError::new("Expected identifier after spread operator", self.pos));
                }
            } else if let Some(Token::Identifier(name)) = self.peek() {
                let name = name.clone();
                self.advance();
                
                // Check for default value
                let target = if let Some(Token::Equal) = self.peek() {
                    self.advance();
                    let default_value = self.parse_expr()?;
                    DestructuringTarget::new(name).with_default(default_value)
                } else {
                    DestructuringTarget::new(name)
                };
                
                targets.push(target);
            } else if let Some(Token::Comma) = self.peek() {
                // Skip empty slots
                targets.push(DestructuringTarget::new(String::new()));
            } else {
                return Err(ParseError::new("Expected identifier or spread in array destructuring", self.pos));
            }
            
            // Expect comma between elements, except for the last one
            if let Some(Token::Comma) = self.peek() {
                self.advance();
            } else if let Some(Token::RBracket) = self.peek() {
                // End of pattern
                break;
            } else {
                return Err(ParseError::new("Expected comma or closing bracket in array destructuring", self.pos));
            }
        }
        
        // Skip ']'
        self.expect(&Token::RBracket)?;
        
        // Expect '='
        self.expect(&Token::Equal)?;
        
        // Parse the value expression
        let value = self.parse_expr()?;
        
        // Expect semicolon
        self.expect(&Token::Semicolon)?;
        
        Ok(Some(AstNode::ArrayDestructuring {
            targets,
            value: Box::new(value),
        }))
    }
    
    fn parse_object_destructuring(&mut self) -> ParseResult<Option<AstNode>> {
        // Skip '{'
        self.advance();
        
        let mut targets = Vec::new();
        
        // Parse the destructuring pattern
        while let Some(token) = self.peek() {
            if *token == Token::RBrace {
                break;
            }
            
            // Check for rest element
            if let Some(Token::Ellipsis) = self.peek() {
                self.advance();
                
                if let Some(Token::Identifier(name)) = self.peek() {
                    let name = name.clone();
                    self.advance();
                    
                    targets.push(DestructuringTarget::new(name).as_rest());
                } else {
                    return Err(ParseError::new("Expected identifier after spread operator", self.pos));
                }
            } else if let Some(Token::Identifier(prop)) = self.peek() {
                let prop = prop.clone();
                self.advance();
                
                // Check for property renaming
                let target = if let Some(Token::Colon) = self.peek() {
                    self.advance();
                    
                    if let Some(Token::Identifier(name)) = self.peek() {
                        let name = name.clone();
                        self.advance();
                        
                        // Check for default value
                        if let Some(Token::Equal) = self.peek() {
                            self.advance();
                            let default_value = self.parse_expr()?;
                            DestructuringTarget::new(prop).with_alias(name).with_default(default_value)
                        } else {
                            DestructuringTarget::new(prop).with_alias(name)
                        }
                    } else {
                        return Err(ParseError::new("Expected identifier after colon in object destructuring", self.pos));
                    }
                } else if let Some(Token::Equal) = self.peek() {
                    // Default value without renaming
                    self.advance();
                    let default_value = self.parse_expr()?;
                    DestructuringTarget::new(prop).with_default(default_value)
                } else {
                    // Simple property extraction
                    DestructuringTarget::new(prop)
                };
                
                targets.push(target);
            } else {
                return Err(ParseError::new("Expected property name or spread in object destructuring", self.pos));
            }
            
            // Expect comma between properties, except for the last one
            if let Some(Token::Comma) = self.peek() {
                self.advance();
            } else if let Some(Token::RBrace) = self.peek() {
                // End of pattern
                break;
            } else {
                return Err(ParseError::new("Expected comma or closing brace in object destructuring", self.pos));
            }
        }
        
        // Skip '}'
        self.expect(&Token::RBrace)?;
        
        // Expect '='
        self.expect(&Token::Equal)?;
        
        // Parse the value expression
        let value = self.parse_expr()?;
        
        // Expect semicolon
        self.expect(&Token::Semicolon)?;
        
        Ok(Some(AstNode::ObjectDestructuring {
            targets,
            value: Box::new(value),
        }))
    }

    fn parse_function(&mut self) -> ParseResult<Option<AstNode>> {
        self.advance(); // consume fn
        
        // Clone the token to avoid borrowing issues
        let name_token = self.advance().cloned();
        if let Some(Token::Identifier(name)) = name_token {
            self.expect(&Token::LParen)?;
            
            let mut params = Vec::new();
            
            // Parse parameters
            if let Some(Token::RParen) = self.peek() {
                self.advance(); // consume )
            } else {
                loop {
                    // Clone the token to avoid borrowing issues
                    let param_token = self.advance().cloned();
                    if let Some(Token::Identifier(param)) = param_token {
                        params.push(param);
                    } else {
                        return Err(ParseError::new("Expected parameter name", self.pos));
                    }
                    
                    if let Some(Token::Comma) = self.peek() {
                        self.advance(); // consume ,
                    } else {
                        break;
                    }
                }
                
                self.expect(&Token::RParen)?;
            }
            
            self.expect(&Token::LBrace)?;
            
            let mut body = Vec::new();
            
            // Parse function body
            while let Some(tok) = self.peek() {
                if *tok == Token::RBrace {
                    break;
                }
                
                if let Some(node) = self.parse_statement()? {
                    body.push(node);
                }
            }
            
            self.expect(&Token::RBrace)?;
            
            Ok(Some(AstNode::Function {
                name: name,
                params,
                body,
            }))
        } else {
            Err(ParseError::new("Expected function name", self.pos))
        }
    }

    fn parse_return(&mut self) -> ParseResult<Option<AstNode>> {
        self.advance(); // consume return
        
        let expr = self.parse_expr()?;
        
        // Expect semicolon
        self.expect(&Token::Semicolon)?;
        
        Ok(Some(AstNode::Return(Box::new(expr))))
    }

    fn parse_expr(&mut self) -> ParseResult<AstNode> {
        // Handle pre-increment and pre-decrement operators
        if let Some(token) = self.peek() {
            match token {
                Token::Increment => {
                    self.advance(); // consume ++
                    let expr = self.parse_primary()?;
                    return Ok(AstNode::PreIncrement(Box::new(expr)));
                },
                Token::Decrement => {
                    self.advance(); // consume --
                    let expr = self.parse_primary()?;
                    return Ok(AstNode::PreDecrement(Box::new(expr)));
                },
                _ => {}
            }
        }
        
        // Handle regular expressions and post-increment/decrement
        let mut expr = self.parse_term()?;
        
        // Check for post-increment and post-decrement
        if let Some(token) = self.peek() {
            match token {
                Token::Increment => {
                    self.advance(); // consume ++
                    expr = AstNode::PostIncrement(Box::new(expr));
                },
                Token::Decrement => {
                    self.advance(); // consume --
                    expr = AstNode::PostDecrement(Box::new(expr));
                },
                _ => {}
            }
        }
        
        Ok(expr)
    }

    fn parse_term(&mut self) -> ParseResult<AstNode> {
        let mut left = self.parse_factor()?;
        
        while let Some(tok) = self.peek() {
            match tok {
                Token::Plus | Token::Minus => {
                    let op = match self.advance().unwrap() {
                        Token::Plus => "+".to_string(),
                        Token::Minus => "-".to_string(),
                        _ => unreachable!(),
                    };
                    
                    let right = self.parse_factor()?;
                    
                    left = AstNode::BinaryOp {
                        left: Box::new(left),
                        op,
                        right: Box::new(right),
                    };
                }
                _ => break,
            }
        }
        
        Ok(left)
    }

    fn parse_factor(&mut self) -> ParseResult<AstNode> {
        let mut left = self.parse_primary()?;
        
        while let Some(tok) = self.peek() {
            match tok {
                Token::Star | Token::Slash => {
                    let op = match self.advance().unwrap() {
                        Token::Star => "*".to_string(),
                        Token::Slash => "/".to_string(),
                        _ => unreachable!(),
                    };
                    
                    let right = self.parse_primary()?;
                    
                    left = AstNode::BinaryOp {
                        left: Box::new(left),
                        op,
                        right: Box::new(right),
                    };
                }
                _ => break,
            }
        }
        
        Ok(left)
    }

    fn parse_primary(&mut self) -> ParseResult<AstNode> {
        let token = self.advance().cloned();
        if let Some(tok) = token {
            match tok {
                Token::Number(n) => Ok(AstNode::Number(n)),
                Token::Float(f) => Ok(AstNode::Float(f)),
                Token::String(s) => Ok(AstNode::String(s)),
                Token::Bool(b) => Ok(AstNode::Boolean(b)),
                Token::Null => Ok(AstNode::Null),
                Token::Identifier(name) => {
                    // Clone the name to avoid borrowing issues
                    let name_clone = name.clone();
                    
                    // Check if this is a function call
                    let is_function_call = if let Some(Token::LParen) = self.peek() {
                        true
                    } else {
                        false
                    };
                    
                    if is_function_call {
                        self.advance(); // consume (
                        
                        let mut args = Vec::new();
                        
                        // Parse arguments
                        let has_args = if let Some(Token::RParen) = self.peek() {
                            false
                        } else {
                            true
                        };
                        
                        if !has_args {
                            self.advance(); // consume )
                        } else {
                            loop {
                                let arg = self.parse_expr()?;
                                args.push(arg);
                                
                                let has_more_args = if let Some(Token::Comma) = self.peek() {
                                    self.advance(); // consume ,
                                    true
                                } else {
                                    false
                                };
                                
                                if !has_more_args {
                                    break;
                                }
                            }
                            
                            self.expect(&Token::RParen)?;
                        }
                        
                        Ok(AstNode::FunctionCall {
                            name: name_clone,
                            args,
                        })
                    } else {
                        // Check for compound assignment operators
                        let expr = AstNode::Identifier(name_clone);
                        
                        if let Some(token) = self.peek() {
                            match token {
                                Token::PlusEqual => {
                                    self.advance(); // consume +=
                                    let value = self.parse_expr()?;
                                    return Ok(AstNode::CompoundAssignment {
                                        target: Box::new(expr),
                                        op: "+".to_string(),
                                        value: Box::new(value),
                                    });
                                },
                                Token::MinusEqual => {
                                    self.advance(); // consume -=
                                    let value = self.parse_expr()?;
                                    return Ok(AstNode::CompoundAssignment {
                                        target: Box::new(expr),
                                        op: "-".to_string(),
                                        value: Box::new(value),
                                    });
                                },
                                Token::StarEqual => {
                                    self.advance(); // consume *=
                                    let value = self.parse_expr()?;
                                    return Ok(AstNode::CompoundAssignment {
                                        target: Box::new(expr),
                                        op: "*".to_string(),
                                        value: Box::new(value),
                                    });
                                },
                                Token::SlashEqual => {
                                    self.advance(); // consume /=
                                    let value = self.parse_expr()?;
                                    return Ok(AstNode::CompoundAssignment {
                                        target: Box::new(expr),
                                        op: "/".to_string(),
                                        value: Box::new(value),
                                    });
                                },
                                _ => {}
                            }
                        }
                        
                        Ok(expr)
                    }
                }
                Token::LParen => {
                    let expr = self.parse_expr()?;
                    self.expect(&Token::RParen)?;
                    Ok(expr)
                }
                Token::LBracket => {
                    let mut elements = Vec::new();
                    
                    // Parse array elements
                    if let Some(Token::RBracket) = self.peek() {
                        self.advance(); // consume ]
                    } else {
                        loop {
                            // Check for spread operator
                            if let Some(Token::Ellipsis) = self.peek() {
                                self.advance(); // consume ...
                                let spread_expr = self.parse_expr()?;
                                elements.push(AstNode::SpreadElement(Box::new(spread_expr)));
                            } else {
                                let element = self.parse_expr()?;
                                elements.push(element);
                            }
                            
                            if let Some(Token::Comma) = self.peek() {
                                self.advance(); // consume ,
                            } else {
                                break;
                            }
                        }
                        
                        self.expect(&Token::RBracket)?;
                    }
                    
                    Ok(AstNode::ArrayLiteral(elements))
                }
                Token::LBrace => {
                    let mut properties = HashMap::new();
                    
                    // Parse object properties
                    if let Some(Token::RBrace) = self.peek() {
                        self.advance(); // consume }
                    } else {
                        loop {
                            // Check for spread operator
                            if let Some(Token::Ellipsis) = self.peek() {
                                self.advance(); // consume ...
                                
                                // Parse the expression being spread
                                let spread_expr = self.parse_expr()?;
                                
                                // Add a special marker in the properties map to indicate a spread
                                // In a real implementation, you would need a more sophisticated approach
                                // to handle spread operators in objects, as they can appear anywhere in the object
                                let spread_key = format!("__spread_{}", properties.len());
                                properties.insert(spread_key, AstNode::SpreadElement(Box::new(spread_expr)));
                            } else {
                                let key_token = self.advance().cloned();
                                if let Some(Token::Identifier(key)) = key_token {
                                    // Check if this is a shorthand property (no colon)
                                    if let Some(Token::Colon) = self.peek() {
                                        self.advance(); // consume :
                                        
                                        let value = self.parse_expr()?;
                                        properties.insert(key.clone(), value);
                                    } else {
                                        // Shorthand property syntax: { foo, bar }
                                        // Equivalent to { foo: foo, bar: bar }
                                        properties.insert(key.clone(), AstNode::Identifier(key));
                                    }
                                } else {
                                    return Err(ParseError::new("Expected property name or spread operator", self.pos));
                                }
                            }
                            
                            let has_more_props = if let Some(Token::Comma) = self.peek() {
                                self.advance(); // consume ,
                                true
                            } else {
                                false
                            };
                            
                            if !has_more_props {
                                break;
                            }
                        }
                        
                        self.expect(&Token::RBrace)?;
                    }
                    
                    Ok(AstNode::ObjectLiteral(properties))
                }
                _ => Err(ParseError::new(&format!("Unexpected token: {:?}", tok), self.pos)),
            }
        } else {
            Err(ParseError::new("Unexpected end of input", self.pos))
        }
    }
}
