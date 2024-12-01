use axum::{
    extract::{ Multipart, Query },
    response::{ Html, Json },
    routing::{ get, post },
    Router,
    http::StatusCode,
};
use serde::{ Serialize, Deserialize };
use std::net::SocketAddr;
use tokio::fs;
use tower_http::{ services::ServeDir, cors::CorsLayer, limit::RequestBodyLimitLayer };
use tracing::{ info, warn, error, debug, Level };
use tracing_subscriber::FmtSubscriber;
use eyeris::{ AIProvider, ImageProcessor, TokenUsage };
use axum::response::IntoResponse;

#[derive(Debug, Serialize)]
struct ApiResponse<T> {
    success: bool,
    message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<T>,
}

#[derive(Debug, Serialize)]
struct AnalysisResponse {
    analysis: String,
    token_usage: Option<TokenUsage>,
}

#[derive(Debug, Deserialize)]
struct AnalysisOptions {
    #[serde(default = "default_model")]
    model: String,
}

fn default_model() -> String {
    "gpt-4o".to_string()
}

pub async fn run_server() {
    // Initialize logging first, before any other operations
    let _subscriber = FmtSubscriber::builder()
        .with_max_level(Level::DEBUG)
        .with_line_number(true)
        .with_file(true)
        .with_target(true)
        .with_thread_ids(true)
        .with_thread_names(true)
        .with_ansi(true)
        .pretty()
        .init();

    debug!("Initializing server...");

    let assets_path = std::env::current_dir().unwrap();

    // Enable CORS for API endpoints
    let cors = CorsLayer::new()
        .allow_origin(tower_http::cors::Any)
        .allow_methods(tower_http::cors::Any)
        .allow_headers(tower_http::cors::Any);

    debug!("Setting up routes...");

    let app = Router::new()
        .route("/", get(serve_index))
        .route("/api/v1/analyze", post(api_analyze))
        .route("/api/v1/health", get(health_check))
        .layer(cors)
        .layer(RequestBodyLimitLayer::new(100 * 1024 * 1024)) // 100MB
        .nest_service("/assets", ServeDir::new(assets_path));

    let ports = [
        std::env
            ::var("PORT")
            .unwrap_or_else(|_| "8080".to_string())
            .parse()
            .unwrap(),
        8081,
        8082,
        8083,
        8084,
    ];
    let mut server = None;

    for port in ports {
        let addr = SocketAddr::from(([0, 0, 0, 0], port));
        debug!("Attempting to bind to port {}", port);
        match tokio::net::TcpListener::bind(addr).await {
            Ok(listener) => {
                info!("Server running on http://{}:{}", addr.ip(), port);
                server = Some((listener, addr));
                break;
            }
            Err(e) => {
                warn!("Port {} in use ({}), trying next port...", port, e);
            }
        }
    }

    match server {
        Some((listener, addr)) => {
            info!("API documentation available at http://{}/docs", addr);
            debug!("Starting server...");
            axum::serve(listener, app.into_make_service()).await.unwrap_or_else(|e|
                error!("Server error: {}", e)
            );
        }
        None => {
            error!("Could not bind to any port in range 3000-3004");
            std::process::exit(1);
        }
    }
}

// Web interface handlers
async fn serve_index() -> Html<String> {
    let index_html = fs
        ::read_to_string("index.html").await
        .unwrap_or_else(|_| "Failed to load index.html".to_string());
    Html(index_html)
}

// API handlers
#[axum::debug_handler]
async fn api_analyze(
    Query(options): Query<AnalysisOptions>,
    multipart: Multipart
) -> impl IntoResponse {
    debug!("Received analyze request with options: {:?}", options);

    match process_image_upload(multipart, options).await {
        Ok(analysis) => {
            info!("Successfully processed image");
            (
                StatusCode::OK,
                Json(ApiResponse {
                    success: true,
                    message: "Analysis completed successfully".to_string(),
                    data: Some(analysis),
                }),
            ).into_response()
        }
        Err(e) => {
            error!("Failed to process image: {}", e);
            (
                StatusCode::BAD_REQUEST,
                Json(ApiResponse::<AnalysisResponse> {
                    success: false,
                    message: e.to_string(),
                    data: None,
                }),
            ).into_response()
        }
    }
}

async fn health_check() -> impl IntoResponse {
    Json(ApiResponse::<()> {
        success: true,
        message: "Service is healthy".to_string(),
        data: None,
    })
}

// Helper functions
async fn process_image_upload(
    mut multipart: Multipart,
    options: AnalysisOptions
) -> Result<AnalysisResponse, String> {
    debug!("Starting multipart processing");
    let processor = ImageProcessor::new(AIProvider::OpenAI, Some(options.model), None);

    let field = match multipart.next_field().await {
        Ok(Some(field)) => {
            debug!(
                "Received field: name={:?}, filename={:?}, content_type={:?}",
                field.name(),
                field.file_name(),
                field.content_type()
            );
            field
        }
        Ok(None) => {
            let msg = "No fields found in multipart form";
            error!(msg);
            return Err(msg.to_string());
        }
        Err(e) => {
            let msg = format!("Failed to read multipart field: {}", e);
            error!(msg);
            return Err(msg);
        }
    };

    if field.name().unwrap_or("") != "image" {
        let msg = format!("Expected field name 'image', got {:?}", field.name());
        error!(msg);
        return Err(msg);
    }

    debug!("Reading field bytes...");
    let data = field.bytes().await.map_err(|e| {
        let msg = format!("Failed to read field bytes: {}", e);
        error!(msg);
        msg
    })?;

    if data.is_empty() {
        let msg = "Received empty image data";
        error!(msg);
        return Err(msg.to_string());
    }

    debug!("Successfully read {} bytes of image data", data.len());

    debug!("Starting image processing with {} bytes", data.len());
    match processor.process(&data).await {
        Ok((analysis, token_usage)) => {
            info!("Successfully analyzed image. Token usage: {:?}", token_usage);
            Ok(AnalysisResponse {
                analysis,
                token_usage: Some(token_usage),
            })
        }
        Err(e) => {
            let msg = format!("Failed to process image: {}", e);
            error!(msg);
            Err(msg)
        }
    }
}
