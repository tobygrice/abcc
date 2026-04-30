use thiserror::Error;

/// abcc result type.
pub type Result<T> = std::result::Result<T, Error>;

/// abcc error type.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum Error {
    /// Invalid token found during lexing.
    #[error("Invalid token {token} at {line}:{column}")]
    InvalidToken {
        token: String,
        line: u32,
        column: u32,
    },

    #[error("GCC failure: {status}")]
    GccFailure { status: String },
}
