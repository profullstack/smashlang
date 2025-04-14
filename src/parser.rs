use crate::lexer::Token;
use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Clone)]
pub enum AstNode {
    // Literals
    Number(i64),
    Float(f64),
    String(String),
    TemplateLiteral(Vec<AstNode>), // Template literal with interpolation
    Regex(String), // Regular expression literal
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
    
    // Unary operators
    UnaryOp {
        op: String,  // !, ~
        expr: Box<AstNode>,
    },
    
    // Ternary operator
    TernaryOp {
        condition: Box<AstNode>,
        true_expr: Box<AstNode>,
        false_expr: Box<AstNode>,
    },
    
    // Property access (e.g., obj.property)
    PropertyAccess {
        object: Box<AstNode>,
        property: String,
    },
    
    // Computed property access (e.g., obj[expr])
    ComputedPropertyAccess {
        object: Box<AstNode>,
        property: Box<AstNode>,
    },
    
    // Method call (e.g., obj.method())
    MethodCall {
        object: Box<AstNode>,
        method: String,
        args: Vec<AstNode>,
    },
    
    // Increment/Decrement
    PreIncrement(Box<AstNode>),  // ++x
    PostIncrement(Box<AstNode>), // x++
    PreDecrement(Box<AstNode>),  // --x
    PostDecrement(Box<AstNode>), // x--
    
