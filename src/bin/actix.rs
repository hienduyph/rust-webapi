#[actix_web::main]
async fn main() -> std::io::Result<()> {
    rwebapi::apps::actix::server::serve().await
}
