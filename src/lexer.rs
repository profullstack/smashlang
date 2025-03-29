#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Keywords
    Const,
    Let,
    Fn,
    Return,
    Import,
    
    // Literals
    Identifier(String),
    Number(i64),
    Float(f64),
    String(String),
    Regex(String),
    Bool(bool),
    Null,
    
    // Basic operators
    Plus,      // +
    Minus,     // -
    Star,      // *
    Slash,     // /
    Equal,     // =
    
    // Increment/Decrement operators
    Increment, // ++
    Decrement, // --
    
    // Compound assignment operators
    PlusEqual,  // +=
    MinusEqual, // -=
    StarEqual,  // *=
    SlashEqual, // /=
    
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
                
                if let Some(&next_ch) = chars.peek() {
                    match next_ch {
                        '/' => {
                            // Skip single-line comment
                            chars.next(); // consume the second '/'
                            while let Some(&c) = chars.peek() {
                                if c == '\n' {
                                    break;
                                }
                                chars.next();
                            }
                        },
                        '=' => {
                            // Handle /= operator
                            tokens.push(Token::SlashEqual);
                            chars.next(); // consume the '='
                        },
                        _ => {
                            // Regex literal
                            let mut regex = String::new();
                            while let Some(&c) = chars.peek() {
                                if c == '/' {
                                    chars.next(); // Consume closing '/'
                                    break;
                                }
                                regex.push(c);
                                chars.next();
                            }
                            tokens.push(Token::Regex(regex));
                        }
                    }
                } else {
                    tokens.push(Token::Slash);
                }
            }
            '=' => {
                tokens.push(Token::Equal);
                chars.next();
            }
            ':' => {
                tokens.push(Token::Colon);
                chars.next();
            }
            '(' => {
                tokens.push(Token::LParen);
                chars.next();
            }
            ')' => {
                tokens.push(Token::RParen);
                chars.next();
            }
            '{' => {
                tokens.push(Token::LBrace);
                chars.next();
            }
            '}' => {
                tokens.push(Token::RBrace);
                chars.next();
            }
            '[' => {
                tokens.push(Token::LBracket);
                chars.next();
            }
            ']' => {
                tokens.push(Token::RBracket);
                chars.next();
            }
            ',' => {
                tokens.push(Token::Comma);
                chars.next();
            }
            ';' => {
                tokens.push(Token::Semicolon);
                chars.next();
            }
            '.' => {
                chars.next(); // consume the first '.'
                
                // Check for ellipsis (...)
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
            '"' => {
                chars.next();
                let mut string = String::new();
                while let Some(&c) = chars.peek() {
                    if c == '"' {
                        chars.next();
                        break;
                    }
                    string.push(c);
                    chars.next();
                }
                tokens.push(Token::String(string));
            }
            '0'..='9' => {
                let mut num = String::new();
                let mut is_float = false;
                
                // Parse the integer part
                while let Some(&digit) = chars.peek() {
                    if digit.is_ascii_digit() {
                        num.push(digit);
                        chars.next();
                    } else {
                        break;
                    }
                }
                
                // Check for decimal point
                if let Some(&ch) = chars.peek() {
                    if ch == '.' {
                        // Look ahead to see if this is a spread operator
                        let mut temp_chars = chars.clone();
                        temp_chars.next(); // Skip the first dot
                        
                        if let Some(&next_ch) = temp_chars.peek() {
                            if next_ch == '.' {
                                // This is likely the start of a spread operator, don't consume the dot
                                // Just finish the number as an integer
                            } else if next_ch.is_ascii_digit() {
                                // This is a decimal point in a float
                                is_float = true;
                                num.push('.');
                                chars.next(); // Consume the dot
                                
                                // Parse the fractional part
                                while let Some(&digit) = chars.peek() {
                                    if digit.is_ascii_digit() {
                                        num.push(digit);
                                        chars.next();
                                    } else {
                                        break;
                                    }
                                }
                            }
                        } else {
                            // Just a single dot at the end, treat as a decimal point
                            is_float = true;
                            num.push('.');
                            chars.next();
                        }
                    }
                }
                
                if is_float {
                    tokens.push(Token::Float(num.parse().unwrap()));
                } else {
                    tokens.push(Token::Number(num.parse().unwrap()));
                }
            }
            'a'..='z' | 'A'..='Z' | '_' => {
                let mut ident = String::new();
                while let Some(&c) = chars.peek() {
                    if c.is_ascii_alphanumeric() || c == '_' {
                        ident.push(c);
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
                    "true" => tokens.push(Token::Bool(true)),
                    "false" => tokens.push(Token::Bool(false)),
                    "null" => tokens.push(Token::Null),
                    _ => tokens.push(Token::Identifier(ident)),
                }
            }
            _ => {
                println!("Unexpected character: {}", ch);
                chars.next();
            }
        }
    }

    tokens
}
