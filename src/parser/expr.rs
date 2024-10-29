use crate::parser::Literal as LiteralType;

pub enum Expr {
    Literal(LiteralType),
}

impl Expr {
    pub const NIL: Self = Self::Literal(LiteralType::Nil);
}

impl std::fmt::Debug for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Literal(arg0) => write!(f, "{:?}", arg0),
        }
    }
}
