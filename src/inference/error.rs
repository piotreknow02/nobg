use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Output format does not support transparency. Use PNG, WebP, GIF, TIFF, or ICO.")]
    OutputFormatError,

    #[error("Model not found: {0}")]
    ModelNotFound(String),

    #[error("Failed to create session: {0}")]
    SessionError(#[from] ort::Error),

    #[error("Failed to convert output shape: {0}")]
    ShapeError(#[from] ndarray::ShapeError),

    #[error("Failed to open or save image: {0}")]
    ImageError(#[from] image::ImageError),

    #[error("Failed to get model path: {0}")]
    ModelPathError(#[from] crate::model::error::Error),
}
