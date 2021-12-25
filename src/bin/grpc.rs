#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    rwebapi::apps::grpc::server::serve().await
}

