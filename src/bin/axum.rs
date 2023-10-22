#[tokio::main]
async fn main() -> std::io::Result<()>{
    rwebapi::apps::axum::serve().await
}
