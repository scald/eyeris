mod ollama;
mod openai;

use crate::errors::ProcessorError;
use async_trait::async_trait;
pub use ollama::OllamaProvider;
pub use openai::OpenAIProvider;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
#[derive(Default)]
pub struct TokenUsage {
    pub prompt_tokens: usize,
    pub completion_tokens: usize,
    pub total_tokens: usize,
}


#[derive(Debug, Clone, Copy)]
pub enum AIProvider {
    OpenAI,
    Ollama,
}

#[async_trait]
pub trait Provider: Send + Sync {
    async fn analyze(
        &self,
        base64_image: &str,
        prompt: &str,
    ) -> Result<(String, Option<TokenUsage>), ProcessorError>;
}
