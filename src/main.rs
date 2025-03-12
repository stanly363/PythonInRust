use std::time::Instant;
use std::collections::HashMap;
use std::io::{self, BufWriter, Write};

mod token;
mod ast;
mod lexer;
mod parser;
mod evaluator;
use crate::lexer::lex;
use crate::parser::Parser;
use crate::evaluator::evaluate;
use crate::ast::Value;

fn main() {
    const CODE: &str = "
x = 1000000
for i in range(0, x):
    print(i)
";
    // Start timing before lexing/parsing/evaluation.
    let start = Instant::now();

    let tokens = lex(CODE);
    let mut parser = Parser::new(tokens);
    let ast = parser.parse();
    let mut variables: HashMap<String, Value> = HashMap::new();

    let stdout = io::stdout();
    let mut writer = BufWriter::new(stdout.lock());

    for expr in &ast {
        evaluate(expr, &mut variables, &mut writer);
    }
    writer.flush().unwrap();

    // Stop timing after execution.
    let duration = start.elapsed();
    println!("Time elapsed: {:?}", duration);
}
