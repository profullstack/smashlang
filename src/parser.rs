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
            Err(ParseError::new("Expected identifier after let", self.pos))
        }
    }

    fn parse_function(&mut self) -> ParseResult<Option<AstNode>> {
        self.advance(); // consume fn
        
        if let Some(Token::Identifier(name)) = self.advance() {
            self.expect(&Token::LParen)?;
            
            let mut params = Vec::new();
            
            // Parse parameters
            if let Some(Token::RParen) = self.peek() {
                self.advance(); // consume )
            } else {
                loop {
                    if let Some(Token::Identifier(param)) = self.advance() {
                        params.push(param.clone());
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
                name: name.clone(),
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
        if let Some(tok) = self.advance() {
            match tok {
                Token::Number(n) => Ok(AstNode::Number(*n)),
                Token::Float(f) => Ok(AstNode::Float(*f)),
                Token::String(s) => Ok(AstNode::String(s.clone())),
                Token::Bool(b) => Ok(AstNode::Boolean(*b)),
                Token::Null => Ok(AstNode::Null),
                Token::Identifier(name) => {
                    // Check if this is a function call
                    if let Some(Token::LParen) = self.peek() {
                        self.advance(); // consume (
                        
                        let mut args = Vec::new();
                        
                        // Parse arguments
                        if let Some(Token::RParen) = self.peek() {
                            self.advance(); // consume )
                        } else {
                            loop {
                                let arg = self.parse_expr()?;
                                args.push(arg);
                                
                                if let Some(Token::Comma) = self.peek() {
                                    self.advance(); // consume ,
                                } else {
                                    break;
                                }
                            }
                            
                            self.expect(&Token::RParen)?;
                        }
                        
                        Ok(AstNode::FunctionCall {
                            name: name.clone(),
                            args,
                        })
                    } else {
                        // Check for compound assignment operators
                        let mut expr = AstNode::Identifier(name.clone());
                        
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
                            let element = self.parse_expr()?;
                            elements.push(element);
                            
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
                            if let Some(Token::Identifier(key)) = self.advance() {
                                self.expect(&Token::Colon)?;
                                
                                let value = self.parse_expr()?;
                                properties.insert(key.clone(), value);
                            } else {
                                return Err(ParseError::new("Expected property name", self.pos));
                            }
                            
                            if let Some(Token::Comma) = self.peek() {
                                self.advance(); // consume ,
                            } else {
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
