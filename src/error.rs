use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Unknown Error: {0}")]
    Unknown(#[from] std::io::Error),
    #[error("Error: {0}")]
    Custom(String),
}