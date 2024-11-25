use anyhow::Result;
use axum::{
    extract::{Multipart, Query, State},
    http::StatusCode,
    response::Json,
    routing::post,
    Router,
};
use bytes::Bytes;
use eyeris::{processor::ImageProcessor, prompts::PromptFormat, providers::AIProvider};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::sync::OnceLock;
use std::time::Instant;
use tokio::sync::Semaphore;
use eyeris::providers::TokenUsage;

// Create a static processor pool
static PROCESSOR_POOL: OnceLock<Arc<ImageProcessor>> = OnceLock::new();

#[derive(Clone)]
struct AppState {
    semaphore: Arc<Semaphore>, // Rate limiting
}

#[derive(Serialize)]
struct ProcessResponse {
    success: bool,
    message: String,
    data: Option<ProcessedData>,
}

#[derive(Serialize)]
struct ProcessedData {
    analysis: String,
    token_usage: TokenUsage,
}

#[derive(Deserialize)]
struct ProcessOptions {
    #[serde(default = "default_provider")]
    provider: String,
    #[serde(default = "default_model")]
    model: String,
    #[serde(default)]
    format: PromptFormat,
}

fn default_provider() -> String {
    "ollama".to_string()
}

fn default_model() -> String {
    "moondream".to_string()
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    // Create app state with just the semaphore
    let state = AppState {
        semaphore: Arc::new(Semaphore::new(10)),
    };

    // Build router
    let app = Router::new()
        .route("/process", post(process_image))
        .with_state(state);

    // Start server
    let addr = "0.0.0.0:3000";
    tracing::info!("Starting server on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn process_image(
    State(state): State<AppState>,
    Query(options): Query<ProcessOptions>,
    mut multipart: Multipart,
) -> Result<Json<ProcessResponse>, StatusCode> {
    let start = Instant::now();

    // Get or initialize the processor pool
    let processor = PROCESSOR_POOL.get_or_init(|| {
        let init_start = Instant::now();
        let processor = Arc::new(ImageProcessor::new(
            match options.provider.to_lowercase().as_str() {
                "openai" => AIProvider::OpenAI,
                "ollama" => AIProvider::Ollama,
                _ => AIProvider::OpenAI,
            },
            Some(options.model),
            Some(options.format),
        ));
        tracing::info!(
            duration_ms = init_start.elapsed().as_millis(),
            "Processor pool initialized"
        );
        processor
    });

    let permit_start = Instant::now();
    let _permit = state
        .semaphore
        .acquire()
        .await
        .map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?;
    tracing::info!(
        duration_ms = permit_start.elapsed().as_millis(),
        "Rate limit permit acquired"
    );

    let multipart_start = Instant::now();
    let image_data: Bytes = if let Some(field) = multipart
        .next_field()
        .await
        .map_err(|_| StatusCode::BAD_REQUEST)?
    {
        if field.name().unwrap_or("") != "image" {
            return Ok(Json(ProcessResponse {
                success: false,
                message: "No image field found".to_string(),
                data: None,
            }));
        }
        field.bytes().await.map_err(|_| StatusCode::BAD_REQUEST)?
    } else {
        return Ok(Json(ProcessResponse {
            success: false,
            message: "No image provided".to_string(),
            data: None,
        }));
    };
    tracing::info!(
        duration_ms = multipart_start.elapsed().as_millis(),
        bytes = image_data.len(),
        "Multipart image extraction completed"
    );

    let process_start = Instant::now();
    let result = processor.process(&image_data).await;
    tracing::info!(
        duration_ms = process_start.elapsed().as_millis(),
        "Image processing completed"
    );

    tracing::info!(
        total_duration_ms = start.elapsed().as_millis(),
        "Total request handling completed"
    );

    match result {
        Ok((analysis, token_usage)) => {
            Ok(Json(ProcessResponse {
                success: true,
                message: "Image processed successfully".to_string(),
                data: Some(ProcessedData {
                    analysis,
                    token_usage,
                }),
            }))
        }
        Err(e) => Ok(Json(ProcessResponse {
            success: false,
            message: format!("Processing failed: {}", e),
            data: None,
        })),
    }
}
