use actix_web::web;
use serde::{Deserialize, Serialize};

use rwebapi_users::{User, UserAuthService};

use crate::error::ApiError;

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
    auth_service: web::Data<Box<dyn UserAuthService>>,
    data: web::Json<LoginRequest>,
) -> Result<web::Json<LoginResponse>, ApiError> {
    let (user, token) = auth_service.login(&data.email, &data.password).await?;
    let resp = LoginResponse { user, token };
    Ok(web::Json(resp))
}
