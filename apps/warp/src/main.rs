use warp::Filter;

mod controllers;
mod error;

fn routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("health")
        .and(warp::get())
        .and_then(controllers::health)
}

#[tokio::main]
async fn main() {
    let routes = routes();
    let port = 8000;
    println!("Listenting {}", port);
    warp::serve(routes).run(([0, 0, 0, 0], port)).await;
}
