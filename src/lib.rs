//! eyeris is a high-performance image analysis service.
//! 
//! # Features
//! 
//! - Multiple AI provider support (OpenAI, Ollama)
//! - Image optimization and processing
//! - Customizable analysis formats
//! 
//! # Example
//! 
//! ```rust,no_run
//! use eyeris::{ImageProcessor, AIProvider};
//! 
//! #[tokio::main]
//! async fn main() {
//!     let processor = ImageProcessor::new(
//!         AIProvider::Ollama,
//!         Some("moondream".to_string()),
//!         None
//!     );
//!     
//!     // Process an image
//!     let image_data = std::fs::read("image.jpg").unwrap();
//!     let analysis = processor.process(&image_data).await.unwrap();
//!     println!("Analysis: {}", analysis);
//! }
//! ```

pub mod processor;
pub mod prompts;
pub mod providers;
pub mod errors;
pub mod utils;

// Re-export commonly used types
pub use processor::ImageProcessor;
pub use prompts::{ImagePrompt, PromptFormat};
pub use errors::ProcessorError;
pub use providers::AIProvider; 