use thiserror::Error;

/// TODO: Add Errors for the application, such as: File not found, error reading file, search thread down, etc.
#[derive(Debug, Error)]
pub enum Error {
    #[error("Unknown Error: {0}")]
    Unknown(#[from] std::io::Error),
    #[error("Error: {0}")]
    Custom(String),
}