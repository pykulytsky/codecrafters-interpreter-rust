#[derive(Clone)]
pub enum Literal {
    Str(String),
    Number(f64),
    Logical(bool),
    Nil,
}

impl std::fmt::Debug for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Str(arg0) => write!(f, "{}", arg0),
            Self::Number(arg0) => {
                if arg0.fract() == 0.0 {
                    write!(f, "{:.1}", arg0)
                } else {
                    write!(f, "{}", arg0)
                }
            }
            Self::Logical(arg0) => write!(f, "{}", arg0),
            Self::Nil => write!(f, "nil"),
        }
    }
}
