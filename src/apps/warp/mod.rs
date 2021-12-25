use warp::Filter;
use crate::core::QueryParamsImpl;

pub mod health;
pub mod users;
pub mod error;


pub fn routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    // construct di
    let user_component = crate::container::UserContainer::new();
    let user_service = user_component.user_service.clone();

    let index = warp::path::end()
        .and(warp::any())
        .and_then(self::health::health);

    let health = warp::path!("health")
        .and(warp::get())
        .and_then(self::health::health);
    let users = warp::path!("users")
        .and(warp::get())
        .and(warp::any().map(move || user_service.clone()))
        .and(warp::query::<QueryParamsImpl>())
        .and_then(self::users::get_user);
    health.or(users).or(index)
}
