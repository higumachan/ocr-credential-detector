//! Wrapper for ocrs library

use crate::error::CredentialDetectorError;
use image::{self};
use ocrs::{ImageSource, OcrEngine as OcrsEngine, OcrEngineParams};
use rten::Model;
use std::path::Path;

/// Wrapper for OCR engine
pub struct OcrEngine {
    engine: OcrsEngine,
}

/// Text line information
#[derive(Debug, Clone)]
pub struct TextLine {
    /// Text content of the line
    pub text: String,
    /// X coordinate
    pub x: f32,
    /// Y coordinate
    pub y: f32,
    /// Width
    pub width: f32,
    /// Height
    pub height: f32,
    /// Detection score
    pub score: f32,
}

impl OcrEngine {
    /// Create a new OCR engine
    pub fn new() -> Result<Self, CredentialDetectorError> {
        // Specify model file paths
        let detection_model_path = Path::new("models/text-detection.rten");
        let recognition_model_path = Path::new("models/text-recognition.rten");

        // Load models
        let detection_model = match Model::load_file(detection_model_path) {
            Ok(model) => model,
            Err(e) => {
                return Err(CredentialDetectorError::OcrEngineInitError(format!(
                    "Failed to load detection model: {}",
                    e
                )));
            }
        };

        let recognition_model = match Model::load_file(recognition_model_path) {
            Ok(model) => model,
            Err(e) => {
                return Err(CredentialDetectorError::OcrEngineInitError(format!(
                    "Failed to load recognition model: {}",
                    e
                )));
            }
        };

        // Set OcrEngineParams
        let params = OcrEngineParams {
            detection_model: Some(detection_model),
            recognition_model: Some(recognition_model),
            ..Default::default()
        };

        // Initialize OcrEngine
        match OcrsEngine::new(params) {
            Ok(engine) => Ok(Self { engine }),
            Err(e) => Err(CredentialDetectorError::OcrEngineInitError(e.to_string())),
        }
    }

    /// Extract text from image
    pub fn extract_text(
        &self,
        image_data: &[u8],
    ) -> Result<Vec<TextLine>, CredentialDetectorError> {
        // Load image data
        let img = match image::load_from_memory(image_data) {
            Ok(img) => img,
            Err(e) => return Err(CredentialDetectorError::ImageLoadError(e.to_string())),
        };

        // Convert to RGB8
        let rgb_img = img.to_rgb8();

        // Create ImageSource
        let img_source = match ImageSource::from_bytes(rgb_img.as_raw(), rgb_img.dimensions()) {
            Ok(source) => source,
            Err(e) => return Err(CredentialDetectorError::ImageLoadError(e.to_string())),
        };

        // Prepare OCR input
        let ocr_input = match self.engine.prepare_input(img_source) {
            Ok(input) => input,
            Err(e) => return Err(CredentialDetectorError::OcrProcessError(e.to_string())),
        };

        // Detect word boundary boxes
        let word_rects = match self.engine.detect_words(&ocr_input) {
            Ok(rects) => rects,
            Err(e) => return Err(CredentialDetectorError::OcrProcessError(e.to_string())),
        };

        // Group words into lines
        let line_rects = self.engine.find_text_lines(&ocr_input, &word_rects);

        // Recognize text for each line
        let line_texts = match self.engine.recognize_text(&ocr_input, &line_rects) {
            Ok(texts) => texts,
            Err(e) => return Err(CredentialDetectorError::OcrProcessError(e.to_string())),
        };

        // Convert results to TextLine structs
        let mut text_lines = Vec::new();

        for line in line_texts.iter().flatten() {
            if line.to_string().len() <= 1 {
                continue; // Skip lines with 1 or fewer characters (noise removal)
            }

            // Adjust according to the actual API of ocrs::TextLine
            // In the example, line.to_string() was used, but
            // the use of bbox() and score() methods could not be confirmed.
            // Here, we attempt to access the fields directly.
            text_lines.push(TextLine {
                text: line.to_string(),
                x: 0.0,      // Adjust according to the actual API
                y: 0.0,      // Adjust according to the actual API
                width: 0.0,  // Adjust according to the actual API
                height: 0.0, // Adjust according to the actual API
                score: 1.0,  // Adjust according to the actual API
            });
        }

        Ok(text_lines)
    }
}
