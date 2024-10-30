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
        }
    }
}

impl Expr {
    pub const NIL: Self = Self::Literal(LiteralType::Nil);
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
