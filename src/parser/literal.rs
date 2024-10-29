pub enum Literal {
    Str(String),
    Number(isize),
    Logical(bool),
    Nil,
}

impl std::fmt::Debug for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Str(arg0) => write!(f, "{}", arg0),
            Self::Number(arg0) => write!(f, "{}", arg0),
            Self::Logical(arg0) => write!(f, "{}", arg0),
            Self::Nil => write!(f, "nil"),
        }
    }
}
