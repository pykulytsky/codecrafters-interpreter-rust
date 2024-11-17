#![allow(dead_code, unused)]

pub const RESERVED_WORDS: &[&str] = &[
    "and", "class", "else", "false", "for", "fun", "if", "nil", "or", "print", "return", "super",
    "this", "true", "var", "while",
];

#[derive(Clone, PartialEq, PartialOrd, Eq, Ord)]
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

    pub fn reserved(s: String) -> Self {
        let kind = match s.as_str() {
            "and" => AND,
            "class" => CLASS,
            "else" => ELSE,
            "false" => FALSE,
            "for" => FOR,
            "fun" => FUN,
            "if" => IF,
            "nil" => NIL,
            "or" => OR,
            "print" => PRINT,
            "return" => RETURN,
            "super" => SUPER,
            "this" => THIS,
            "true" => TRUE,
            "var" => VAR,
            "while" => WHILE,
            _ => unreachable!("Unsupported reserved word"),
        };
        Self {
            kind,
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
#[allow(clippy::upper_case_acronyms)]
#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
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
    AND,
    CLASS,
    ELSE,
    FALSE,
    FOR,
    FUN,
    IF,
    NIL,
    OR,
    PRINT,
    RETURN,
    SUPER,
    THIS,
    TRUE,
    VAR,
    WHILE,
    Eof,
}

impl std::fmt::Debug for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LeftParen => write!(f, "LEFT_PAREN"),
            RightParen => write!(f, "RIGHT_PAREN"),
            LeftBrace => write!(f, "LEFT_BRACE"),
            RightBrace => write!(f, "RIGHT_BRACE"),
            Star => write!(f, "STAR"),
            Dot => write!(f, "DOT"),
            Comma => write!(f, "COMMA"),
            Plus => write!(f, "PLUS"),
            Minus => write!(f, "MINUS"),
            Semicolon => write!(f, "SEMICOLON"),
            Slash => write!(f, "SLASH"),
            Equal => write!(f, "EQUAL"),
            EqualEqual => write!(f, "EQUAL_EQUAL"),
            Bang => write!(f, "BANG"),
            BangEqual => write!(f, "BANG_EQUAL"),
            Less => write!(f, "LESS"),
            LessEqual => write!(f, "LESS_EQUAL"),
            Greater => write!(f, "GREATER"),
            GreaterEqual => write!(f, "GREATER_EQUAL"),
            StringLiteral => write!(f, "STRING"),
            NumberLiteral => write!(f, "NUMBER"),
            Identifier => write!(f, "IDENTIFIER"),
            Eof => write!(f, "EOF"),
            AND => write!(f, "AND"),
            CLASS => write!(f, "CLASS"),
            ELSE => write!(f, "ELSE"),
            FALSE => write!(f, "FALSE"),
            FOR => write!(f, "FOR"),
            FUN => write!(f, "FUN"),
            IF => write!(f, "IF"),
            NIL => write!(f, "NIL"),
            OR => write!(f, "OR"),
            PRINT => write!(f, "PRINT"),
            RETURN => write!(f, "RETURN"),
            SUPER => write!(f, "SUPER"),
            THIS => write!(f, "THIS"),
            TRUE => write!(f, "TRUE"),
            VAR => write!(f, "VAR"),
            WHILE => write!(f, "WHILE"),
        }
    }
}
