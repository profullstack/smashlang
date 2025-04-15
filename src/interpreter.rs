use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;
use std::cell::RefCell;
use crate::parser::AstNode;
use crate::runtime::promise::Promise;
use crate::runtime::class::{Class, ClassInstance};
use crate::runtime::collections::{Map, Set, WeakMap, WeakSet};
use crate::runtime::timers::{TimerManager, create_set_timeout_function, create_set_interval_function, create_clear_timeout_function, create_clear_interval_function};
use crate::runtime::json::create_json_object;
use crate::runtime::browser::BrowserEnvironment;

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
            Value::Identifier(_) => todo!("Identifier type_name not implemented"),
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
            Value::Undefined => "undefined",
            Value::Identifier(_) => todo!("Identifier type_name not implemented"), false,
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
            Value::Undefined => "undefined",
            Value::Identifier(_) => todo!("Identifier type_name not implemented"), write!(f, "undefined"),
        }
    }
}

/// Function represents a callable function in the SmashLang language
pub struct Function {
    pub name: Option<String>,
    pub params: Vec<String>,
    pub body: Vec<AstNode>,
    pub closure: Environment,
    pub native_fn: Option<Box<dyn Fn(Value, &[Value], &Environment) -> Result<Value, String> + 'static>>,
}

impl std::fmt::Debug for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Function")
            .field("name", &self.name)
            .field("params", &self.params)
            .finish()
    }
}

impl Clone for Function {
    fn clone(&self) -> Self {
        Function {
            name: self.name.clone(),
            params: self.params.clone(),
            body: self.body.clone(),
            closure: self.closure.clone(),
            native_fn: None // Cannot clone closures
        }
    }
}

impl Function {
    pub fn new(name: Option<String>, params: Vec<String>, body: Vec<AstNode>, closure: Environment) -> Self {
        Self {
            name,
            params,
            body,
            closure,
            native_fn: None,
        }
    }
    
    pub fn new_native<F>(name: Option<String>, params: Vec<String>, f: F) -> Self
    where
        F: Fn(Value, &[Value], &Environment) -> Result<Value, String> + 'static,
    {
        Self {
            name,
            params,
            body: Vec::new(),
            closure: Environment::new(),
            native_fn: Some(Box::new(f)),
        }
    }
    
