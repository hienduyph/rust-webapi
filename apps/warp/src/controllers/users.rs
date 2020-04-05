use std::sync::Arc;

use warp::reject;

use rwebapi_core::QueryParamsImpl;
use rwebapi_users::UserService;

use crate::error::ApiError;

pub async fn get_user(
    user_services: Arc<Box<dyn UserService>>,
    params: QueryParamsImpl,
) -> Result<impl warp::Reply, warp::Rejection> {
    let users = user_services
        .as_ref()
        .users(&params)
        .await
        .map_err(|e| reject::custom(ApiError::from(e)))?;
    Ok(warp::reply::json(&users))
}
