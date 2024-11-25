use crate::errors::ProcessorError;
use crate::prompts::{ImagePrompt, PromptFormat};
use crate::providers::{AIProvider, OllamaProvider, OpenAIProvider, Provider};
use base64::Engine;
use image::codecs::jpeg::JpegEncoder;
use image::imageops::FilterType;
use image::{DynamicImage, ImageBuffer};
use rayon::prelude::*;
use std::time::Instant;

#[derive(Debug, Clone, Default)]
pub struct TokenStats {
    pub prompt_tokens: usize,
    pub completion_tokens: usize,
    pub total_tokens: usize,
}

/// The main processor for analyzing images.
pub struct ImageProcessor {
    provider: Box<dyn Provider>,
    format: PromptFormat,
    pub token_stats: std::sync::Arc<parking_lot::RwLock<TokenStats>>,
}

impl ImageProcessor {
    /// Creates a new image processor with the specified AI provider.
    ///
    /// # Arguments
    ///
    /// * `provider` - The AI provider to use (OpenAI or Ollama)
    /// * `model` - Optional model name to use
    /// * `format` - Optional output format specification
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use eyeris::{ImageProcessor, AIProvider};
    ///
    /// let processor = ImageProcessor::new(
    ///     AIProvider::Ollama,
    ///     Some("moondream".to_string()),
    ///     None
    /// );
    /// ```
    pub fn new(provider: AIProvider, model: Option<String>, format: Option<PromptFormat>) -> Self {
        let token_stats = std::sync::Arc::new(parking_lot::RwLock::new(TokenStats::default()));

        let provider: Box<dyn Provider> = match provider {
            AIProvider::OpenAI => Box::new(OpenAIProvider::new(model, token_stats.clone())),
            AIProvider::Ollama => Box::new(OllamaProvider::new(model)),
        };

        Self {
            provider,
            format: format.unwrap_or_default(),
            token_stats,
        }
    }

    pub async fn process(&self, image_data: &[u8]) -> Result<String, ProcessorError> {
        let start = Instant::now();

        // Pre-allocate the base64 string
        let base64_start = Instant::now();
        let base64_image = base64::engine::general_purpose::STANDARD.encode(image_data);
        tracing::info!(
            duration_ms = base64_start.elapsed().as_millis(),
            bytes = image_data.len(),
            "Base64 encoding completed"
        );

        // Run analysis and thumbnail generation in parallel
        let parallel_start = Instant::now();
        let analysis_future = self.analyze_image(&base64_image);
        let thumbnail_future = async {
            let img = image::load_from_memory(image_data)?;
            self.create_thumbnail(img).await
        };

        let (analysis, _thumbnail_result) = tokio::join!(analysis_future, thumbnail_future);

        tracing::info!(
            duration_ms = parallel_start.elapsed().as_millis(),
            "Parallel processing completed"
        );

        tracing::info!(
            total_duration_ms = start.elapsed().as_millis(),
            "Total image processing completed"
        );

        analysis
    }

