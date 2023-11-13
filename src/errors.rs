use thiserror::Error;

#[derive(Error, Debug)]
pub enum EbioticError {
    #[error("Network error: {0}")]
    NetworkError(#[from] reqwest::Error),
    #[error("Parse error: {0}")]
    ParseError(#[from] serde_json::Error),
    #[error("Error: {0}")]
    ServiceError(String),
}
