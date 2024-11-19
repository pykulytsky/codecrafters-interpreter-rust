use std::collections::BTreeMap;

use crate::{
    lexer::Token,
    parser::{
        error::EvaluationResult,
        expr::{EvaluationValue, Ident},
        Expr,
    },
};

pub enum Stmt {
    Expr(Expr),
    Print(Expr),
    Declaration(Ident, Expr),
}

impl Stmt {
    pub fn run(
        &self,
        global_variables: &mut BTreeMap<Ident, Expr>,
    ) -> EvaluationResult<EvaluationValue> {
        match self {
            Stmt::Expr(expr) => {
                let evaluation_result = expr.evaluate(global_variables)?;
                Ok(EvaluationValue::Void)
            }
            Stmt::Print(expr) => {
                println!("{:?}", expr.evaluate(global_variables)?);
                Ok(EvaluationValue::Void)
            }
            Stmt::Declaration(_left, _right) => Ok(EvaluationValue::Void),
            // Stmt::Assignment(_left, _right) => Ok(EvaluationValue::Void),
        }
    }
}

impl std::fmt::Debug for Stmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Stmt::Expr(expr) => expr.fmt(f),
            Stmt::Print(expr) => write!(f, "print {:?};", expr),
            Stmt::Declaration(left, right) => write!(f, "var {:?} = {:?};", left.0, right),
            // Stmt::Assignment(left, right) => write!(f, "{:?} = {:?};", left.0, right),
        }
    }
}