    async fn analyze_image(&self, base64_image: &str) -> Result<String, ProcessorError> {
        let start = Instant::now();

        // Get the length before moving the string
        let original_size = base64_image.len();

        // Clone the string before moving into spawn_blocking
        let base64_image_owned = base64_image.to_string();
        let optimized_image = tokio::task::spawn_blocking(move || {
            // Use base64_image_owned instead of base64_image
            let image_data = base64::engine::general_purpose::STANDARD
                .decode(&base64_image_owned)
                .map_err(|e| ProcessorError::Base64Error(e.to_string()))?;

            // Load image
            let img = image::load_from_memory(&image_data)?;

            // Resize to smaller dimensions while maintaining aspect ratio
            // Most vision models don't need images larger than 768px
            let max_dim = 768;
            let resized = if img.width() > max_dim || img.height() > max_dim {
                let ratio = max_dim as f32 / img.width().max(img.height()) as f32;
                let new_width = (img.width() as f32 * ratio) as u32;
                let new_height = (img.height() as f32 * ratio) as u32;
                img.resize(new_width, new_height, FilterType::Lanczos3)
            } else {
                img
            };

            // Convert to RGB and compress as JPEG with moderate quality
            let mut buffer = Vec::new();
            let mut encoder = JpegEncoder::new_with_quality(&mut buffer, 10);
            encoder
                .encode(
                    resized.to_rgb8().as_raw(),
                    resized.width(),
                    resized.height(),
                    image::ColorType::Rgb8,
                )
                .map_err(|e| ProcessorError::ImageLoadError(e))?;

            // Convert back to base64
            Ok::<_, ProcessorError>(base64::engine::general_purpose::STANDARD.encode(&buffer))
        })
        .await
        .map_err(|e| ProcessorError::ThumbnailError(e.to_string()))??;

        tracing::info!(
            original_size = original_size,
            optimized_size = optimized_image.len(),
            reduction_percent =
                ((original_size - optimized_image.len()) as f32 / original_size as f32 * 100.0),
            "Image optimization completed"
        );

        // Create the prompt before calling the provider
        let prompt = ImagePrompt::new(self.format.clone());

        let result = self.provider.analyze(&optimized_image, &prompt.text).await;

        tracing::info!(
            total_duration_ms = start.elapsed().as_millis(),
            "Total analysis completed"
        );

        result
    }

    async fn create_thumbnail(&self, image: DynamicImage) -> Result<Vec<u8>, ProcessorError> {
        let start = Instant::now();

        let result = tokio::task::spawn_blocking(move || {
            let resize_start = Instant::now();
            let thumbnail = image.resize(300, 300, FilterType::Triangle);
            let rgb_image = thumbnail.to_rgb8();
            tracing::info!(
                duration_ms = resize_start.elapsed().as_millis(),
                "Image resize completed"
            );

            let enhance_start = Instant::now();
            let width = rgb_image.width() as usize;
            let height = rgb_image.height() as usize;

            let enhanced_pixels: Vec<_> = rgb_image
                .chunks_exact(width * 3)
                .enumerate()
                .par_bridge()
                .flat_map(|(y, row)| {
                    (0..width)
                        .map(move |x| {
                            let pixel = image::Rgb([
                                (row[x * 3] as f32 * 1.1).min(255.0) as u8,
                                (row[x * 3 + 1] as f32 * 1.1).min(255.0) as u8,
                                (row[x * 3 + 2] as f32 * 1.1).min(255.0) as u8,
                            ]);
                            (x as u32, y as u32, pixel)
                        })
                        .collect::<Vec<_>>()
                })
                .collect();

            tracing::info!(
                duration_ms = enhance_start.elapsed().as_millis(),
                "Parallel enhancement completed"
            );

            let buffer_start = Instant::now();
            let mut enhanced = ImageBuffer::new(width as u32, height as u32);
            for (x, y, pixel) in enhanced_pixels {
                enhanced.put_pixel(x, y, pixel);
            }

            let mut output = Vec::with_capacity(width * height * 3);
            let mut encoder = JpegEncoder::new_with_quality(&mut output, 85);
            encoder
                .encode(
                    enhanced.as_raw(),
                    enhanced.width(),
                    enhanced.height(),
                    image::ColorType::Rgb8,
                )
                .map_err(|e| ProcessorError::ThumbnailError(e.to_string()))?;

            tracing::info!(
                duration_ms = buffer_start.elapsed().as_millis(),
                "Buffer creation and JPEG encoding completed"
            );

            Ok(output)
        })
        .await
        .map_err(|e| ProcessorError::ThumbnailError(e.to_string()))?;

        tracing::info!(
            total_duration_ms = start.elapsed().as_millis(),
            "Total thumbnail creation completed"
        );

        result
    }
}
