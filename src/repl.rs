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
                                Value::Function(_, _, _) => result.push_str("[Function]"),
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
            
            // For simplicity, we'll just return a placeholder for other node types
            _ => Err(format!("Evaluation not implemented for this AST node: {:?}", ast))
        }
    }
}
