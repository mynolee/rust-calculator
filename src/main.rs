mod ast;
mod error;
mod eval;
mod lexer;
mod parser;
mod token;

use crate::eval::{Env, eval_expr};
use crate::lexer::tokenize;
use crate::parser::parse_line;
use std::io::{self, Write};

fn main() {
    println!("Rust Expression Engine (변수 지원)");
    println!("한 줄에 하나의 수식 또는 대입식을 입력하세요.");
    println!("예: x = 1 + 2 * 3  /  x + 10");
    println!("빈 줄을 입력하면 종료됩니다.\n");

    let mut env = Env::new();
    let stdin = io::stdin();

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        let bytes_read = stdin.read_line(&mut input).unwrap();

        if bytes_read == 0 {
            println!("\nEOF detected. Bye!");
            break;
        }

        let trimmed = input.trim();
        if trimmed.is_empty() {
            println!("Bye!");
            break;
        }

        let tokens = match tokenize(trimmed) {
            Ok(t) => t,
            Err(e) => {
                println!("Tokenize error: {:?}", e);
                continue;
            }
        };

        let expr = match parse_line(&tokens) {
            Ok(e) => e,
            Err(e) => {
                println!("Parse error: {:?}", e);
                continue;
            }
        };

        match eval_expr(&expr, &mut env) {
            Ok(v) => println!("= {}", v),
            Err(e) => println!("Eval error: {:?}", e),
        }
    }
}
