//! Common type definitions

/// Struct representing text position information
#[derive(Debug, Clone)]
pub struct BoundingBox {
    /// X coordinate
    pub x: f32,
    /// Y coordinate
    pub y: f32,
    /// Width
    pub width: f32,
    /// Height
    pub height: f32,
}

/// Struct representing credential detection result
#[derive(Debug, Clone)]
pub struct CredentialDetection {
    /// Detected text
    pub text: String,
    /// Text position information
    pub bounding_box: BoundingBox,
    /// Text detection score (OCR confidence)
    pub text_detection_score: f32,
    /// Credential score (probability of being a credential)
    pub credential_score: f32,
    /// Name of the detector used for detection
    pub detector_name: String,
}
