#[tokio::main]
async fn main() {
    let routes = rwebapi::apps::warp::routes().await;
    let port = 8000;
    println!("Listenting {}", port);
    warp::serve(routes).run(([0, 0, 0, 0], port)).await;
}
