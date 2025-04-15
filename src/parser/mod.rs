// Parser module for SmashLang

// Re-export components
pub mod ast;
pub mod core;
pub mod methods;

// Re-export main types for easier access
pub use ast::AstNode;
pub use ast::{
    Parameter, ClassMember, DestructuringTarget,
    SwitchCase, ImportSpecifier, ExportSpecifier
};
pub use core::SmashParser;
pub use core::Rule;