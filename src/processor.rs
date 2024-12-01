use crate::{
    errors::ProcessorError,
    prompts::{ ImagePrompt, PromptFormat },
    providers::{ AIProvider, Provider, TokenUsage },
    utils::enhance_image,
};
use base64::Engine;
use image::{ DynamicImage, ImageFormat };
use std::time::Instant;
use tracing::{ info, debug, error };

pub struct ImageProcessor {
    provider: Box<dyn Provider>,
    prompt_format: PromptFormat,
}

impl ImageProcessor {
    pub fn new(provider: AIProvider, model: Option<String>, format: Option<PromptFormat>) -> Self {
        let provider: Box<dyn Provider> = match provider {
            AIProvider::OpenAI => Box::new(crate::providers::OpenAIProvider::new(model)),
            AIProvider::Ollama => Box::new(crate::providers::OllamaProvider::new(model)),
        };

        Self {
            provider,
            prompt_format: format.unwrap_or_default(),
        }
    }

    pub async fn process(&self, image_data: &[u8]) -> Result<(String, TokenUsage), ProcessorError> {
        let start = Instant::now();
        debug!("Starting image processing with {} bytes", image_data.len());

        // Try to determine image format
        let format = image::guess_format(image_data).map_err(|e| {
            error!("Failed to guess image format: {}", e);
            ProcessorError::ImageError(format!("Failed to determine image format: {}", e))
        })?;
        debug!("Detected image format: {:?}", format);

        // Load image
        let img = image::load_from_memory_with_format(image_data, format).map_err(|e| {
            error!("Failed to load image: {} (data size: {})", e, image_data.len());
            ProcessorError::ImageError(
                format!("Failed to load image (size: {}): {}", image_data.len(), e)
            )
        })?;
        debug!("Successfully loaded image: {}x{}", img.width(), img.height());

        // Process image
        let enhanced = enhance_image(&img)?;
        debug!("Image enhancement complete");

        // Convert to base64
        let mut base64_data = String::with_capacity((image_data.len() * 4) / 3 + 4);
        base64::engine::general_purpose::STANDARD.encode_string(image_data, &mut base64_data);
        info!(
            "Base64 encoding completed, duration_ms: {}, bytes: {}",
            start.elapsed().as_millis(),
            image_data.len()
        );

        // Create prompt
        let prompt = ImagePrompt::new(self.prompt_format.clone()).to_string();
        debug!("Using prompt format: {:?}", self.prompt_format);

        // Analyze with AI provider
        let (analysis, token_usage) = self.provider.analyze(&base64_data, &prompt).await?;
        info!(
            "Total image processing completed, total_duration_ms: {}",
            start.elapsed().as_millis()
        );

        Ok((analysis, token_usage.unwrap_or_default()))
    }
}
