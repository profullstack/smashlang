use logos::Logos;
use std::ops::Range;

/// Token represents all possible token types in the SmashLang language
#[derive(Logos, Debug, Clone, PartialEq)]
pub enum Token {
    // Keywords
    #[token("const")]
    Const,
    
    #[token("let")]
    Let,
    
    #[token("fn")]
    Fn,
    
    #[token("return")]
    Return,
    
    #[token("import")]
    Import,
    
    #[token("async")]
    Async,
    
    #[token("await")]
    Await,
    
    // Error handling keywords
    #[token("try")]
    Try,
    
    #[token("catch")]
    Catch,
    
    #[token("finally")]
    Finally,
    
    #[token("throw")]
    Throw,
    
    #[token("new")]
    New,
    
    // Loop control keywords
    #[token("break")]
    Break,
    
    #[token("continue")]
    Continue,
    
    // Control flow keywords
    #[token("if")]
    If,
    
    #[token("else")]
    Else,
    
    #[token("while")]
    While,
    
    #[token("for")]
    For,
    
    #[token("do")]
    Do,
    
    #[token("switch")]
    Switch,
    
    #[token("case")]
    Case,
    
    #[token("default")]
    Default,
    
    // Iteration keywords
    #[token("in")]
    In,
    
    #[token("of")]
    Of,
    
    // Boolean literals
    #[token("true", |_| true)]
    #[token("false", |_| false)]
    Bool(bool),
    
    #[token("null")]
    Null,
    
    // Identifiers
    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*", |lex| lex.slice().to_string(), priority = 2)]
    Identifier(String),
    
    // Number literals
    #[regex(r"[0-9]+", |lex| lex.slice().parse().ok(), priority = 2)]
    Number(i64),
    
    #[regex(r"[0-9]+\.[0-9]+", |lex| lex.slice().parse().ok(), priority = 2)]
    Float(f64),
    
