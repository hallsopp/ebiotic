use thiserror::Error;

#[derive(Error, Debug)]
pub enum EbioticError {
    #[error("Network error: {0}")]
    NetworkError(#[from] reqwest::Error),
    #[error("Unable to deserialize: {0}")]
    ParseError(#[from] serde_json::Error),
    #[error("Error: {0}")]
    ServiceError(String),
    #[error("Unable to parse Float from Str: {0}")]
    ParseFloatError(#[from] std::num::ParseFloatError),
    #[error("Unable to handle IO: {0}")]
    IoError(#[from] std::io::Error),
}