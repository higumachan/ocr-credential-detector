//! OCR Credential Detection Library
//!
//! This library provides functionality to extract text from images using OCR and
//! detect credentials (API keys, passwords, etc.) within the extracted text.

mod detector;
mod detectors;
mod error;
mod ocr;
mod types;

// Public modules and types
pub use detector::CredentialDetector;
pub use detectors::CredentialDetectorLogic;
pub use error::CredentialDetectorError;
pub use ocr::{OcrEngine, TextLine};
pub use types::{BoundingBox, CredentialDetection};
