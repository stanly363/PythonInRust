// main.rs
mod token;
mod ast;
mod lexer;
mod parser;
mod evaluator;

use crate::lexer::lex;
use crate::parser::Parser;
use crate::evaluator::evaluate;
use std::collections::HashMap;
use std::io::{self, Write};

fn main() {
    const CODE: &str = "
x = 1000000
while x > 2:
    print(x)
    x = x / 2.6
    ";
    let tokens = lex(CODE);
    let mut parser = Parser::new(tokens);
    let ast = parser.parse();
    let mut variables = HashMap::new();

    // Use a BufWriter to buffer output and reduce per-iteration I/O overhead.
    let stdout = io::stdout();
    let mut writer = io::BufWriter::new(stdout.lock());

    for expr in &ast {
        evaluate(expr, &mut variables, &mut writer);
    }
    writer.flush().unwrap();
}
