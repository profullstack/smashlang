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
    },
    ArrowFunction {
        params: Vec<String>,
        body: Vec<AstNode>,
        expression: bool, // true if it's an expression arrow function (x => x + 1), false if it has a block body (x => { return x + 1; })
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
        self.parse_ternary()
    }
    
    // Parse ternary conditional operator: condition ? expr1 : expr2
    fn parse_ternary(&mut self) -> ParseResult<AstNode> {
        let condition = self.parse_assignment()?;
        
        if matches!(self.peek(), Some(Token::QuestionMark)) {
            self.advance(); // Consume ?
            let true_expr = self.parse_expr()?;
            
            if matches!(self.peek(), Some(Token::Colon)) {
                self.advance(); // Consume :
                let false_expr = self.parse_assignment()?;
                
                return Ok(AstNode::TernaryOp {
                    condition: Box::new(condition),
                    true_expr: Box::new(true_expr),
                    false_expr: Box::new(false_expr),
                });
            } else {
                return Err(ParseError::new("Expected ':' in ternary operator", self.pos));
            }
        }
        
        Ok(condition)
    }
    
    fn parse_expr_statement(&mut self) -> ParseResult<Option<AstNode>> {
        let expr = self.parse_expr()?;
        
        // Expect semicolon after expression statement
        self.expect(&Token::Semicolon)?;
        
        Ok(Some(expr))
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
        }
        
        // Otherwise, parse a primary expression
        let expr = self.parse_primary()?;
        
        // Check for postfix increment/decrement
        if matches!(self.peek(), Some(Token::Increment)) {
            self.advance(); // Consume ++
            return Ok(AstNode::PostIncrement(Box::new(expr)));
        } else if matches!(self.peek(), Some(Token::Decrement)) {
            self.advance(); // Consume --
            return Ok(AstNode::PostDecrement(Box::new(expr)));
        }
        
        Ok(expr)
    }
    
    fn parse_assignment(&mut self) -> ParseResult<AstNode> {
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
            let mut param_count = 0;
            
            // Parse parameters until we find a closing parenthesis
            while !matches!(self.peek(), Some(Token::RParen) | None) {
                if matches!(self.peek(), Some(Token::Identifier(_))) {
                    param_count += 1;
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
        let mut expr = self.parse_unary()?;
        
        while let Some(token) = self.peek() {
            match token {
                Token::BitwiseAnd => {
                    self.advance();
                    let right = self.parse_unary()?;
                    expr = AstNode::BinaryOp {
                        left: Box::new(expr),
                        op: "&".to_string(),
                        right: Box::new(right),
                    };
                },
                Token::BitwiseOr => {
                    self.advance();
                    let right = self.parse_unary()?;
                    expr = AstNode::BinaryOp {
                        left: Box::new(expr),
                        op: "|".to_string(),
                        right: Box::new(right),
                    };
                },
                Token::BitwiseXor => {
                    self.advance();
                    let right = self.parse_unary()?;
                    expr = AstNode::BinaryOp {
                        left: Box::new(expr),
                        op: "^".to_string(),
                        right: Box::new(right),
                    };
                },
                Token::BitwiseLeftShift => {
                    self.advance();
                    let right = self.parse_unary()?;
                    expr = AstNode::BinaryOp {
                        left: Box::new(expr),
                        op: "<<".to_string(),
                        right: Box::new(right),
                    };
                },
                Token::BitwiseRightShift => {
                    self.advance();
                    let right = self.parse_unary()?;
                    expr = AstNode::BinaryOp {
                        left: Box::new(expr),
                        op: ">>".to_string(),
                        right: Box::new(right),
                    };
                },
                Token::BitwiseUnsignedRightShift => {
                    self.advance();
                    let right = self.parse_unary()?;
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
    
    // Parse arrow function expressions
    fn parse_arrow_function(&mut self) -> ParseResult<AstNode> {
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
                if let Some(stmt) = self.parse_statement()? {
                    body.push(stmt);
                }
            }
            
            self.expect(&Token::RBrace)?;
        } else {
            // Expression body: => expr
            is_expression = true;
            let expr = self.parse_expr()?;
            body.push(expr);
        }
        
        Ok(AstNode::ArrowFunction { params, body, expression: is_expression })
    }

    fn parse_primary(&mut self) -> ParseResult<AstNode> {
        match self.peek() {
            Some(Token::Number(n)) => {
                let value = *n;
                self.advance();
                Ok(AstNode::Number(value))
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
            Some(Token::TemplateStringPart(s)) => {
                // This is a part of a template string with interpolation
                let string_value = s.clone();
                self.advance();
                
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
                
                // Check for property access (identifier followed by dot and another identifier)
                let mut expr = AstNode::Identifier(id.clone());
                
                // Process property access chains (obj.prop1.prop2)
                while let Some(Token::Dot) = self.peek() {
                    self.advance(); // Consume the dot
                    
                    // Expect an identifier after the dot
                    if let Some(Token::Identifier(prop)) = self.peek() {
                        let property = prop.clone();
                        self.advance(); // Consume the property identifier
                        expr = AstNode::PropertyAccess {
                            object: Box::new(expr),
                            property,
                        };
                    } else {
                        return Err(ParseError::new("Expected property name after dot", self.pos));
                    }
                }
                
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
                    match &func_expr {
                        AstNode::Identifier(id) => {
                            return Ok(AstNode::FunctionCall { name: id.clone(), args });
                        },
                        AstNode::PropertyAccess { object, property } => {
                            // For method calls (obj.method())
                            return Ok(AstNode::MethodCall { 
                                object: object.clone(), 
                                method: property.clone(),
                                args
                            });
                        },
                        _ => {
                            return Err(ParseError::new("Invalid function call expression", self.pos));
                        }
                    }
                }
                
                // Return the expression (identifier or property access)
                Ok(expr)
            },
            Some(Token::LParen) => {
                self.advance();
                let expr = self.parse_expr()?;
                self.expect(&Token::RParen)?;
                Ok(expr)
            },
            _ => Err(ParseError::new("Expected expression", self.pos)),
        }
    }
}
