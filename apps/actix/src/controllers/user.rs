use actix_web::{web, Responder};
use rwebapi_users;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CreateUserRequest {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
}

pub async fn create_user(params: web::Json<CreateUserRequest>) -> impl Responder {
    params
}

pub async fn get_user(
    user_services: web::Data<Box<&dyn rwebapi_users::UserService>>,
) -> Result<web::Json<Vec<rwebapi_users::User>>, crate::error::ApiError> {
    let users = user_services.get_ref().users()?;
    Ok(web::Json(users))
}
