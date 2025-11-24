use crate::ast::{BinaryOp, Expr};
use crate::error::CalcError;
use std::collections::HashMap;

pub type Env = HashMap<String, i64>;

pub fn eval_expr(expr: &Expr, env: &mut Env) -> Result<i64, CalcError> {
    match expr {
        Expr::Number(n) => Ok(*n),

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
    }
}
