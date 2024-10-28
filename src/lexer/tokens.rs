#![allow(dead_code, unused)]

pub struct Token {
    pub kind: TokenKind,
    pub lexeme: String,
    pub literal: Option<String>,
}

macro_rules! new_token {
    ($name:ident, $lex:literal, $kind:ident) => {
        pub fn $name() -> Self {
            Self {
                kind: $kind,
                lexeme: $lex.to_string(),
                literal: None,
            }
        }
    };
}

use TokenKind::*;
impl Token {
    pub const EOF: Self = Self {
        kind: TokenKind::Eof,
        lexeme: String::new(),
        literal: None,
    };

    new_token!(left_paren, "(", LeftParen);
    new_token!(right_paren, ")", RightParen);
    new_token!(left_brace, "{", LeftBrace);
    new_token!(right_brace, "}", RightBrace);
    new_token!(comma, ",", Comma);
    new_token!(dot, ".", Dot);
    new_token!(star, "*", Star);
    new_token!(plus, "+", Plus);
    new_token!(minus, "-", Minus);
    new_token!(semicolon, ";", Semicolon);
    new_token!(slash, "/", Slash);
    new_token!(equal, "=", Equal);
    new_token!(equal_equal, "==", EqualEqual);
    new_token!(bang, "!", Bang);
    new_token!(bang_equal, "!=", BangEqual);
    new_token!(less, "<", Less);
    new_token!(less_equal, "<=", LessEqual);
    new_token!(greater, ">", Greater);
    new_token!(greater_equal, ">=", GreaterEqual);

    pub fn string_literal(s: String) -> Self {
        Self {
            kind: TokenKind::StringLiteral,
            lexeme: s.clone(),
            literal: Some(
                s.strip_prefix('"')
                    .and_then(|s| s.strip_suffix('"'))
                    .unwrap()
                    .to_string(),
            ),
        }
    }

    pub fn number_literal(s: String) -> Self {
        let mut literal = s.clone();
        if !literal.contains('.') {
            literal += ".0";
        } else if let Some(zero_pos) = literal.find(".") {
            literal = literal.trim_end_matches('0').to_string();

            if literal.ends_with('.') {
                literal += "0";
            }
        }
        Self {
            kind: TokenKind::NumberLiteral,
            lexeme: s,
            literal: Some(literal),
        }
    }

    pub fn identifier(s: String) -> Self {
        Self {
            kind: TokenKind::Identifier,
            lexeme: s,
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
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    StringLiteral,
    NumberLiteral,
    Identifier,
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
            Self::Less => write!(f, "LESS"),
            Self::LessEqual => write!(f, "LESS_EQUAL"),
            Self::Greater => write!(f, "GREATER"),
            Self::GreaterEqual => write!(f, "GREATER_EQUAL"),
            Self::StringLiteral => write!(f, "STRING"),
            Self::NumberLiteral => write!(f, "NUMBER"),
            Self::Identifier => write!(f, "IDENTIFIER"),
            Self::Eof => write!(f, "EOF"),
        }
    }
}
