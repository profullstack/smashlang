use crate::lexer::Token;
use std::collections::HashMap;

#[derive(Debug)]
pub enum AstNode {
    Number(i64),
    Identifier(String),
    LetDecl { name: String, value: Box<AstNode> },
    ConstDecl { name: String, value: Box<AstNode> },
    BinaryOp {
        left: Box<AstNode>,
        op: String,
        right: Box<AstNode>,
    },
    Function {
        name: String,
        params: Vec<String>,
        body: Vec<AstNode>,
    },
    FunctionCall {
        name: String,
        args: Vec<AstNode>,
    },
    ArrayLiteral(Vec<AstNode>),
    ObjectLiteral(HashMap<String, AstNode>),
    Return(Box<AstNode>),
    Import(String),
}

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, pos: 0 }
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    fn advance(&mut self) -> Option<&Token> {
        let token = self.tokens.get(self.pos);
        self.pos += 1;
        token
    }

    fn expect(&mut self, expected: &Token) -> bool {
        if let Some(tok) = self.peek() {
            if tok == expected {
                self.advance();
                return true;
            }
        }
        false
    }

    pub fn parse(&mut self) -> Vec<AstNode> {
        let mut nodes = Vec::new();
        while self.peek().is_some() {
            if let Some(const_decl) = self.parse_const() {
                nodes.push(const_decl);
            } else
            if let Some(import) = self.parse_import() {
                nodes.push(import);
            } else if let Some(decl) = self.parse_let() {
                nodes.push(decl);
            } else if let Some(func) = self.parse_function() {
                nodes.push(func);
            } else {
                break;
            }
        }
        nodes
    }

    fn parse_import(&mut self) -> Option<AstNode> {
        if let Some(Token::Identifier(ident)) = self.peek() {
            if ident == "import" {
                self.advance();
                if let Some(Token::Identifier(path)) = self.advance() {
                    self.expect(&Token::Semicolon);
                    return Some(AstNode::Import(path.clone()));
                }
            }
        }
        None
    }

    fn parse_const(&mut self) -> Option<AstNode> {
        if self.expect(&Token::Const) {
            if let Some(Token::Identifier(name)) = self.advance() {
                if self.expect(&Token::Equal) {
                    if let Some(expr) = self.parse_expr() {
                        self.expect(&Token::Semicolon);
                        return Some(AstNode::ConstDecl {
                            name: name.clone(),
                            value: Box::new(expr),
                        });
                    }
                }
            }
        }
        None
    }

    fn parse_let(&mut self) -> Option<AstNode> {
        if self.expect(&Token::Let) {
            if let Some(Token::Identifier(name)) = self.advance() {
                if self.expect(&Token::Equal) {
                    if let Some(expr) = self.parse_expr() {
                        self.expect(&Token::Semicolon);
                        return Some(AstNode::LetDecl {
                            name: name.clone(),
                            value: Box::new(expr),
                        });
                    }
                }
            }
        }
        None
    }

    fn parse_function(&mut self) -> Option<AstNode> {
        if self.expect(&Token::Fn) {
            if let Some(Token::Identifier(name)) = self.advance() {
                if self.expect(&Token::LParen) {
                    let mut params = Vec::new();
                    while let Some(Token::Identifier(param)) = self.peek() {
                        params.push(param.clone());
                        self.advance();
                        if !self.expect(&Token::Comma) {
                            break;
                        }
                    }
                    if self.expect(&Token::RParen) && self.expect(&Token::LBrace) {
                        let mut body = Vec::new();
                        while !self.expect(&Token::RBrace) {
                            if let Some(ret) = self.parse_return() {
                                body.push(ret);
                            } else if let Some(decl) = self.parse_let() {
                                body.push(decl);
                            } else {
                                break;
                            }
                        }
                        return Some(AstNode::Function {
                            name: name.clone(),
                            params,
                            body,
                        });
                    }
                }
            }
        }
        None
    }

    fn parse_return(&mut self) -> Option<AstNode> {
        if self.expect(&Token::Return) {
            if let Some(expr) = self.parse_expr() {
                self.expect(&Token::Semicolon);
                return Some(AstNode::Return(Box::new(expr)));
            }
        }
        None
    }

    fn parse_expr(&mut self) -> Option<AstNode> {
        let token = self.advance()?;

        let mut left = match token {
            Token::Number(n) => AstNode::Number(*n),
            Token::Identifier(name) => {
                if self.expect(&Token::LParen) {
                    let mut args = Vec::new();
                    while !self.expect(&Token::RParen) {
                        if let Some(arg) = self.parse_expr() {
                            args.push(arg);
                            self.expect(&Token::Comma);
                        }
                    }
                    AstNode::FunctionCall {
                        name: name.clone(),
                        args,
                    }
                } else {
                    AstNode::Identifier(name.clone())
                }
            }
            Token::LBracket => {
                let mut items = Vec::new();
                while !self.expect(&Token::RBracket) {
                    if let Some(item) = self.parse_expr() {
                        items.push(item);
                        self.expect(&Token::Comma);
                    }
                }
                AstNode::ArrayLiteral(items)
            }
            Token::LBrace => {
                let mut props = HashMap::new();
                while !self.expect(&Token::RBrace) {
                    if let Some(Token::Identifier(key)) = self.advance() {
                        if self.expect(&Token::Colon) {
                            if let Some(value) = self.parse_expr() {
                                props.insert(key.clone(), value);
                                self.expect(&Token::Comma);
                            }
                        }
                    }
                }
                AstNode::ObjectLiteral(props)
            }
            _ => return None,
        };

        while let Some(tok) = self.peek() {
            let op_name = match tok {
                Token::Plus => "__add__",
                Token::Minus => "__sub__",
                Token::Star => "__mul__",
                Token::Slash => "__div__",
                _ => break,
            };
            self.advance();
            let right = self.parse_expr()?;
            left = AstNode::FunctionCall {
                name: op_name.to_string(),
                args: vec![left, right],
            };
        }

        Some(left)
    }
}
