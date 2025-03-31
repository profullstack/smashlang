#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Keywords
    Const,
    Let,
    Fn,
    Return,
    Import,
    
    // Error handling keywords
    Try,
    Catch,
    Finally,
    Throw,
    New,
    
    // Loop control keywords
    Break,
    Continue,
    
    // Control flow keywords
    If,
    Else,
    While,
    For,
    Do,
    Switch,
    Case,
    Default,
    
    // Iteration keywords
    In,
    Of,
    
    // Literals
    Identifier(String),
    Number(i64),
    Float(f64),
    String(String),
    SingleQuoteString(String), // Single-quoted strings
    TemplateString(String), // Template literals with backticks
    Regex(String),
    Bool(bool),
    Null,
    
    // Basic operators
    Plus,      // +
    Minus,     // -
    Star,      // *
    Slash,     // /
    Equal,     // =
    
    // Comparison operators
    EqualEqual,       // ==
    NotEqual,         // !=
    StrictEqual,      // ===
    StrictNotEqual,   // !==
    LessThan,         // <
    GreaterThan,      // >
    LessThanEqual,    // <=
    GreaterThanEqual, // >=
    
    // Logical operators
    LogicalAnd,        // &&
    LogicalOr,         // ||
    LogicalNot,        // !
    
    // Bitwise operators
    BitwiseAnd,       // &
    BitwiseOr,        // |
    BitwiseXor,       // ^
    BitwiseNot,       // ~
    BitwiseLeftShift,        // <<
    BitwiseRightShift,       // >>
    BitwiseUnsignedRightShift, // >>>
    
    // Increment/Decrement operators
    Increment, // ++
    Decrement, // --
    
    // Compound assignment operators
    PlusEqual,        // +=
    MinusEqual,       // -=
    StarEqual,        // *=
    SlashEqual,       // /=
    BitwiseAndEqual,         // &=
    BitwiseOrEqual,          // |=
    BitwiseXorEqual,         // ^=
    BitwiseLeftShiftEqual,   // <<=
    BitwiseRightShiftEqual,  // >>=
    ModuloEqual,      // %=
    
    // Conditional (ternary) operator
    QuestionMark,     // ?
    
    // Optional chaining and nullish coalescing
    OptionalChaining, // ?.
    NullishCoalescing, // ??
    
    // Modulo operator
    Modulo,     // %
    
    // Delimiters
    Colon,     // :
    LParen,    // (
    RParen,    // )
    LBrace,    // {
    RBrace,    // }
    LBracket,  // [
    RBracket,  // ]
    Comma,     // ,
    Semicolon, // ;
    
    // Special operators
    Dot,       // .
    Ellipsis,  // ... (spread operator)
    Dollar,    // $
    Percent,   // %
    Backtick,  // `
    SingleQuote, // '
    At,        // @
}

