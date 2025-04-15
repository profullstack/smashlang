use std::ops::Range;
use logos::Logos;
use crate::lexer::token::Token;

/// TokenWithSpan represents a token with its position in the source code
#[derive(Debug, Clone)]
pub struct TokenWithSpan {
    pub token: Token,
    pub span: Range<usize>,
}

impl TokenWithSpan {
    pub fn new(token: Token, span: Range<usize>) -> Self {
        Self { token, span }
    }
}

/// Lexer for SmashLang
pub struct Lexer<'a> {
    logos_lexer: logos::Lexer<'a, Token>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            logos_lexer: Token::lexer(input),
        }
    }
    
    pub fn tokenize(&mut self) -> Vec<TokenWithSpan> {
        let mut tokens = Vec::new();
        
        while let Some(token) = self.logos_lexer.next() {
            if let Ok(token) = token {
                let span = self.logos_lexer.span();
                tokens.push(TokenWithSpan::new(token, span));
            }
            // Skip errors
        }
        
        tokens
    }
}