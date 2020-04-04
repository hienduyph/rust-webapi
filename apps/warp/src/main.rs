use std::sync::Arc;
use warp::Filter;

mod controllers;
mod error;

fn routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    // construct di
    let user_component = rwebapi_container::UserContainer::new();
    let user_service = Arc::new(user_component.user_service);

    let health = warp::path!("health")
        .and(warp::get())
        .and_then(controllers::health);
    let users = warp::path!("users")
        .and(warp::get())
        .and(warp::any().map(move || user_service.clone()))
        .and_then(controllers::get_user);
    health.or(users)
}

#[tokio::main]
async fn main() {
    let routes = routes();
    let port = 8000;
    println!("Listenting {}", port);
    warp::serve(routes).run(([0, 0, 0, 0], port)).await;
}
