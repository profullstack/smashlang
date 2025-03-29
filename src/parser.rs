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
    
    // Conditional statements
    If {
        condition: Box<AstNode>,
        then_branch: Box<AstNode>,
        else_branch: Option<Box<AstNode>>,
    },
    
    // Loop statements
    While {
        condition: Box<AstNode>,
        body: Box<AstNode>,
    },
    For {
        init: Option<Box<AstNode>>,
        condition: Option<Box<AstNode>>,
        update: Option<Box<AstNode>>,
        body: Box<AstNode>,
    },
    ForIn {
        var_name: String,
        object: Box<AstNode>,
        body: Box<AstNode>,
    },
    ForOf {
        var_name: String,
        iterable: Box<AstNode>,
        body: Box<AstNode>,
    },
    DoWhile {
        body: Box<AstNode>,
        condition: Box<AstNode>,
    },
    
    // Switch statement
    Switch {
        expression: Box<AstNode>,
        cases: Vec<SwitchCase>,
        default: Option<Vec<AstNode>>,
    },
    
    // Error handling
    Try {
        body: Vec<AstNode>,
        catch_param: Option<String>,
        catch_body: Vec<AstNode>,
        finally_body: Option<Vec<AstNode>>,
    },
    Throw(Box<AstNode>),
    NewExpr {
        constructor: String,
        args: Vec<AstNode>,
    },
    
    // Loop control
    Break,
    Continue,
    
    // Modules
    Import(String),
}

#[derive(Debug)]
pub struct SwitchCase {
    pub value: AstNode,
    pub body: Vec<AstNode>,
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
            Some(Token::Try) => self.parse_try_catch(),
            Some(Token::Throw) => self.parse_throw(),
            Some(Token::Break) => self.parse_break(),
            Some(Token::Continue) => self.parse_continue(),
            Some(Token::LBrace) => self.parse_block(),
            Some(Token::If) => self.parse_if(),
            Some(Token::While) => self.parse_while(),
            Some(Token::For) => self.parse_for(),
            Some(Token::Do) => self.parse_do_while(),
            Some(Token::Switch) => self.parse_switch(),
            Some(_) => self.parse_expr().map(Some),
            None => Ok(None),
        }
    }
    
    fn parse_for(&mut self) -> ParseResult<Option<AstNode>> {
        // Consume the for keyword
        self.advance(); // for
        
        // Expect opening parenthesis
        self.expect(&Token::LParen)?;
        
        // Check if this is a for-in or for-of loop
        if let Some(Token::Let) = self.peek() {
            self.advance(); // consume let
            
            // Get the variable name
            if let Some(Token::Identifier(var_name)) = self.peek().cloned() {
                self.advance(); // consume variable name
                
                // Check if this is a for-in loop
                if let Some(Token::In) = self.peek() {
                    self.advance(); // consume 'in'
                    
                    // Parse the object expression
                    let object = self.parse_expr()?;
                    
                    // Expect closing parenthesis
                    self.expect(&Token::RParen)?;
                    
                    // Parse the loop body
                    let body = if let Some(Token::LBrace) = self.peek() {
                        // Block statement
                        if let Some(AstNode::Block(statements)) = self.parse_block()? {
                            AstNode::Block(statements)
                        } else {
                            return Err(ParseError::new("Expected block after for-in header", self.pos));
                        }
                    } else {
                        // Single statement
                        if let Some(stmt) = self.parse_statement()? {
                            stmt
                        } else {
                            return Err(ParseError::new("Expected statement after for-in header", self.pos));
                        }
                    };
                    
                    return Ok(Some(AstNode::ForIn {
                        var_name,
                        object: Box::new(object),
                        body: Box::new(body),
                    }));
                } else if let Some(Token::Of) = self.peek() {
                    self.advance(); // consume 'of'
                    
                    // Parse the iterable expression
                    let iterable = self.parse_expr()?;
                    
                    // Expect closing parenthesis
                    self.expect(&Token::RParen)?;
                    
                    // Parse the loop body
                    let body = if let Some(Token::LBrace) = self.peek() {
                        // Block statement
                        if let Some(AstNode::Block(statements)) = self.parse_block()? {
                            AstNode::Block(statements)
                        } else {
                            return Err(ParseError::new("Expected block after for-of header", self.pos));
                        }
                    } else {
                        // Single statement
                        if let Some(stmt) = self.parse_statement()? {
                            stmt
                        } else {
                            return Err(ParseError::new("Expected statement after for-of header", self.pos));
                        }
                    };
                    
                    return Ok(Some(AstNode::ForOf {
                        var_name,
                        iterable: Box::new(iterable),
                        body: Box::new(body),
                    }));
                }
            }
        }
        
        // Regular for loop
        // Parse initialization
        let init = if let Some(Token::Semicolon) = self.peek() {
            // No initialization
            self.advance(); // ;
            None
        } else if let Some(Token::Let) = self.peek() {
            // Variable declaration
            if let Some(decl) = self.parse_let()? {
                Some(decl)
            } else {
                return Err(ParseError::new("Expected variable declaration in for loop initialization", self.pos));
            }
        } else {
            // Expression
            let expr = self.parse_expr()?;
            self.expect(&Token::Semicolon)?;
            Some(expr)
        };
        
        // Parse condition
        let condition = if let Some(Token::Semicolon) = self.peek() {
            // No condition
            self.advance(); // ;
            None
        } else {
            // Expression
            let expr = self.parse_expr()?;
            self.expect(&Token::Semicolon)?;
            Some(expr)
        };
        
        // Parse update
        let update = if let Some(Token::RParen) = self.peek() {
            // No update
            None
        } else {
            // Expression
            let expr = self.parse_expr()?;
            Some(expr)
        };
        
        // Expect closing parenthesis
        self.expect(&Token::RParen)?;
        
        // Parse the loop body
        let body = if let Some(Token::LBrace) = self.peek() {
            // Block statement
            if let Some(AstNode::Block(statements)) = self.parse_block()? {
                AstNode::Block(statements)
            } else {
                return Err(ParseError::new("Expected block after for loop header", self.pos));
            }
        } else {
            // Single statement
            if let Some(stmt) = self.parse_statement()? {
                stmt
            } else {
                return Err(ParseError::new("Expected statement after for loop header", self.pos));
            }
        };
        
        Ok(Some(AstNode::For {
            init: init.map(Box::new),
            condition: condition.map(Box::new),
            update: update.map(Box::new),
            body: Box::new(body),
        }))
    }
    
    // Placeholder for expression parsing
    fn parse_expr(&mut self) -> ParseResult<AstNode> {
        // This would be implemented based on the existing expression parsing logic
        Ok(AstNode::Null)
    }
}
