mod web;

#[tokio::main]
async fn main() {
    web::run_server().await;
}
