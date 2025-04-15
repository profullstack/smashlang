//! SmashLang - A JavaScript-like programming language in Rust
//!
//! SmashLang is a dynamically-typed language with JavaScript-like syntax
//! that compiles to native binaries across all major platforms.
//!
//! # Features
//!
//! - JavaScript-like syntax
//! - Dynamic typing
//! - Native date/time support
//! - Regular expressions
//! - Control flow constructs (if, for, while)
//! - Functions and closures
//! - Error handling with try/catch
//! - Async/await and Promises
//! - Cross-platform compilation

pub mod lexer;
pub mod parser;
pub mod interpreter;
pub mod compiler;
pub mod runtime;

/// Re-export main components for easier access
pub use lexer::Lexer;
pub use parser::{SmashParser, AstNode};
use pest::Parser as PestParser;
pub use interpreter::{Interpreter, Value};
pub use compiler::Compiler;

/// Version information
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Parses and executes SmashLang code
///
/// # Examples
///
/// ```
/// use smashlang::execute;
///
/// let result = execute("let x = 42; x * 2;").unwrap();
/// assert_eq!(result.to_string(), "84");
/// ```
pub fn execute(source: &str) -> Result<Value, String> {
    // Parse the source code
    let mut lexer = Lexer::new(source);
    let _tokens = lexer.tokenize();

    let mut pairs = match <SmashParser as PestParser<parser::Rule>>::parse(parser::Rule::program, source) {
        Ok(pairs) => pairs,
        Err(err) => {
            return Err(format!("Parse error: {}", err));
        }
    };
    let ast = match pairs.next().and_then(AstNode::from_pair) {
        Some(ast) => ast,
        None => {
            return Err("Failed to convert parse tree to AST".to_string());
        }
    };

    // Interpret the AST
    let interpreter = Interpreter::new();
    match interpreter.evaluate(&ast) {
        Ok(value) => Ok(value),
        Err(err) => Err(format!("Runtime error: {}", err)),
    }
}

/// Compiles SmashLang code to a native function
///
/// # Examples
///
/// ```
/// use smashlang::compile;
///
/// let compiled_fn = compile("let x = 42; x * 2;").unwrap();
/// let result = unsafe { compiled_fn.execute() };
/// assert_eq!(result, 84);
/// ```
pub fn compile(source: &str) -> Result<compiler::CompiledFunction, String> {
    // Parse the source code
    let mut lexer = Lexer::new(source);
    let _tokens = lexer.tokenize();

    let mut pairs = match <SmashParser as PestParser<parser::Rule>>::parse(parser::Rule::program, source) {
        Ok(pairs) => pairs,
        Err(err) => {
            return Err(format!("Parse error: {}", err));
        }
    };
    let ast = match pairs.next().and_then(AstNode::from_pair) {
        Some(ast) => ast,
        None => {
            return Err("Failed to convert parse tree to AST".to_string());
        }
    };

    // Compile the AST
    let mut compiler = Compiler::new();
    compiler.compile(&ast)
}
