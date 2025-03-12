#[derive(Debug)]
pub enum Expr {
    Number(i64),
    Float(f64),
    String(String), // new variant for string literals
    Variable(String),
    Assignment(String, Box<Expr>),
    Arithmetic {
        left: Box<Expr>,
        operator: String,
        right: Box<Expr>,
    },
    FunctionDef {
        name: String,
        params: Vec<String>,
        body: Vec<Expr>,
    },
    IfCondition {
        condition: Box<Expr>,
        body: Vec<Expr>,
    },
    ForLoop {
        iterator: String,
        range_start: Box<Expr>,
        range_end: Box<Expr>,
        body: Vec<Expr>,
    },
    WhileLoop {
        condition: Box<Expr>,
        body: Vec<Expr>,
    },
    Print(Vec<Expr>), // now holds a list of expressions
    Return(Box<Expr>),
}

// A common Value type used during evaluation.
#[derive(Debug, Clone)]
pub enum Value {
    Number(f64),
    Str(String),
}
