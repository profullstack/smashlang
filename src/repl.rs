use std::io::{self, Write};
use std::collections::HashMap;
use colored::*;
use crate::lexer::tokenize;
use crate::parser::{Parser, AstNode};

#[derive(Clone)]
pub struct Scope {
    variables: HashMap<String, Value>,
    parent: Option<Box<Scope>>,
}

impl Scope {
    fn new() -> Self {
        Scope {
            variables: HashMap::new(),
            parent: None,
        }
    }
    
    fn with_parent(parent: Box<Scope>) -> Self {
        Scope {
            variables: HashMap::new(),
            parent: Some(parent),
        }
    }
    
    fn get(&self, name: &str) -> Option<Value> {
        if let Some(value) = self.variables.get(name) {
            Some(value.clone())
        } else if let Some(parent) = &self.parent {
            parent.get(name)
        } else {
            None
        }
    }
    
    fn set(&mut self, name: &str, value: Value) {
        self.variables.insert(name.to_string(), value);
    }
    
    #[allow(dead_code)]
    fn has_own(&self, name: &str) -> bool {
        self.variables.contains_key(name)
    }
}

pub struct Repl {
    history: Vec<String>,
    context: String, // Accumulated code for multi-line input
    global_scope: Scope, // Global scope for the REPL session
}

// A value type for our REPL to support various SmashLang features
#[derive(Debug, Clone)]
pub enum Value {
    Number(i64),
    Float(f64),
    String(String),
    Boolean(bool),
    Null,
    Array(Vec<Value>),
    Object(HashMap<String, Value>),
    Function(String, Vec<String>, Box<AstNode>), // name, params, body
    Undefined,
}

impl Repl {
    pub fn new() -> Self {
        let mut repl = Repl {
            history: Vec::new(),
            context: String::new(),
            global_scope: Scope::new(),
        };
        
        // Add example variables to the global scope
        repl.global_scope.set("counter", Value::Number(0));
        repl.global_scope.set("value", Value::Number(10));
        
        repl
    }

    pub fn run(&mut self) {
        println!("{}", "SmashLang REPL v0.1.0".bright_cyan().bold());
        println!("Type {} for available commands or {} to quit", ".help".green(), ".exit".red());

        loop {
            print!("{} ", ">".bright_green());
            io::stdout().flush().unwrap();

            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let input = input.trim();

            if input.is_empty() {
                continue;
            }

            // Handle REPL commands
            match input {
                ".exit" => break,
                ".help" => self.show_help(),
                ".history" => self.show_history(),
                ".clear" => self.clear_context(),
                ".vars" => self.show_variables(),
                _ if input.starts_with(".") => {
                    println!("{}: {}", "Unknown command".red(), input);
                    println!("Type {} for available commands", ".help".green());
                },
                _ => self.evaluate(input),
            }
        }

        println!("Goodbye!");
    }

