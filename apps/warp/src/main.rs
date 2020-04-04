use std::sync::Arc;
use warp::Filter;

mod controllers;
mod error;

fn routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    // construct di
    let svc: Box<&dyn rwebapi_users::UserService> = Box::new(&rwebapi_users::UserServiceImpl {});
    let user_services = Arc::new(svc);
    let health = warp::path!("health")
        .and(warp::get())
        .and_then(controllers::health);
    let users = warp::path!("users")
        .and(warp::get())
        .and(warp::any().map(move || user_services.clone()))
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