    pub fn call(&self, this: Value, args: &[Value], env: &Environment) -> Result<Value, String> {
        if let Some(native_fn) = &self.native_fn {
            native_fn(this, args, env)
        } else {
            // For now, just return a simple value
            // In a real implementation, this would execute the function body
            Ok(Value::Number(42.0))
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
    timer_manager: TimerManager,
    browser_environment: BrowserEnvironment,
}

impl Interpreter {
    pub fn new() -> Self {
        let mut env = Environment::new();
        
        // Create timer manager
        let timer_manager = TimerManager::new(env.clone());
        
        // Create browser environment
        let browser_environment = BrowserEnvironment::new();
        
        // Define global functions and objects
        env.define("console", Value::Object({
            let mut console = HashMap::new();
            console.insert("log".to_string(), Value::Function(Function::new_native(
                Some("log".to_string()),
                vec!["...args".to_string()],
                |_this, args, _env| {
                    // Print all arguments
                    for (i, arg) in args.iter().enumerate() {
                        if i > 0 {
                            print!(" ");
                        }
                        print!("{}", arg);
                    }
                    println!();
                    Ok(Value::Undefined)
                },
            )));
            console
        }));
        
        // Define Promise constructor and static methods
        env.define("Promise", Value::Object({
            let mut promise_obj = HashMap::new();
            
            // Promise constructor
            promise_obj.insert("constructor".to_string(), Value::Function(Function::new_native(
                Some("Promise".to_string()),
                vec!["executor".to_string()],
                |_this, args, env| {
                    if args.is_empty() {
                        return Err("Promise constructor requires an executor function".to_string());
                    }
                    
                    if let Value::Function(executor) = &args[0] {
                        let promise = Promise::with_executor(executor, env);
                        Ok(Value::Promise(promise))
                    } else {
                        Err("Promise executor must be a function".to_string())
                    }
                },
            )));
            
            // Promise.resolve
            promise_obj.insert("resolve".to_string(), Value::Function(Function::new_native(
                Some("resolve".to_string()),
                vec!["value".to_string()],
                |_this, args, _env| {
                    let value = args.first().cloned().unwrap_or(Value::Undefined);
                    let promise = Promise::resolve_with(value);
                    Ok(Value::Promise(promise))
                },
            )));
            
            // Promise.reject
            promise_obj.insert("reject".to_string(), Value::Function(Function::new_native(
                Some("reject".to_string()),
                vec!["reason".to_string()],
                |_this, args, _env| {
                    let reason = args.first().cloned().unwrap_or(Value::Undefined);
                    let promise = Promise::reject_with(reason);
                    Ok(Value::Promise(promise))
                },
            )));
            
            // Promise.all
            promise_obj.insert("all".to_string(), Value::Function(Function::new_native(
                Some("all".to_string()),
                vec!["iterable".to_string()],
                |_this, args, _env| {
                    let iterable = args.first().cloned().unwrap_or(Value::Undefined);
                    let promise = Promise::all(iterable);
                    Ok(Value::Promise(promise))
                },
            )));
            
            // Promise.race
            promise_obj.insert("race".to_string(), Value::Function(Function::new_native(
                Some("race".to_string()),
                vec!["iterable".to_string()],
                |_this, args, _env| {
                    let iterable = args.first().cloned().unwrap_or(Value::Undefined);
                    let promise = Promise::race(iterable);
                    Ok(Value::Promise(promise))
                },
            )));
            
            // Promise.allSettled
            promise_obj.insert("allSettled".to_string(), Value::Function(Function::new_native(
                Some("allSettled".to_string()),
                vec!["iterable".to_string()],
                |_this, args, _env| {
                    let iterable = args.first().cloned().unwrap_or(Value::Undefined);
                    let promise = Promise::all_settled(iterable);
                    Ok(Value::Promise(promise))
                },
            )));
            
            // Promise.any
            promise_obj.insert("any".to_string(), Value::Function(Function::new_native(
                Some("any".to_string()),
                vec!["iterable".to_string()],
                |_this, args, _env| {
                    let iterable = args.first().cloned().unwrap_or(Value::Undefined);
                    let promise = Promise::any(iterable);
                    Ok(Value::Promise(promise))
                },
            )));
            
            promise_obj
        }));
        
        // Define Map constructor
        env.define("Map", Value::Function(Function::new_native(
            Some("Map".to_string()),
            vec!["iterable".to_string()],
            |_this, args, _env| {
                if args.is_empty() {
                    Ok(Value::Map(Map::new()))
                } else {
                    let iterable = &args[0];
                    match Map::from_iterable(iterable) {
                        Ok(map) => Ok(Value::Map(map)),
                        Err(err) => Err(err),
                    }
                }
            },
        )));
        
        // Define Set constructor
        env.define("Set", Value::Function(Function::new_native(
            Some("Set".to_string()),
            vec!["iterable".to_string()],
            |_this, args, _env| {
                if args.is_empty() {
                    Ok(Value::Set(Set::new()))
                } else {
                    let iterable = &args[0];
                    match Set::from_iterable(iterable) {
                        Ok(set) => Ok(Value::Set(set)),
                        Err(err) => Err(err),
                    }
                }
            },
        )));
        
        // Define WeakMap constructor
        env.define("WeakMap", Value::Function(Function::new_native(
            Some("WeakMap".to_string()),
            vec!["iterable".to_string()],
            |_this, args, _env| {
                if args.is_empty() {
                    Ok(Value::WeakMap(WeakMap::new()))
                } else {
                    let iterable = &args[0];
                    match WeakMap::from_iterable(iterable) {
                        Ok(weak_map) => Ok(Value::WeakMap(weak_map)),
                        Err(err) => Err(err),
                    }
                }
            },
        )));
        
        // Define WeakSet constructor
        env.define("WeakSet", Value::Function(Function::new_native(
            Some("WeakSet".to_string()),
            vec!["iterable".to_string()],
            |_this, args, _env| {
                if args.is_empty() {
                    Ok(Value::WeakSet(WeakSet::new()))
                } else {
                    let iterable = &args[0];
                    match WeakSet::from_iterable(iterable) {
                        Ok(weak_set) => Ok(Value::WeakSet(weak_set)),
                        Err(err) => Err(err),
                    }
                }
            },
        )));
        
        // Define timer functions
        env.define("setTimeout", Value::Function(create_set_timeout_function(timer_manager.clone())));
        env.define("setInterval", Value::Function(create_set_interval_function(timer_manager.clone())));
        env.define("clearTimeout", Value::Function(create_clear_timeout_function(timer_manager.clone())));
        env.define("clearInterval", Value::Function(create_clear_interval_function(timer_manager.clone())));
        
        // Define JSON object
        env.define("JSON", create_json_object());
        
        // Define browser globals
        let window = browser_environment.create_window();
        env.define("window", window.clone());
        
        // Extract properties from window object to global scope
        if let Value::Object(window_obj) = window {
            for (key, value) in window_obj {
                env.define(&key, value);
            }
        }
        
        Self {
            environment: env,
            timer_manager,
            browser_environment,
        }
    }
    
    pub fn evaluate(&self, _node: &AstNode) -> Result<Value, String> {
        // For now, just return a simple value
        Ok(Value::Number(42.0))
    }
}