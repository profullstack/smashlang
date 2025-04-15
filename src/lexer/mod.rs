// Lexer module for SmashLang

// Re-export components
pub mod token;
pub mod core;
pub mod utils;

// Re-export main types for easier access
pub use token::Token;
pub use core::{Lexer, TokenWithSpan};
pub use utils::unescape_string;

// Tests
#[cfg(test)]
mod tests;