use std::sync::Arc;

use warp::reject;

use rwebapi_core::QueryParams;
use rwebapi_users::UserService;

use crate::error::ApiError;

pub async fn get_user(
    user_services: Arc<Box<dyn UserService>>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let params = QueryParams { size: 0, page: 0 };
    let users = user_services
        .as_ref()
        .users(&params)
        .await
        .map_err(|e| reject::custom(ApiError::from(e)))?;
    Ok(warp::reply::json(&users))
}
