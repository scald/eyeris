mod ollama;
mod openai;

use crate::errors::ProcessorError;
use async_trait::async_trait;
pub use ollama::OllamaProvider;
pub use openai::OpenAIProvider;

#[derive(Debug, Clone, Copy)]
pub enum AIProvider {
    OpenAI,
    Ollama,
}

#[async_trait]
pub trait Provider: Send + Sync {
    async fn analyze(&self, base64_image: &str, prompt: &str) -> Result<String, ProcessorError>;
}
