//! Example of batch processing multiple images

use ocr_credential_detector::{CredentialDetection, CredentialDetector};
use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get image directory path from command line arguments
    let args: Vec<String> = env::args().collect();
    let dir_path = if args.len() > 1 {
        &args[1]
    } else {
        eprintln!("Usage: cargo run --example batch_processing <image directory path>");
        return Ok(());
    };

    println!("Image directory: {}", dir_path);

    // Get image files in the directory
    let image_paths = get_image_files(dir_path)?;

    if image_paths.is_empty() {
        println!("No image files found in the directory.");
        return Ok(());
    }

    println!("Number of image files to process: {}", image_paths.len());

    // Initialize CredentialDetector
    let detector = CredentialDetector::new()?;

    // Process each image
    let mut all_detections: HashMap<PathBuf, Vec<CredentialDetection>> = HashMap::new();
    let mut total_detections = 0;

    for path in &image_paths {
        println!("Processing: {}", path.display());

        match detector.detect_from_path(path) {
            Ok(detections) => {
                if !detections.is_empty() {
                    println!("  Number of detected credentials: {}", detections.len());
                    total_detections += detections.len();
                    all_detections.insert(path.clone(), detections);
                } else {
                    println!("  No credentials were detected.");
                }
            }
            Err(e) => {
                eprintln!("  Error: {}", e);
            }
        }
    }

    // Display summary of results
    println!("\n===== Processing Results =====");
    println!("Number of processed image files: {}", image_paths.len());
    println!(
        "Number of files with detected credentials: {}",
        all_detections.len()
    );
    println!("Total number of detected credentials: {}", total_detections);

    // Display details of files with detected credentials
    if !all_detections.is_empty() {
        println!("\n===== Detected Credentials =====");

        for (path, detections) in &all_detections {
            println!("\nFile: {}", path.display());

            for (i, detection) in detections.iter().enumerate() {
                println!("  Detection #{}", i + 1);
                println!("    Text: {}", detection.text);
                println!("    Detector: {}", detection.detector_name);
                println!("    Credential score: {:.2}", detection.credential_score);
            }
        }
    }

    Ok(())
}

/// Get paths of image files in a directory
fn get_image_files(dir_path: &str) -> Result<Vec<PathBuf>, Box<dyn std::error::Error>> {
    let mut image_paths = Vec::new();

    let entries = fs::read_dir(dir_path)?;

    for entry in entries {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            if let Some(extension) = path.extension() {
                let ext = extension.to_string_lossy().to_lowercase();

                // Common image file formats
                if ["jpg", "jpeg", "png", "gif", "bmp", "tiff", "webp"].contains(&ext.as_str()) {
                    image_paths.push(path);
                }
            }
        }
    }

    Ok(image_paths)
}
