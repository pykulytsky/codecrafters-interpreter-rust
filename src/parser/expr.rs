use std::ops::Deref;

use crate::parser::Literal as LiteralType;

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

pub enum EvaluationResult {
    Nil,
    Number(f64),
    Str(String),
    Logical(bool),
}

impl std::fmt::Debug for EvaluationResult {
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

    pub fn evaluate(&self) -> EvaluationResult {
        match self {
            Expr::Literal(literal) => match literal {
                LiteralType::Str(s) => EvaluationResult::Str(s.to_string()),
                LiteralType::Number(n) => EvaluationResult::Number(*n),
                LiteralType::Logical(l) => EvaluationResult::Logical(*l),
                LiteralType::Nil => EvaluationResult::Nil,
            },
            Expr::Unary(unary_kind, expr) => {
                // let value = match expr.deref() {
                //     Expr::Group(group) => {
                //         for expr in group {
                //             dbg!(&expr);
                //             println!("Evaluated group: {:?}", expr.evaluate())
                //         }
                //         todo!()
                //     }
                //     _ => expr.evaluate(),
                // };
                let value = expr.evaluate();
                match (unary_kind, value) {
                    (UnaryKind::Negation, EvaluationResult::Nil) => todo!(),
                    (UnaryKind::Negation, EvaluationResult::Number(n)) => {
                        EvaluationResult::Number(-n)
                    }
                    (UnaryKind::Negation, EvaluationResult::Str(_)) => todo!(),
                    (UnaryKind::Negation, EvaluationResult::Logical(_)) => todo!(),
                    (UnaryKind::LogicalNot, EvaluationResult::Nil) => {
                        EvaluationResult::Logical(true)
                    }
                    (UnaryKind::LogicalNot, EvaluationResult::Number(n)) => {
                        EvaluationResult::Logical(n == 0.0)
                    }
                    (UnaryKind::LogicalNot, EvaluationResult::Str(_)) => todo!(),
                    (UnaryKind::LogicalNot, EvaluationResult::Logical(l)) => {
                        EvaluationResult::Logical(!l)
                    }
                }
            }
            Expr::Binary { op, left, right } => {
                let left = left.evaluate();
                let right = right.evaluate();
                match (op, left, right) {
                    (
                        BinaryKind::Multiplication,
                        EvaluationResult::Number(left),
                        EvaluationResult::Number(right),
                    ) => EvaluationResult::Number(left * right),
                    (
                        BinaryKind::Division,
                        EvaluationResult::Number(left),
                        EvaluationResult::Number(right),
                    ) => EvaluationResult::Number(left / right),
                    (
                        BinaryKind::Addition,
                        EvaluationResult::Number(left),
                        EvaluationResult::Number(right),
                    ) => EvaluationResult::Number(left + right),
                    (
                        BinaryKind::Subtraction,
                        EvaluationResult::Number(left),
                        EvaluationResult::Number(right),
                    ) => EvaluationResult::Number(left - right),
                    (
                        BinaryKind::Addition,
                        EvaluationResult::Str(left),
                        EvaluationResult::Str(right),
                    ) => EvaluationResult::Str(left + &right),

                    (
                        BinaryKind::Less,
                        EvaluationResult::Number(left),
                        EvaluationResult::Number(right),
                    ) => EvaluationResult::Logical(left < right),
                    (
                        BinaryKind::LessEqual,
                        EvaluationResult::Number(left),
                        EvaluationResult::Number(right),
                    ) => EvaluationResult::Logical(left <= right),
                    (
                        BinaryKind::Greater,
                        EvaluationResult::Number(left),
                        EvaluationResult::Number(right),
                    ) => EvaluationResult::Logical(left > right),
                    (
                        BinaryKind::GreaterEqual,
                        EvaluationResult::Number(left),
                        EvaluationResult::Number(right),
                    ) => EvaluationResult::Logical(left >= right),
                    _ => todo!(),
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
