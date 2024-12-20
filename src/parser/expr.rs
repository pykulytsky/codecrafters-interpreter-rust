use std::{collections::BTreeMap, ops::Deref};

use crate::parser::{
    error::{EvaluationError, EvaluationResult}, stmt::Scope, Literal as LiteralType
};

#[derive(Debug, Clone, PartialEq, PartialOrd, Ord, Eq)]
pub struct Ident(pub String);

#[derive(Clone)]
pub enum Expr {
    Literal(LiteralType),
    Unary(UnaryKind, Box<Expr>),
    Binary {
        op: BinaryKind,
        left: Box<Expr>,
        right: Box<Expr>,
    },
    Group(Vec<Expr>),
    Ident(Ident),
    Assignment(Ident, Box<Expr>),
}

#[derive(Clone)]
pub enum UnaryKind {
    Negation,
    LogicalNot,
}

impl std::fmt::Debug for UnaryKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Negation => write!(f, "-"),
            Self::LogicalNot => write!(f, "!"),
        }
    }
}

#[derive(Clone)]
pub enum BinaryKind {
    Addition,
    Subtraction,
    Multiplication,
    Division,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    Equality,
    NotEquality,
}

impl std::fmt::Debug for BinaryKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Addition => write!(f, "+"),
            Self::Subtraction => write!(f, "-"),
            Self::Multiplication => write!(f, "*"),
            Self::Division => write!(f, "/"),
            Self::Less => write!(f, "<"),
            Self::LessEqual => write!(f, "<="),
            Self::Greater => write!(f, ">"),
            Self::GreaterEqual => write!(f, ">="),
            Self::Equality => write!(f, "=="),
            Self::NotEquality => write!(f, "!="),
        }
    }
}

pub enum EvaluationValue {
    Nil,
    Number(f64),
    Str(String),
    Logical(bool),
    Void,
}

impl EvaluationValue {
    pub fn to_expr(&self) -> Expr {
        match self {
            EvaluationValue::Nil => Expr::NIL,
            EvaluationValue::Number(n) => Expr::Literal(LiteralType::Number(*n)),
            EvaluationValue::Str(s) => Expr::Literal(LiteralType::Str(s.to_owned())),
            EvaluationValue::Logical(l) => Expr::Literal(LiteralType::Logical(*l)),
            EvaluationValue::Void => todo!(),
        }
    }
}

impl std::fmt::Debug for EvaluationValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Nil => write!(f, "nil"),
            Self::Number(arg0) => write!(f, "{arg0}"),
            Self::Str(arg0) => write!(f, "{arg0}"),
            Self::Logical(arg0) => write!(f, "{arg0}"),
            Self::Void => Ok(()),
        }
    }
}

impl Expr {
    pub const NIL: Self = Self::Literal(LiteralType::Nil);

