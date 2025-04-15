use crate::parser::AstNode;
use crate::interpreter::value::Value;
use crate::interpreter::environment::Environment;

/// Interpreter for SmashLang
pub struct Interpreter {
    environment: Environment,
}

impl Interpreter {
    pub fn new() -> Self {
        let mut env = Environment::new();
        
        // Define global functions and objects
        env.define("console", Value::Object({
            let mut console = std::collections::HashMap::new();
            console.insert("log".to_string(), Value::Undefined);
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