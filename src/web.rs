use axum::{ extract::Multipart, response::{ Html, Json }, routing::{ get, post }, Router };
use serde::Serialize;
use std::net::SocketAddr;
use tokio::fs;
use tower_http::services::ServeDir;

use eyeris::{ AIProvider, ImageProcessor, TokenUsage };
use tokio::net::TcpListener;

#[derive(Serialize)]
struct AnalysisResponse {
    analysis: String,
    token_usage: Option<TokenUsage>,
}

pub async fn run_server() {
    let assets_path = std::env::current_dir().unwrap();

    let app = Router::new()
        .route("/", get(serve_index))
        .route("/analyze", post(handle_upload))
        .nest_service("/assets", ServeDir::new(assets_path));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server running on http://{}", addr);

    axum::serve(TcpListener::bind(addr).await.unwrap(), app.into_make_service()).await.unwrap();
}

async fn serve_index() -> Html<String> {
    let index_html = fs
        ::read_to_string("index.html").await
        .unwrap_or_else(|_| "Failed to load index.html".to_string());
    Html(index_html)
}

async fn handle_upload(mut multipart: Multipart) -> Json<AnalysisResponse> {
    let processor = ImageProcessor::new(AIProvider::OpenAI, None, None);

    while let Some(field) = multipart.next_field().await.unwrap() {
        if field.name().unwrap() == "image" {
            let data = field.bytes().await.unwrap();

            let (analysis, token_usage) = processor.process(&data).await.unwrap();

            return Json(AnalysisResponse {
                analysis,
                token_usage: Some(token_usage),
            });
        }
    }

    Json(AnalysisResponse {
        analysis: "No image found in upload".to_string(),
        token_usage: None,
    })
}