    // String literals
    #[regex(r#""([^"\\]|\\["\\nt])*""#, |lex| {
        let slice = lex.slice();
        let content = &slice[1..slice.len()-1]; // Remove quotes
        unescape_string(content)
    }, priority = 2)]
    String(String),
    
    #[regex(r#"'([^'\\]|\\['\\nt])*'"#, |lex| {
        let slice = lex.slice();
        let content = &slice[1..slice.len()-1]; // Remove quotes
        unescape_string(content)
    }, priority = 2)]
    SingleQuoteString(String),
    
    // Template literals
    #[regex(r"`[^`]*`", |lex| {
        let slice = lex.slice();
        let content = &slice[1..slice.len()-1]; // Remove backticks
        unescape_string(content)
    }, priority = 2)]
    TemplateString(String),
    
    // Regular expressions
    #[regex(r"/([^/\\]|\\.)+/[gimuy]*", |lex| {
        let slice = lex.slice();
        slice.to_string()
    }, priority = 2)]
    Regex(String),
    
    // Basic operators
    #[token("+")]
    Plus,
    
    #[token("-")]
    Minus,
    
    #[token("*")]
    Star,
    
    #[token("/")]
    Slash,
    
    #[token("=")]
    Equal,
    
    // Comparison operators
    #[token("==")]
    EqualEqual,
    
    #[token("!=")]
    NotEqual,
    
    #[token("===")]
    StrictEqual,
    
    #[token("!==")]
    StrictNotEqual,
    
    #[token("<")]
    LessThan,
    
    #[token(">")]
    GreaterThan,
    
    #[token("<=")]
    LessThanEqual,
    
    #[token(">=")]
    GreaterThanEqual,
    
    // Logical operators
    #[token("&&")]
    LogicalAnd,
    
    #[token("||")]
    LogicalOr,
    
    #[token("!")]
    LogicalNot,
    
    // Bitwise operators
    #[token("&")]
    BitwiseAnd,
    
    #[token("|")]
    BitwiseOr,
    
    #[token("^")]
    BitwiseXor,
    
    #[token("~")]
    BitwiseNot,
    
    #[token("<<")]
    BitwiseLeftShift,
    
    #[token(">>")]
    BitwiseRightShift,
    
    #[token(">>>")]
    BitwiseUnsignedRightShift,
    
    // Increment/Decrement operators
    #[token("++")]
    Increment,
    
    #[token("--")]
    Decrement,
    
    // Compound assignment operators
    #[token("+=")]
    PlusEqual,
    
    #[token("-=")]
    MinusEqual,
    
    #[token("*=")]
    StarEqual,
    
    #[token("/=")]
    SlashEqual,
    
    #[token("&=")]
    BitwiseAndEqual,
    
    #[token("|=")]
    BitwiseOrEqual,
    
    #[token("^=")]
    BitwiseXorEqual,
    
    #[token("<<=")]
    BitwiseLeftShiftEqual,
    
    #[token(">>=")]
    BitwiseRightShiftEqual,
    
    #[token("%=")]
    ModuloEqual,
    
    // Conditional (ternary) operator
    #[token("?")]
    QuestionMark,
    
    // Optional chaining and nullish coalescing
    #[token("?.")]
    OptionalChaining,
    
    #[token("??")]
    NullishCoalescing,
    
    // Modulo operator (with higher priority than Percent)
    #[token("%", priority = 2)]
    Modulo,
    
    // Arrow functions
    #[token("=>")]
    FatArrow,
    
    // Delimiters
    #[token(":")]
    Colon,
    
    #[token("(")]
    LParen,
    
    #[token(")")]
    RParen,
    
    #[token("{")]
    LBrace,
    
    #[token("}")]
    RBrace,
    
    #[token("[")]
    LBracket,
    
    #[token("]")]
    RBracket,
    
    #[token(",")]
    Comma,
    
    #[token(";")]
    Semicolon,
    
    // Special operators
    #[token(".")]
    Dot,
    
    #[token("...")]
    Ellipsis,
    
    #[token("$")]
    Dollar,
    
    // Percent sign (with lower priority than Modulo)
    #[token("%", priority = 1)]
    Percent,
    
    #[token("`")]
    Backtick,
    
    #[token("'")]
    SingleQuote,
    
    #[token("@")]
    At,
    
    // Skip whitespace and comments
    #[regex(r"[ \t\n\r]+", logos::skip, priority = 3)]
    #[regex(r"//[^\n]*", logos::skip, priority = 3)]
    #[regex(r"/\*([^*]|\*[^/])*\*/", logos::skip, priority = 3)]
    // Error fallback
    #[regex(r".", logos::skip, priority = 1)]
    Error,
}

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

/// Helper function to unescape string literals
fn unescape_string(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    let mut chars = s.chars().peekable();
    
    while let Some(c) = chars.next() {
        if c == '\\' {
            if let Some(next) = chars.next() {
                match next {
                    'n' => result.push('\n'),
                    'r' => result.push('\r'),
                    't' => result.push('\t'),
                    '\\' => result.push('\\'),
                    '\'' => result.push('\''),
                    '"' => result.push('"'),
                    '`' => result.push('`'),
                    _ => {
                        result.push('\\');
                        result.push(next);
                    }
                }
            } else {
                result.push('\\');
            }
        } else {
            result.push(c);
        }
    }
    
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_lexer_basic() {
        let input = "let x = 42;";
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize();
        
        assert_eq!(tokens.len(), 5);
        assert!(matches!(tokens[0].token, Token::Let));
        assert!(matches!(tokens[1].token, Token::Identifier(ref s) if s == "x"));
        assert!(matches!(tokens[2].token, Token::Equal));
        assert!(matches!(tokens[3].token, Token::Number(42)));
        assert!(matches!(tokens[4].token, Token::Semicolon));
    }
    
    #[test]
    fn test_lexer_string_literals() {
        let input = r#"let message = "Hello, world!"; let single = 'Single quotes';"#;
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize();
        
        assert!(matches!(tokens[3].token, Token::String(ref s) if s == "Hello, world!"));
        assert!(matches!(tokens[8].token, Token::SingleQuoteString(ref s) if s == "Single quotes"));
    }
    
    #[test]
    fn test_lexer_regex() {
        let input = r#"let pattern = /[a-z]+/g;"#;
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize();
        
        assert!(matches!(tokens[3].token, Token::Regex(ref s) if s == "/[a-z]+/g"));
    }
}
