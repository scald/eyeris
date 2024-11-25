use thiserror::Error;

#[derive(Error, Debug)]
pub enum ProcessorError {
    #[error("Failed to load image: {0}")]
    ImageLoadError(#[from] image::error::ImageError),
    
    #[error("AI Provider error: {0}")]
    AIProviderError(String),
    
    #[error("Failed to encode/decode base64: {0}")]
    Base64Error(String),
    
    #[error("Environment variable error: {0}")]
    EnvError(#[from] std::env::VarError),
    
    #[error("Network request failed: {0}")]
    RequestError(#[from] reqwest::Error),
    
    #[error("Invalid API response: {0}")]
    ResponseParseError(String),
    
    #[error("Thumbnail generation failed: {0}")]
    ThumbnailError(String),
} 