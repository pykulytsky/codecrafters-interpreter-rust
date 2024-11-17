use std::process::Termination;

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

#[derive(Debug, Error)]
pub enum EvaluationError {
    #[error("Operand must be a number.\n[line 1]")]
    MustBeNumber(usize),

    #[error("Operands must be a number.\n[line 1]")]
    OperandsMustBeNumber(usize),

    #[error("Undefined variable '{0}'.")]
    UndefinedVariable(String),
}

pub type EvaluationResult<T> = std::result::Result<T, EvaluationError>;

impl Termination for EvaluationError {
    fn report(self) -> std::process::ExitCode {
        65.into()
    }
}
