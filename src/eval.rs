use crate::{bin_op::BinOp, expr::Expr};

/// Evaluates an expression.
pub fn eval_expr(expr: &Expr) -> f64 {
    match expr {
        Expr::Literal(value) => *value,
        Expr::Negate(expr) => -eval_expr(expr),
        Expr::Binary { lhs, op, rhs } => {
            let lhs = eval_expr(lhs);
            let rhs = eval_expr(rhs);

            match op {
                BinOp::Assign => {
                    println!("{lhs} <- {rhs}"); // TODO: Implement assignments.
                    rhs
                }
                BinOp::Add => lhs + rhs,
                BinOp::Subtract => lhs - rhs,
                BinOp::Multiply => lhs * rhs,
                BinOp::Divide => lhs / rhs,
            }
        }
    }
}
