use crate::entity::error;
use crate::entity::user;
use crate::services::users::UserService;
use actix_web::{web, Responder};
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
    user_services: web::Data<Box<&dyn UserService>>,
) -> Result<web::Json<Vec<user::User>>, error::CommonError> {
    let users = user_services.get_ref().users()?;
    Ok(web::Json(users))
}
