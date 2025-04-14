use pest_derive::Parser;
use std::collections::HashMap;
use std::fmt;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct SmashParser;

#[derive(Debug, Clone)]
pub enum AstNode {
    // Literals
    Number(i64),
    Float(f64),
    String(String),
    TemplateLiteral(Vec<AstNode>),
    Regex(String),
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
        op: String,
        expr: Box<AstNode>,
    },
    
    // Ternary operator
    TernaryOp {
        condition: Box<AstNode>,
        true_expr: Box<AstNode>,
        false_expr: Box<AstNode>,
    },
    
    // Property access
    PropertyAccess {
        object: Box<AstNode>,
        property: String,
    },
    
    // Computed property access
    ComputedPropertyAccess {
        object: Box<AstNode>,
        property: Box<AstNode>,
    },
    
    // Method call
    MethodCall {
        object: Box<AstNode>,
        method: String,
        args: Vec<AstNode>,
    },
    
    // Increment/Decrement
    PreIncrement(Box<AstNode>),
    PostIncrement(Box<AstNode>),
    PreDecrement(Box<AstNode>),
    PostDecrement(Box<AstNode>),
    
    // Assignments
    Assignment {
        target: Box<AstNode>,
        value: Box<AstNode>,
    },
    CompoundAssignment {
        target: Box<AstNode>,
        op: String,
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
        expression: bool,
        is_async: bool,
    },
    AwaitExpr {
        expr: Box<AstNode>,
    },
    FunctionCall {
        callee: Box<AstNode>,
        args: Vec<AstNode>,
    },
    
    // Data structures
    ArrayLiteral(Vec<AstNode>),
    ObjectLiteral(HashMap<String, AstNode>),
    SpreadElement(Box<AstNode>),
    
    // Control flow
    Block(Vec<AstNode>),
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
        constructor: Box<AstNode>,
        args: Vec<AstNode>,
    },
    
    // Loop control
    Break,
    Continue,
    
    // Modules
    Import(String),
    
    // Program
    Program(Vec<AstNode>),
}

#[derive(Debug, Clone)]
pub struct SwitchCase {
    pub value: AstNode,
    pub body: Vec<AstNode>,
}

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

#[derive(Debug)]
pub enum ParseError {
    PestError(String),
    Custom(String),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParseError::PestError(e) => write!(f, "Parse error: {}", e),
            ParseError::Custom(msg) => write!(f, "Parse error: {}", msg),
        }
    }
}

pub type Result<T> = std::result::Result<T, ParseError>;

pub struct SmashLangParser;

impl SmashLangParser {
    pub fn parse(_source: &str) -> Result<AstNode> {
        // For now, just return a simple program with no statements
        Ok(AstNode::Program(Vec::new()))
    }
}
