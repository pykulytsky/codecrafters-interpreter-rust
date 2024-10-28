#![allow(dead_code, unused)]

pub struct Token {
    pub kind: TokenKind,
    pub lexeme: String,
    pub literal: Option<String>,
}

impl Token {
    pub const EOF: Self = Self {
        kind: TokenKind::Eof,
        lexeme: String::new(),
        literal: None,
    };

    pub fn new_left_paren() -> Self {
        Self {
            kind: TokenKind::LeftParen,
            lexeme: "(".to_string(),
            literal: None,
        }
    }

    pub fn new_right_paren() -> Self {
        Self {
            kind: TokenKind::RightParen,
            lexeme: ")".to_string(),
            literal: None,
        }
    }

    pub fn new_left_brace() -> Self {
        Self {
            kind: TokenKind::LeftBrace,
            lexeme: "{".to_string(),
            literal: None,
        }
    }

    pub fn new_right_brace() -> Self {
        Self {
            kind: TokenKind::RightBrace,
            lexeme: "}".to_string(),
            literal: None,
        }
    }
}

impl std::fmt::Debug for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:?} {} {}",
            self.kind,
            self.lexeme,
            match self.literal {
                Some(ref l) => l,
                None => "null",
            }
        )
    }
}

pub enum TokenKind {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Eof,
}

impl std::fmt::Debug for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::LeftParen => write!(f, "LEFT_PAREN"),
            Self::RightParen => write!(f, "RIGHT_PAREN"),
            Self::LeftBrace => write!(f, "LEFT_BRACE"),
            Self::RightBrace => write!(f, "RIGHT_BRACE"),
            Self::Eof => write!(f, "EOF"),
        }
    }
}
