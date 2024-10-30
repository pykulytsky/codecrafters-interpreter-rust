use thiserror::Error;

#[derive(Debug, Error)]
pub enum ParserError {
    #[error("[line {line}] Error: Expected expression: {lexeme}")]
    ExpectedExpression { line: usize, lexeme: String },

    #[error("Lexer Error")]
    LexerError(#[from] crate::lexer::error::LexerError),

    #[error("[line {_0}] Error: Unmatched parens")]
    UnmatchedParens(usize),

    #[error("[line {_0}] Error: Unexpected token")]
    UnexpectedToken(usize),
}

pub type ParserResult<T> = std::result::Result<T, ParserError>;
