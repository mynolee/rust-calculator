mod ast;
mod error;
mod eval;
mod lexer;
mod parser;
mod token;

use crate::eval::{Env, eval_expr};
use crate::lexer::tokenize;
use crate::parser::parse_line;

fn main() {
    let mut env = Env::new();

    let line1 = "x = 1 + 2 * 3";
    println!("input: {}", line1);
    let tokens1 = tokenize(line1).expect("tokenize failed");
    println!("tokens1: {:?}", tokens1);
    let expr1 = parse_line(&tokens1).expect("parse_line failed");
    println!("ast1: {:?}", expr1);
    let result1 = eval_expr(&expr1, &mut env);
    println!("result1: {:?}", result1);
    println!("env after line1: {:?}\n", env);

    let line2 = "x + 10";
    println!("input: {}", line2);
    let tokens2 = tokenize(line2).expect("tokenize failed");
    println!("tokens2: {:?}", tokens2);
    let expr2 = parse_line(&tokens2).expect("parse_line failed");
    println!("ast2: {:?}", expr2);
    let result2 = eval_expr(&expr2, &mut env);
    println!("result2: {:?}", result2);
    println!("env after line2: {:?}\n", env);
}
