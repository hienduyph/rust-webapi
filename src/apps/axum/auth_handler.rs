use std::sync::Arc;

use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};

use crate::users::{User, UserAuthService};

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    email: String,
    password: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    token: String,
    user: User,
}

pub async fn login(
    auth_service: State<Arc<dyn UserAuthService>>,
    Json(data): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, super::error::ApiError> {
    let (user, token) = auth_service.login(&data.email, &data.password).await?;
    let resp = LoginResponse { user, token };
    Ok(Json(resp))
}
