mod ast;
mod error;
mod eval;
mod lexer;
mod parser;
mod token;

use crate::eval::eval_expr;
use crate::lexer::tokenize;
use crate::parser::parse_expr;

fn main() {
    let input = "1 + 2 * (3 - 4)";
    println!("input: {}", input);

    let tokens = tokenize(input).expect("tokenize failed");
    println!("tokens: {:?}", tokens);

    let ast = parse_expr(&tokens).expect("parse failed");
    println!("ast: {:?}", ast);

    let result = eval_expr(&ast);
    println!("result: {:?}", result);
}
