use crate::error::CalcError;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Number(i64),
    Ident(String),
    Plus,
    Minus,
    Mul,
    Div,
    Assign,
    LParen,
    RParen,
    Comma,
}

pub type TokenStream = Vec<Token>;
