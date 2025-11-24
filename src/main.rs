mod ast;
mod error;
mod eval;
mod lexer;
mod parser;
mod token;

use crate::lexer::tokenize;

fn main() {
    let input = "x = 12 + foo(3, 4)";
    println!("input: {}", input);

    match tokenize(input) {
        Ok(tokens) => {
            println!("tokens: {:?}", tokens);
        }
        Err(e) => {
            println!("Tokenize error: {:?}", e);
        }
    }
}
