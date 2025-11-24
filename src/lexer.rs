use crate::error::CalcError;
use crate::token::{Token, TokenStream};

pub fn tokenize(input: &str) -> Result<TokenStream, CalcError> {
    let mut tokens = Vec::new();
    let chars: Vec<char> = input.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        let c = chars[i];

        if c.is_whitespace() {
            i += 1;
            continue;
        }

        if c.is_ascii_digit() {
            let mut num = (c as u8 - b'0') as i64;
            i += 1;
            while i < chars.len() && chars[i].is_ascii_digit() {
                num = num * 10 + ((chars[i] as u8 - b'0') as i64);
                i += 1;
            }
            tokens.push(Token::Number(num));
            continue;
        }

        if c.is_ascii_alphabetic() || c == '_' {
            let mut ident = String::new();
            ident.push(c);
            i += 1;
            while i < chars.len() && (chars[i].is_ascii_alphanumeric() || chars[i] == '_') {
                ident.push(chars[i]);
                i += 1;
            }
            tokens.push(Token::Ident(ident));
            continue;
        }

        match c {
            '+' => tokens.push(Token::Plus),
            '-' => tokens.push(Token::Minus),
            '*' => tokens.push(Token::Mul),
            '/' => tokens.push(Token::Div),
            '=' => tokens.push(Token::Assign),
            '(' => tokens.push(Token::LParen),
            ')' => tokens.push(Token::RParen),
            ',' => tokens.push(Token::Comma),
            _ => return Err(CalcError::InvalidChar(c)),
        }

        i += 1;
    }

    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token::Token;

    #[test]
    fn tokenize_simple_expression() {
        let tokens = tokenize("1 + 2 * 3").unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::Number(1),
                Token::Plus,
                Token::Number(2),
                Token::Mul,
                Token::Number(3),
            ]
        );
    }

    #[test]
    fn tokenize_ident_and_assignment() {
        let tokens = tokenize("x = 10").unwrap();
        assert_eq!(
            tokens,
            vec![Token::Ident("x".into()), Token::Assign, Token::Number(10),]
        );
    }

    #[test]
    fn tokenize_function_call() {
        let tokens = tokenize("max(3, 5)").unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::Ident("max".into()),
                Token::LParen,
                Token::Number(3),
                Token::Comma,
                Token::Number(5),
                Token::RParen,
            ]
        );
    }
}
