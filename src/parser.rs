use crate::ast::{BinaryOp, Expr};
use crate::error::CalcError;
use crate::token::Token;

pub fn parse_line(tokens: &[Token]) -> Result<Expr, CalcError> {
    if tokens.len() >= 3 {
        if let Token::Ident(name) = &tokens[0] {
            if let Token::Assign = tokens[1] {
                let expr = parse_expr(&tokens[2..])?;
                return Ok(Expr::Assign {
                    name: name.clone(),
                    expr: Box::new(expr),
                });
            }
        }
    }

    parse_expr(tokens)
}

pub fn parse_expr(tokens: &[Token]) -> Result<Expr, CalcError> {
    let (expr, pos) = parse_add_sub(tokens, 0)?;
    if pos != tokens.len() {
        return Err(CalcError::ParseError(format!(
            "extra tokens at position {}",
            pos
        )));
    }
    Ok(expr)
}

fn parse_add_sub(tokens: &[Token], mut pos: usize) -> Result<(Expr, usize), CalcError> {
    let (mut node, mut pos2) = parse_mul_div(tokens, pos)?;
    pos = pos2;

    while pos < tokens.len() {
        match tokens[pos] {
            Token::Plus => {
                let (rhs, next) = parse_mul_div(tokens, pos + 1)?;
                node = Expr::Binary {
                    op: BinaryOp::Add,
                    left: Box::new(node),
                    right: Box::new(rhs),
                };
                pos = next;
            }
            Token::Minus => {
                let (rhs, next) = parse_mul_div(tokens, pos + 1)?;
                node = Expr::Binary {
                    op: BinaryOp::Sub,
                    left: Box::new(node),
                    right: Box::new(rhs),
                };
                pos = next;
            }
            _ => break,
        }
    }

    Ok((node, pos))
}

fn parse_mul_div(tokens: &[Token], mut pos: usize) -> Result<(Expr, usize), CalcError> {
    let (mut node, mut pos2) = parse_primary(tokens, pos)?;
    pos = pos2;

    while pos < tokens.len() {
        match tokens[pos] {
            Token::Mul => {
                let (rhs, next) = parse_primary(tokens, pos + 1)?;
                node = Expr::Binary {
                    op: BinaryOp::Mul,
                    left: Box::new(node),
                    right: Box::new(rhs),
                };
                pos = next;
            }
            Token::Div => {
                let (rhs, next) = parse_primary(tokens, pos + 1)?;
                node = Expr::Binary {
                    op: BinaryOp::Div,
                    left: Box::new(node),
                    right: Box::new(rhs),
                };
                pos = next;
            }
            _ => break,
        }
    }

    Ok((node, pos))
}

fn parse_primary(tokens: &[Token], pos: usize) -> Result<(Expr, usize), CalcError> {
    if pos >= tokens.len() {
        return Err(CalcError::ParseError("unexpected end of input".into()));
    }

    match &tokens[pos] {
        Token::Minus => {
            let (inner, next_pos) = parse_primary(tokens, pos + 1)?;
            Ok((Expr::UnaryNeg(Box::new(inner)), next_pos))
        }

        Token::Number(n) => Ok((Expr::Number(*n), pos + 1)),

        Token::Ident(name) => {
            if matches!(tokens.get(pos + 1), Some(Token::LParen)) {
                parse_call(tokens, pos)
            } else {
                Ok((Expr::Var(name.clone()), pos + 1))
            }
        }

        Token::LParen => {
            let (expr, pos2) = parse_add_sub(tokens, pos + 1)?;
            match tokens.get(pos2) {
                Some(Token::RParen) => Ok((expr, pos2 + 1)),
                _ => Err(CalcError::ParseError("expected ')'".into())),
            }
        }

        other => Err(CalcError::InvalidToken(format!(
            "unexpected token: {:?}",
            other
        ))),
    }
}

fn parse_call(tokens: &[Token], pos: usize) -> Result<(Expr, usize), CalcError> {
    let name = if let Token::Ident(name) = &tokens[pos] {
        name.clone()
    } else {
        return Err(CalcError::ParseError("expected function name".into()));
    };

    let mut args = Vec::new();
    let mut cur_pos = pos + 2;

    if matches!(tokens.get(cur_pos), Some(Token::RParen)) {
        return Ok((Expr::Call { name, args }, cur_pos + 1));
    }

    loop {
        let (arg_expr, next_pos) = parse_add_sub(tokens, cur_pos)?;
        args.push(arg_expr);
        cur_pos = next_pos;

        match tokens.get(cur_pos) {
            Some(Token::Comma) => {
                cur_pos += 1;
            }
            Some(Token::RParen) => {
                return Ok((Expr::Call { name, args }, cur_pos + 1));
            }
            _ => {
                return Err(CalcError::ParseError(
                    "expected ',' or ')' in argument list".into(),
                ));
            }
        }
    }
}
