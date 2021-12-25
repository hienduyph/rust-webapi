use std::sync::Arc;

use warp::reject;

use crate::core::QueryParamsImpl;
use crate::users::UserService;

use super::error::ApiError;

pub async fn get_user(
    user_services: Arc<dyn UserService>,
    params: QueryParamsImpl,
) -> Result<impl warp::Reply, warp::Rejection> {
    let users = user_services
        .as_ref()
        .users(&params)
        .await
        .map_err(|e| reject::custom(ApiError::from(e)))?;
    Ok(warp::reply::json(&users))
}

