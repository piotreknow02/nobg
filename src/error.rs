use thiserror::Error;

use crate::model::error::Error as ModelError;

#[derive(Error, Debug)]
#[allow(dead_code)]
pub enum Error {
    #[error("Model error: {0}")]
    ModelError(#[from] ModelError),

    #[error("Inference error: {0}")]
    InferenceError(String),

    #[error("Web UI error: {0}")]
    WebUIError(String),

    #[error("Image error: {0}")]
    ImageError(#[from] image::ImageError),

    #[error("ORT error: {0}")]
    ORTError(#[from] ort::Error),

    #[error("Model not found: {0}")]
    ModelNotFound(String),

    #[error("Output format does not support transparency. Use PNG, WebP, GIF, TIFF, or ICO.")]
    OutputFormatError,
}
