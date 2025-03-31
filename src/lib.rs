// SmashLang library module exports

pub mod lexer;
pub mod parser;
pub mod repl;
pub mod compiler;
pub mod codegen;
pub mod runtime_regex;

// Test modules - temporarily disabled
/*
#[cfg(test)]
mod tests {
    pub mod lexer_tests;
    pub mod parser_tests;
    pub mod codegen_tests;
}
*/
