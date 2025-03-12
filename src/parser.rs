use crate::ast::Expr;
use crate::token::Token;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Vec<Expr> {
        let mut expressions = Vec::new();
        while self.current < self.tokens.len() {
            if let Some(Token::Newline) = self.tokens.get(self.current) {
                self.current += 1;
                continue;
            }
            if let Some(expr) = self.parse_statement() {
                expressions.push(expr);
            } else {
                self.current += 1;
            }
        }
        expressions
    }

    fn parse_statement(&mut self) -> Option<Expr> {
        match self.tokens.get(self.current) {
            Some(Token::Identifier(_)) => self.parse_assignment(),
            Some(Token::Def) => self.parse_function_def(),
            Some(Token::If) => self.parse_if_condition(),
            Some(Token::Print) => self.parse_print(),
            Some(Token::Return) => self.parse_return(),
            Some(Token::For) => self.parse_for_loop(),
            Some(Token::While) => self.parse_while_loop(),
            _ => None,
        }
    }

    fn parse_assignment(&mut self) -> Option<Expr> {
        if let Some(Token::Identifier(name)) = self.tokens.get(self.current).cloned() {
            self.current += 1; // consume identifier
            if let Some(Token::Equals) = self.tokens.get(self.current) {
                self.current += 1; // consume '='
                let value = self.parse_expression()?;
                if let Some(Token::Newline) = self.tokens.get(self.current) {
                    self.current += 1;
                }
                return Some(Expr::Assignment(name, Box::new(value)));
            }
        }
        None
    }

    fn parse_function_def(&mut self) -> Option<Expr> {
        self.current += 1; // consume 'def'
        let name = if let Some(Token::Identifier(name)) = self.tokens.get(self.current).cloned() {
            self.current += 1;
            name
        } else {
            return None;
        };

        if let Some(Token::OpenParen) = self.tokens.get(self.current) {
            self.current += 1;
        } else {
            return None;
        }

        let mut params = Vec::new();
        while let Some(token) = self.tokens.get(self.current) {
            match token {
                Token::Identifier(param) => {
                    params.push(param.clone());
                    self.current += 1;
                }
                Token::Comma => { self.current += 1; },
                Token::CloseParen => { self.current += 1; break; },
                _ => { self.current += 1; }
            }
        }

        if let Some(Token::Colon) = self.tokens.get(self.current) {
            self.current += 1;
        } else {
            return None;
        }

        let body = self.parse_block();
        Some(Expr::FunctionDef { name, params, body })
    }

    fn parse_if_condition(&mut self) -> Option<Expr> {
        self.current += 1; // consume 'if'
        let condition = self.parse_expression()?;
        if let Some(Token::Colon) = self.tokens.get(self.current) {
            self.current += 1;
        } else {
            return None;
        }
        let body = self.parse_block();
        Some(Expr::IfCondition { condition: Box::new(condition), body })
    }

    fn parse_print(&mut self) -> Option<Expr> {
        self.current += 1; // consume 'print'
        let mut args = Vec::new();
    
        if let Some(Token::OpenParen) = self.tokens.get(self.current) {
            self.current += 1; // consume '('
    
            // Parse the first expression.
            if let Some(expr) = self.parse_expression() {
                args.push(expr);
            } else {
                return None;
            }          
            while let Some(tok) = self.tokens.get(self.current) {
                if let Token::Comma = tok {
                    self.current += 1; // consume comma
                    if let Some(expr) = self.parse_expression() {
                        args.push(expr);
                    } else {
                        return None;
                    }
                } else {
                    break;
                }
            }
            if let Some(Token::CloseParen) = self.tokens.get(self.current) {
                self.current += 1; // consume ')'
            } else {
                return None;
            }
        } else {
            if let Some(expr) = self.parse_expression() {
                args.push(expr);
            } else {
                return None;
            }
        }
    
        if let Some(Token::Newline) = self.tokens.get(self.current) {
            self.current += 1;
        }
    
        Some(Expr::Print(args))
    }

    fn parse_return(&mut self) -> Option<Expr> {
        self.current += 1; // consume 'return'
        let expr = self.parse_expression()?;
        if let Some(Token::Newline) = self.tokens.get(self.current) {
            self.current += 1;
        }
        Some(Expr::Return(Box::new(expr)))
    }

    fn parse_for_loop(&mut self) -> Option<Expr> {
        self.current += 1; // consume 'for'
        let iterator = if let Some(Token::Identifier(name)) = self.tokens.get(self.current).cloned() {
            self.current += 1;
            name
        } else {
            return None;
        };

        if let Some(Token::In) = self.tokens.get(self.current) {
            self.current += 1;
        } else {
            return None;
        }

        if let Some(Token::Range) = self.tokens.get(self.current) {
            self.current += 1;
        } else {
            return None;
        }

        if let Some(Token::OpenParen) = self.tokens.get(self.current) {
            self.current += 1;
        } else {
            return None;
        }

        let start = self.parse_expression()?;
        if let Some(Token::Comma) = self.tokens.get(self.current) {
            self.current += 1;
        } else {
            return None;
        }
        let end = self.parse_expression()?;
        if let Some(Token::CloseParen) = self.tokens.get(self.current) {
            self.current += 1;
        } else {
            return None;
        }
        if let Some(Token::Colon) = self.tokens.get(self.current) {
            self.current += 1;
        } else {
            return None;
        }
        let body = self.parse_block();
        Some(Expr::ForLoop {
            iterator,
            range_start: Box::new(start),
            range_end: Box::new(end),
            body,
        })
    }

    fn parse_while_loop(&mut self) -> Option<Expr> {
        self.current += 1; // consume 'while'
        let condition = self.parse_expression()?;
        if let Some(Token::Colon) = self.tokens.get(self.current) {
            self.current += 1;
        } else {
            return None;
        }
        let body = self.parse_block();
        Some(Expr::WhileLoop { condition: Box::new(condition), body })
    }

    fn parse_expression(&mut self) -> Option<Expr> {
        self.parse_additive_expression()
    }

    fn parse_additive_expression(&mut self) -> Option<Expr> {
        let mut left = self.parse_multiplicative_expression()?;
        while let Some(token) = self.tokens.get(self.current) {
            match token {
                Token::Plus => {
                    self.current += 1;
                    let right = self.parse_multiplicative_expression()?;
                    left = Expr::Arithmetic {
                        left: Box::new(left),
                        operator: "+".to_string(),
                        right: Box::new(right),
                    };
                }
                Token::Minus => {
                    self.current += 1;
                    let right = self.parse_multiplicative_expression()?;
                    left = Expr::Arithmetic {
                        left: Box::new(left),
                        operator: "-".to_string(),
                        right: Box::new(right),
                    };
                }
                _ => break,
            }
        }
        Some(left)
    }

    fn parse_multiplicative_expression(&mut self) -> Option<Expr> {
        let mut left = self.parse_power()?;
        while let Some(token) = self.tokens.get(self.current) {
            match token {
                Token::Star => {
                    self.current += 1;
                    let right = self.parse_power()?;
                    left = Expr::Arithmetic {
                        left: Box::new(left),
                        operator: "*".to_string(),
                        right: Box::new(right),
                    };
                }
                Token::Slash => {
                    self.current += 1;
                    let right = self.parse_power()?;
                    left = Expr::Arithmetic {
                        left: Box::new(left),
                        operator: "/".to_string(),
                        right: Box::new(right),
                    };
                }
                Token::DoubleSlash => {
                    self.current += 1;
                    let right = self.parse_power()?;
                    left = Expr::Arithmetic {
                        left: Box::new(left),
                        operator: "//".to_string(),
                        right: Box::new(right),
                    };
                }
                _ => break,
            }
        }
        Some(left)
    }

    fn parse_power(&mut self) -> Option<Expr> {
        let left = self.parse_unary()?;
        if let Some(Token::DoubleStar) = self.tokens.get(self.current) {
            self.current += 1;
            let right = self.parse_power()?;
            return Some(Expr::Arithmetic {
                left: Box::new(left),
                operator: "**".to_string(),
                right: Box::new(right),
            });
        }
        Some(left)
    }

    fn parse_unary(&mut self) -> Option<Expr> {
        if let Some(Token::Minus) = self.tokens.get(self.current) {
            self.current += 1;
            let expr = self.parse_unary()?;
            return Some(Expr::Arithmetic {
                left: Box::new(Expr::Number(0)),
                operator: "-".to_string(),
                right: Box::new(expr),
            });
        }
        self.parse_primary()
    }

    fn parse_primary(&mut self) -> Option<Expr> {
        if let Some(Token::OpenParen) = self.tokens.get(self.current) {
            self.current += 1;
            let expr = self.parse_expression();
            if let Some(Token::CloseParen) = self.tokens.get(self.current) {
                self.current += 1;
                return expr;
            } else {
                return None;
            }
        }
        match self.tokens.get(self.current) {
            Some(Token::Number(value)) => { self.current += 1; Some(Expr::Number(*value)) },
            Some(Token::Float(value)) => { self.current += 1; Some(Expr::Float(*value)) },
            Some(Token::StringLiteral(s)) => { self.current += 1; Some(Expr::String(s.clone())) },
            Some(Token::Identifier(name)) => { self.current += 1; Some(Expr::Variable(name.clone())) },
            _ => None,
        }
    }

    fn parse_block(&mut self) -> Vec<Expr> {
        let mut statements = Vec::new();
        while let Some(Token::Newline) = self.tokens.get(self.current) {
            self.current += 1;
        }
        if let Some(Token::Indent) = self.tokens.get(self.current) {
            self.current += 1;
        } else {
            return statements;
        }
        while self.current < self.tokens.len() {
            if let Some(Token::Dedent) = self.tokens.get(self.current) {
                self.current += 1;
                break;
            }
            if let Some(Token::Newline) = self.tokens.get(self.current) {
                self.current += 1;
                continue;
            }
            if let Some(statement) = self.parse_statement() {
                statements.push(statement);
            } else {
                self.current += 1;
            }
        }
        statements
    }
}
