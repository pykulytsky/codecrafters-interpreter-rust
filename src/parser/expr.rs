use std::ops::Deref;

use crate::parser::{
    error::{EvaluationError, EvaluationResult},
    Literal as LiteralType,
};

pub enum Expr {
    Literal(LiteralType),
    Unary(UnaryKind, Box<Expr>),
    Binary {
        op: BinaryKind,
        left: Box<Expr>,
        right: Box<Expr>,
    },
    Group(Vec<Expr>),
}

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
}

impl std::fmt::Debug for EvaluationValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Nil => write!(f, "nil"),
            Self::Number(arg0) => write!(f, "{arg0}"),
            Self::Str(arg0) => write!(f, "{arg0}"),
            Self::Logical(arg0) => write!(f, "{arg0}"),
        }
    }
}

impl Expr {
    pub const NIL: Self = Self::Literal(LiteralType::Nil);

    pub fn evaluate(&self) -> EvaluationResult<EvaluationValue> {
        match self {
            Expr::Literal(literal) => match literal {
                LiteralType::Str(s) => Ok(EvaluationValue::Str(s.to_string())),
                LiteralType::Number(n) => Ok(EvaluationValue::Number(*n)),
                LiteralType::Logical(l) => Ok(EvaluationValue::Logical(*l)),
                LiteralType::Nil => Ok(EvaluationValue::Nil),
            },
            Expr::Unary(unary_kind, expr) => {
                let value = expr.evaluate()?;
                match (unary_kind, value) {
                    (UnaryKind::Negation, EvaluationValue::Nil) => todo!(),
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
                let left = left.evaluate()?;
                let right = right.evaluate()?;
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
                    // TODO: handle specific cases, like string and number
                    (BinaryKind::Equality, _, _) => Ok(EvaluationValue::Logical(false)),
                    (BinaryKind::NotEquality, _, _) => Ok(EvaluationValue::Logical(false)),
                    _ => Err(EvaluationError::OperandsMustBeNumber(1)),
                }
            }
            Expr::Group(group) => group[0].evaluate(),
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
        }
    }
}
