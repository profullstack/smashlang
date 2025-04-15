use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;
use std::cell::RefCell;
use crate::runtime::promise::Promise;
use crate::runtime::class::{Class, ClassInstance};
use crate::runtime::collections::{Map, Set, WeakMap, WeakSet};
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
    Promise(Rc<Promise>),
    Class(Rc<RefCell<Class>>),
    ClassInstance(Rc<RefCell<ClassInstance>>),
    Map(Rc<Map>),
    Set(Rc<Set>),
    WeakMap(Rc<WeakMap>),
    WeakSet(Rc<WeakSet>),
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
            Value::Promise(_) => "promise",
            Value::Class(_) => "class",
            Value::ClassInstance(_) => "object",
            Value::Map(_) => "map",
            Value::Set(_) => "set",
            Value::WeakMap(_) => "weakmap",
            Value::WeakSet(_) => "weakset",
            Value::Null => "null", 
            Value::Undefined => "undefined",
            Value::Identifier(_) => "identifier", // Fixed the todo
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
            Value::Promise(_) => true,
            Value::Class(_) => true,
            Value::ClassInstance(_) => true,
            Value::Map(_) => true,
            Value::Set(_) => true,
            Value::WeakMap(_) => true,
            Value::WeakSet(_) => true,
            Value::Null => false,
            Value::Undefined => false, // Fixed the type mismatch
            Value::Identifier(_) => true, // Fixed the todo
        }
    }
    
    pub fn is_object(&self) -> bool {
        matches!(self, Value::Object(_) | Value::ClassInstance(_))
    }
    
    pub fn is_function(&self) -> bool {
        matches!(self, Value::Function(_))
    }
    
    pub fn is_class(&self) -> bool {
        matches!(self, Value::Class(_))
    }
    
    pub fn is_class_instance(&self) -> bool {
        matches!(self, Value::ClassInstance(_))
    }
    
    pub fn is_map(&self) -> bool {
        matches!(self, Value::Map(_))
    }
    
    pub fn is_set(&self) -> bool {
        matches!(self, Value::Set(_))
    }
    
    pub fn is_weak_map(&self) -> bool {
        matches!(self, Value::WeakMap(_))
    }
    
    pub fn is_weak_set(&self) -> bool {
        matches!(self, Value::WeakSet(_))
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
            Value::Promise(_) => write!(f, "[Promise]"),
            Value::Class(class) => write!(f, "[Class: {}]", class.borrow().name),
            Value::ClassInstance(instance) => write!(f, "[{} instance]", instance.borrow().class.name),
            Value::Map(map) => {
                write!(f, "Map({{")?;
                let entries = map.entries();
                for (i, (key, val)) in entries.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{} => {}", key, val)?;
                }
                write!(f, "}})")
            },
            Value::Set(set) => {
                write!(f, "Set({{")?;
                let values = set.values();
                for (i, val) in values.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", val)?;
                }
                write!(f, "}})")
            },
            Value::WeakMap(_) => write!(f, "[WeakMap]"),
            Value::WeakSet(_) => write!(f, "[WeakSet]"),
            Value::Null => write!(f, "null"),
            Value::Undefined => write!(f, "undefined"), // Fixed the type mismatch
            Value::Identifier(name) => write!(f, "{}", name), // Fixed the todo
        }
    }
}