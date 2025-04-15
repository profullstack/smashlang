use std::collections::HashMap;
use std::fmt;
use crate::interpreter::function::Function;

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
    Identifier(String),
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
            Value::Identifier(_) => "identifier",
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
            Value::Identifier(_) => true,
        }
    }
    
    pub fn is_object(&self) -> bool {
        matches!(self, Value::Object(_))
    }
    
    pub fn is_function(&self) -> bool {
        matches!(self, Value::Function(_))
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
            Value::Identifier(name) => write!(f, "{}", name),
        }
    }
}