use super::Provider;
use crate::errors::ProcessorError;
use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
struct OllamaRequest {
    model: String,
    prompt: String,
    images: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct OllamaResponse {
    response: String,
}

pub struct OllamaProvider {
    client: Client,
    model: String,
}

impl OllamaProvider {
    pub fn new(model: Option<String>) -> Self {
        Self {
            client: Client::new(),
            model: model.unwrap_or_else(|| "moondream".to_string()),
        }
    }
}

#[async_trait]
impl Provider for OllamaProvider {
    async fn analyze(&self, base64_image: &str, prompt: &str) -> Result<String, ProcessorError> {
        let ollama_request = OllamaRequest {
            model: self.model.clone(),
            prompt: prompt.to_string(),
            images: vec![base64_image.to_string()],
        };

        let response = self
            .client
            .post("http://localhost:11434/api/generate")
            .json(&ollama_request)
            .send()
            .await?;

        let status = response.status();
        if !status.is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Failed to get error message".to_string());
            return Err(ProcessorError::AIProviderError(format!(
                "Ollama API request failed with status {}: {}",
                status, error_text
            )));
        }

        // Ollama returns streaming responses, so we need to collect all response chunks
        let text = response.text().await?;

        // Parse each line as a separate JSON response
        let mut full_response = String::new();
        for line in text.lines() {
            if let Ok(chunk) = serde_json::from_str::<OllamaResponse>(line) {
                full_response.push_str(&chunk.response);
            }
        }

        if full_response.is_empty() {
            return Err(ProcessorError::ResponseParseError(
                "Empty response from Ollama".to_string(),
            ));
        }

        Ok(full_response)
    }
}
