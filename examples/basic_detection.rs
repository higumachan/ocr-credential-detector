//! Basic credential detection example

use ocr_credential_detector::{CredentialDetector, OcrEngine};
use std::env;
use std::fs;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get image file path from command line arguments
    let args: Vec<String> = env::args().collect();
    let image_path = if args.len() > 1 {
        &args[1]
    } else {
        eprintln!("Usage: cargo run --example basic_detection <image file path>");
        return Ok(());
    };

    println!("Image file: {}", image_path);

    // Load image data
    let image_data = match fs::read(image_path) {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Failed to load image file: {}", e);
            return Err(Box::new(e));
        }
    };

    // Initialize OCR engine
    let ocr_engine = match OcrEngine::new() {
        Ok(engine) => engine,
        Err(e) => {
            eprintln!("Failed to initialize OCR engine: {}", e);
            return Err(Box::new(e));
        }
    };

    // Extract text from image
    println!("Extracting text...");
    let text_lines = match ocr_engine.extract_text(&image_data) {
        Ok(lines) => lines,
        Err(e) => {
            eprintln!("Failed to extract text: {}", e);
            return Err(Box::new(e));
        }
    };

    // Display extracted text
    if text_lines.is_empty() {
        println!("No text was extracted.");
    } else {
        println!("Number of extracted text lines: {}", text_lines.len());
        println!("---");
        for (i, line) in text_lines.iter().enumerate() {
            println!("Line #{}: {}", i + 1, line.text);
        }
        println!("---");
    }

    // Initialize CredentialDetector
    let detector = match CredentialDetector::new() {
        Ok(detector) => detector,
        Err(e) => {
            eprintln!("Failed to initialize credential detector: {}", e);
            return Err(Box::new(e));
        }
    };

    // Detect credentials from image
    println!("Detecting credentials...");
    let detections = match detector.detect_from_path(image_path) {
        Ok(detections) => detections,
        Err(e) => {
            eprintln!("Failed to detect credentials: {}", e);
            return Err(Box::new(e));
        }
    };

    // Display detection results
    if detections.is_empty() {
        println!("No credentials were detected.");
    } else {
        println!("Number of detected credentials: {}", detections.len());

        for (i, detection) in detections.iter().enumerate() {
            println!("Detection #{}", i + 1);
            println!("  Text: {}", detection.text);
            println!(
                "  Position: ({:.1}, {:.1}) - {:.1}x{:.1}",
                detection.bounding_box.x,
                detection.bounding_box.y,
                detection.bounding_box.width,
                detection.bounding_box.height
            );
            println!(
                "  Text detection score: {:.2}",
                detection.text_detection_score
            );
            println!("  Credential score: {:.2}", detection.credential_score);
            println!("  Detector: {}", detection.detector_name);
        }
    }

    Ok(())
}
