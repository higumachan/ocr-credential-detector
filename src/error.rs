//! Error type definitions

use thiserror::Error;

/// Credential detector error type
#[derive(Error, Debug)]
pub enum CredentialDetectorError {
    /// Error when OCR engine initialization fails
    #[error("OCR engine initialization failed: {0}")]
    OcrEngineInitError(String),

    /// Error when image loading fails
    #[error("Image loading failed: {0}")]
    ImageLoadError(String),

    /// Error when OCR processing fails
    #[error("OCR processing failed: {0}")]
    OcrProcessError(String),

    /// Error when file is not found
    #[error("File not found: {0}")]
    FileNotFoundError(String),

    /// Unknown error
    #[error("Unknown error: {0}")]
    UnknownError(String),
}
