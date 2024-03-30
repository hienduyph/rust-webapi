use structured_logger::{async_json::new_writer, Builder};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Initialize the logger.
    Builder::with_level("info")
        .with_target_writer("*", new_writer(tokio::io::stdout()))
        .init();
    rwebapi::apps::axum::serve().await
}
