use crate::parser::Literal as LiteralType;

pub enum Expr {
    Literal(LiteralType),
    Unary(UnaryKind, Box<Expr>),
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
        }
    }
}
