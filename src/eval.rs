use crate::ast::{BinaryOp, Expr};
use crate::error::CalcError;

pub fn eval_expr(expr: &Expr) -> Result<i64, CalcError> {
    match expr {
        Expr::Number(n) => Ok(*n),

        Expr::Binary { op, left, right } => {
            let l = eval_expr(left)?;
            let r = eval_expr(right)?;
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

        Expr::Var(name) => Err(CalcError::UndefinedVariable(name.clone())),
        Expr::Assign { name, .. } => Err(CalcError::ParseError(format!(
            "assignment not supported in this context: {}",
            name
        ))),
    }
}
