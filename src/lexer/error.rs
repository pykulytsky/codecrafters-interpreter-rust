use thiserror::Error;

#[derive(Debug, Error)]
pub enum LexerError {
    #[error("could not parse source")]
    SourceError(#[from] tokio::io::Error),

    #[error("[line {line}] Error: Unexpected character: {ch}")]
    UnexpectedCharacter { line: usize, ch: char },
}

pub type LexerResult<T> = std::result::Result<T, LexerError>;
