use crate::ast::{Expr, Value};
use std::collections::HashMap;
use std::io::Write;

pub fn evaluate<W: Write>(
    expr: &Expr,
    variables: &mut HashMap<String, Value>,
    writer: &mut W,
) -> Option<Value> {
    match expr {
        Expr::Number(val) => Some(Value::Number(*val as f64)),
        Expr::Float(val) => Some(Value::Number(*val)),
        Expr::String(s) => Some(Value::Str(s.clone())),
        Expr::Variable(name) => variables.get(name).cloned(),
        Expr::Assignment(var, value) => {
            if let Some(result) = evaluate(value, variables, writer) {
                variables.insert(var.clone(), result.clone());
                Some(result)
            } else {
                None
            }
        }
        Expr::Arithmetic { left, operator, right } => {
            let left_value = match evaluate(left, variables, writer)? {
                Value::Number(n) => n,
                _ => return None,
            };
            let right_value = match evaluate(right, variables, writer)? {
                Value::Number(n) => n,
                _ => return None,
            };
            let result = match operator.as_str() {
                "-" => left_value - right_value,
                "+" => left_value + right_value,
                "*" => left_value * right_value,
                "/" => left_value / right_value, // returns decimals
                "//" => (left_value / right_value).floor(), // floor division
                "**" => left_value.powf(right_value),
                ">"  => if left_value > right_value { 1.0 } else { 0.0 },
                "<"  => if left_value < right_value { 1.0 } else { 0.0 },
                _ => 0.0,
            };
            Some(Value::Number(result))
        }
        Expr::Print(args) => {
            let mut parts = Vec::new();
            for arg in args {
                if let Some(val) = evaluate(arg, variables, writer) {
                    let s = match val {
                        Value::Number(n) => n.to_string(),
                        Value::Str(s) => s,
                    };
                    parts.push(s);
                }
            }
            let output = parts.join(" ");
            writeln!(writer, "{}", output).unwrap();
            None
        }
        Expr::ForLoop { iterator, range_start, range_end, body } => {
            if let Some(Value::Number(start)) = evaluate(range_start, variables, writer) {
                if let Some(Value::Number(end)) = evaluate(range_end, variables, writer) {
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
            if let Some(Value::Number(cond_val)) = evaluate(condition, variables, writer) {
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
    variables: &mut HashMap<String, Value>,
    writer: &mut W,
) {
    for i in (start as i64)..(end as i64) {
        let i_f = i as f64;
        // Update the loop variable without printing it:
        variables.insert(iterator.to_string(), Value::Number(i_f));
        // Evaluate the body of the loop
        for expr in body {
            evaluate(expr, variables, writer);
        }
    }
}

fn evaluate_while_loop<W: Write>(
    condition: &Expr,
    body: &[Expr],
    variables: &mut HashMap<String, Value>,
    writer: &mut W,
) {
    while let Some(Value::Number(cond_val)) = evaluate(condition, variables, writer) {
        if cond_val <= 0.0 {
            break;
        }
        for expr in body {
            evaluate(expr, variables, writer);
        }
    }
}
