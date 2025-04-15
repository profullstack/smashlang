use pest_derive::Parser;
use pest::Parser;
use pest::iterators::{Pair, Pairs};
use crate::parser::ast::AstNode;

/// Parser for SmashLang
#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct SmashParser;

impl SmashParser {
    /// Parse a string into an AST
    pub fn parse(source: &str) -> Result<Pairs<Rule>, pest::error::Error<Rule>> {
        <SmashParser as Parser<Rule>>::parse(Rule::program, source)
    }
}

impl AstNode {
    /// Convert a pest Pair to an AstNode
    pub fn from_pair(pair: Pair<Rule>) -> Option<Self> {
        match pair.as_rule() {
            Rule::program => {
                let nodes = pair.into_inner()
                    .filter_map(AstNode::from_pair)
                    .collect();
                Some(AstNode::Program(nodes))
            }
            // Add more rules here as needed for deeper parsing
            _ => None,
        }
    }
    
    /// Convert pest Pairs to an AstNode
    pub fn from_pairs(_pairs: Pairs<'_, Rule>) -> Self {
        // Placeholder: returns a dummy node
        AstNode::Undefined
    }
}