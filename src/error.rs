use thiserror::Error;

use crate::inference::error::Error as InferenceError;
use crate::model::error::Error as ModelError;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Model error: {0}")]
    ModelError(#[from] ModelError),

    #[error("Inference error: {0}")]
    InferenceError(#[from] InferenceError),

    #[error("Web UI error: {0}")]
    WebUIError(String),

    #[error("Model not found: {0}")]
    ModelNotFound(String),
}
