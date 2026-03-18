use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Cannot access the app directory")]
    FailedToGetBaseDir,

    #[error("Model {0} has already been downloaded")]
    ModelAlreadyDownloaded(String),

    #[error("Request error: {0}")]
    ReqwestError(#[from] reqwest::Error),

    #[error("Cannot save model in {0}")]
    ModelIOError(#[from] std::io::Error),

    #[error("Cannot parse url: {0}")]
    ParseError(#[from] url::ParseError),

    #[error("Model not found: {0}")]
    ModelNotFound(String),
}
