use std::sync::Arc;

use axum::{
    extract::{self, Path, State},
    Extension, Json,
};
use deadpool_redis::redis::cmd;
use deadpool_redis::{redis::FromRedisValue, Pool};
use deadpool_redis::{Config, Manager, Runtime};

use crate::{
    container::UserContainer,
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
    State(redis_conn): State<Pool>,
    State(user_service): State<UserContainer>,
    Path(user_id): Path<String>,
) -> Result<Json<User>, ApiError> {
    let mut conn = redis_conn.get().await.unwrap();
    let user_cached = cmd("GET")
        .arg(&[format!("user::{}", user_id)])
        .query_async::<_, String>(&mut conn)
        .await;

    match user_cached {
        Ok(v) => {
            log::info!("Cache hit for user {:?}. Raw {:?}", user_id, v);
            Ok(Json(serde_json::from_str::<User>(&v).unwrap()))
        }
        Err(_) => {
            let user = user_service.user_service.find_by_id(&user_id).await?;
            cmd("SET")
                .arg(&[
                    format!("user::{}", user_id),
                    serde_json::to_string(&user).unwrap(),
                ])
                .query_async::<_, ()>(&mut conn)
                .await
                .unwrap();
            Ok(Json(user))
        }
    }
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
