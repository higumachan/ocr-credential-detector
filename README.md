# OCR Credential Detection Library

A Rust library for extracting text from images using OCR and detecting credentials (API keys, passwords, etc.) within that text.

## Overview

This library automatically detects sensitive information such as API keys and passwords that may be contained in screenshots or images. It uses OCR (Optical Character Recognition) technology to extract text from images and then identifies credentials using techniques such as regular expression pattern matching.

Main features:
- Text extraction from images (OCR)
- Detection of API keys (AWS, GitHub, Google, etc.)
- Password detection
- Support for adding custom detectors
- Batch processing for scanning multiple images

## Installation

Add the following to your Cargo.toml:

```toml
[dependencies]
ocr-credential-detector = "0.1.0"
```

Or run the following command:

```bash
cargo add ocr-credential-detector
```

## Usage Examples

### Basic Usage

```rust
use ocr_credential_detector::CredentialDetector;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the credential detector
    let detector = CredentialDetector::new()?;
    
    // Detect credentials from an image
    let detections = detector.detect_from_path("path/to/image.png")?;
    
    // Display detection results
    for detection in detections {
        println!("Detected credential: {}", detection.text);
        println!("Detector: {}", detection.detector_name);
        println!("Credential score: {:.2}", detection.credential_score);
    }
    
    Ok(())
}
```

### Adding Custom Detectors

You can implement your own credential detection logic:

```rust
use ocr_credential_detector::{
    BoundingBox, CredentialDetection, CredentialDetector, CredentialDetectorLogic, TextLine,
};
use regex::Regex;

// Credit card number detector
struct CreditCardDetector {
    pattern: Regex,
}

impl CreditCardDetector {
    fn new() -> Self {
        // Pattern for credit card numbers
        let pattern = Regex::new(r"(?:\d{4}[-\s]?){3}\d{4}").unwrap();
        CreditCardDetector { pattern }
    }
}

impl CredentialDetectorLogic for CreditCardDetector {
    fn name(&self) -> &str {
        "Credit Card Number Detector"
    }

    fn detect(&self, text_lines: &[TextLine]) -> Vec<CredentialDetection> {
        // Implementation...
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize with basic detectors
    let detector = CredentialDetector::new()?;
    
    // Add a custom detector
    let custom_detector = Box::new(CreditCardDetector::new());
    let detector = detector.with_detector(custom_detector);
    
    // Detect credentials from an image
    let detections = detector.detect_from_path("path/to/image.png")?;
    
    // Display detection results
    // ...
    
    Ok(())
}
```

### Batch Processing

Example of processing multiple images:

```rust
use ocr_credential_detector::CredentialDetector;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the credential detector
    let detector = CredentialDetector::new()?;
    
    // Get image files from a directory
    let image_paths = get_image_files("path/to/directory")?;
    
    // Process each image
    for path in &image_paths {
        println!("Processing: {}", path.display());
        
        match detector.detect_from_path(path) {
            Ok(detections) => {
                if !detections.is_empty() {
                    println!("  Number of credentials detected: {}", detections.len());
                    // Process detection results...
                }
            }
            Err(e) => {
                eprintln!("  Error: {}", e);
            }
        }
    }
    
    Ok(())
}
```

## Disclaimer on Current Performance

This library is currently in the Proof of Concept (PoC) stage, with a processing speed of approximately 1 fps. While this is not yet practical for real-time applications, we plan significant improvements in future development.

Our goal is to achieve 120 fps by efficiently utilizing common CPU or GPU resources. This will enable real-time credential detection and support a wider range of use cases.

The current implementation focuses on demonstrating functionality, with performance optimization planned for future development phases.

## Why Rust?

Our choice of Rust is not solely for processing speed. If speed were the only consideration, Python would be more suitable for experimenting with and improving detection models.

The main reasons for choosing Rust are:

### Excellent Portability

Rust code offers exceptional portability. It runs consistently across various platforms and devices, with easy dependency management. This allows the library to be used in diverse environments.

### Long-term Vision

In this project, we plan to maintain Rust implementation even as we advance detection model optimization (such as quantization and distillation), rather than migrating to languages like Python. This ensures consistency in the codebase and facilitates maintenance.

### Safety and Reliability

For handling sensitive processes like credential detection, Rust's memory safety and concurrency safety are significant advantages. Strict compile-time checks prevent many bugs and security issues before they occur.

### Personal Choice

Ultimately, Rust and Go were the main candidates, but Rust was chosen out of personal interest and as a technical challenge. While both languages have excellent characteristics, Rust's ecosystem and expressiveness were deemed most suitable for this project's goals.

## License

MIT