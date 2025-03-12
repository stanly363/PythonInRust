// lexer.rs
use crate::token::Token;
use std::iter::Peekable;

pub fn lex(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut indent_stack: Vec<usize> = vec![0];
    
    // Process the input line by line to handle indentation.
    for line in input.lines() {
        // Count leading spaces.
        let indent = line.chars().take_while(|c| *c == ' ').count();
        let trimmed = line.trim_start();
        // Skip empty lines.
        if trimmed.is_empty() {
            continue;
        }
        // Handle indent/dedent tokens.
        let current_indent = *indent_stack.last().unwrap();
        if indent > current_indent {
            indent_stack.push(indent);
            tokens.push(Token::Indent);
        } else {
            while indent < *indent_stack.last().unwrap() {
                indent_stack.pop();
                tokens.push(Token::Dedent);
            }
        }
        // Lex the tokens for the current (trimmed) line.
        let mut chars = trimmed.chars().peekable();
        while let Some(&ch) = chars.peek() {
            match ch {
                ' ' | '\t' => { chars.next(); } // skip inner whitespace
                '#' => {
                    // Skip comments.
                    while let Some(c) = chars.next() {
                        if c == '\n' {
                            break;
                        }
                    }
                }
                '0'..='9' => {
                    let mut number = String::new();
                    while let Some(&digit) = chars.peek() {
                        if digit.is_digit(10) || digit == '.' {
                            number.push(digit);
                            chars.next();
                        } else {
                            break;
                        }
                    }
                    if number.contains('.') {
                        if let Ok(float_value) = number.parse::<f64>() {
                            tokens.push(Token::Float(float_value));
                        } else {
                            tokens.push(Token::Unknown('.'));
                        }
                    } else if let Ok(int_value) = number.parse::<i64>() {
                        tokens.push(Token::Number(int_value));
                    }
                }
                'a'..='z' | 'A'..='Z' => {
                    let mut ident = String::new();
                    while let Some(&c) = chars.peek() {
                        if c.is_alphanumeric() || c == '_' {
                            ident.push(c);
                            chars.next();
                        } else {
                            break;
                        }
                    }
                    match ident.as_str() {
                        "print" => tokens.push(Token::Print),
                        "def" => tokens.push(Token::Def),
                        "if" => tokens.push(Token::If),
                        "else" => tokens.push(Token::Else),
                        "return" => tokens.push(Token::Return),
                        "for" => tokens.push(Token::For),
                        "while" => tokens.push(Token::While),
                        "in" => tokens.push(Token::In),
                        "range" => tokens.push(Token::Range),
                        _ => tokens.push(Token::Identifier(ident)),
                    }
                }
                '>' => { tokens.push(Token::Greater); chars.next(); }
                '<' => { tokens.push(Token::Less); chars.next(); }
                '-' => { tokens.push(Token::Minus); chars.next(); }
                '+' => { tokens.push(Token::Plus); chars.next(); }
                '*' => {
                    chars.next();
                    if let Some(&'*') = chars.peek() {
                        chars.next();
                        tokens.push(Token::DoubleStar);
                    } else {
                        tokens.push(Token::Star);
                    }
                }
                '/' => {
                    chars.next();
                    if let Some(&'/') = chars.peek() {
                        chars.next();
                        tokens.push(Token::DoubleSlash);
                    } else {
                        tokens.push(Token::Slash);
                    }
                }
                '=' => { tokens.push(Token::Equals); chars.next(); }
                ',' => { tokens.push(Token::Comma); chars.next(); }
                '(' => { tokens.push(Token::OpenParen); chars.next(); }
                ')' => { tokens.push(Token::CloseParen); chars.next(); }
                ':' => { tokens.push(Token::Colon); chars.next(); }
                _ => {
                    tokens.push(Token::Unknown(ch));
                    chars.next();
                }
            }
        }
        // End of the line.
        tokens.push(Token::Newline);
    }
    // If any indented block is still open, close it.
    while indent_stack.len() > 1 {
        indent_stack.pop();
        tokens.push(Token::Dedent);
    }
    tokens
}
