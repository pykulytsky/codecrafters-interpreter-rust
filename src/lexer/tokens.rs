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

    pub fn new_comma() -> Self {
        Self {
            kind: TokenKind::Comma,
            lexeme: ",".to_string(),
            literal: None,
        }
    }

    pub fn new_dot() -> Self {
        Self {
            kind: TokenKind::Dot,
            lexeme: ".".to_string(),
            literal: None,
        }
    }

    pub fn new_star() -> Self {
        Self {
            kind: TokenKind::Star,
            lexeme: "*".to_string(),
            literal: None,
        }
    }

    pub fn new_plus() -> Self {
        Self {
            kind: TokenKind::Plus,
            lexeme: "+".to_string(),
            literal: None,
        }
    }

    pub fn new_minus() -> Self {
        Self {
            kind: TokenKind::Minus,
            lexeme: "-".to_string(),
            literal: None,
        }
    }

    pub fn new_semicolon() -> Self {
        Self {
            kind: TokenKind::Semicolon,
            lexeme: ";".to_string(),
            literal: None,
        }
    }

    pub fn new_slash() -> Self {
        Self {
            kind: TokenKind::Slash,
            lexeme: "/".to_string(),
            literal: None,
        }
    }

    pub fn new_equal() -> Self {
        Self {
            kind: TokenKind::Equal,
            lexeme: "=".to_string(),
            literal: None,
        }
    }

    pub fn new_equal_equal() -> Self {
        Self {
            kind: TokenKind::EqualEqual,
            lexeme: "==".to_string(),
            literal: None,
        }
    }

    pub fn new_bang() -> Self {
        Self {
            kind: TokenKind::Bang,
            lexeme: "!".to_string(),
            literal: None,
        }
    }

    pub fn new_bang_equal() -> Self {
        Self {
            kind: TokenKind::BangEqual,
            lexeme: "!=".to_string(),
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
    Star,
    Dot,
    Comma,
    Plus,
    Minus,
    Semicolon,
    Slash,
    Equal,
    EqualEqual,
    Bang,
    BangEqual,
    Eof,
}

impl std::fmt::Debug for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::LeftParen => write!(f, "LEFT_PAREN"),
            Self::RightParen => write!(f, "RIGHT_PAREN"),
            Self::LeftBrace => write!(f, "LEFT_BRACE"),
            Self::RightBrace => write!(f, "RIGHT_BRACE"),
            Self::Star => write!(f, "STAR"),
            Self::Dot => write!(f, "DOT"),
            Self::Comma => write!(f, "COMMA"),
            Self::Plus => write!(f, "PLUS"),
            Self::Minus => write!(f, "MINUS"),
            Self::Semicolon => write!(f, "SEMICOLON"),
            Self::Slash => write!(f, "SLASH"),
            Self::Equal => write!(f, "EQUAL"),
            Self::EqualEqual => write!(f, "EQUAL_EQUAL"),
            Self::Bang => write!(f, "BANG"),
            Self::BangEqual => write!(f, "BANG_EQUAL"),
            Self::Eof => write!(f, "EOF"),
        }
    }
}