pub fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(&ch) = chars.peek() {
        match ch {
            ' ' | '\n' | '\r' | '\t' => {
                chars.next();
            }
            '+' => {
                chars.next(); // consume the first '+'
                
                // Check for ++ or +=
                if let Some(&next_ch) = chars.peek() {
                    match next_ch {
                        '+' => {
                            tokens.push(Token::Increment);
                            chars.next(); // consume the second '+'
                        },
                        '=' => {
                            tokens.push(Token::PlusEqual);
                            chars.next(); // consume the '='
                        },
                        _ => tokens.push(Token::Plus),
                    }
                } else {
                    tokens.push(Token::Plus);
                }
            }
            '-' => {
                chars.next(); // consume the first '-'
                
                // Check for -- or -=
                if let Some(&next_ch) = chars.peek() {
                    match next_ch {
                        '-' => {
                            tokens.push(Token::Decrement);
                            chars.next(); // consume the second '-'
                        },
                        '=' => {
                            tokens.push(Token::MinusEqual);
                            chars.next(); // consume the '='
                        },
                        _ => tokens.push(Token::Minus),
                    }
                } else {
                    tokens.push(Token::Minus);
                }
            }
            '*' => {
                chars.next(); // consume the '*'
                
                // Check for *=
                if let Some(&next_ch) = chars.peek() {
                    if next_ch == '=' {
                        tokens.push(Token::StarEqual);
                        chars.next(); // consume the '='
                    } else {
                        tokens.push(Token::Star);
                    }
                } else {
                    tokens.push(Token::Star);
                }
            }
            '/' => {
                chars.next(); // consume the '/'
                
                // Check for /= or // (comment) or /* (block comment)
                if let Some(&next_ch) = chars.peek() {
                    match next_ch {
                        '=' => {
                            tokens.push(Token::SlashEqual);
                            chars.next(); // consume the '='
                        },
                        '/' => {
                            // Line comment, consume until end of line
                            chars.next(); // consume the second '/'
                            while let Some(&ch) = chars.peek() {
                                if ch == '\n' {
                                    break;
                                }
                                chars.next();
                            }
                        },
                        '*' => {
                            // Block comment, consume until */
                            chars.next(); // consume the '*'
                            let mut prev_char = ' ';
                            
                            while let Some(&ch) = chars.peek() {
                                if prev_char == '*' && ch == '/' {
                                    chars.next(); // consume the '/'
                                    break;
                                }
                                prev_char = ch;
                                chars.next();
                            }
                        },
                        _ => tokens.push(Token::Slash),
                    }
                } else {
                    tokens.push(Token::Slash);
                }
            }
            '=' => {
                chars.next(); // consume the '='
                
                // Check for == or ===
                if let Some(&next_ch) = chars.peek() {
                    if next_ch == '=' {
                        chars.next(); // consume the second '='
                        
                        // Check for ===
                        if let Some(&third_ch) = chars.peek() {
                            if third_ch == '=' {
                                tokens.push(Token::StrictEqual);
                                chars.next(); // consume the third '='
                            } else {
                                tokens.push(Token::EqualEqual);
                            }
                        } else {
                            tokens.push(Token::EqualEqual);
                        }
                    } else {
                        tokens.push(Token::Equal);
                    }
                } else {
                    tokens.push(Token::Equal);
                }
            }
            ':' => {
                chars.next(); // consume the ':'
                tokens.push(Token::Colon);
            }
            '(' => {
                chars.next(); // consume the '('
                tokens.push(Token::LParen);
            }
            ')' => {
                chars.next(); // consume the ')'
                tokens.push(Token::RParen);
            }
            '{' => {
                chars.next(); // consume the '{'
                tokens.push(Token::LBrace);
            }
            '}' => {
                chars.next(); // consume the '}'
                tokens.push(Token::RBrace);
            }
            '[' => {
                chars.next(); // consume the '['
                tokens.push(Token::LBracket);
            }
            ']' => {
                chars.next(); // consume the ']'
                tokens.push(Token::RBracket);
            }
            ',' => {
                chars.next(); // consume the ','
                tokens.push(Token::Comma);
            }
            ';' => {
                chars.next(); // consume the ';'
                tokens.push(Token::Semicolon);
            }
            '.' => {
                chars.next(); // consume the first '.'
                
                // Check for ... (spread operator)
                if let Some(&next_ch) = chars.peek() {
                    if next_ch == '.' {
                        chars.next(); // consume the second '.'
                        
                        if let Some(&third_ch) = chars.peek() {
                            if third_ch == '.' {
                                chars.next(); // consume the third '.'
                                tokens.push(Token::Ellipsis);
                            } else {
                                // This is not a valid token in our language
                                // For now, just push one dot
                                tokens.push(Token::Dot);
                                // Put back the second dot
                                tokens.push(Token::Dot);
                            }
                        } else {
                            // End of input after two dots
                            tokens.push(Token::Dot);
                            tokens.push(Token::Dot);
                        }
                    } else {
                        tokens.push(Token::Dot);
                    }
                } else {
                    tokens.push(Token::Dot);
                }
            }
            '0'..='9' => {
                let mut number = String::new();
                let mut is_float = false;
                
                while let Some(&ch) = chars.peek() {
                    if ch.is_digit(10) {
                        number.push(ch);
                        chars.next();
                    } else if ch == '.' {
                        // Check if this is a decimal point
                        if is_float {
                            // Already have a decimal point, this must be the end of the number
                            break;
                        }
                        
                        // Look ahead to see if this is a spread operator
                        let mut temp_chars = chars.clone();
                        temp_chars.next(); // Skip the dot
                        
                        if let Some(next_ch) = temp_chars.next() {
                            if next_ch == '.' {
                                // This is likely the start of a spread operator, don't consume the dot
                                // Just finish the number as an integer
                                break;
                            }
                        }
                        
                        is_float = true;
                        number.push(ch);
                        chars.next();
                        
                        // Parse the fractional part
                        while let Some(&digit) = chars.peek() {
                            if digit.is_digit(10) {
                                number.push(digit);
                                chars.next();
                            } else {
                                break;
                            }
                        }
                    } else {
                        break;
                    }
                }
                
                if is_float {
                    if let Ok(f) = number.parse::<f64>() {
                        tokens.push(Token::Float(f));
                    } else {
                        println!("Failed to parse float: {}", number);
                    }
                } else {
                    if let Ok(n) = number.parse::<i64>() {
                        tokens.push(Token::Number(n));
                    } else {
                        println!("Failed to parse integer: {}", number);
                    }
                }
            }
            '"' => {
                chars.next(); // consume the opening quote
                let mut string = String::new();
                
                while let Some(&ch) = chars.peek() {
                    if ch == '"' {
                        chars.next(); // consume the closing quote
                        break;
                    } else if ch == '\\' {
                        // Handle escape sequences
                        chars.next(); // consume the backslash
                        
                        if let Some(&next_ch) = chars.peek() {
                            match next_ch {
                                'n' => string.push('\n'),
                                't' => string.push('\t'),
                                'r' => string.push('\r'),
                                '\\' => string.push('\\'),
                                '"' => string.push('"'),
                                _ => string.push(next_ch),
                            }
                            chars.next(); // consume the escaped character
                        }
                    } else {
                        string.push(ch);
                        chars.next();
                    }
                }
                
                tokens.push(Token::String(string));
            },
            '\'' => {
                chars.next(); // consume the opening single quote
                let mut string = String::new();
                
                while let Some(&ch) = chars.peek() {
                    if ch == '\'' {
                        chars.next(); // consume the closing quote
                        break;
                    } else if ch == '\\' {
                        // Handle escape sequences
                        chars.next(); // consume the backslash
                        
                        if let Some(&next_ch) = chars.peek() {
                            match next_ch {
                                'n' => string.push('\n'),
                                't' => string.push('\t'),
                                'r' => string.push('\r'),
                                '\\' => string.push('\\'),
                                '\'' => string.push('\''),
                                _ => string.push(next_ch),
                            }
                            chars.next(); // consume the escaped character
                        }
                    } else {
                        string.push(ch);
                        chars.next();
                    }
                }
                
                tokens.push(Token::SingleQuoteString(string));
            },
            '`' => {
                chars.next(); // consume the opening backtick
                let mut string = String::new();
                
                while let Some(&ch) = chars.peek() {
                    if ch == '`' {
                        chars.next(); // consume the closing backtick
                        break;
                    } else if ch == '\\' {
                        // Handle escape sequences
                        chars.next(); // consume the backslash
                        
                        if let Some(&next_ch) = chars.peek() {
                            match next_ch {
                                'n' => string.push('\n'),
                                't' => string.push('\t'),
                                'r' => string.push('\r'),
                                '\\' => string.push('\\'),
                                '`' => string.push('`'),
                                _ => string.push(next_ch),
                            }
                            chars.next(); // consume the escaped character
                        }
                    } else if ch == '$' {
                        // Handle template interpolation ${...}
                        chars.next(); // consume the $
                        string.push('$');
                        
                        if let Some(&next_ch) = chars.peek() {
                            if next_ch == '{' {
                                // For now, just treat ${} as part of the string
                                // In the future, we'll need to handle interpolation
                                string.push('{');
                                chars.next(); // consume the {
                            }
                        }
                    } else {
                        string.push(ch);
                        chars.next();
                    }
                }
                
                tokens.push(Token::TemplateString(string));
            }
            'a'..='z' | 'A'..='Z' | '_' => {
                let mut ident = String::new();
                
                while let Some(&ch) = chars.peek() {
                    if ch.is_alphanumeric() || ch == '_' {
                        ident.push(ch);
                        chars.next();
                    } else {
                        break;
                    }
                }
                match ident.as_str() {
                    "let" => tokens.push(Token::Let),
                    "fn" => tokens.push(Token::Fn),
                    "return" => tokens.push(Token::Return),
                    "const" => tokens.push(Token::Const),
                    "import" => tokens.push(Token::Import),
                    "try" => tokens.push(Token::Try),
                    "catch" => tokens.push(Token::Catch),
                    "finally" => tokens.push(Token::Finally),
                    "throw" => tokens.push(Token::Throw),
                    "new" => tokens.push(Token::New),
                    "break" => tokens.push(Token::Break),
                    "continue" => tokens.push(Token::Continue),
                    "if" => tokens.push(Token::If),
                    "else" => tokens.push(Token::Else),
                    "while" => tokens.push(Token::While),
                    "for" => tokens.push(Token::For),
                    "do" => tokens.push(Token::Do),
                    "switch" => tokens.push(Token::Switch),
                    "case" => tokens.push(Token::Case),
                    "default" => tokens.push(Token::Default),
                    "in" => tokens.push(Token::In),
                    "of" => tokens.push(Token::Of),
                    "true" => tokens.push(Token::Bool(true)),
                    "false" => tokens.push(Token::Bool(false)),
                    "null" => tokens.push(Token::Null),
                    _ => tokens.push(Token::Identifier(ident)),
                }
            }
            // Single quotes and backticks are handled earlier in the code
            '<' => {
                chars.next(); // consume the '<'
                
                if let Some(&next_ch) = chars.peek() {
                    match next_ch {
                        '=' => {
                            tokens.push(Token::LessThanEqual);
                            chars.next(); // consume the '='
                        },
                        '<' => {
                            chars.next(); // consume the second '<'
                            
                            // Check for <<=
                            if let Some(&third_ch) = chars.peek() {
                                if third_ch == '=' {
                                    tokens.push(Token::BitwiseLeftShiftEqual);
                                    chars.next(); // consume the '='
                                } else {
                                    tokens.push(Token::BitwiseLeftShift);
                                }
                            } else {
                                tokens.push(Token::BitwiseLeftShift);
                            }
                        },
                        _ => tokens.push(Token::LessThan),
                    }
                } else {
                    tokens.push(Token::LessThan);
                }
            }
            '>' => {
                chars.next(); // consume the '>'
                
                if let Some(&next_ch) = chars.peek() {
                    match next_ch {
                        '=' => {
                            tokens.push(Token::GreaterThanEqual);
                            chars.next(); // consume the '='
                        },
                        '>' => {
                            chars.next(); // consume the second '>'
                            
                            // Check for >>= or >>>
                            if let Some(&third_ch) = chars.peek() {
                                match third_ch {
                                    '=' => {
                                        tokens.push(Token::BitwiseRightShiftEqual);
                                        chars.next(); // consume the '='
                                    },
                                    '>' => {
                                        tokens.push(Token::BitwiseUnsignedRightShift);
                                        chars.next(); // consume the third '>'
                                    },
                                    _ => tokens.push(Token::BitwiseRightShift),
                                }
                            } else {
                                tokens.push(Token::BitwiseRightShift);
                            }
                        },
                        _ => tokens.push(Token::GreaterThan),
                    }
                } else {
                    tokens.push(Token::GreaterThan);
                }
            }
            '!' => {
                chars.next(); // consume the '!'
                
                if let Some(&next_ch) = chars.peek() {
                    match next_ch {
                        '=' => {
                            chars.next(); // consume the '='
                            
                            // Check for !==
                            if let Some(&third_ch) = chars.peek() {
                                if third_ch == '=' {
                                    tokens.push(Token::StrictNotEqual);
                                    chars.next(); // consume the second '='
                                } else {
                                    tokens.push(Token::NotEqual);
                                }
                            } else {
                                tokens.push(Token::NotEqual);
                            }
                        },
                        _ => tokens.push(Token::LogicalNot),
                    }
                } else {
                    tokens.push(Token::LogicalNot);
                }
            }
            '&' => {
                chars.next(); // consume the '&'
                
                if let Some(&next_ch) = chars.peek() {
                    match next_ch {
                        '&' => {
                            tokens.push(Token::LogicalAnd);
                            chars.next(); // consume the second '&'
                        },
                        '=' => {
                            tokens.push(Token::BitwiseAndEqual);
                            chars.next(); // consume the '='
                        },
                        _ => tokens.push(Token::BitwiseAnd),
                    }
                } else {
                    tokens.push(Token::BitwiseAnd);
                }
            }
            '|' => {
                chars.next(); // consume the '|'
                
                if let Some(&next_ch) = chars.peek() {
                    match next_ch {
                        '|' => {
                            tokens.push(Token::LogicalOr);
                            chars.next(); // consume the second '|'
                        },
                        '=' => {
                            tokens.push(Token::BitwiseOrEqual);
                            chars.next(); // consume the '='
                        },
                        _ => tokens.push(Token::BitwiseOr),
                    }
                } else {
                    tokens.push(Token::BitwiseOr);
                }
            }
            '^' => {
                chars.next(); // consume the '^'
                
                if let Some(&next_ch) = chars.peek() {
                    if next_ch == '=' {
                        tokens.push(Token::BitwiseXorEqual);
                        chars.next(); // consume the '='
                    } else {
                        tokens.push(Token::BitwiseXor);
                    }
                } else {
                    tokens.push(Token::BitwiseXor);
                }
            }
            '~' => {
                chars.next(); // consume the '~'
                tokens.push(Token::BitwiseNot);
            }
            '?' => {
                chars.next(); // consume the '?'
                
                // Check for ?. (optional chaining) or ?? (nullish coalescing)
                if let Some(&next_ch) = chars.peek() {
                    match next_ch {
                        '.' => {
                            tokens.push(Token::OptionalChaining);
                            chars.next(); // consume the '.'
                        },
                        '?' => {
                            tokens.push(Token::NullishCoalescing);
                            chars.next(); // consume the second '?'
                        },
                        _ => tokens.push(Token::QuestionMark),
                    }
                } else {
                    tokens.push(Token::QuestionMark);
                }
            }
            // Dollar sign is handled elsewhere
            '%' => {
                chars.next(); // consume the '%'
                
                // Check for %=
                if let Some(&next_ch) = chars.peek() {
                    if next_ch == '=' {
                        tokens.push(Token::ModuloEqual);
                        chars.next(); // consume the '='
                    } else {
                        tokens.push(Token::Percent);
                    }
                } else {
                    tokens.push(Token::Percent);
                }
            }
            '@' => {
                chars.next(); // consume the '@'
                tokens.push(Token::At);
            },
            // Caret is handled elsewhere
            
            // Tilde is handled elsewhere
            
            // Percent is handled elsewhere
            
            // Dollar sign is handled elsewhere
            _ => {
                println!("Unexpected character: {}", ch);
                chars.next();
            }
        }
    }

    tokens
}
