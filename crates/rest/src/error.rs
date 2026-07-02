use thiserror::Error;

#[derive(Debug, Error)]
pub enum SonorustRestError {
    #[error("Serde error occurred: {0}")]
    Serde(#[from] serde_json::Error),
    #[error("IO error occurred: {0}")]
    Io(#[from] std::io::Error),
    #[error("Reqwest error occurred: {0}")]
    Reqwest(#[from] reqwest::Error),
    #[error("Url parsing error occurred: {0}")]
    Url(#[from] url::ParseError),
}
