use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Output format does not support transparency. Use PNG, WebP, GIF, TIFF, or ICO.")]
    OutputFormatError,
}