    fn show_help(&self) {
        println!("{}", "Available commands:".bright_cyan());
        println!("  {}  - Show this help message", ".help".green());
        println!("  {}  - Exit the REPL", ".exit".red());
        println!("  {} - Show command history", ".history".green());
        println!("  {}  - Clear the current context", ".clear".green());
        println!("  {}   - Show all variables", ".vars".green());
        
        println!("
{}", "Operator Examples:".bright_cyan());
        println!("  {}  - Increment (counter++, ++counter)", "++".yellow());
        println!("  {}  - Decrement (counter--, --counter)", "--".yellow());
        println!("  {}  - Add and assign (counter += 5)", "+=".yellow());
        println!("  {}  - Subtract and assign (counter -= 2)", "-=".yellow());
        println!("  {}  - Multiply and assign (counter *= 3)", "*=".yellow());
        println!("  {}  - Divide and assign (counter /= 2)", "/=".yellow());
        println!("  {}  - Comparison (==, ===, !=, !==, <, >, <=, >=)", "==, etc".yellow());
        println!("  {}  - Logical operators (&&, ||, !)", "&&, ||, !".yellow());
        println!("  {}  - Bitwise operators (&, |, ^, ~, <<, >>)", "&, |, etc".yellow());
        println!("  {}  - Ternary operator (condition ? expr1 : expr2)", "? :".yellow());
        
        println!("
{}", "Block Scoping Examples:".bright_cyan());
        println!("  {}
  {}", "let x = 10;".yellow(), "{ let x = 20; console.log(x); }  // Prints 20".yellow());
        println!("  {}
  {}", "{ let y = 5; }".yellow(), "console.log(y);  // Error: y is not defined".yellow());
        println!("  {}
  {}
  {}", "let z = 1;".yellow(), "{ z++; let z = 100; z++; }".yellow(), "console.log(z);  // Prints 2 (outer z was incremented before shadowing)".yellow());
        
        println!("
{}", "String Examples:".bright_cyan());
        println!("  {}  - Double-quoted strings", "\"Hello, world!\"".yellow());
        println!("  {}  - Single-quoted strings", "'Hello, world!'".yellow());
        println!("  {}  - Template literals", "`Hello, ${name}!`".yellow());
        
        println!("
{}", "Array and Object Examples:".bright_cyan());
        println!("  {}  - Array literal", "[1, 2, 3, 4]".yellow());
        println!("  {}  - Object literal", "{{ name: 'John', age: 30 }}".yellow());
        println!("  {}  - Array/Object access", "arr[0], obj.name".yellow());
        
        println!("
{}", "SmashLang expressions and statements can be entered directly.".bright_cyan());
        println!("Multi-line input is supported - the code will be evaluated when a complete statement is detected.");
    }

    fn show_history(&self) {
        if self.history.is_empty() {
            println!("No history yet");
            return;
        }

        for (i, cmd) in self.history.iter().enumerate() {
            println!("{}: {}", i + 1, cmd);
        }
    }

    fn clear_context(&mut self) {
        self.context.clear();
        println!("Context cleared");
    }

    fn show_variables(&self) {
        if self.global_scope.variables.is_empty() {
            println!("{}", "No variables defined".yellow());
            return;
        }

        println!("{}", "Current variables:".bright_cyan());
        for (name, value) in &self.global_scope.variables {
            println!("  {} = {}", name.yellow(), format!("{:?}", value).cyan());
        }
    }

    fn evaluate(&mut self, input: &str) {
        // Add to history
        self.history.push(input.to_string());
        
        // Append to context with automatic semicolon insertion if needed
        let mut processed_input = input.trim().to_string();
        
        // Add semicolon if it's missing and the input looks like a statement
        if !processed_input.ends_with(';') && 
           !processed_input.ends_with('}') && 
           !processed_input.ends_with('{') && 
           !processed_input.is_empty() {
            processed_input.push(';');
        }
        
        self.context.push_str(&processed_input);
        self.context.push('\n');
        
        // Try to tokenize and parse
        let tokens = tokenize(&self.context);
        let mut parser = Parser::new(tokens.clone());
        
        match parser.parse() {
            Ok(ast) => {
                // We have a complete expression or statement
                println!("{}: {:?}", "Parsed AST".bright_green(), ast);
                
                // Simple evaluation of the AST for demonstration purposes
                // Evaluate each statement in the AST
                let mut result = Ok(Value::Null);
                for node in &ast {
                    result = self.evaluate_ast(node);
                    if result.is_err() {
                        break;
                    }
                }
                
                match result {
                    Ok(result) => {
                        println!("{}: {:?}", "Result".bright_green(), result);
                        
                        // For increment/decrement operations, show the current value of the variable
                        if let Some(last_node) = ast.last() {
                            if let AstNode::PostIncrement(expr) | AstNode::PreIncrement(expr) | 
                                   AstNode::PostDecrement(expr) | AstNode::PreDecrement(expr) = last_node {
                                if let AstNode::Identifier(name) = &**expr {
                                    if let Some(current_value) = self.global_scope.get(name) {
                                        println!("{}: {} = {:?}", "Current Value".bright_blue(), name.yellow(), current_value);
                                    }
                                }
                            }
                        }
                    },
                    Err(err) => {
                        println!("{}: {}", "Evaluation error".red(), err);
                    }
                }
                
                // Clear context after successful execution
                self.context.clear();
            },
            Err(e) => {
                // Check if this is an incomplete input or a syntax error
                if e.to_string().contains("Unexpected end of input") {
                    // This is incomplete input, wait for more
                    print!("{} ", "...".bright_yellow());
                    io::stdout().flush().unwrap();
                } else {
                    // This is a syntax error
                    println!("{}: {}", "Syntax error".red(), e);
                    // Clear context on error
                    self.context.clear();
                }
            },
        }
    }
    
    // Simple AST evaluation for demonstration purposes
    fn evaluate_ast(&mut self, ast: &AstNode) -> Result<Value, String> {
        // Create a mutable clone of the global scope to avoid borrowing issues
        let mut scope_clone = self.global_scope.clone();
        let result = self.evaluate_ast_with_scope(ast, &mut scope_clone);
        // Update the global scope with any changes
        self.global_scope = scope_clone;
        result
    }
    
    fn evaluate_ast_with_scope(&mut self, ast: &AstNode, scope: &mut Scope) -> Result<Value, String> {
        match ast {
            AstNode::Number(n) => Ok(Value::Number(*n)),
            AstNode::Float(f) => Ok(Value::Float(*f)),
            AstNode::String(s) => Ok(Value::String(s.clone())),
            AstNode::Boolean(b) => Ok(Value::Boolean(*b)),
            AstNode::Null => Ok(Value::Null),
            
            // Binary operations
            AstNode::BinaryOp { left, op, right } => {
                let left_val = self.evaluate_ast_with_scope(left, scope)?;
                let right_val = self.evaluate_ast_with_scope(right, scope)?;
                
                // Clone values before matching to avoid ownership issues
                let left_clone = left_val.clone();
                let right_clone = right_val.clone();
                
                match (left_val, op.as_str(), right_val) {
                    // Arithmetic operators
                    (Value::Number(l), "+", Value::Number(r)) => Ok(Value::Number(l + r)),
                    (Value::Number(l), "-", Value::Number(r)) => Ok(Value::Number(l - r)),
                    (Value::Number(l), "*", Value::Number(r)) => Ok(Value::Number(l * r)),
                    (Value::Number(l), "/", Value::Number(r)) => {
                        if r == 0 {
                            return Err("Division by zero".to_string());
                        }
                        Ok(Value::Number(l / r))
                    },
                    (Value::Number(l), "%", Value::Number(r)) => {
                        if r == 0 {
                            return Err("Modulo by zero".to_string());
                        }
                        Ok(Value::Number(l % r))
                    },
                    
                    // String concatenation
                    (Value::String(l), "+", Value::String(r)) => Ok(Value::String(l + &r)),
                    (Value::String(l), "+", Value::Number(r)) => Ok(Value::String(l + &r.to_string())),
                    (Value::Number(l), "+", Value::String(r)) => Ok(Value::String(l.to_string() + &r)),
                    
                    // Comparison operators
                    (Value::Number(l), "==", Value::Number(r)) => Ok(Value::Boolean(l == r)),
                    (Value::String(l), "==", Value::String(r)) => Ok(Value::Boolean(l == r)),
                    (Value::Boolean(l), "==", Value::Boolean(r)) => Ok(Value::Boolean(l == r)),
                    (Value::Number(l), "!=", Value::Number(r)) => Ok(Value::Boolean(l != r)),
                    (Value::String(l), "!=", Value::String(r)) => Ok(Value::Boolean(l != r)),
                    (Value::Boolean(l), "!=", Value::Boolean(r)) => Ok(Value::Boolean(l != r)),
                    (Value::Number(l), "<", Value::Number(r)) => Ok(Value::Boolean(l < r)),
                    (Value::Number(l), ">", Value::Number(r)) => Ok(Value::Boolean(l > r)),
                    (Value::Number(l), "<=", Value::Number(r)) => Ok(Value::Boolean(l <= r)),
                    (Value::Number(l), ">=", Value::Number(r)) => Ok(Value::Boolean(l >= r)),
                    
                    // Logical operators
                    (Value::Boolean(l), "&&", Value::Boolean(r)) => Ok(Value::Boolean(l && r)),
                    (Value::Boolean(l), "||", Value::Boolean(r)) => Ok(Value::Boolean(l || r)),
                    
                    // Bitwise operators
                    (Value::Number(l), "&", Value::Number(r)) => Ok(Value::Number(l & r)),
                    (Value::Number(l), "|", Value::Number(r)) => Ok(Value::Number(l | r)),
                    (Value::Number(l), "^", Value::Number(r)) => Ok(Value::Number(l ^ r)),
                    (Value::Number(l), "<<", Value::Number(r)) => Ok(Value::Number(l << r as u32)),
                    (Value::Number(l), ">>", Value::Number(r)) => Ok(Value::Number(l >> r as u32)),
                    
                    _ => Err(format!("Invalid operation: {:?} {} {:?}", left_clone, op, right_clone))
                }
            },
            
            AstNode::Identifier(name) => {
                if let Some(value) = scope.get(name) {
                    Ok(value)
                } else {
                    Err(format!("Variable '{}' not found", name))
                }
            },
            
            // Handle block-level scoping
            AstNode::Block(statements) => {
                // Create a new scope with the current scope as parent
                let mut block_scope = Scope::with_parent(Box::new(scope.clone()));
                let mut result = Value::Null;
                
                // Execute each statement in the block with the new scope
                for stmt in statements {
                    result = self.evaluate_ast_with_scope(stmt, &mut block_scope)?;
                }
                
                // Return the result of the last statement
                Ok(result)
            },
            
            AstNode::LetDecl { name, value } => {
                // Special handling for arrow functions to associate them with a name
                if let AstNode::ArrowFunction { params, body, expression } = &**value {
                    // Create a named function value
                    let function_value = Value::Function(name.clone(), params.clone(), Box::new(if *expression && body.len() == 1 {
                        // For expression bodies, wrap in a return statement
                        AstNode::Block(vec![AstNode::Return(Box::new(body[0].clone()))])
                    } else {
                        // For block bodies, use the block as is
                        AstNode::Block(body.clone())
                    }));
                    
                    // Store in scope
                    scope.set(name, function_value.clone());
                    Ok(function_value)
                } else {
                    // Regular variable declaration
                    let evaluated_value = self.evaluate_ast_with_scope(value, scope)?;
                    scope.set(name, evaluated_value.clone());
                    Ok(evaluated_value)
                }
            },
            
            AstNode::ConstDecl { name, value } => {
                // Special handling for arrow functions to associate them with a name
                if let AstNode::ArrowFunction { params, body, expression } = &**value {
                    // Create a named function value
                    let function_value = Value::Function(name.clone(), params.clone(), Box::new(if *expression && body.len() == 1 {
                        // For expression bodies, wrap in a return statement
                        AstNode::Block(vec![AstNode::Return(Box::new(body[0].clone()))])
                    } else {
                        // For block bodies, use the block as is
                        AstNode::Block(body.clone())
                    }));
                    
                    // Store in scope
                    scope.set(name, function_value.clone());
                    Ok(function_value)
                } else {
                    // Regular constant declaration
                    let evaluated_value = self.evaluate_ast_with_scope(value, scope)?;
                    scope.set(name, evaluated_value.clone());
                    Ok(evaluated_value)
                }
            },
            
            // Handle increment/decrement operators
            AstNode::PreIncrement(expr) => {
                if let AstNode::Identifier(name) = &**expr {
                    if let Some(Value::Number(n)) = scope.get(name) {
                        let new_value = n + 1;
                        scope.set(name, Value::Number(new_value));
                        Ok(Value::Number(new_value))
                    } else {
                        Err("Can only increment numeric variables".to_string())
                    }
                } else {
                    Err("Can only increment variables".to_string())
                }
            },
            
            AstNode::PostIncrement(expr) => {
                if let AstNode::Identifier(name) = &**expr {
                    if let Some(Value::Number(n)) = scope.get(name) {
                        let old_value = n;
                        scope.set(name, Value::Number(old_value + 1));
                        Ok(Value::Number(old_value)) // Return the original value for post-increment
                    } else {
                        Err("Can only increment numeric variables".to_string())
                    }
                } else {
                    Err("Can only increment variables".to_string())
                }
            },
            
            AstNode::PreDecrement(expr) => {
                if let AstNode::Identifier(name) = &**expr {
                    if let Some(Value::Number(n)) = scope.get(name) {
                        let new_value = n - 1;
                        scope.set(name, Value::Number(new_value));
                        Ok(Value::Number(new_value))
                    } else {
                        Err("Can only decrement numeric variables".to_string())
                    }
                } else {
                    Err("Can only decrement variables".to_string())
                }
            },
            
            AstNode::PostDecrement(expr) => {
                if let AstNode::Identifier(name) = &**expr {
                    if let Some(Value::Number(n)) = scope.get(name) {
                        let old_value = n;
                        scope.set(name, Value::Number(old_value - 1));
                        Ok(Value::Number(old_value)) // Return the original value for post-decrement
                    } else {
                        Err("Can only decrement numeric variables".to_string())
                    }
                } else {
                    Err("Can only decrement variables".to_string())
                }
            },
            
            // Handle compound assignments
            AstNode::CompoundAssignment { target, op, value } => {
                if let AstNode::Identifier(name) = &**target {
                    let right_value = self.evaluate_ast_with_scope(value, scope)?;
                    
                    if let Some(current_value) = scope.get(name) {
                        let new_value = match (current_value.clone(), &right_value, op.as_str()) {
                            // Handle both + and += for addition
                            (Value::Number(left), Value::Number(right), op) if op == "+" || op == "+=" => {
                                Value::Number(left + *right)
                            },
                            // Handle both - and -= for subtraction
                            (Value::Number(left), Value::Number(right), op) if op == "-" || op == "-=" => {
                                Value::Number(left - *right)
                            },
                            // Handle both * and *= for multiplication
                            (Value::Number(left), Value::Number(right), op) if op == "*" || op == "*=" => {
                                Value::Number(left * *right)
                            },
                            // Handle both / and /= for division
                            (Value::Number(left), Value::Number(right), op) if op == "/" || op == "/=" => {
                                if *right == 0 {
                                    return Err("Division by zero".to_string());
                                }
                                Value::Number(left / *right)
                            },
                            _ => return Err(format!("Invalid operation: {:?} {} {:?}", current_value.clone(), op, right_value))
                        };
                        
                        scope.set(name, new_value.clone());
                        Ok(new_value)
                    } else {
                        Err(format!("Variable '{}' not found", name))
                    }
                } else {
                    Err("Left side of assignment must be a variable".to_string())
                }
            },
            
            // Handle function calls
            AstNode::FunctionCall { name, args } => {
                // Evaluate all arguments first
                let mut evaluated_args = Vec::new();
                for arg in args {
                    let value = self.evaluate_ast_with_scope(arg, scope)?;
                    evaluated_args.push(value);
                }
                
                // Check if it's a built-in function
                match name.as_str() {
                    "print" => {
                        // Print each argument
                        for (i, arg) in evaluated_args.iter().enumerate() {
                            if i > 0 {
                                print!(" ");
                            }
                            match arg {
                                Value::Number(n) => print!("{}", n),
                                Value::Float(f) => print!("{}", f),
                                Value::String(s) => print!("{}", s),
                                Value::Boolean(b) => print!("{}", b),
                                Value::Null => print!("null"),
                                Value::Array(_) => print!("[Array]"),
                                Value::Object(_) => print!("{{}}"),
                                Value::Function(_, _, _) => print!("[Function]"),
                                Value::Undefined => print!("undefined"),
                            }
                        }
                        println!();
                        
                        // Return the last argument or null if no arguments
                        if let Some(last) = evaluated_args.last() {
                            Ok(last.clone())
                        } else {
                            Ok(Value::Null)
                        }
                    },
                    "map" => {
                        // Handle array.map(callback)
                        // First argument should be the array (this)
                        // Second argument should be the callback function
                        if evaluated_args.len() < 2 {
                            return Err("map requires an array and a callback function".to_string());
                        }
                        
                        let array = &evaluated_args[0];
                        let callback = &evaluated_args[1];
                        
                        match (array, callback) {
                            (Value::Array(arr), Value::Function(_, param_names, body)) => {
                                // Create a new array to hold the mapped values
                                let mut result = Vec::new();
                                
                                // Apply the callback to each element
                                for (i, item) in arr.iter().enumerate() {
                                    // Create a new scope for each callback invocation
                                    let mut callback_scope = Scope::with_parent(Box::new(scope.clone()));
                                    
                                    // Bind the current item as the first parameter
                                    if param_names.len() > 0 {
                                        callback_scope.set(&param_names[0], item.clone());
                                    }
                                    
                                    // Bind the index as the second parameter if it exists
                                    if param_names.len() > 1 {
                                        callback_scope.set(&param_names[1], Value::Number(i as i64));
                                    }
                                    
                                    // Bind the array as the third parameter if it exists
                                    if param_names.len() > 2 {
                                        callback_scope.set(&param_names[2], Value::Array(arr.clone()));
                                    }
                                    
                                    // Call the callback function
                                    let mapped_value = self.evaluate_ast_with_scope(body, &mut callback_scope)?;
                                    result.push(mapped_value);
                                }
                                
                                Ok(Value::Array(result))
                            },
                            _ => Err("map requires an array and a callback function".to_string())
                        }
                    },
                    "filter" => {
                        // Handle array.filter(callback)
                        // First argument should be the array (this)
                        // Second argument should be the callback function
                        if evaluated_args.len() < 2 {
                            return Err("filter requires an array and a callback function".to_string());
                        }
                        
                        let array = &evaluated_args[0];
                        let callback = &evaluated_args[1];
                        
                        match (array, callback) {
                            (Value::Array(arr), Value::Function(_, param_names, body)) => {
                                // Create a new array to hold the filtered values
                                let mut result = Vec::new();
                                
                                // Apply the callback to each element
                                for (i, item) in arr.iter().enumerate() {
                                    // Create a new scope for each callback invocation
                                    let mut callback_scope = Scope::with_parent(Box::new(scope.clone()));
                                    
                                    // Bind the current item as the first parameter
                                    if param_names.len() > 0 {
                                        callback_scope.set(&param_names[0], item.clone());
                                    }
                                    
                                    // Bind the index as the second parameter if it exists
                                    if param_names.len() > 1 {
                                        callback_scope.set(&param_names[1], Value::Number(i as i64));
                                    }
                                    
                                    // Bind the array as the third parameter if it exists
                                    if param_names.len() > 2 {
                                        callback_scope.set(&param_names[2], Value::Array(arr.clone()));
                                    }
                                    
                                    // Call the callback function
                                    let result_value = self.evaluate_ast_with_scope(body, &mut callback_scope)?;
                                    
                                    // Check if the result is truthy
                                    let is_truthy = match result_value {
                                        Value::Boolean(b) => b,
                                        Value::Number(n) => n != 0,
                                        Value::Float(f) => f != 0.0,
                                        Value::String(s) => !s.is_empty(),
                                        Value::Array(arr) => !arr.is_empty(),
                                        Value::Object(obj) => !obj.is_empty(),
                                        Value::Function(_, _, _) => true,
                                        Value::Null => false,
                                        Value::Undefined => false,
                                    };
                                    
                                    // If the result is truthy, include the item in the result
                                    if is_truthy {
                                        result.push(item.clone());
                                    }
                                }
                                
                                Ok(Value::Array(result))
                            },
                            _ => Err("filter requires an array and a callback function".to_string())
                        }
                    },
                    _ => {
                        // Check if it's a user-defined function
                        if let Some(Value::Function(_, param_names, body)) = scope.get(name) {
                            // Create a new scope with the function's parameters
                            let mut function_scope = Scope::with_parent(Box::new(scope.clone()));
                            
                            // Bind arguments to parameters
                            for (i, param) in param_names.iter().enumerate() {
                                let arg_value = if i < evaluated_args.len() {
                                    evaluated_args[i].clone()
                                } else {
                                    Value::Undefined // Default value if argument is missing
                                };
                                function_scope.set(param, arg_value);
                            }
                            
                            // Execute the function body
                            self.evaluate_ast_with_scope(&body, &mut function_scope)
                        } else {
                            Err(format!("Function '{}' not found", name))
                        }
                    }
                }
            },
            
            // Handle template literals
            AstNode::TemplateLiteral(parts) => {
                let mut result = String::new();
                
                for part in parts {
                    match part {
                        AstNode::String(s) => {
                            result.push_str(s);
                        },
                        // In the actual AST, interpolated expressions are already parsed
                        // and included as regular expressions in the template parts list
                        // So we just need to handle them in the default case below
                        
                        _ => {
                            // Evaluate the expression and convert to string
                            let value = self.evaluate_ast_with_scope(part, scope)?;
                            match value {
                                Value::Number(n) => result.push_str(&n.to_string()),
                                Value::Float(f) => result.push_str(&f.to_string()),
                                Value::String(s) => result.push_str(&s),
                                Value::Boolean(b) => result.push_str(if b { "true" } else { "false" }),
                                Value::Null => result.push_str("null"),
                                Value::Array(_) => result.push_str("[Array]"),
                                Value::Object(_) => result.push_str("{Object}"),
                                Value::Function(name, _, _) => result.push_str(&format!("[Function: {}]", if name.is_empty() { "anonymous" } else { &name })),
                                Value::Undefined => result.push_str("undefined"),
                            }
                        }
                    }
                }
                
                Ok(Value::String(result))
            },
            
            // Handle arrow functions
            AstNode::ArrowFunction { params, body, expression } => {
                // Create a function value
                // We'll use an empty name for anonymous functions
                Ok(Value::Function(String::new(), params.clone(), Box::new(if *expression && body.len() == 1 {
                    // For expression bodies, we wrap the expression in a return statement
                    AstNode::Block(vec![AstNode::Return(Box::new(body[0].clone()))])
                } else {
                    // For block bodies, we use the block as is
                    AstNode::Block(body.clone())
                })))
            },
            
            // Handle ternary operator
            AstNode::TernaryOp { condition, true_expr, false_expr } => {
                // Evaluate the condition
                let cond_value = self.evaluate_ast_with_scope(condition, scope)?;
                
                // Determine which branch to evaluate based on the condition
                match cond_value {
                    Value::Boolean(true) => self.evaluate_ast_with_scope(true_expr, scope),
                    Value::Boolean(false) => self.evaluate_ast_with_scope(false_expr, scope),
                    _ => {
                        // For non-boolean conditions, do a truthy/falsy check
                        let is_truthy = match cond_value {
                            Value::Number(n) => n != 0,
                            Value::Float(f) => f != 0.0,
                            Value::String(s) => !s.is_empty(),
                            Value::Boolean(b) => b,
                            Value::Null => false,
                            Value::Undefined => false,
                            Value::Array(arr) => !arr.is_empty(),
                            Value::Object(obj) => !obj.is_empty(),
                            Value::Function(_, _, _) => true,
                        };
                        
                        if is_truthy {
                            self.evaluate_ast_with_scope(true_expr, scope)
                        } else {
                            self.evaluate_ast_with_scope(false_expr, scope)
                        }
                    }
                }
            },
            
            // Handle method calls (obj.method())
            AstNode::MethodCall { object, method, args } => {
                // Evaluate the object first
                let obj_value = self.evaluate_ast_with_scope(object, scope)?;
                
                // Evaluate all arguments
                let mut evaluated_args = Vec::new();
                for arg in args {
                    let value = self.evaluate_ast_with_scope(arg, scope)?;
                    evaluated_args.push(value);
                }
                
                // Handle different method calls based on the object type
                match obj_value {
                    Value::String(s) => {
                        // Handle string methods
                        match method.as_str() {
                            "toUpperCase" => Ok(Value::String(s.to_uppercase())),
                            "toLowerCase" => Ok(Value::String(s.to_lowercase())),
                            "trim" => Ok(Value::String(s.trim().to_string())),
                            "trimStart" => Ok(Value::String(s.trim_start().to_string())),
                            "trimEnd" => Ok(Value::String(s.trim_end().to_string())),
                            "charAt" => {
                                // Get character at index
                                if evaluated_args.len() < 1 {
                                    return Err("charAt requires an index argument".to_string());
                                }
                                if let Value::Number(idx) = &evaluated_args[0] {
                                    let idx = *idx as usize;
                                    if idx < s.len() {
                                        let ch = s.chars().nth(idx).unwrap();
                                        Ok(Value::String(ch.to_string()))
                                    } else {
                                        Ok(Value::String("".to_string()))
                                    }
                                } else {
                                    Err("charAt requires a number index".to_string())
                                }
                            },
                            "concat" => {
                                // Concatenate strings
                                let mut result = s.clone();
                                for arg in &evaluated_args {
                                    if let Value::String(arg_str) = arg {
                                        result.push_str(arg_str);
                                    } else {
                                        // Convert non-string args to string
                                        result.push_str(&format!("{:?}", arg));
                                    }
                                }
                                Ok(Value::String(result))
                            },
                            "includes" => {
                                // Check if string includes substring
                                if evaluated_args.len() < 1 {
                                    return Err("includes requires a search string argument".to_string());
                                }
                                if let Value::String(search) = &evaluated_args[0] {
                                    Ok(Value::Boolean(s.contains(search)))
                                } else {
                                    Err("includes requires a string argument".to_string())
                                }
                            },
                            "indexOf" => {
                                // Find index of substring
                                if evaluated_args.len() < 1 {
                                    return Err("indexOf requires a search string argument".to_string());
                                }
                                if let Value::String(search) = &evaluated_args[0] {
                                    if let Some(idx) = s.find(search) {
                                        Ok(Value::Number(idx as i64))
                                    } else {
                                        Ok(Value::Number(-1))
                                    }
                                } else {
                                    Err("indexOf requires a string argument".to_string())
                                }
                            },
                            "repeat" => {
                                // Repeat string n times
                                if evaluated_args.len() < 1 {
                                    return Err("repeat requires a count argument".to_string());
                                }
                                if let Value::Number(count) = &evaluated_args[0] {
                                    let count = *count as usize;
                                    Ok(Value::String(s.repeat(count)))
                                } else {
                                    Err("repeat requires a number argument".to_string())
                                }
                            },
                            "slice" => {
                                // Get substring
                                if evaluated_args.len() < 1 {
                                    return Err("slice requires a start index argument".to_string());
                                }
                                if let Value::Number(start) = &evaluated_args[0] {
                                    let start = *start as usize;
                                    let end = if evaluated_args.len() > 1 {
                                        if let Value::Number(end) = &evaluated_args[1] {
                                            *end as usize
                                        } else {
                                            return Err("slice end index must be a number".to_string());
                                        }
                                    } else {
                                        s.len()
                                    };
                                    
                                    if start <= s.len() && end <= s.len() && start <= end {
                                        Ok(Value::String(s[start..end].to_string()))
                                    } else {
                                        Ok(Value::String("".to_string()))
                                    }
                                } else {
                                    Err("slice requires number indices".to_string())
                                }
                            },
                            "split" => {
                                // Split string into array
                                let separator = if evaluated_args.len() > 0 {
                                    if let Value::String(sep) = &evaluated_args[0] {
                                        sep.as_str()
                                    } else {
                                        return Err("split separator must be a string".to_string());
                                    }
                                } else {
                                    ""
                                };
                                
                                let parts: Vec<Value> = s.split(separator)
                                    .map(|part| Value::String(part.to_string()))
                                    .collect();
                                
                                Ok(Value::Array(parts))
                            },
                            "toString" => {
                                // Return the string itself
                                Ok(Value::String(s.clone()))
                            },
                            "valueOf" => {
                                // Return the string itself
                                Ok(Value::String(s.clone()))
                            },
                            _ => Err(format!("Method '{}' not found on string", method))
                        }
                    },
                    Value::Number(n) => {
                        // Handle number methods
                        match method.as_str() {
                            "toString" => {
                                // Convert number to string, optionally with a radix
                                let radix = if evaluated_args.len() > 0 {
                                    if let Value::Number(r) = &evaluated_args[0] {
                                        *r as u32
                                    } else {
                                        return Err("toString radix must be a number".to_string());
                                    }
                                } else {
                                    10 // Default to base 10
                                };
                                
                                if radix < 2 || radix > 36 {
                                    return Err("toString radix must be between 2 and 36".to_string());
                                }
                                
                                // For now, just convert to base 10 string since Rust doesn't have a built-in
                                // for arbitrary base conversion
                                Ok(Value::String(n.to_string()))
                            },
                            "toFixed" => {
                                // Format number with fixed decimal places
                                let digits = if evaluated_args.len() > 0 {
                                    if let Value::Number(d) = &evaluated_args[0] {
                                        *d as usize
                                    } else {
                                        return Err("toFixed digits must be a number".to_string());
                                    }
                                } else {
                                    0 // Default to 0 decimal places
                                };
                                
                                // Format with specified number of decimal places
                                Ok(Value::String(format!("{:.1$}", n as f64, digits)))
                            },
                            "toPrecision" => {
                                // Format number with specified precision
                                if evaluated_args.len() < 1 {
                                    return Err("toPrecision requires a precision argument".to_string());
                                }
                                if let Value::Number(precision) = &evaluated_args[0] {
                                    // Simple implementation for now
                                    Ok(Value::String(format!("{:.1$e}", n as f64, *precision as usize)))
                                } else {
                                    Err("toPrecision requires a number argument".to_string())
                                }
                            },
                            "toExponential" => {
                                // Format number in exponential notation
                                let frac_digits = if evaluated_args.len() > 0 {
                                    if let Value::Number(d) = &evaluated_args[0] {
                                        *d as usize
                                    } else {
                                        return Err("toExponential digits must be a number".to_string());
                                    }
                                } else {
                                    6 // Default to 6 decimal places
                                };
                                
                                // Format in exponential notation
                                Ok(Value::String(format!("{:.1$e}", n as f64, frac_digits)))
                            },
                            "valueOf" => {
                                // Return the number itself
                                Ok(Value::Number(*n))
                            },
                            _ => Err(format!("Method '{}' not found on number", method))
                        }
                    },
                    Value::Array(arr) => {
                        // Handle array methods
                        match method.as_str() {
                            "push" => {
                                let mut new_arr = arr.clone();
                                for arg in evaluated_args {
                                    new_arr.push(arg);
                                }
                                Ok(Value::Array(new_arr))
                            },
                            "pop" => {
                                let mut new_arr = arr.clone();
                                if let Some(item) = new_arr.pop() {
                                    Ok(item)
                                } else {
                                    Ok(Value::Undefined)
                                }
                            },
                            "map" => {
                                // Handle array.map(callback)
                                if evaluated_args.len() < 1 {
                                    return Err("map requires a callback function".to_string());
                                }
                                
                                let callback = &evaluated_args[0];
                                
                                match callback {
                                    Value::Function(_, param_names, body) => {
                                        // Create a new array to hold the mapped values
                                        let mut result = Vec::new();
                                        
                                        // Apply the callback to each element
                                        for (i, item) in arr.iter().enumerate() {
                                            // Create a new scope for each callback invocation
                                            let mut callback_scope = Scope::with_parent(Box::new(scope.clone()));
                                            
                                            // Bind the current item as the first parameter
                                            if param_names.len() > 0 {
                                                callback_scope.set(&param_names[0], item.clone());
                                            }
                                            
                                            // Bind the index as the second parameter if it exists
                                            if param_names.len() > 1 {
                                                callback_scope.set(&param_names[1], Value::Number(i as i64));
                                            }
                                            
                                            // Bind the array as the third parameter if it exists
                                            if param_names.len() > 2 {
                                                callback_scope.set(&param_names[2], Value::Array(arr.clone()));
                                            }
                                            
                                            // Call the callback function
                                            let mapped_value = self.evaluate_ast_with_scope(body, &mut callback_scope)?;
                                            result.push(mapped_value);
                                        }
                                        
                                        Ok(Value::Array(result))
                                    },
                                    _ => Err("map requires a callback function".to_string())
                                }
                            },
                            "filter" => {
                                // Handle array.filter(callback)
                                if evaluated_args.len() < 1 {
                                    return Err("filter requires a callback function".to_string());
                                }
                                
                                let callback = &evaluated_args[0];
                                
                                match callback {
                                    Value::Function(_, param_names, body) => {
                                        // Create a new array to hold the filtered values
                                        let mut result = Vec::new();
                                        
                                        // Apply the callback to each element
                                        for (i, item) in arr.iter().enumerate() {
                                            // Create a new scope for each callback invocation
                                            let mut callback_scope = Scope::with_parent(Box::new(scope.clone()));
                                            
                                            // Bind the current item as the first parameter
                                            if param_names.len() > 0 {
                                                callback_scope.set(&param_names[0], item.clone());
                                            }
                                            
                                            // Bind the index as the second parameter if it exists
                                            if param_names.len() > 1 {
                                                callback_scope.set(&param_names[1], Value::Number(i as i64));
                                            }
                                            
                                            // Bind the array as the third parameter if it exists
                                            if param_names.len() > 2 {
                                                callback_scope.set(&param_names[2], Value::Array(arr.clone()));
                                            }
                                            
                                            // Call the callback function
                                            let result_value = self.evaluate_ast_with_scope(body, &mut callback_scope)?;
                                            
                                            // Check if the result is truthy
                                            let is_truthy = match result_value {
                                                Value::Boolean(b) => b,
                                                Value::Number(n) => n != 0,
                                                Value::Float(f) => f != 0.0,
                                                Value::String(s) => !s.is_empty(),
                                                Value::Array(arr) => !arr.is_empty(),
                                                Value::Object(obj) => !obj.is_empty(),
                                                Value::Function(_, _, _) => true,
                                                Value::Null => false,
                                                Value::Undefined => false,
                                            };
                                            
                                            // If the result is truthy, include the item in the result
                                            if is_truthy {
                                                result.push(item.clone());
                                            }
                                        }
                                        
                                        Ok(Value::Array(result))
                                    },
                                    _ => Err("filter requires a callback function".to_string())
                                }
                            },
                            "forEach" => {
                                // Handle array.forEach(callback)
                                if evaluated_args.len() < 1 {
                                    return Err("forEach requires a callback function".to_string());
                                }
                                
                                let callback = &evaluated_args[0];
                                
                                match callback {
                                    Value::Function(_, param_names, body) => {
                                        // Apply the callback to each element
                                        for (i, item) in arr.iter().enumerate() {
                                            // Create a new scope for each callback invocation
                                            let mut callback_scope = Scope::with_parent(Box::new(scope.clone()));
                                            
                                            // Bind the current item as the first parameter
                                            if param_names.len() > 0 {
                                                callback_scope.set(&param_names[0], item.clone());
                                            }
                                            
                                            // Bind the index as the second parameter if it exists
                                            if param_names.len() > 1 {
                                                callback_scope.set(&param_names[1], Value::Number(i as i64));
                                            }
                                            
                                            // Bind the array as the third parameter if it exists
                                            if param_names.len() > 2 {
                                                callback_scope.set(&param_names[2], Value::Array(arr.clone()));
                                            }
                                            
                                            // Call the callback function and ignore the result
                                            let _ = self.evaluate_ast_with_scope(body, &mut callback_scope)?;
                                        }
                                        
                                        // forEach returns undefined
                                        Ok(Value::Undefined)
                                    },
                                    _ => Err("forEach requires a callback function".to_string())
                                }
                            },
                            "find" => {
                                // Handle array.find(callback)
                                if evaluated_args.len() < 1 {
                                    return Err("find requires a callback function".to_string());
                                }
                                
                                let callback = &evaluated_args[0];
                                
                                match callback {
                                    Value::Function(_, param_names, body) => {
                                        // Apply the callback to each element
                                        for (i, item) in arr.iter().enumerate() {
                                            // Create a new scope for each callback invocation
                                            let mut callback_scope = Scope::with_parent(Box::new(scope.clone()));
                                            
                                            // Bind the current item as the first parameter
                                            if param_names.len() > 0 {
                                                callback_scope.set(&param_names[0], item.clone());
                                            }
                                            
                                            // Bind the index as the second parameter if it exists
                                            if param_names.len() > 1 {
                                                callback_scope.set(&param_names[1], Value::Number(i as i64));
                                            }
                                            
                                            // Bind the array as the third parameter if it exists
                                            if param_names.len() > 2 {
                                                callback_scope.set(&param_names[2], Value::Array(arr.clone()));
                                            }
                                            
                                            // Call the callback function
                                            let result_value = self.evaluate_ast_with_scope(body, &mut callback_scope)?;
                                            
                                            // Check if the result is truthy
                                            let is_truthy = match result_value {
                                                Value::Boolean(b) => b,
                                                Value::Number(n) => n != 0,
                                                Value::String(s) => !s.is_empty(),
                                                Value::Array(a) => !a.is_empty(),
                                                Value::Object(_) => true,
                                                Value::Null => false,
                                                Value::Undefined => false,
                                                Value::Function(_, _, _) => true,
                                            };
                                            
                                            // If truthy, return this item
                                            if is_truthy {
                                                return Ok(item.clone());
                                            }
                                        }
                                        
                                        // If no item was found, return undefined
                                        Ok(Value::Undefined)
                                    },
                                    _ => Err("find requires a callback function".to_string())
                                }
                            },
                            "join" => {
                                // Handle array.join(separator)
                                let separator = if evaluated_args.len() > 0 {
                                    if let Value::String(sep) = &evaluated_args[0] {
                                        sep.clone()
                                    } else {
                                        format!("{:?}", evaluated_args[0])
                                    }
                                } else {
                                    ",".to_string() // Default separator is comma
                                };
                                
                                // Convert array elements to strings and join them
                                let strings: Vec<String> = arr.iter().map(|item| {
                                    match item {
                                        Value::String(s) => s.clone(),
                                        Value::Number(n) => n.to_string(),
                                        Value::Boolean(b) => b.to_string(),
                                        Value::Null => "null".to_string(),
                                        Value::Undefined => "undefined".to_string(),
                                        _ => format!("{:?}", item)
                                    }
                                }).collect();
                                
                                Ok(Value::String(strings.join(&separator)))
                            },
                            "reverse" => {
                                // Handle array.reverse()
                                let mut new_arr = arr.clone();
                                new_arr.reverse();
                                Ok(Value::Array(new_arr))
                            },
                            "slice" => {
                                // Handle array.slice(start, end)
                                let start = if evaluated_args.len() > 0 {
                                    if let Value::Number(s) = &evaluated_args[0] {
                                        *s as usize
                                    } else {
                                        return Err("slice start index must be a number".to_string());
                                    }
                                } else {
                                    0 // Default start is 0
                                };
                                
                                let end = if evaluated_args.len() > 1 {
                                    if let Value::Number(e) = &evaluated_args[1] {
                                        *e as usize
                                    } else {
                                        return Err("slice end index must be a number".to_string());
                                    }
                                } else {
                                    arr.len() // Default end is array length
                                };
                                
                                // Create a new array with the sliced elements
                                if start <= arr.len() && end <= arr.len() && start <= end {
                                    let sliced: Vec<Value> = arr[start..end].to_vec();
                                    Ok(Value::Array(sliced))
                                } else {
                                    Ok(Value::Array(vec![]))
                                }
                            },
                            _ => Err(format!("Method '{}' not found on array", method))
                        }
                    },
                    Value::Object(obj) => {
                        // First check for built-in object methods
                        match method.as_str() {
                            "hasOwnProperty" => {
                                // Check if object has own property
                                if evaluated_args.len() < 1 {
                                    return Err("hasOwnProperty requires a property name".to_string());
                                }
                                
                                if let Value::String(prop) = &evaluated_args[0] {
                                    Ok(Value::Boolean(obj.contains_key(prop)))
                                } else {
                                    Err("hasOwnProperty requires a string argument".to_string())
                                }
                            },
                            "keys" => {
                                // Get object keys as array
                                let keys: Vec<Value> = obj.keys()
                                    .map(|k| Value::String(k.clone()))
                                    .collect();
                                
                                Ok(Value::Array(keys))
                            },
                            "values" => {
                                // Get object values as array
                                let values: Vec<Value> = obj.values()
                                    .cloned()
                                    .collect();
                                
                                Ok(Value::Array(values))
                            },
                            "entries" => {
                                // Get object entries as array of [key, value] arrays
                                let entries: Vec<Value> = obj.iter()
                                    .map(|(k, v)| {
                                        let entry = vec![Value::String(k.clone()), v.clone()];
                                        Value::Array(entry)
                                    })
                                    .collect();
                                
                                Ok(Value::Array(entries))
                            },
                            "toString" => {
                                // Convert object to string representation
                                let mut parts = Vec::new();
                                for (key, value) in obj.iter() {
                                    let value_str = match value {
                                        Value::String(s) => format!("\"{}\"" , s),
                                        Value::Number(n) => n.to_string(),
                                        Value::Boolean(b) => b.to_string(),
                                        Value::Null => "null".to_string(),
                                        Value::Undefined => "undefined".to_string(),
                                        Value::Array(_) => "[Array]".to_string(),
                                        Value::Object(_) => "[Object]".to_string(),
                                        Value::Function(_, _, _) => "[Function]".to_string(),
                                    };
                                    parts.push(format!("\"{}\":{}", key, value_str));
                                }
                                
                                Ok(Value::String(format!("{{{}}}", parts.join(","))))
                            },
                            _ => {
                                // Then check for user-defined methods
                                if let Some(Value::Function(_, param_names, body)) = obj.get(method) {
                                    // Create a new scope with the function's parameters
                                    let mut function_scope = Scope::with_parent(Box::new(scope.clone()));
                                    
                                    // Bind arguments to parameters
                                    for (i, param) in param_names.iter().enumerate() {
                                        let arg_value = if i < evaluated_args.len() {
                                            evaluated_args[i].clone()
                                        } else {
                                            Value::Undefined // Default value if argument is missing
                                        };
                                        function_scope.set(param, arg_value);
                                    }
                                    
                                    // Execute the function body
                                    self.evaluate_ast_with_scope(body, &mut function_scope)
                                } else {
                                    Err(format!("Method '{}' not found on object", method))
                                }
                            }
                        }
                    },
                    _ => Err(format!("Cannot call method '{}' on this value type", method))
                }
            },
            
            // Handle property access (obj.property)
            AstNode::PropertyAccess { object, property } => {
                // Evaluate the object first
                let obj_value = self.evaluate_ast_with_scope(object, scope)?;
                
                // Handle different property accesses based on the object type
                match obj_value {
                    Value::String(s) => {
                        // Handle string properties
                        match property.as_str() {
                            "length" => Ok(Value::Number(s.len() as i64)),
                            "toUpperCase" => Ok(Value::Function("toUpperCase".to_string(), vec![], Box::new(AstNode::Block(vec![])))),
                            "toLowerCase" => Ok(Value::Function("toLowerCase".to_string(), vec![], Box::new(AstNode::Block(vec![])))),
                            "trim" => Ok(Value::Function("trim".to_string(), vec![], Box::new(AstNode::Block(vec![])))),
                            "trimStart" => Ok(Value::Function("trimStart".to_string(), vec![], Box::new(AstNode::Block(vec![])))),
                            "trimEnd" => Ok(Value::Function("trimEnd".to_string(), vec![], Box::new(AstNode::Block(vec![])))),
                            "charAt" => Ok(Value::Function("charAt".to_string(), vec!["index".to_string()], Box::new(AstNode::Block(vec![])))),
                            "charCodeAt" => Ok(Value::Function("charCodeAt".to_string(), vec!["index".to_string()], Box::new(AstNode::Block(vec![])))),
                            "concat" => Ok(Value::Function("concat".to_string(), vec!["str".to_string()], Box::new(AstNode::Block(vec![])))),
                            "includes" => Ok(Value::Function("includes".to_string(), vec!["searchString".to_string(), "position".to_string()], Box::new(AstNode::Block(vec![])))),
                            "indexOf" => Ok(Value::Function("indexOf".to_string(), vec!["searchValue".to_string(), "fromIndex".to_string()], Box::new(AstNode::Block(vec![])))),
                            "lastIndexOf" => Ok(Value::Function("lastIndexOf".to_string(), vec!["searchValue".to_string(), "fromIndex".to_string()], Box::new(AstNode::Block(vec![])))),
                            "padStart" => Ok(Value::Function("padStart".to_string(), vec!["targetLength".to_string(), "padString".to_string()], Box::new(AstNode::Block(vec![])))),
                            "padEnd" => Ok(Value::Function("padEnd".to_string(), vec!["targetLength".to_string(), "padString".to_string()], Box::new(AstNode::Block(vec![])))),
                            "repeat" => Ok(Value::Function("repeat".to_string(), vec!["count".to_string()], Box::new(AstNode::Block(vec![])))),
                            "replace" => Ok(Value::Function("replace".to_string(), vec!["searchValue".to_string(), "replaceValue".to_string()], Box::new(AstNode::Block(vec![])))),
                            "slice" => Ok(Value::Function("slice".to_string(), vec!["start".to_string(), "end".to_string()], Box::new(AstNode::Block(vec![])))),
                            "split" => Ok(Value::Function("split".to_string(), vec!["separator".to_string(), "limit".to_string()], Box::new(AstNode::Block(vec![])))),
                            "startsWith" => Ok(Value::Function("startsWith".to_string(), vec!["searchString".to_string(), "position".to_string()], Box::new(AstNode::Block(vec![])))),
                            "endsWith" => Ok(Value::Function("endsWith".to_string(), vec!["searchString".to_string(), "length".to_string()], Box::new(AstNode::Block(vec![])))),
                            "substring" => Ok(Value::Function("substring".to_string(), vec!["start".to_string(), "end".to_string()], Box::new(AstNode::Block(vec![])))),
                            "toString" => Ok(Value::Function("toString".to_string(), vec![], Box::new(AstNode::Block(vec![])))),
                            "valueOf" => Ok(Value::Function("valueOf".to_string(), vec![], Box::new(AstNode::Block(vec![])))),
                            _ => Err(format!("Property '{}' not found on string", property))
                        }
                    },
                    Value::Array(arr) => {
                        // Handle array properties
                        match property.as_str() {
                            "length" => Ok(Value::Number(arr.len() as i64)),
                            "push" => Ok(Value::Function("push".to_string(), vec!["item".to_string()], Box::new(AstNode::Block(vec![])))),
                            "pop" => Ok(Value::Function("pop".to_string(), vec![], Box::new(AstNode::Block(vec![])))),
                            "map" => {
                                // Create a map function that takes a callback function
                                Ok(Value::Function(
                                    "map".to_string(), 
                                    vec!["callback".to_string()], 
                                    Box::new(AstNode::Block(vec![]))
                                ))
                            },
                            "filter" => {
                                // Create a filter function that takes a callback function
                                Ok(Value::Function(
                                    "filter".to_string(), 
                                    vec!["callback".to_string()], 
                                    Box::new(AstNode::Block(vec![]))
                                ))
                            },
                            "forEach" => {
                                Ok(Value::Function(
                                    "forEach".to_string(), 
                                    vec!["callback".to_string()], 
                                    Box::new(AstNode::Block(vec![]))
                                ))
                            },
                            "reduce" => {
                                Ok(Value::Function(
                                    "reduce".to_string(), 
                                    vec!["callback".to_string(), "initialValue".to_string()], 
                                    Box::new(AstNode::Block(vec![]))
                                ))
                            },
                            "reduceRight" => {
                                Ok(Value::Function(
                                    "reduceRight".to_string(), 
                                    vec!["callback".to_string(), "initialValue".to_string()], 
                                    Box::new(AstNode::Block(vec![]))
                                ))
                            },
                            "find" => {
                                Ok(Value::Function(
                                    "find".to_string(), 
                                    vec!["callback".to_string()], 
                                    Box::new(AstNode::Block(vec![]))
                                ))
                            },
                            "findIndex" => {
                                Ok(Value::Function(
                                    "findIndex".to_string(), 
                                    vec!["callback".to_string()], 
                                    Box::new(AstNode::Block(vec![]))
                                ))
                            },
                            "every" => {
                                Ok(Value::Function(
                                    "every".to_string(), 
                                    vec!["callback".to_string()], 
                                    Box::new(AstNode::Block(vec![]))
                                ))
                            },
                            "some" => {
                                Ok(Value::Function(
                                    "some".to_string(), 
                                    vec!["callback".to_string()], 
                                    Box::new(AstNode::Block(vec![]))
                                ))
                            },
                            "includes" => {
                                Ok(Value::Function(
                                    "includes".to_string(), 
                                    vec!["element".to_string(), "fromIndex".to_string()], 
                                    Box::new(AstNode::Block(vec![]))
                                ))
                            },
                            "indexOf" => {
                                Ok(Value::Function(
                                    "indexOf".to_string(), 
                                    vec!["element".to_string(), "fromIndex".to_string()], 
                                    Box::new(AstNode::Block(vec![]))
                                ))
                            },
                            "lastIndexOf" => {
                                Ok(Value::Function(
                                    "lastIndexOf".to_string(), 
                                    vec!["element".to_string(), "fromIndex".to_string()], 
                                    Box::new(AstNode::Block(vec![]))
                                ))
                            },
                            "join" => {
                                Ok(Value::Function(
                                    "join".to_string(), 
                                    vec!["separator".to_string()], 
                                    Box::new(AstNode::Block(vec![]))
                                ))
                            },
                            "slice" => {
                                Ok(Value::Function(
                                    "slice".to_string(), 
                                    vec!["start".to_string(), "end".to_string()], 
                                    Box::new(AstNode::Block(vec![]))
                                ))
                            },
                            "concat" => {
                                Ok(Value::Function(
                                    "concat".to_string(), 
                                    vec!["array".to_string()], 
                                    Box::new(AstNode::Block(vec![]))
                                ))
                            },
                            "shift" => {
                                Ok(Value::Function(
                                    "shift".to_string(), 
                                    vec![], 
                                    Box::new(AstNode::Block(vec![]))
                                ))
                            },
                            "unshift" => {
                                Ok(Value::Function(
                                    "unshift".to_string(), 
                                    vec!["element".to_string()], 
                                    Box::new(AstNode::Block(vec![]))
                                ))
                            },
                            "reverse" => {
                                Ok(Value::Function(
                                    "reverse".to_string(), 
                                    vec![], 
                                    Box::new(AstNode::Block(vec![]))
                                ))
                            },
                            "sort" => {
                                Ok(Value::Function(
                                    "sort".to_string(), 
                                    vec!["compareFunction".to_string()], 
                                    Box::new(AstNode::Block(vec![]))
                                ))
                            },
                            "fill" => {
                                Ok(Value::Function(
                                    "fill".to_string(), 
                                    vec!["value".to_string(), "start".to_string(), "end".to_string()], 
                                    Box::new(AstNode::Block(vec![]))
                                ))
                            },
                            "flat" => {
                                Ok(Value::Function(
                                    "flat".to_string(), 
                                    vec!["depth".to_string()], 
                                    Box::new(AstNode::Block(vec![]))
                                ))
                            },
                            "flatMap" => {
                                Ok(Value::Function(
                                    "flatMap".to_string(), 
                                    vec!["callback".to_string()], 
                                    Box::new(AstNode::Block(vec![]))
                                ))
                            },
                            "toString" => {
                                Ok(Value::Function(
                                    "toString".to_string(), 
                                    vec![], 
                                    Box::new(AstNode::Block(vec![]))
                                ))
                            },
                            _ => Err(format!("Property '{}' not found on array", property))
                        }
                    },
                    Value::Number(n) => {
                        // Handle number properties
                        match property.as_str() {
                            "toString" => Ok(Value::Function("toString".to_string(), vec!["radix".to_string()], Box::new(AstNode::Block(vec![])))),
                            "toFixed" => Ok(Value::Function("toFixed".to_string(), vec!["digits".to_string()], Box::new(AstNode::Block(vec![])))),
                            "toPrecision" => Ok(Value::Function("toPrecision".to_string(), vec!["precision".to_string()], Box::new(AstNode::Block(vec![])))),
                            "toExponential" => Ok(Value::Function("toExponential".to_string(), vec!["fractionDigits".to_string()], Box::new(AstNode::Block(vec![])))),
                            "valueOf" => Ok(Value::Function("valueOf".to_string(), vec![], Box::new(AstNode::Block(vec![])))),
                            _ => Err(format!("Property '{}' not found on number", property))
                        }
                    },
                    Value::Object(obj) => {
                        // Handle object properties by looking them up in the object
                        if let Some(value) = obj.get(property) {
                            Ok(value.clone())
                        } else {
                            // Check for common object methods
                            match property.as_str() {
                                "toString" => Ok(Value::Function("toString".to_string(), vec![], Box::new(AstNode::Block(vec![])))),
                                "valueOf" => Ok(Value::Function("valueOf".to_string(), vec![], Box::new(AstNode::Block(vec![])))),
                                "hasOwnProperty" => Ok(Value::Function("hasOwnProperty".to_string(), vec!["property".to_string()], Box::new(AstNode::Block(vec![])))),
                                "keys" => Ok(Value::Function("keys".to_string(), vec![], Box::new(AstNode::Block(vec![])))),
                                "values" => Ok(Value::Function("values".to_string(), vec![], Box::new(AstNode::Block(vec![])))),
                                "entries" => Ok(Value::Function("entries".to_string(), vec![], Box::new(AstNode::Block(vec![])))),
                                _ => Err(format!("Property '{}' not found on object", property))
                            }
                        }
                    },
                    _ => Err(format!("Cannot access property '{}' on this value type", property))
                }
            },
            
            // For simplicity, we'll just return a placeholder for other node types
            _ => Err(format!("Evaluation not implemented for this AST node: {:?}", ast))
        }
    }
}
