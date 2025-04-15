use crate::parser::AstNode;
use crate::interpreter::value::Value;
use crate::interpreter::environment::Environment;

/// Function represents a callable function in the SmashLang language
pub struct Function {
    pub name: Option<String>,
    pub params: Vec<String>,
    pub body: Vec<AstNode>,
    pub native_fn: Option<Box<dyn Fn(Value, &[Value], &Environment) -> Result<Value, String> + 'static>>,
}

impl std::fmt::Debug for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Function")
            .field("name", &self.name)
            .field("params", &self.params)
            .field("body", &self.body)
            .field("native_fn", &if self.native_fn.is_some() { "Some(native_fn)" } else { "None" })
            .finish()
    }
}

impl Clone for Function {
    fn clone(&self) -> Self {
        Function {
            name: self.name.clone(),
            params: self.params.clone(),
            body: self.body.clone(),
            native_fn: None // Cannot clone closures
        }
    }
}

impl Function {
    pub fn new(name: Option<String>, params: Vec<String>, body: Vec<AstNode>) -> Self {
        Self {
            name,
            params,
            body,
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