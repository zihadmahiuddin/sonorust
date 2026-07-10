use thiserror::Error;

#[derive(Debug, Error)]
pub enum SonorustResourceError {
    #[error("Serde error occurred: {0}")]
    Serde(#[from] serde_json::Error),
    #[error("IO error occurred: {0}")]
    Io(#[from] std::io::Error),
}