    pub fn evaluate(
        &self,
        scope: &Scope,
    ) -> EvaluationResult<EvaluationValue> {
        match self {
            Expr::Literal(literal) => match literal {
                LiteralType::Str(s) => Ok(EvaluationValue::Str(s.to_string())),
                LiteralType::Number(n) => Ok(EvaluationValue::Number(*n)),
                LiteralType::Logical(l) => Ok(EvaluationValue::Logical(*l)),
                LiteralType::Nil => Ok(EvaluationValue::Nil),
            },
            Expr::Unary(unary_kind, expr) => {
                let value = expr.evaluate(scope)?;
                match (unary_kind, value) {
                    (UnaryKind::Negation, EvaluationValue::Nil) => todo!(),
                    (_, EvaluationValue::Void) => todo!(),
                    (UnaryKind::Negation, EvaluationValue::Number(n)) => {
                        Ok(EvaluationValue::Number(-n))
                    }
                    (UnaryKind::Negation, _) => Err(EvaluationError::MustBeNumber(1)),
                    (UnaryKind::LogicalNot, EvaluationValue::Nil) => {
                        Ok(EvaluationValue::Logical(true))
                    }
                    (UnaryKind::LogicalNot, EvaluationValue::Number(n)) => {
                        Ok(EvaluationValue::Logical(n == 0.0))
                    }
                    (UnaryKind::LogicalNot, EvaluationValue::Str(_)) => todo!(),
                    (UnaryKind::LogicalNot, EvaluationValue::Logical(l)) => {
                        Ok(EvaluationValue::Logical(!l))
                    }
                }
            }
            Expr::Binary { op, left, right } => {
                let left = left.evaluate(scope)?;
                let right = right.evaluate(scope)?;
                match (op, left, right) {
                    (
                        BinaryKind::Multiplication,
                        EvaluationValue::Number(left),
                        EvaluationValue::Number(right),
                    ) => Ok(EvaluationValue::Number(left * right)),
                    (
                        BinaryKind::Division,
                        EvaluationValue::Number(left),
                        EvaluationValue::Number(right),
                    ) => Ok(EvaluationValue::Number(left / right)),
                    (
                        BinaryKind::Addition,
                        EvaluationValue::Number(left),
                        EvaluationValue::Number(right),
                    ) => Ok(EvaluationValue::Number(left + right)),
                    (
                        BinaryKind::Subtraction,
                        EvaluationValue::Number(left),
                        EvaluationValue::Number(right),
                    ) => Ok(EvaluationValue::Number(left - right)),
                    (
                        BinaryKind::Addition,
                        EvaluationValue::Str(left),
                        EvaluationValue::Str(right),
                    ) => Ok(EvaluationValue::Str(left + &right)),

                    (
                        BinaryKind::Less,
                        EvaluationValue::Number(left),
                        EvaluationValue::Number(right),
                    ) => Ok(EvaluationValue::Logical(left < right)),
                    (
                        BinaryKind::LessEqual,
                        EvaluationValue::Number(left),
                        EvaluationValue::Number(right),
                    ) => Ok(EvaluationValue::Logical(left <= right)),
                    (
                        BinaryKind::Greater,
                        EvaluationValue::Number(left),
                        EvaluationValue::Number(right),
                    ) => Ok(EvaluationValue::Logical(left > right)),
                    (
                        BinaryKind::GreaterEqual,
                        EvaluationValue::Number(left),
                        EvaluationValue::Number(right),
                    ) => Ok(EvaluationValue::Logical(left >= right)),
                    (
                        BinaryKind::Equality,
                        EvaluationValue::Number(left),
                        EvaluationValue::Number(right),
                    ) => Ok(EvaluationValue::Logical(left == right)),
                    (
                        BinaryKind::NotEquality,
                        EvaluationValue::Number(left),
                        EvaluationValue::Number(right),
                    ) => Ok(EvaluationValue::Logical(left != right)),
                    (
                        BinaryKind::Equality,
                        EvaluationValue::Str(left),
                        EvaluationValue::Str(right),
                    ) => Ok(EvaluationValue::Logical(left == right)),
                    (
                        BinaryKind::NotEquality,
                        EvaluationValue::Str(left),
                        EvaluationValue::Str(right),
                    ) => Ok(EvaluationValue::Logical(left != right)),
                    (
                        BinaryKind::Equality,
                        EvaluationValue::Logical(left),
                        EvaluationValue::Logical(right),
                    ) => Ok(EvaluationValue::Logical(left == right)),
                    (
                        BinaryKind::NotEquality,
                        EvaluationValue::Logical(left),
                        EvaluationValue::Logical(right),
                    ) => Ok(EvaluationValue::Logical(left != right)),
                    // TODO: handle specific cases, like string and number
                    (BinaryKind::Equality, _, _) => Ok(EvaluationValue::Logical(false)),
                    (BinaryKind::NotEquality, _, _) => Ok(EvaluationValue::Logical(false)),
                    _ => Err(EvaluationError::OperandsMustBeNumber(1)),
                }
            }
            Expr::Group(group) => group[0].evaluate(scope),
            Expr::Ident(ident) => {
                let Some(expr) = scope.get(ident) else {
                    return Err(EvaluationError::UndefinedVariable(ident.0.clone()));
                };
                let expr = expr.to_owned();
                expr.evaluate(scope)
            }
            Expr::Assignment(left, right) => {
                // global_variables.insert(left.to_owned(), *right.clone());
                right.evaluate(scope)
            }
        }
    }
}

impl std::fmt::Debug for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Literal(arg0) => write!(f, "{:?}", arg0),
            Self::Group(group) => {
                write!(f, "(group")?;
                for expr in group {
                    write!(f, " {:?}", expr)?;
                }
                write!(f, ")")?;
                Ok(())
            }
            Self::Unary(kind, operand) => write!(f, "({:?} {:?})", kind, operand),
            Self::Binary { op, left, right } => write!(f, "({:?} {:?} {:?})", op, left, right),
            Self::Ident(ident) => write!(f, "{}", ident.0),
            Self::Assignment(ident, right) => write!(f, "{} = {:?}", ident.0, right),
        }
    }
}
