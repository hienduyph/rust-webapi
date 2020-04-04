use crate::error::ApiError;
use rwebapi_users::UserService;
use std::sync::Arc;
use warp::reject;

pub async fn get_user(
    user_services: Arc<Box<dyn UserService>>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let users = user_services
        .as_ref()
        .users()
        .await
        .map_err(|e| reject::custom(ApiError::from(e)))?;
    Ok(warp::reply::json(&users))
}