    // Assignments
    Assignment {
        name: String,
        value: Box<AstNode>,
    },
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
        is_async: bool,
    },
    ArrowFunction {
        params: Vec<String>,
        body: Vec<AstNode>,
        expression: bool, // true if it's an expression arrow function (x => x + 1), false if it has a block body (x => { return x + 1; })
        is_async: bool,
    },
    AwaitExpr {
        expr: Box<AstNode>,
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

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
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

    fn peek_next(&self) -> Option<&Token> {
        self.tokens.get(self.pos + 1)
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
            Some(Token::Identifier(id)) if id == "function" => {
                self.advance(); // Consume 'function' identifier
                self.parse_function_expression()
            },
            Some(Token::Async) => {
                self.advance(); // Consume Async token
                if let Some(Token::Fn) = self.peek() {
                    // Handle async function with fn keyword
                    self.pos -= 1; // Move back to the Async token
                    self.parse_function() // parse_function will handle the async keyword
                } else if let Some(Token::Identifier(id)) = self.peek() {
                    if id == "function" {
                        // Handle async function with function keyword
                        self.advance(); // Consume 'function' identifier
                        self.parse_async_function_expression()
                    } else {
                        // Handle top-level await expression
                        self.advance(); // Re-consume Async token
                        let expr = self.parse_expr()?;
                        self.expect(&Token::Semicolon)?;
                        Ok(Some(expr))
                    }
                } else {
                    // Handle top-level await expression
                    self.advance(); // Re-consume Async token
                    let expr = self.parse_expr()?;
                    self.expect(&Token::Semicolon)?;
                    Ok(Some(expr))
                }
            },
            Some(Token::Await) => {
                // Handle top-level await expression
                self.advance(); // Consume Await token
                let expr = self.parse_expr()?;
                let await_expr = AstNode::AwaitExpr { expr: Box::new(expr) };
                self.expect(&Token::Semicolon)?;
                Ok(Some(await_expr))
            },
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
            Some(_) => self.parse_expr_statement(),
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
    
    // Parse async JavaScript-style function declarations: async function name() {}
    fn parse_async_function_expression(&mut self) -> ParseResult<Option<AstNode>> {
        // This is an async function
        let is_async = true;
        
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
            
            Ok(Some(AstNode::Function { name, params, body, is_async }))
        } else {
            Err(ParseError::new("Expected function name", self.pos))
        }
    }

    // Parse JavaScript-style function declarations: function name() {}
    fn parse_function_expression(&mut self) -> ParseResult<Option<AstNode>> {
        // Check if this is an async function
        let is_async = false; // JavaScript-style functions don't have async prefix in the same way
        
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
            
            Ok(Some(AstNode::Function { name, params, body, is_async }))
        } else {
            Err(ParseError::new("Expected function name", self.pos))
        }
    }

    // Parse SmashLang-style function declarations: fn name() {}
    fn parse_function(&mut self) -> ParseResult<Option<AstNode>> {
        // Check if this is an async function
        let is_async = matches!(self.peek(), Some(Token::Async));
        if is_async {
            self.advance(); // Consume Async token
            
            // Make sure the next token is Fn
            if !matches!(self.peek(), Some(Token::Fn)) {
                return Err(ParseError::new("Expected 'fn' after 'async'", self.pos));
            }
        }
        
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
            
            Ok(Some(AstNode::Function { name, params, body, is_async }))
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
        
        // Make semicolon optional - try to consume it if it exists
        if matches!(self.peek(), Some(Token::Semicolon)) {
            self.advance(); // Consume the semicolon if present
        }
        
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
        
        // Make semicolon optional - try to consume it if it exists
        if matches!(self.peek(), Some(Token::Semicolon)) {
            self.advance(); // Consume the semicolon if present
        }
        
        Ok(Some(AstNode::Throw(Box::new(expr))))
    }
    
    fn parse_break(&mut self) -> ParseResult<Option<AstNode>> {
        self.advance(); // Consume Break token
        
        // Make semicolon optional - try to consume it if it exists
        if matches!(self.peek(), Some(Token::Semicolon)) {
            self.advance(); // Consume the semicolon if present
        }
        
        Ok(Some(AstNode::Break))
    }
    
    fn parse_continue(&mut self) -> ParseResult<Option<AstNode>> {
        self.advance(); // Consume Continue token
        
        // Make semicolon optional - try to consume it if it exists
        if matches!(self.peek(), Some(Token::Semicolon)) {
            self.advance(); // Consume the semicolon if present
        }
        
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
    
    pub fn parse_do_while(&mut self) -> ParseResult<Option<AstNode>> {
        self.advance(); // Consume Do token
        
        // Parse body
        let body = if let Some(stmt) = self.parse_statement()? {
            stmt
        } else {
            return Err(ParseError::new("Expected statement after do", self.pos));
        };
        
        // Expect while keyword
        self.expect(&Token::While)?;
        
        // Parse condition
        self.expect(&Token::LParen)?;
        let condition = self.parse_expr()?;
        self.expect(&Token::RParen)?;
        
        // Expect semicolon
        self.expect(&Token::Semicolon)?;
        
        Ok(Some(AstNode::DoWhile {
            body: Box::new(body),
            condition: Box::new(condition),
        }))
    }
    
    pub fn parse_switch(&mut self) -> ParseResult<Option<AstNode>> {
        self.advance(); // Consume Switch token
        
        // Parse expression
        self.expect(&Token::LParen)?;
        let expression = self.parse_expr()?;
        self.expect(&Token::RParen)?;
        
        // Parse switch body
        self.expect(&Token::LBrace)?;
        
        let mut cases = Vec::new();
        let mut default = None;
        
        while !matches!(self.peek(), Some(Token::RBrace) | None) {
            match self.peek() {
                Some(Token::Case) => {
                    self.advance(); // Consume Case token
                    
                    // Parse case value
                    let value = self.parse_expr()?;
                    
                    // Expect colon
                    self.expect(&Token::Colon)?;
                    
                    // Parse case body
                    let mut body = Vec::new();
                    
                    // Parse statements until we hit a break, case, default, or end of switch
                    while !matches!(self.peek(), Some(Token::Break) | Some(Token::Case) | Some(Token::Default) | Some(Token::RBrace) | None) {
                        if let Some(stmt) = self.parse_statement()? {
                            body.push(stmt);
                        }
                    }
                    
                    // Skip the break statement if present
                    if matches!(self.peek(), Some(Token::Break)) {
                        self.advance(); // Consume Break token
                        self.expect(&Token::Semicolon)?;
                    }
                    
                    cases.push(SwitchCase { value, body });
                },
                Some(Token::Default) => {
                    self.advance(); // Consume Default token
                    
                    // Expect colon
                    self.expect(&Token::Colon)?;
                    
                    // Parse default body
                    let mut body = Vec::new();
                    
                    // Parse statements until we hit a break or end of switch
                    while !matches!(self.peek(), Some(Token::Break) | Some(Token::Case) | Some(Token::Default) | Some(Token::RBrace) | None) {
                        if let Some(stmt) = self.parse_statement()? {
                            body.push(stmt);
                        }
                    }
                    
                    // Skip the break statement if present
                    if matches!(self.peek(), Some(Token::Break)) {
                        self.advance(); // Consume Break token
                        self.expect(&Token::Semicolon)?;
                    }
                    
                    default = Some(body);
                },
                _ => return Err(ParseError::new("Expected case or default in switch statement", self.pos)),
            }
        }
        
        self.expect(&Token::RBrace)?;
        
        Ok(Some(AstNode::Switch {
            expression: Box::new(expression),
            cases,
            default,
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
    
    // Basic expression parsing
    fn parse_expr(&mut self) -> ParseResult<AstNode> {
        println!("parse_expr: Current token: {:?}", self.peek());
        println!("parse_expr: Current position: {}", self.pos);
        
        // Let parse_assignment (and eventually parse_primary) handle all expressions
        let result = self.parse_assignment();
        println!("parse_expr result: {:?}", result.is_ok());
        result
    }
    
    fn parse_assignment(&mut self) -> ParseResult<AstNode> {
        println!("parse_assignment: Current token: {:?}", self.peek());
        // Check for arrow function
        // Single parameter arrow function: x => ...
        if matches!(self.peek(), Some(Token::Identifier(_))) && matches!(self.peek_next(), Some(Token::FatArrow)) {
            return self.parse_arrow_function();
        }
        
        // Multiple parameters arrow function: (x, y) => ...
        if matches!(self.peek(), Some(Token::LParen)) {
            // Save current position to backtrack if needed
            let current_pos = self.pos;
            let tokens_clone = self.tokens.clone();
            
            // Try to parse as arrow function
            self.advance(); // Consume LParen
            let mut is_arrow_fn = true;
            let mut _param_count = 0;
            
            // Parse parameters until we find a closing parenthesis
            while !matches!(self.peek(), Some(Token::RParen) | None) {
                if matches!(self.peek(), Some(Token::Identifier(_))) {
                    _param_count += 1;
                    self.advance();
                    
                    if matches!(self.peek(), Some(Token::Comma)) {
                        self.advance();
                    } else if !matches!(self.peek(), Some(Token::RParen)) {
                        is_arrow_fn = false;
                        break;
                    }
                } else {
                    is_arrow_fn = false;
                    break;
                }
            }
            
            // Check if we have a closing parenthesis followed by a fat arrow
            if is_arrow_fn && matches!(self.peek(), Some(Token::RParen)) {
                self.advance(); // Consume RParen
                if matches!(self.peek(), Some(Token::FatArrow)) {
                    // Reset position and parse as arrow function
                    self.pos = current_pos;
                    self.tokens = tokens_clone;
                    return self.parse_arrow_function();
                }
            }
            
            // Reset position if not an arrow function
            self.pos = current_pos;
            self.tokens = tokens_clone;
        }
        
        let expr = self.parse_equality()?;
        
        // Handle regular assignment
        if matches!(self.peek(), Some(Token::Equal)) {
            self.advance(); // Consume =
            let value = self.parse_assignment()?;
            
            if let AstNode::Identifier(name) = expr {
                return Ok(AstNode::Assignment {
                    name,
                    value: Box::new(value),
                });
            }
            
            return Err(ParseError::new("Invalid assignment target", self.pos));
        }
        
        // Handle compound assignments (+=, -=, *=, /=)
        if let Some(token) = self.peek() {
            let op = match token {
                Token::PlusEqual => {
                    self.advance(); // Consume +=
                    String::from("+=")
                },
                Token::MinusEqual => {
                    self.advance(); // Consume -=
                    String::from("-=")
                },
                Token::StarEqual => {
                    self.advance(); // Consume *=
                    String::from("*=")
                },
                Token::SlashEqual => {
                    self.advance(); // Consume /=
                    String::from("/=")
                },
                _ => return Ok(expr),
            };
            
            let value = self.parse_assignment()?;
            
            return Ok(AstNode::CompoundAssignment {
                target: Box::new(expr),
                op,
                value: Box::new(value),
            });
        }
        
        Ok(expr)
    }
    
    fn parse_equality(&mut self) -> ParseResult<AstNode> {
        let mut expr = self.parse_comparison()?;
        
        while let Some(token) = self.peek() {
            match token {
                Token::Equal => {
                    self.advance();
                    let right = self.parse_comparison()?;
                    expr = AstNode::BinaryOp {
                        left: Box::new(expr),
                        op: "==".to_string(),
                        right: Box::new(right),
                    };
                },
                Token::StrictEqual => {
                    self.advance();
                    let right = self.parse_comparison()?;
                    expr = AstNode::BinaryOp {
                        left: Box::new(expr),
                        op: "===".to_string(),
                        right: Box::new(right),
                    };
                },
                Token::NotEqual => {
                    self.advance();
                    let right = self.parse_comparison()?;
                    expr = AstNode::BinaryOp {
                        left: Box::new(expr),
                        op: "!=".to_string(),
                        right: Box::new(right),
                    };
                },
                Token::StrictNotEqual => {
                    self.advance();
                    let right = self.parse_comparison()?;
                    expr = AstNode::BinaryOp {
                        left: Box::new(expr),
                        op: "!==".to_string(),
                        right: Box::new(right),
                    };
                },
                _ => break,
            }
        }
        
        Ok(expr)
    }
    
    fn parse_comparison(&mut self) -> ParseResult<AstNode> {
        let mut expr = self.parse_logical()?;
        
        while let Some(token) = self.peek() {
            match token {
                Token::LessThan => {
                    self.advance();
                    let right = self.parse_logical()?;
                    expr = AstNode::BinaryOp {
                        left: Box::new(expr),
                        op: "<".to_string(),
                        right: Box::new(right),
                    };
                },
                Token::GreaterThan => {
                    self.advance();
                    let right = self.parse_logical()?;
                    expr = AstNode::BinaryOp {
                        left: Box::new(expr),
                        op: ">".to_string(),
                        right: Box::new(right),
                    };
                },
                Token::LessThanEqual => {
                    self.advance();
                    let right = self.parse_logical()?;
                    expr = AstNode::BinaryOp {
                        left: Box::new(expr),
                        op: "<=".to_string(),
                        right: Box::new(right),
                    };
                },
                Token::GreaterThanEqual => {
                    self.advance();
                    let right = self.parse_logical()?;
                    expr = AstNode::BinaryOp {
                        left: Box::new(expr),
                        op: ">=".to_string(),
                        right: Box::new(right),
                    };
                },
                _ => break,
            }
        }
        
        Ok(expr)
    }
    
    fn parse_logical(&mut self) -> ParseResult<AstNode> {
        let mut expr = self.parse_bitwise()?;
        
        while let Some(token) = self.peek() {
            match token {
                Token::LogicalAnd => {
                    self.advance();
                    let right = self.parse_bitwise()?;
                    expr = AstNode::BinaryOp {
                        left: Box::new(expr),
                        op: "&&".to_string(),
                        right: Box::new(right),
                    };
                },
                Token::LogicalOr => {
                    self.advance();
                    let right = self.parse_bitwise()?;
                    expr = AstNode::BinaryOp {
                        left: Box::new(expr),
                        op: "||".to_string(),
                        right: Box::new(right),
                    };
                },
                _ => break,
            }
        }
        
        Ok(expr)
    }
    
    fn parse_bitwise(&mut self) -> ParseResult<AstNode> {
        let mut expr = self.parse_additive()?;
        
        while let Some(token) = self.peek() {
            match token {
                Token::BitwiseAnd => {
                    self.advance();
                    let right = self.parse_additive()?;
                    expr = AstNode::BinaryOp {
                        left: Box::new(expr),
                        op: "&".to_string(),
                        right: Box::new(right),
                    };
                },
                Token::BitwiseOr => {
                    self.advance();
                    let right = self.parse_additive()?;
                    expr = AstNode::BinaryOp {
                        left: Box::new(expr),
                        op: "|".to_string(),
                        right: Box::new(right),
                    };
                },
                Token::BitwiseXor => {
                    self.advance();
                    let right = self.parse_additive()?;
                    expr = AstNode::BinaryOp {
                        left: Box::new(expr),
                        op: "^".to_string(),
                        right: Box::new(right),
                    };
                },
                Token::BitwiseLeftShift => {
                    self.advance();
                    let right = self.parse_additive()?;
                    expr = AstNode::BinaryOp {
                        left: Box::new(expr),
                        op: "<<".to_string(),
                        right: Box::new(right),
                    };
                },
                Token::BitwiseRightShift => {
                    self.advance();
                    let right = self.parse_additive()?;
                    expr = AstNode::BinaryOp {
                        left: Box::new(expr),
                        op: ">>".to_string(),
                        right: Box::new(right),
                    };
                },
                Token::BitwiseUnsignedRightShift => {
                    self.advance();
                    let right = self.parse_additive()?;
                    expr = AstNode::BinaryOp {
                        left: Box::new(expr),
                        op: ">>>".to_string(),
                        right: Box::new(right),
                    };
                },
                _ => break,
            }
        }
        
        Ok(expr)
    }
    
    // Parse addition and subtraction operations, including string concatenation
    fn parse_additive(&mut self) -> ParseResult<AstNode> {
        let mut expr = self.parse_unary()?;
        
        while let Some(token) = self.peek() {
            match token {
                Token::Plus => {
                    self.advance(); // Consume +
                    let right = self.parse_unary()?;
                    expr = AstNode::BinaryOp {
                        left: Box::new(expr),
                        op: "+".to_string(),
                        right: Box::new(right),
                    };
                },
                Token::Minus => {
                    self.advance(); // Consume -
                    let right = self.parse_unary()?;
                    expr = AstNode::BinaryOp {
                        left: Box::new(expr),
                        op: "-".to_string(),
                        right: Box::new(right),
                    };
                },
                _ => break,
            }
        }
        
        Ok(expr)
    }
    
    // Parse arrow function expressions
    fn parse_arrow_function(&mut self) -> ParseResult<AstNode> {
        // Check if this is an async arrow function
        let is_async = matches!(self.peek(), Some(Token::Async));
        if is_async {
            self.advance(); // Consume Async token
        }
        
        // Parse parameter list
        let mut params = Vec::new();
        
        if matches!(self.peek(), Some(Token::Identifier(_))) {
            // Single parameter without parentheses: x => ...
            if let Some(Token::Identifier(param)) = self.peek() {
                params.push(param.clone());
                self.advance();
            }
        } else if matches!(self.peek(), Some(Token::LParen)) {
            // Multiple parameters with parentheses: (x, y) => ...
            self.advance(); // Consume LParen
            
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
        } else {
            return Err(ParseError::new("Expected parameter or parameter list", self.pos));
        }
        
        // Expect fat arrow
        self.expect(&Token::FatArrow)?;
        
        // Parse function body
        let mut body = Vec::new();
        let is_expression;
        
        if matches!(self.peek(), Some(Token::LBrace)) {
            // Block body: => { ... }
            is_expression = false;
            self.advance(); // Consume LBrace
            
            while !matches!(self.peek(), Some(Token::RBrace) | None) {
                // Special case for setTimeout in Promise constructors
                if matches!(self.peek(), Some(Token::Identifier(id)) if id == "setTimeout") {
                    self.advance(); // Consume setTimeout
                    
                    // Parse the setTimeout call
                    if let Some(Token::LParen) = self.peek() {
                        self.advance(); // Consume left paren
                        
                        // Parse arguments
                        let mut setTimeout_args = Vec::new();
                        
                        // Parse arguments generically to handle any expression
                        if !matches!(self.peek(), Some(Token::RParen)) {
                            loop {
                                let arg = self.parse_expr()?;
                                setTimeout_args.push(arg);
                                
                                match self.peek() {
                                    Some(Token::Comma) => {
                                        self.advance(); // Consume comma
                                    }
                                    Some(Token::RParen) => break,
                                    _ => return Err(ParseError::new("Expected comma or closing parenthesis in setTimeout arguments", self.pos)),
                                }
                            }
                        }
                        
                        self.expect(&Token::RParen)?; // Consume the right parenthesis
                        
                        // Create function call node
                        let func_call = AstNode::FunctionCall {
                            name: "setTimeout".to_string(),
                            args: setTimeout_args,
                        };
                        
                        body.push(func_call);
                        
                        // Check for semicolon
                        if let Some(Token::Semicolon) = self.peek() {
                            self.advance(); // Consume semicolon
                        }
                    } else {
                        return Err(ParseError::new("Expected opening parenthesis after setTimeout", self.pos));
                    }
                } else {
                    // Try to parse as a statement first
                    if let Ok(Some(stmt)) = self.parse_statement() {
                        body.push(stmt);
                    } else {
                        // If that fails, try to parse as an expression
                        let expr = self.parse_expr()?;
                        body.push(expr);
                        
                        // Check for semicolon
                        if let Some(Token::Semicolon) = self.peek() {
                            self.advance(); // Consume semicolon
                        }
                    }
                }
            }
            
            self.expect(&Token::RBrace)?;
        } else {
            // Expression body: => expr
            is_expression = true;
            let expr = self.parse_expr()?;
            body.push(expr);
        }
        
        Ok(AstNode::ArrowFunction { params, body, expression: is_expression, is_async })
    }

    fn parse_unary(&mut self) -> ParseResult<AstNode> {
        // Check for prefix operators
        if matches!(self.peek(), Some(Token::Increment)) {
            self.advance(); // Consume ++
            let expr = self.parse_primary()?;
            return Ok(AstNode::PreIncrement(Box::new(expr)));
        } else if matches!(self.peek(), Some(Token::Decrement)) {
            self.advance(); // Consume --
            let expr = self.parse_primary()?;
            return Ok(AstNode::PreDecrement(Box::new(expr)));
        } else if matches!(self.peek(), Some(Token::LogicalNot)) {
            self.advance(); // Consume !
            let expr = self.parse_unary()?;
            return Ok(AstNode::UnaryOp {
                op: "!".to_string(),
                expr: Box::new(expr),
            });
        } else if matches!(self.peek(), Some(Token::BitwiseNot)) {
            self.advance(); // Consume ~
            let expr = self.parse_unary()?;
            return Ok(AstNode::UnaryOp {
                op: "~".to_string(),
                expr: Box::new(expr),
            });
        } else if matches!(self.peek(), Some(Token::Await)) {
            self.advance(); // Consume await
            let expr = self.parse_unary()?;
            return Ok(AstNode::AwaitExpr {
                expr: Box::new(expr),
            });
        }
        
        // Otherwise, parse a primary expression
        let expr = self.parse_primary()?;
        
        // Check for postfix increment/decrement
        if matches!(self.peek(), Some(Token::Increment)) {
            self.advance(); // Consume ++
            self.advance(); // Consume --
            return Ok(AstNode::PostDecrement(Box::new(expr)));
        }
        
        Ok(expr)
    }
    
    fn parse_expr_statement(&mut self) -> ParseResult<Option<AstNode>> {
        let expr = self.parse_expr()?;
        
        // Expect semicolon after expression statement
        self.expect(&Token::Semicolon)?;
        
        Ok(Some(expr))
    }
    
    // Process property access and method calls on an expression
    // This handles method chaining like fetch().then().catch()
    fn process_member_access(&mut self, mut expr: AstNode) -> ParseResult<AstNode> {
        loop {
            if let Some(Token::Dot) = self.peek() {
                self.advance(); // Consume the dot
                
                // Expect an identifier after the dot
                if let Some(Token::Identifier(prop)) = self.peek() {
                    let property = prop.clone();
                    self.advance(); // Consume the property identifier
                    
                    // Check if this is a method call (property followed by parentheses)
                    if let Some(Token::LParen) = self.peek() {
                        self.advance(); // Consume the left parenthesis
                        
                        // Parse arguments
                        let mut args = Vec::new();
                        if !matches!(self.peek(), Some(Token::RParen)) {
                            loop {
                                let arg = self.parse_expr()?;
                                args.push(arg);
                                
                                match self.peek() {
                                    Some(Token::Comma) => {
                                        self.advance(); // Consume comma
                                    }
                                    Some(Token::RParen) => break,
                                    _ => return Err(ParseError::new("Expected comma or closing parenthesis", self.pos)),
                                }
                            }
                        }
                        
                        self.expect(&Token::RParen)?; // Consume the right parenthesis
                        
                        // Create a method call node
                        expr = AstNode::MethodCall {
                            object: Box::new(expr),
                            method: property,
                            args,
                        };
                        
                        // Immediately check for further method chaining
                        // This is crucial for Promise chains like fetch().then().catch()
                        continue;
                    } else {
                        // Regular property access
                        expr = AstNode::PropertyAccess {
                            object: Box::new(expr),
                            property,
                        };
                    }
                } else {
                    return Err(ParseError::new("Expected property name after dot", self.pos));
                }
            } else if let Some(Token::LBracket) = self.peek() {
                self.advance(); // Consume the left bracket
                
                // Parse the expression inside the brackets
                let property_expr = self.parse_expr()?;
                
                // Expect a closing bracket
                if let Some(Token::RBracket) = self.peek() {
                    self.advance(); // Consume the right bracket
                    expr = AstNode::ComputedPropertyAccess {
                        object: Box::new(expr),
                        property: Box::new(property_expr),
                    };
                } else {
                    return Err(ParseError::new("Expected closing bracket after property expression", self.pos));
                }
            } else {
                // No more property access, break the loop
                break;
            }
        }
        
        Ok(expr)
    }

    fn parse_primary(&mut self) -> ParseResult<AstNode> {
        println!("parse_primary: Current token: {:?}", self.peek());
        match self.peek() {
            Some(Token::Number(n)) => {
                let value = *n;
                self.advance();
                Ok(AstNode::Number(value))
            },
            Some(Token::Float(f)) => {
                let value = *f;
                self.advance();
                Ok(AstNode::Float(value))
            },
            Some(Token::String(s)) => {
                let value = s.clone();
                self.advance();
                Ok(AstNode::String(value))
            },
            Some(Token::SingleQuoteString(s)) => {
                let value = s.clone();
                self.advance();
                Ok(AstNode::String(value))
            },
            Some(Token::Regex(r)) => {
                println!("Processing regex token: {}", r);
                let value = r.clone();
                self.advance();
                println!("Created AstNode::Regex");
                Ok(AstNode::Regex(value))
            },
            Some(Token::TemplateStringPart(s)) => {
                // This is a part of a template string with interpolation
                let string_value = s.clone();
                self.advance(); // Consume the initial template string part
                
                // Process all parts of the template string
                let mut template_parts = Vec::new();
                template_parts.push(AstNode::String(string_value));
                
                // Continue processing until we reach the end of the template string
                while !matches!(self.peek(), Some(Token::TemplateString(_))) {
                    match self.peek() {
                        Some(Token::TemplateInterpolation(_)) => {
                            // Parse the interpolated expression
                            let tokens = if let Some(Token::TemplateInterpolation(tokens)) = self.peek() {
                                tokens.clone()
                            } else {
                                return Err(ParseError::new("Expected template interpolation", self.pos));
                            };
                            
                            // Create a temporary parser for the interpolation tokens
                            let mut interp_parser = Parser::new(tokens);
                            let expr = interp_parser.parse_expr()?;
                            template_parts.push(expr);
                            self.advance();
                        },
                        Some(Token::TemplateStringPart(s)) => {
                            // Another string part after interpolation
                            template_parts.push(AstNode::String(s.clone()));
                            self.advance();
                        },
                        _ => {
                            return Err(ParseError::new("Unexpected token in template string", self.pos));
                        }
                    }
                }
                
                // Consume the final TemplateString token
                self.advance();
                
                // If there's only one part and it's a string, just return that
                if template_parts.len() == 1 {
                    if let AstNode::String(s) = &template_parts[0] {
                        return Ok(AstNode::String(s.clone()));
                    }
                }
                
                // Otherwise, return a template literal node
                Ok(AstNode::TemplateLiteral(template_parts))
            },
            Some(Token::TemplateString(s)) => {
                // This is a simple template string without interpolation
                let value = s.clone();
                self.advance();
                Ok(AstNode::String(value))
            },
            Some(Token::Bool(value)) => {
                let bool_value = *value;
                self.advance();
                Ok(AstNode::Boolean(bool_value))
            },
            Some(Token::Null) => {
                self.advance();
                Ok(AstNode::Null)
            },
            Some(Token::Identifier(name)) => {
                let id = name.clone();
                self.advance();
                
                // Create identifier node and process any property access or method calls
                let mut expr = AstNode::Identifier(id.clone());
                expr = self.process_member_access(expr)?;
                
                // Check if this is a function call (identifier followed by left parenthesis)
                if let Some(Token::LParen) = self.peek() {
                    // Save the current expression (which might be a property access)
                    let func_expr = expr;
                    self.advance(); // Consume the left parenthesis
                    
                    // Parse arguments
                    let mut args = Vec::new();
                    
                    // Parse arguments list
                    if let Some(Token::RParen) = self.peek() {
                        // Empty argument list
                        self.advance(); // Consume the right parenthesis
                    } else {
                        // Parse comma-separated arguments
                        loop {
                            let arg = self.parse_expr()?;
                            args.push(arg);
                            
                            // Check for comma or closing parenthesis
                            if let Some(Token::Comma) = self.peek() {
                                self.advance(); // Consume the comma
                            } else if let Some(Token::RParen) = self.peek() {
                                self.advance(); // Consume the right parenthesis
                                break;
                            } else {
                                return Err(ParseError::new("Expected comma or closing parenthesis in argument list", self.pos));
                            }
                        }
                    }
                    
                    // Handle different types of function calls
                    let call_expr = match &func_expr {
                        AstNode::Identifier(id) => {
                            AstNode::FunctionCall { name: id.clone(), args }
                        },
                        AstNode::PropertyAccess { object, property } => {
                            // For method calls (obj.method())
                            AstNode::MethodCall { 
                                object: object.clone(), 
                                method: property.clone(),
                                args
                            }
                        },
                        _ => {
                            return Err(ParseError::new("Invalid function call expression", self.pos));
                        }
                    };
                    
                    // Process any method chains on the function call result
                    // This is crucial for Promise chains like fetch().then().catch()
                    let result = self.process_member_access(call_expr)?;
                    return Ok(result)
                }
                
                // Return the expression (identifier or property access)
                Ok(expr)
            },
            Some(Token::New) => {
                self.advance(); // Consume the 'new' keyword
                
                // Parse the constructor name
                if let Some(Token::Identifier(constructor)) = self.peek() {
                    let constructor_name = constructor.clone();
                    self.advance(); // Consume the constructor identifier
                    
                    // Parse constructor arguments
                    let mut args = Vec::new();
                    if let Some(Token::LParen) = self.peek() {
                        self.advance(); // Consume the left parenthesis
                        
                        // Special handling for Promise constructor
                        if constructor_name == "Promise" {
                            println!("Parsing Promise constructor");
                            
                            // Check if we have a function as the argument
                            match self.peek() {
                                // Arrow function syntax: (resolve, reject) => { ... }
                                Some(Token::LParen) => {
                                    // This is likely an arrow function for the Promise executor
                                    self.advance(); // Consume the left parenthesis
                                    
                                    // Parse parameters (resolve, reject)
                                    let mut params = Vec::new();
                                    if !matches!(self.peek(), Some(Token::RParen)) {
                                        loop {
                                            if let Some(Token::Identifier(param)) = self.peek() {
                                                params.push(param.clone());
                                                self.advance(); // Consume the parameter
                                                
                                                // Check for comma or closing parenthesis
                                                if let Some(Token::Comma) = self.peek() {
                                                    self.advance(); // Consume the comma
                                                } else if let Some(Token::RParen) = self.peek() {
                                                    self.advance(); // Consume the right parenthesis
                                                    break;
                                                } else {
                                                    return Err(ParseError::new("Expected comma or closing parenthesis in arrow function parameters", self.pos));
                                                }
                                            } else {
                                                return Err(ParseError::new("Expected identifier in arrow function parameters", self.pos));
                                            }
                                        }
                                    } else {
                                        self.advance(); // Consume the right parenthesis for empty parameters
                                    }
                                    
                                    // Parse the fat arrow
                                    if let Some(Token::FatArrow) = self.peek() {
                                        self.advance(); // Consume the fat arrow
                                     
                                        // Parse the body
                                        let mut body = Vec::new();
                                        
                                        // Check if it's a block body
                                        if let Some(Token::LBrace) = self.peek() {
                                            self.advance(); // Consume the left brace
                                            
                                            // Parse statements until we hit the closing brace
                                            while !matches!(self.peek(), Some(Token::RBrace)) {
                                                // Special case for setTimeout
                                                if matches!(self.peek(), Some(Token::Identifier(id)) if id == "setTimeout") {
                                                    self.advance(); // Consume setTimeout
                                                    
                                                    // Parse the setTimeout call
                                                    if let Some(Token::LParen) = self.peek() {
                                                        self.advance(); // Consume left paren
                                                        
                                                        // Parse arguments
                                                        let mut setTimeout_args = Vec::new();
                                                        
                                                        // Parse arguments
                                                        if !matches!(self.peek(), Some(Token::RParen)) {
                                                            loop {
                                                                let arg = self.parse_expr()?;
                                                                setTimeout_args.push(arg);
                                                                
                                                                match self.peek() {
                                                                    Some(Token::Comma) => {
                                                                        self.advance(); // Consume comma
                                                                    }
                                                                    Some(Token::RParen) => break,
                                                                    _ => return Err(ParseError::new("Expected comma or closing parenthesis in setTimeout arguments", self.pos)),
                                                                }
                                                            }
                                                        }
                                                        
                                                        self.expect(&Token::RParen)?; // Consume the right parenthesis
                                                        
                                                        // Create function call node
                                                        let func_call = AstNode::FunctionCall {
                                                            name: "setTimeout".to_string(),
                                                            args: setTimeout_args,
                                                        };
                                                        
                                                        body.push(func_call);
                                                        
                                                        // Check for semicolon
                                                        if let Some(Token::Semicolon) = self.peek() {
                                                            self.advance(); // Consume semicolon
                                                        }
                                                    } else {
                                                        return Err(ParseError::new("Expected opening parenthesis after setTimeout", self.pos));
                                                    }
                                                } else {
                                                    // Try to parse as a statement first
                                                    if let Ok(Some(stmt)) = self.parse_statement() {
                                                        body.push(stmt);
                                                    } else {
                                                        // If that fails, try to parse as an expression
                                                        let expr = self.parse_expr()?;
                                                        body.push(expr);
                                                        
                                                        // Check for semicolon
                                                        if let Some(Token::Semicolon) = self.peek() {
                                                            self.advance(); // Consume semicolon
                                                        }
                                                    }
                                                }
                                            }
                                            
                                            self.advance(); // Consume the right brace
                                        } else {
                                            // Expression body
                                            let expr = self.parse_expr()?;
                                            body.push(expr);
                                        }
                                        
                                        // Create the arrow function node
                                        let arrow_func = AstNode::ArrowFunction {
                                            params,
                                            body,
                                            expression: true,
                                            is_async: false,
                                        };
                                        
                                        args.push(arrow_func);
                                        
                                        // Consume the closing parenthesis of the Promise constructor
                                        if let Some(Token::RParen) = self.peek() {
                                            self.advance(); // Consume the right parenthesis
                                        } else {
                                            return Err(ParseError::new("Expected closing parenthesis after Promise constructor arguments", self.pos));
                                        }
                                    } else {
                                        return Err(ParseError::new("Expected fat arrow in arrow function", self.pos));
                                    }
                                },
                                // Function expression syntax: function(resolve, reject) { ... }
                                Some(Token::Identifier(id)) if id == "function" => {
                                    self.advance(); // Consume 'function' identifier
                                    
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
                                    
                                    // Create the function expression node
                                    let func_expr = AstNode::ArrowFunction {
                                        params,
                                        body,
                                        expression: false,
                                        is_async: false,
                                    };
                                    
                                    args.push(func_expr);
                                    
                                    // Consume the closing parenthesis of the Promise constructor
                                    if let Some(Token::RParen) = self.peek() {
                                        self.advance(); // Consume the right parenthesis
                                    } else {
                                        return Err(ParseError::new("Expected closing parenthesis after Promise constructor arguments", self.pos));
                                    }
                                },
                                // Regular argument
                                _ => {
                                    if !matches!(self.peek(), Some(Token::RParen)) {
                                        let arg = self.parse_expr()?;
                                        args.push(arg);
                                    }
                                    
                                    // Consume the closing parenthesis
                                    if let Some(Token::RParen) = self.peek() {
                                        self.advance(); // Consume the right parenthesis
                                    } else {
                                        return Err(ParseError::new("Expected closing parenthesis in Promise constructor", self.pos));
                                    }
                                }
                            }
                        } else {
                            // Regular constructor arguments
                            if !matches!(self.peek(), Some(Token::RParen)) {
                                loop {
                                    let arg = self.parse_expr()?;
                                    args.push(arg);
                                    
                                    // Check for comma or closing parenthesis
                                    if let Some(Token::Comma) = self.peek() {
                                        self.advance(); // Consume the comma
                                    } else if let Some(Token::RParen) = self.peek() {
                                        self.advance(); // Consume the right parenthesis
                                        break;
                                    } else {
                                        return Err(ParseError::new("Expected comma or closing parenthesis in constructor arguments", self.pos));
                                    }
                                }
                            } else {
                                self.advance(); // Consume the right parenthesis for empty arguments
                            }
                        }
                    }
                    
                    return Ok(AstNode::NewExpr {
                        constructor: constructor_name,
                        args,
                    });
                } else {
                    return Err(ParseError::new("Expected constructor name after 'new'", self.pos));
                }
            },
            Some(Token::LParen) => {
                self.advance();
                let expr = self.parse_expr()?;
                self.expect(&Token::RParen)?;
                Ok(expr)
            },
            Some(Token::LBracket) => {
                // Parse array literal [elem1, elem2, ...]
                self.advance(); // Consume the left bracket
                
                // Parse array elements
                let mut elements = Vec::new();
                
                // Handle empty array
                if let Some(Token::RBracket) = self.peek() {
                    self.advance(); // Consume the right bracket
                    return Ok(AstNode::ArrayLiteral(elements));
                }
                
                // Parse comma-separated elements
                loop {
                    let element = self.parse_expr()?;
                    elements.push(element);
                    
                    // Check for comma or closing bracket
                    if let Some(Token::Comma) = self.peek() {
                        self.advance(); // Consume the comma
                    } else if let Some(Token::RBracket) = self.peek() {
                        self.advance(); // Consume the right bracket
                        break;
                    } else {
                        return Err(ParseError::new("Expected comma or closing bracket in array literal", self.pos));
                    }
                }
                
                Ok(AstNode::ArrayLiteral(elements))
            },
            Some(Token::LBrace) => {
                // Parse object literal {key1: value1, key2: value2, ...}
                self.advance(); // Consume the left brace
                
                // Parse object properties
                let mut properties = HashMap::new();
                
                // Handle empty object
                if let Some(Token::RBrace) = self.peek() {
                    self.advance(); // Consume the right brace
                    return Ok(AstNode::ObjectLiteral(properties));
                }
                
                // Parse comma-separated key-value pairs
                loop {
                    // Parse the key (identifier or string)
                    let key = match self.peek() {
                        Some(Token::Identifier(name)) => {
                            let key = name.clone();
                            self.advance(); // Consume the identifier
                            key
                        },
                        Some(Token::String(s)) => {
                            let key = s.clone();
                            self.advance(); // Consume the string
                            key
                        },
                        Some(Token::SingleQuoteString(s)) => {
                            let key = s.clone();
                            self.advance(); // Consume the string
                            key
                        },
                        _ => {
                            return Err(ParseError::new("Expected property name in object literal", self.pos));
                        }
                    };
                    
                    // Expect colon after key
                    if let Some(Token::Colon) = self.peek() {
                        self.advance(); // Consume the colon
                    } else {
                        return Err(ParseError::new("Expected colon after property name in object literal", self.pos));
                    }
                    
                    // Parse the value
                    let value = self.parse_expr()?;
                    
                    // Add the key-value pair to the properties map
                    properties.insert(key, value);
                    
                    // Check for comma or closing brace
                    if let Some(Token::Comma) = self.peek() {
                        self.advance(); // Consume the comma
                        // Check if the next token is the closing brace (for trailing comma)
                        if let Some(Token::RBrace) = self.peek() {
                            self.advance(); // Consume the right brace
                            break; // Exit loop after trailing comma
                        }
                        // If not RBrace after comma, continue loop to expect next property
                    } else if let Some(Token::RBrace) = self.peek() {
                        self.advance(); // Consume the right brace
                        break;
                    } else {
                        return Err(ParseError::new("Expected comma or closing brace in object literal", self.pos));
                    }
                }
                
                Ok(AstNode::ObjectLiteral(properties))
            },
            _ => Err(ParseError::new("Expected expression", self.pos)),
        }
    }
}
