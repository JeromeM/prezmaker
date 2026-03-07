use thiserror::Error;

#[derive(Error, Debug)]
pub enum PrezError {
    #[error("HTTP request failed: {0}")]
    Http(#[from] reqwest::Error),

    #[error("JSON parsing failed: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Config error: {0}")]
    Config(String),

    #[error("API key missing: {0}")]
    MissingApiKey(String),

    #[error("No results found for query: {0}")]
    NoResults(String),

    #[error("Allocine scraping failed: {0}")]
    Scraping(String),

    #[error("User cancelled selection")]
    Cancelled,

    #[error("Clipboard error: {0}")]
    Clipboard(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("{0}")]
    Other(String),
}
