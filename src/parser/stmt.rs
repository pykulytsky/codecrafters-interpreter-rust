use crate::parser::{error::EvaluationResult, expr::EvaluationValue, Expr};

pub enum Stmt {
    Expr(Expr),
    Print(Expr),
    // Assignment(Expr, Expr),
}

impl Stmt {
    pub fn run(&self) -> EvaluationResult<EvaluationValue> {
        match self {
            Stmt::Expr(expr) => {
                let evaluation_result = expr.evaluate()?;
                Ok(EvaluationValue::Void)
            }
            Stmt::Print(expr) => {
                println!("{:?}", expr.evaluate()?);
                Ok(EvaluationValue::Void)
            }
        }
    }
}

impl std::fmt::Debug for Stmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Stmt::Expr(expr) => expr.fmt(f),
            Stmt::Print(expr) => write!(f, "print {:?};", expr),
            // Stmt::Assignment(left, right) => write!(f, "var {:?} = {:?};", left, right),
        }
    }
}
