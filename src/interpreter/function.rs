use std::rc::Rc;
use crate::parser::AstNode;
use crate::interpreter::value::Value;
use crate::interpreter::environment::Environment;

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