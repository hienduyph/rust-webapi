use std::sync::Arc;

use axum::{
    extract::{self, Path, State},
    Extension, Json,
};

use crate::{
    core::{QueryParamsImpl, ResultPaging},
    users::{CreateUserRequest, User, UserIdentity, UserService},
};

use super::error::ApiError;

pub async fn users_list(
    Extension(_user): Extension<UserIdentity>,
    user_service: State<Arc<dyn UserService>>,
) -> Result<Json<ResultPaging<User>>, ApiError> {
    let params = QueryParamsImpl::new();
    let users = user_service.users(&params).await?;
    Ok(Json(users))
}

pub async fn users_by_id(
    Extension(_user): Extension<UserIdentity>,
    user_service: State<Arc<dyn UserService>>,
    Path(user_id): Path<String>,
) -> Result<Json<User>, ApiError> {
    let user = user_service.find_by_id(&user_id).await?;
    Ok(Json(user))
}

pub async fn users_create(
    Extension(user): Extension<UserIdentity>,
    user_service: State<Arc<dyn UserService>>,
    extract::Json(user_create): extract::Json<CreateUserRequest>,
) -> Result<Json<User>, ApiError> {
    let user = user_create.to_user(user.user_id);
    let resp = user_service.create(&user).await?;
    Ok(Json(resp))
}
