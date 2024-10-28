use thiserror::Error;

#[derive(Debug, Error)]
pub enum LexerError {
    #[error("could not parse source")]
    SourceError(#[from] tokio::io::Error),
}

pub type LexerResult<T> = std::result::Result<T, LexerError>;
