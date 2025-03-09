//! Credential detector implementation

use std::fs;
use std::path::Path;

use crate::detectors::{ApiKeyDetector, CredentialDetectorLogic, PasswordDetector};
use crate::error::CredentialDetectorError;
use crate::ocr::OcrEngine;
use crate::types::CredentialDetection;

/// Credential detector
pub struct CredentialDetector {
    /// OCR engine
    ocr_engine: OcrEngine,
    /// List of credential detection logic
    detectors: Vec<Box<dyn CredentialDetectorLogic>>,
}

impl CredentialDetector {
    /// Create a new credential detector
    pub fn new() -> Result<Self, CredentialDetectorError> {
        // Initialize OCR engine
        let ocr_engine = OcrEngine::new()?;

        // Register default detectors
        let detectors: Vec<Box<dyn CredentialDetectorLogic>> = vec![
            Box::new(ApiKeyDetector::new()),
            Box::new(PasswordDetector::new()),
        ];

        Ok(Self {
            ocr_engine,
            detectors,
        })
    }

    /// Add a custom detector
    pub fn with_detector(mut self, detector: Box<dyn CredentialDetectorLogic>) -> Self {
        self.detectors.push(detector);
        self
    }

    /// Detect credentials from image data
    pub fn detect_from_image(
        &self,
        image_data: &[u8],
    ) -> Result<Vec<CredentialDetection>, CredentialDetectorError> {
        // Extract text from image
        let text_lines = self.ocr_engine.extract_text(image_data)?;

        // Detect credentials using each detector
        let mut detections = Vec::new();
        for detector in &self.detectors {
            let mut detector_results = detector.detect(&text_lines);
            detections.append(&mut detector_results);
        }

        // Sort by score in descending order
        detections.sort_by(|a, b| b.credential_score.partial_cmp(&a.credential_score).unwrap());

        Ok(detections)
    }

    /// Detect credentials from file path
    pub fn detect_from_path<P: AsRef<Path>>(
        &self,
        path: P,
    ) -> Result<Vec<CredentialDetection>, CredentialDetectorError> {
        // Load image data from file
        let image_data = match fs::read(path) {
            Ok(data) => data,
            Err(e) => return Err(CredentialDetectorError::FileNotFoundError(e.to_string())),
        };

        // Detect credentials from image data
        self.detect_from_image(&image_data)
    }
}
