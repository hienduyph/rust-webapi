use actix_web::{web, Responder};
use serde::{Deserialize, Serialize};

use rwebapi_core::{QueryParams, ResultPaging};
use rwebapi_users::{User, UserService};

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
    user_services: web::Data<Box<dyn UserService>>,
) -> Result<web::Json<ResultPaging<User>>, crate::error::ApiError> {
    let params = QueryParams { size: 0, page: 0 };
    let users = user_services.get_ref().users(&params).await?;
    Ok(web::Json(users))
}
