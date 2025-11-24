use crate::ast::{BinaryOp, Expr};
use crate::error::CalcError;
use std::collections::HashMap;

pub type Env = HashMap<String, i64>;

pub fn eval_expr(expr: &Expr, env: &mut Env) -> Result<i64, CalcError> {
    match expr {
        Expr::Number(n) => Ok(*n),

        Expr::UnaryNeg(inner) => {
            let v = eval_expr(inner, env)?;
            Ok(-v)
        }

        Expr::Binary { op, left, right } => {
            let l = eval_expr(left, env)?;
            let r = eval_expr(right, env)?;
            match op {
                BinaryOp::Add => Ok(l + r),
                BinaryOp::Sub => Ok(l - r),
                BinaryOp::Mul => Ok(l * r),
                BinaryOp::Div => {
                    if r == 0 {
                        Err(CalcError::DivideByZero)
                    } else {
                        Ok(l / r)
                    }
                }
            }
        }

        Expr::Var(name) => {
            if let Some(v) = env.get(name) {
                Ok(*v)
            } else {
                Err(CalcError::UndefinedVariable(name.clone()))
            }
        }

        Expr::Assign { name, expr } => {
            let value = eval_expr(expr, env)?;
            env.insert(name.clone(), value);
            Ok(value)
        }
        Expr::Call { name, args } => {
            let mut values = Vec::new();
            for arg in args {
                values.push(eval_expr(arg, env)?);
            }

            match name.as_str() {
                "max" => {
                    if values.len() != 2 {
                        return Err(CalcError::ArityMismatch {
                            name: name.clone(),
                            expected: 2,
                            found: values.len(),
                        });
                    }
                    Ok(values[0].max(values[1]))
                }
                "min" => {
                    if values.len() != 2 {
                        return Err(CalcError::ArityMismatch {
                            name: name.clone(),
                            expected: 2,
                            found: values.len(),
                        });
                    }
                    Ok(values[0].min(values[1]))
                }
                "abs" => {
                    if values.len() != 1 {
                        return Err(CalcError::ArityMismatch {
                            name: name.clone(),
                            expected: 1,
                            found: values.len(),
                        });
                    }
                    Ok(values[0].abs())
                }
                _ => Err(CalcError::UndefinedFunction(name.clone())),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::tokenize;
    use crate::parser::parse_line;

    fn eval_line(input: &str, env: &mut Env) -> i64 {
        let tokens = tokenize(input).expect("tokenize failed");
        let expr = parse_line(&tokens).expect("parse_line failed");
        eval_expr(&expr, env).expect("eval_expr failed")
    }

    #[test]
    fn eval_simple_arithmetic() {
        let mut env = Env::new();
        let result = eval_line("1 + 2 * 3", &mut env);
        assert_eq!(result, 7);
    }

    #[test]
    fn eval_unary_minus_and_parentheses() {
        let mut env = Env::new();
        let result = eval_line("-(1 + 2) * 4", &mut env);
        assert_eq!(result, -12);
    }

    #[test]
    fn eval_variable_assignment_and_use() {
        let mut env = Env::new();
        let v1 = eval_line("x = 3 + 4", &mut env);
        assert_eq!(v1, 7);
        let v2 = eval_line("x * 2", &mut env);
        assert_eq!(v2, 14);
    }

    #[test]
    fn eval_builtin_functions() {
        let mut env = Env::new();
        let v1 = eval_line("max(3, 5)", &mut env);
        assert_eq!(v1, 5);
        let v2 = eval_line("min(10, 4 + 3)", &mut env);
        assert_eq!(v2, 7);
        let v3 = eval_line("abs(-3 + 1)", &mut env);
        assert_eq!(v3, 2);
    }
}
