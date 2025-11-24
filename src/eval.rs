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
