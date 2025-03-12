// evaluator.rs
use crate::ast::Expr;
use std::collections::HashMap;
use std::io::Write;

pub fn evaluate<W: Write>(
    expr: &Expr,
    variables: &mut HashMap<String, f64>,
    writer: &mut W,
) -> Option<f64> {
    match expr {
        Expr::Number(val) => Some(*val as f64),
        Expr::Float(val) => Some(*val),
        Expr::Variable(name) => variables.get(name).copied(),
        Expr::Assignment(var, value) => {
            if let Some(result) = evaluate(value, variables, writer) {
                variables.insert(var.clone(), result);
                Some(result)
            } else {
                None
            }
        }
        Expr::Arithmetic { left, operator, right } => {
            let left_value = evaluate(left, variables, writer).unwrap_or(0.0);
            let right_value = evaluate(right, variables, writer).unwrap_or(0.0);
            let result = match operator.as_str() {
                "-" => left_value - right_value,
                "+" => left_value + right_value,
                "*" => left_value * right_value,
                "/" => left_value / right_value, // normal division returns decimals
                "//" => (left_value / right_value).floor(), // integer division: floor the result
                "**" => left_value.powf(right_value),
                ">"  => if left_value > right_value { 1.0 } else { 0.0 },
                "<"  => if left_value < right_value { 1.0 } else { 0.0 },
                _ => 0.0,
            };
            Some(result)
        }
        Expr::Print(value) => {
            if let Some(result) = evaluate(value, variables, writer) {
                writeln!(writer, "{}", result).unwrap();
            }
            None
        }
        Expr::ForLoop { iterator, range_start, range_end, body } => {
            if let Some(start) = evaluate(range_start, variables, writer) {
                if let Some(end) = evaluate(range_end, variables, writer) {
                    evaluate_for_loop(iterator, start, end, body, variables, writer);
                }
            }
            None
        }
        Expr::WhileLoop { condition, body } => {
            evaluate_while_loop(condition, body, variables, writer);
            None
        }
        Expr::FunctionDef { .. } => None,
        Expr::IfCondition { condition, body } => {
            if let Some(cond_val) = evaluate(condition, variables, writer) {
                if cond_val != 0.0 {
                    for expr in body {
                        evaluate(expr, variables, writer);
                    }
                }
            }
            None
        }
        Expr::Return(_) => None,
    }
}

fn evaluate_for_loop<W: Write>(
    iterator: &str,
    start: f64,
    end: f64,
    body: &[Expr],
    variables: &mut HashMap<String, f64>,
    writer: &mut W,
) {
    // For loops typically iterate over integers.
    // We cast the start and end values to i64.
    for i in (start as i64)..(end as i64) {
        let i_f = i as f64;
        variables.insert(iterator.to_string(), i_f);
        writeln!(writer, "{} = {}", iterator, i_f).unwrap();
        for expr in body {
            evaluate(expr, variables, writer);
        }
    }
}

fn evaluate_while_loop<W: Write>(
    condition: &Expr,
    body: &[Expr],
    variables: &mut HashMap<String, f64>,
    writer: &mut W,
) {
    while let Some(cond_val) = evaluate(condition, variables, writer) {
        if cond_val <= 0.0 {
            break;
        }
        for expr in body {
            evaluate(expr, variables, writer);
        }
    }
}
