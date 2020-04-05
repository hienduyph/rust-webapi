use actix_web::{web, Responder};
use serde::{Deserialize, Serialize};

use rwebapi_core::{QueryParamsImpl, ResultPaging};
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
    params: web::Query<QueryParamsImpl>,
) -> Result<web::Json<ResultPaging<User>>, crate::error::ApiError> {
    let users = user_services.get_ref().users(&params.into_inner()).await?;
    Ok(web::Json(users))
}
