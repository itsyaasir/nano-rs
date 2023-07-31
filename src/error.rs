use thiserror::Error;

#[derive(Error, Debug)]
pub enum NanoError {
    #[error("Crossterm error: {0}")]
    Crossterm(#[from] crossterm::ErrorKind),

    #[error("File error: {0}")]
    FileError(String),

    #[error(transparent)]
    Syntect(#[from] syntect::Error),
}

/// A type alias for handling Nano errors
pub type NanoResult<T> = Result<T, NanoError>;
