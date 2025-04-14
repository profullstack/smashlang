use std::collections::HashMap;
use std::fmt;
use crate::parser::AstNode;

/// Value represents a runtime value in the SmashLang language
#[derive(Debug, Clone)]
pub enum Value {
    Number(f64),
    String(String),
    Boolean(bool),
    Array(Vec<Value>),
    Object(HashMap<String, Value>),
    Function(Function),
    Null,
    Undefined,
}

impl Value {
    pub fn type_name(&self) -> &'static str {
        match self {
            Value::Number(_) => "number",
            Value::String(_) => "string",
            Value::Boolean(_) => "boolean",
            Value::Array(_) => "array",
            Value::Object(_) => "object",
            Value::Function(_) => "function",
            Value::Null => "null",
            Value::Undefined => "undefined",
        }
    }
    
    pub fn is_truthy(&self) -> bool {
        match self {
            Value::Number(n) => *n != 0.0 && !n.is_nan(),
            Value::String(s) => !s.is_empty(),
            Value::Boolean(b) => *b,
            Value::Array(_) => true,
            Value::Object(_) => true,
            Value::Function(_) => true,
            Value::Null => false,
            Value::Undefined => false,
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Number(n) => {
                // Check if the number is an integer
                if n.fract() == 0.0 {
                    write!(f, "{}", *n as i64)
                } else {
                    write!(f, "{}", n)
                }
            },
            Value::String(s) => write!(f, "{}", s),
            Value::Boolean(b) => write!(f, "{}", b),
            Value::Array(arr) => {
                write!(f, "[")?;
                for (i, val) in arr.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", val)?;
                }
                write!(f, "]")
            },
            Value::Object(obj) => {
                write!(f, "{{")?;
                for (i, (key, val)) in obj.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}: {}", key, val)?;
                }
                write!(f, "}}")
            },
            Value::Function(_) => write!(f, "[Function]"),
            Value::Null => write!(f, "null"),
            Value::Undefined => write!(f, "undefined"),
        }
    }
}

/// Function represents a callable function in the SmashLang language
#[derive(Debug, Clone)]
pub struct Function {
    pub name: Option<String>,
    pub params: Vec<String>,
    pub body: Vec<AstNode>,
    pub closure: Environment,
}

impl Function {
    pub fn new(name: Option<String>, params: Vec<String>, body: Vec<AstNode>, closure: Environment) -> Self {
        Self {
            name,
            params,
            body,
            closure,
        }
    }
}

/// Environment represents a scope in the SmashLang language
#[derive(Debug, Clone)]
pub struct Environment {
    values: HashMap<String, Value>,
    parent: Option<Box<Environment>>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
            parent: None,
        }
    }
    
    pub fn with_parent(parent: Environment) -> Self {
        Self {
            values: HashMap::new(),
            parent: Some(Box::new(parent)),
        }
    }
    
    pub fn define(&mut self, name: &str, value: Value) {
        self.values.insert(name.to_string(), value);
    }
    
    pub fn assign(&mut self, name: &str, value: Value) -> Result<(), String> {
        if self.values.contains_key(name) {
            self.values.insert(name.to_string(), value);
            Ok(())
        } else if let Some(parent) = &mut self.parent {
            parent.assign(name, value)
        } else {
            Err(format!("Variable '{}' is not defined", name))
        }
    }
    
    pub fn get(&self, name: &str) -> Option<Value> {
        if let Some(value) = self.values.get(name) {
            Some(value.clone())
        } else if let Some(parent) = &self.parent {
            parent.get(name)
        } else {
            None
        }
    }
}

/// Interpreter for SmashLang
pub struct Interpreter {
    environment: Environment,
}

impl Interpreter {
    pub fn new() -> Self {
        let mut env = Environment::new();
        
        // Define global functions and objects
        env.define("console", Value::Object({
            let mut console = HashMap::new();
            console.insert("log".to_string(), Value::Function(Function::new(
                Some("log".to_string()),
                vec!["message".to_string()],
                Vec::new(),
                Environment::new(),
            )));
            console
        }));
        
        Self {
            environment: env,
        }
    }
    
    pub fn evaluate(&self, _node: &AstNode) -> Result<Value, String> {
        // For now, just return a simple value
        Ok(Value::Number(42.0))
    }
}