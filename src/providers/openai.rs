use super::{ Provider, TokenUsage };
use crate::errors::ProcessorError;
use async_trait::async_trait;
use reqwest::Client;
use serde::Deserialize;
use serde_json::json;

#[derive(Debug, Deserialize)]
struct OpenAIResponse {
    choices: Vec<OpenAIChoice>,
    usage: Option<OpenAIUsage>,
}

#[derive(Debug, Deserialize)]
struct OpenAIChoice {
    message: OpenAIMessage,
}

#[derive(Debug, Deserialize)]
struct OpenAIMessage {
    content: String,
}

#[derive(Debug, Deserialize)]
struct OpenAIUsage {
    prompt_tokens: usize,
    completion_tokens: usize,
    total_tokens: usize,
}

pub struct OpenAIProvider {
    client: Client,
    model: String,
    temperature: f32,
}

impl OpenAIProvider {
    pub fn new(model: Option<String>) -> Self {
        Self {
            client: Client::new(),
            model: model.unwrap_or_else(|| "gpt-4o".to_string()),
            temperature: 0.0,
        }
    }
}

#[async_trait]
impl Provider for OpenAIProvider {
    async fn analyze(
        &self,
        base64_image: &str,
        prompt: &str
    ) -> Result<(String, Option<TokenUsage>), ProcessorError> {
        let api_key = std::env::var("OPENAI_API_KEY").map_err(ProcessorError::EnvError)?;

        let system_prompt =
            "You are a detailed image analysis system. When analyzing images, please provide a complete and thorough analysis in a structured JSON format. Include all visible text, elements, and details. Never truncate or summarize the content - provide everything you can see in the image. If the content is long, break it into appropriate sections but ensure ALL content is captured.";

        let request_body =
            json!({
            "model": self.model,
            "temperature": self.temperature,
            "max_completion_tokens": 16384,
            "messages": [
                {
                    "role": "system",
                    "content": system_prompt
                },
                {
                    "role": "user",
                    "content": [
                        {
                            "type": "text",
                            "text": format!("{}\nPlease analyze this image completely and provide ALL visible content in a structured JSON format. Do not omit or summarize any text or elements.", prompt)
                        },
                        {
                            "type": "image_url",
                            "image_url": {
                                "url": format!("data:image/jpeg;base64,{}", base64_image)
                            }
                        }
                    ]
                }
            ]
        });

        let response = self.client
            .post("https://api.openai.com/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", api_key))
            .json(&request_body)
            .send().await?;

        let status = response.status();
        if !status.is_success() {
            let error_text = response
                .text().await
                .unwrap_or_else(|_| "Failed to get error message".to_string());
            return Err(
                ProcessorError::AIProviderError(
                    format!("OpenAI API request failed with status {}: {}", status, error_text)
                )
            );
        }

        let response_text = response.text().await?;
        let response: OpenAIResponse = serde_json
            ::from_str(&response_text)
            .map_err(|e| {
                ProcessorError::ResponseParseError(
                    format!(
                        "Failed to parse OpenAI response: {}. Response text: {}",
                        e,
                        response_text
                    )
                )
            })?;

        let analysis = response.choices
            .first()
            .ok_or_else(|| {
                ProcessorError::ResponseParseError("No choices in response".to_string())
            })?
            .message.content.clone();

        let token_usage = response.usage.map(|usage| TokenUsage {
            prompt_tokens: usage.prompt_tokens,
            completion_tokens: usage.completion_tokens,
            total_tokens: usage.total_tokens,
        });

        Ok((analysis, token_usage))
    }
}
