use thiserror::Error;

use crate::model::error::Error as ModelError;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Model error: {0}")]
    ModelError(#[from] ModelError),

    #[error("Inference error: {0}")]
    InferenceError(String),

    #[error("Web UI error: {0}")]
    WebUIError(String),
}
