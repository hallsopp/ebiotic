use thiserror::Error;

#[allow(clippy::enum_variant_names)]
#[derive(Error, Debug)]
pub enum EbioticError {
    // Froms
    #[error("Network error: {0}")]
    NetworkError(#[from] reqwest::Error),
    #[error("Unable to deserialize: {0}")]
    ParseError(#[from] serde_json::Error),
    #[error("Unable to parse Float from Str: {0}")]
    ParseFloatError(#[from] std::num::ParseFloatError),
    #[error("Unable to handle IO: {0}")]
    IoError(#[from] std::io::Error),

    // Custom
    #[error("Error: {0}")] // This is a catch-all for now
    ServiceError(String),
    #[error("Return format {0} not available for database {1}")]
    ReturnFormatNotAvailable(String, String),
    #[error("Empty queries are not permitted. Add something!")]
    EmptyEbiSearchQuery,
    #[error("Too many query commands. Maximum of 4 allowed.")]
    TooManyQueryCommands,
    #[error("Query string/search term must be the last or only command.")]
    QueryStrOrTermNotFirst,
}
