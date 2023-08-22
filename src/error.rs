use thiserror::Error;

#[derive(Error, Debug)]
pub enum NanoError {
    #[error("Crossterm error: {0}")]
    Crossterm(#[from] std::io::Error),

    #[error("File error: {0}")]
    FileError(String),

    #[error("syntect error: {0}")]
    Syntect(#[from] syntect::Error),

    #[error("generic error: {0}")]
    Generic(String),
}

/// A type alias for handling Nano errors
pub type NanoResult<T> = Result<T, NanoError>;
