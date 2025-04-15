// Interpreter module for SmashLang

// Re-export components
pub mod value;
pub mod function;
pub mod environment;
pub mod core;

// Re-export main types for easier access
pub use value::Value;
pub use function::Function;
pub use environment::Environment;
pub use core::Interpreter;