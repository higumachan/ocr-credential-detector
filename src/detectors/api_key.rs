//! API key detector implementation

use crate::detectors::CredentialDetectorLogic;
use crate::ocr::TextLine;
use crate::types::{BoundingBox, CredentialDetection};
use regex::Regex;

/// API key detector
pub struct ApiKeyDetector {
    /// List of regular expressions representing API key patterns
    patterns: Vec<Regex>,
}

impl ApiKeyDetector {
    /// Create a new API key detector
    pub fn new() -> Self {
        // Common API key patterns
        // Examples: long alphanumeric strings, strings with specific prefixes, etc.
        let patterns = vec![
            // 20+ alphanumeric characters (common API keys)
            Regex::new(r"[A-Za-z0-9_\-]{20,}").unwrap(),
            // AWS Access Key (AKIA...)
            Regex::new(r"AKIA[0-9A-Z]{16}").unwrap(),
            // GitHub Personal Access Token (ghp_...)
            Regex::new(r"ghp_[A-Za-z0-9]{36}").unwrap(),
            // Google API Key
            Regex::new(r"AIza[0-9A-Za-z\-_]{35}").unwrap(),
        ];

        Self { patterns }
    }
}

impl CredentialDetectorLogic for ApiKeyDetector {
    fn name(&self) -> &str {
        "API Key Detector"
    }

    fn detect(&self, text_lines: &[TextLine]) -> Vec<CredentialDetection> {
        let mut detections = Vec::new();

        for line in text_lines {
            for pattern in &self.patterns {
                // Search for pattern matches in the text line
                for capture in pattern.captures_iter(&line.text) {
                    let matched_text = capture[0].to_string();

                    // Calculate position information for the matched part (simplified implementation)
                    // In a real implementation, the exact position of the matched part should be calculated
                    let bounding_box = BoundingBox {
                        x: line.x,
                        y: line.y,
                        width: line.width,
                        height: line.height,
                    };

                    // Calculate credential score (simplified implementation)
                    // In a real implementation, a more sophisticated scoring logic would be used
                    let credential_score = 0.9;

                    detections.push(CredentialDetection {
                        text: matched_text,
                        bounding_box,
                        text_detection_score: line.score,
                        credential_score,
                        detector_name: self.name().to_string(),
                    });
                }
            }
        }

        detections
    }
}
