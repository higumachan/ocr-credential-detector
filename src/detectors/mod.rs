//! Credential detector implementations

mod api_key;
mod password;

pub use api_key::ApiKeyDetector;
pub use password::PasswordDetector;

use crate::ocr::TextLine;
use crate::types::CredentialDetection;

/// Credential detector interface
pub trait CredentialDetectorLogic {
    /// Returns the name of the detector
    fn name(&self) -> &str;

    /// Detects credentials from text lines
    fn detect(&self, text_lines: &[TextLine]) -> Vec<CredentialDetection>;
}
