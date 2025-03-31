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
    
    fn parse_import(&mut self) -> ParseResult<Option<AstNode>> {
        self.advance(); // Consume Import token
        
        // Parse the module path as a string
        if let Some(Token::String(path)) = self.peek() {
            let path = path.clone();
            self.advance();
            
            // Expect semicolon
            self.expect(&Token::Semicolon)?;
            
            Ok(Some(AstNode::Import(path)))
        } else {
            Err(ParseError::new("Expected string literal after import", self.pos))
        }
    }
    
    fn parse_const(&mut self) -> ParseResult<Option<AstNode>> {
        self.advance(); // Consume Const token
        
        // Parse identifier
        if let Some(Token::Identifier(name)) = self.peek() {
            let name = name.clone();
            self.advance();
            
            // Expect assignment operator
            self.expect(&Token::Equal)?;
            
            // Parse the value expression
            let value = self.parse_expr()?;
            
            // Expect semicolon
            self.expect(&Token::Semicolon)?;
            
            Ok(Some(AstNode::ConstDecl { name, value: Box::new(value) }))
        } else {
            Err(ParseError::new("Expected identifier after const", self.pos))
        }
    }
    
    fn parse_let(&mut self) -> ParseResult<Option<AstNode>> {
        self.advance(); // Consume Let token
        
        // Parse identifier
        if let Some(Token::Identifier(name)) = self.peek() {
            let name = name.clone();
            self.advance();
            
            // Expect assignment operator
            self.expect(&Token::Equal)?;
            
            // Parse the value expression
            let value = self.parse_expr()?;
            
            // Expect semicolon
            self.expect(&Token::Semicolon)?;
            
            Ok(Some(AstNode::LetDecl { name, value: Box::new(value) }))
        } else {
            Err(ParseError::new("Expected identifier after let", self.pos))
        }
    }
    
    fn parse_function(&mut self) -> ParseResult<Option<AstNode>> {
        self.advance(); // Consume Fn token
        
        // Parse function name
        if let Some(Token::Identifier(name)) = self.peek() {
            let name = name.clone();
            self.advance();
            
            // Parse parameter list
            self.expect(&Token::LParen)?;
            let mut params = Vec::new();
            
            if !matches!(self.peek(), Some(Token::RParen)) {
                loop {
                    if let Some(Token::Identifier(param)) = self.peek() {
                        params.push(param.clone());
                        self.advance();
                    } else {
                        return Err(ParseError::new("Expected parameter name", self.pos));
                    }
                    
                    match self.peek() {
                        Some(Token::Comma) => {
                            self.advance();
                        }
                        Some(Token::RParen) => break,
                        _ => return Err(ParseError::new("Expected comma or closing parenthesis", self.pos)),
                    }
                }
            }
            
            self.expect(&Token::RParen)?;
            
            // Parse function body
            self.expect(&Token::LBrace)?;
            let mut body = Vec::new();
            
            while !matches!(self.peek(), Some(Token::RBrace) | None) {
                if let Some(stmt) = self.parse_statement()? {
                    body.push(stmt);
                }
            }
            
            self.expect(&Token::RBrace)?;
            
            Ok(Some(AstNode::Function { name, params, body }))
        } else {
            Err(ParseError::new("Expected function name", self.pos))
        }
    }
    
    fn parse_return(&mut self) -> ParseResult<Option<AstNode>> {
        self.advance(); // Consume Return token
        
        // Check if there's a return value
        let value = if !matches!(self.peek(), Some(Token::Semicolon)) {
            self.parse_expr()?
        } else {
            AstNode::Null
        };
        
        // Expect semicolon
        self.expect(&Token::Semicolon)?;
        
        Ok(Some(AstNode::Return(Box::new(value))))
    }
    
    fn parse_try_catch(&mut self) -> ParseResult<Option<AstNode>> {
        self.advance(); // Consume Try token
        
        // Parse try block
        self.expect(&Token::LBrace)?;
        let mut try_body = Vec::new();
        
        while !matches!(self.peek(), Some(Token::RBrace) | None) {
            if let Some(stmt) = self.parse_statement()? {
                try_body.push(stmt);
            }
        }
        
        self.expect(&Token::RBrace)?;
        
        // Parse catch block
        self.expect(&Token::Catch)?;
        
        // Parse optional catch parameter
        let catch_param = if matches!(self.peek(), Some(Token::LParen)) {
            self.advance(); // Consume LParen
            
            if let Some(Token::Identifier(param)) = self.peek() {
                let param = param.clone();
                self.advance();
                self.expect(&Token::RParen)?;
                Some(param)
            } else {
                return Err(ParseError::new("Expected identifier in catch clause", self.pos));
            }
        } else {
            None
        };
        
        // Parse catch body
        self.expect(&Token::LBrace)?;
        let mut catch_body = Vec::new();
        
        while !matches!(self.peek(), Some(Token::RBrace) | None) {
            if let Some(stmt) = self.parse_statement()? {
                catch_body.push(stmt);
            }
        }
        
        self.expect(&Token::RBrace)?;
        
        // Parse optional finally block
        let finally_body = if matches!(self.peek(), Some(Token::Finally)) {
            self.advance(); // Consume Finally token
            
            self.expect(&Token::LBrace)?;
            let mut finally_stmts = Vec::new();
            
            while !matches!(self.peek(), Some(Token::RBrace) | None) {
                if let Some(stmt) = self.parse_statement()? {
                    finally_stmts.push(stmt);
                }
            }
            
            self.expect(&Token::RBrace)?;
            Some(finally_stmts)
        } else {
            None
        };
        
        Ok(Some(AstNode::Try {
            body: try_body,
            catch_param,
            catch_body,
            finally_body,
        }))
    }
    
    fn parse_throw(&mut self) -> ParseResult<Option<AstNode>> {
        self.advance(); // Consume Throw token
        
        // Parse the expression to throw
        let expr = self.parse_expr()?;
        
        // Expect semicolon
        self.expect(&Token::Semicolon)?;
        
        Ok(Some(AstNode::Throw(Box::new(expr))))
    }
    
    fn parse_break(&mut self) -> ParseResult<Option<AstNode>> {
        self.advance(); // Consume Break token
        
        // Expect semicolon
        self.expect(&Token::Semicolon)?;
        
        Ok(Some(AstNode::Break))
    }
    
    fn parse_continue(&mut self) -> ParseResult<Option<AstNode>> {
        self.advance(); // Consume Continue token
        
        // Expect semicolon
        self.expect(&Token::Semicolon)?;
        
        Ok(Some(AstNode::Continue))
    }
    
    fn parse_block(&mut self) -> ParseResult<Option<AstNode>> {
        self.advance(); // Consume LBrace token
        
        let mut statements = Vec::new();
        
        while !matches!(self.peek(), Some(Token::RBrace) | None) {
            if let Some(stmt) = self.parse_statement()? {
                statements.push(stmt);
            }
        }
        
        self.expect(&Token::RBrace)?;
        
        Ok(Some(AstNode::Block(statements)))
    }
    
    fn parse_if(&mut self) -> ParseResult<Option<AstNode>> {
        self.advance(); // Consume If token
        
        // Parse condition
        self.expect(&Token::LParen)?;
        let condition = self.parse_expr()?;
        self.expect(&Token::RParen)?;
        
        // Parse then branch
        let then_branch = if let Some(stmt) = self.parse_statement()? {
            stmt
        } else {
            return Err(ParseError::new("Expected statement after if condition", self.pos));
        };
        
        // Parse optional else branch
        let else_branch = if matches!(self.peek(), Some(Token::Else)) {
            self.advance(); // Consume Else token
            
            if let Some(stmt) = self.parse_statement()? {
                Some(Box::new(stmt))
            } else {
                return Err(ParseError::new("Expected statement after else", self.pos));
            }
        } else {
            None
        };
        
        Ok(Some(AstNode::If {
            condition: Box::new(condition),
            then_branch: Box::new(then_branch),
            else_branch,
        }))
    }
    
    fn parse_while(&mut self) -> ParseResult<Option<AstNode>> {
        self.advance(); // Consume While token
        
        // Parse condition
        self.expect(&Token::LParen)?;
        let condition = self.parse_expr()?;
        self.expect(&Token::RParen)?;
        
        // Parse body
        let body = if let Some(stmt) = self.parse_statement()? {
            stmt
        } else {
            return Err(ParseError::new("Expected statement after while condition", self.pos));
        };
        
        Ok(Some(AstNode::While {
            condition: Box::new(condition),
            body: Box::new(body),
        }))
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
