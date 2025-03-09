//! Password detector implementation

use crate::detectors::CredentialDetectorLogic;
use crate::ocr::TextLine;
use crate::types::{BoundingBox, CredentialDetection};
use regex::Regex;

/// Password detector
pub struct PasswordDetector {
    /// Password-related keywords
    keywords: Vec<&'static str>,
    /// Regular expression representing password patterns
    pattern: Regex,
}

impl PasswordDetector {
    /// Create a new password detector
    pub fn new() -> Self {
        // Password-related keywords
        let keywords = vec![
            "password", "pwd", "pass", "passwd", "password", // Translated from Japanese
            "pass",     // Translated from Japanese
            "pin",      // Translated from Japanese
        ];

        // Password pattern (8+ characters with alphanumeric and special characters)
        let pattern = Regex::new(r"[A-Za-z0-9\-_!@#$%^&*()+=]{8,}").unwrap();

        Self { keywords, pattern }
    }

    /// Determine if text contains password-related keywords
    fn contains_password_keyword(&self, text: &str) -> bool {
        let text_lower = text.to_lowercase();
        self.keywords
            .iter()
            .any(|&keyword| text_lower.contains(keyword))
    }
}

impl CredentialDetectorLogic for PasswordDetector {
    fn name(&self) -> &str {
        "Password Detector"
    }

    fn detect(&self, text_lines: &[TextLine]) -> Vec<CredentialDetection> {
        let mut detections = Vec::new();

        // Window size for detecting passwords including adjacent lines
        const WINDOW_SIZE: usize = 1;

        for i in 0..text_lines.len() {
            let current_line = &text_lines[i];

            // If the current line contains password-related keywords
            if self.contains_password_keyword(&current_line.text) {
                // Include adjacent lines in the search range
                let start = i.saturating_sub(WINDOW_SIZE);
                let end = (i + WINDOW_SIZE + 1).min(text_lines.len());

                for j in start..end {
                    if i == j {
                        continue; // Skip the line containing the keyword itself
                    }

                    let line = &text_lines[j];

                    // Search for parts matching the password pattern
                    for capture in self.pattern.captures_iter(&line.text) {
                        let matched_text = capture[0].to_string();

                        // Calculate position information for the matched part (simplified implementation)
                        let bounding_box = BoundingBox {
                            x: line.x,
                            y: line.y,
                            width: line.width,
                            height: line.height,
                        };

                        // Calculate score based on distance from keyword
                        let distance = (i as isize - j as isize).abs() as f32;
                        let credential_score = 0.9 - (distance * 0.1).min(0.4);

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
        }

        detections
    }
}
