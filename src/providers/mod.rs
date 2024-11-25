mod openai;
mod ollama;

pub use openai::OpenAIProvider;
pub use ollama::OllamaProvider;
use async_trait::async_trait;
use crate::errors::ProcessorError;

#[derive(Debug, Clone, Copy)]
pub enum AIProvider {
    OpenAI,
    Ollama,
}

#[async_trait]
pub trait Provider: Send + Sync {
    async fn analyze(&self, base64_image: &str, prompt: &str) -> Result<String, ProcessorError>;
} 