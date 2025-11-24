use crate::token::Token;

#[derive(Debug, Clone)]
pub enum Expr {
    Number(i64),
    Var(String),
    UnaryNeg(Box<Expr>),
    Binary {
        op: BinaryOp,
        left: Box<Expr>,
        right: Box<Expr>,
    },
    Assign {
        name: String,
        expr: Box<Expr>,
    },
    Call {
        name: String,
        args: Vec<Expr>,
    },
}

#[derive(Debug, Clone, Copy)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
}
