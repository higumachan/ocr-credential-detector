//! Example of credential detection using a custom detector

use ocr_credential_detector::{
    BoundingBox, CredentialDetection, CredentialDetector, CredentialDetectorLogic, TextLine,
};
use regex::Regex;
use std::env;
use std::path::Path;

/// Credit card number detector
struct CreditCardDetector {
    pattern: Regex,
}

impl CreditCardDetector {
    fn new() -> Self {
        // Credit card number pattern (16 digits, possibly separated by spaces or hyphens every 4 digits)
        let pattern = Regex::new(r"(?:\d{4}[-\s]?){3}\d{4}").unwrap();
        CreditCardDetector { pattern }
    }
}

impl CredentialDetectorLogic for CreditCardDetector {
    fn name(&self) -> &str {
        "Credit Card Number Detector"
    }

    fn detect(&self, text_lines: &[TextLine]) -> Vec<CredentialDetection> {
        let mut detections = Vec::new();

        for line in text_lines {
            for capture in self.pattern.captures_iter(&line.text) {
                let matched_text = capture[0].to_string();

                // Calculate position information for the matched part (simplified implementation)
                let bounding_box = BoundingBox {
                    x: line.x,
                    y: line.y,
                    width: line.width,
                    height: line.height,
                };

                detections.push(CredentialDetection {
                    text: matched_text,
                    bounding_box,
                    text_detection_score: line.score,
                    credential_score: 0.95, // Set high score for credit card numbers
                    detector_name: self.name().to_string(),
                });
            }
        }

        detections
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get image file path from command line arguments
    let args: Vec<String> = env::args().collect();
    let image_path = if args.len() > 1 {
        &args[1]
    } else {
        eprintln!("Usage: cargo run --example custom_detector <image file path>");
        return Ok(());
    };

    println!("Image file: {}", image_path);

    // Initialize with basic detectors
    let detector = CredentialDetector::new()?;

    // Add custom detector
    let custom_detector = Box::new(CreditCardDetector::new());
    let detector = detector.with_detector(custom_detector);

    // Detect credentials from image
    println!("Detecting credentials...");
    let detections = detector.detect_from_path(image_path)?;

    // Display detection results
    if detections.is_empty() {
        println!("No credentials were detected.");
    } else {
        println!("Number of detected credentials: {}", detections.len());

        for (i, detection) in detections.iter().enumerate() {
            println!("Detection #{}", i + 1);
            println!("  Text: {}", detection.text);
            println!("  Detector: {}", detection.detector_name);
            println!("  Credential score: {:.2}", detection.credential_score);
        }
    }

    Ok(())
}
