use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Cannot access the app directory {}")]
    FailedToGetBaseDir,

    #[error("Model {0} has allready been downloaded")]
    ModelAllreadyDownloaded(String),

    #[error("Request error: {0}")]
    ReqwestError(#[from] reqwest::Error),
}
