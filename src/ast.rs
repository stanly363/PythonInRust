// ast.rs
#[derive(Debug)]
pub enum Expr {
    Number(i64),
    Float(f64),
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
    Print(Box<Expr>),
    Return(Box<Expr>),
}
