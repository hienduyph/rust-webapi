use actix_web::{web, HttpResponse};
use chrono::Utc;

use crate::core::{QueryParamsImpl, ResultPaging};
use crate::users::{
    CreateUserRequest, UpdateUserRequest, User, UserIdentity, UserService, UserUpdate,
};

use super::error::ApiError;

pub async fn create_user(
    user_services: web::Data<dyn UserService>,
    params: web::Json<CreateUserRequest>,
    identity: UserIdentity,
) -> Result<web::Json<User>, ApiError> {
    let user = params.to_user(identity.user_id);
    let created_user = user_services.create(&user).await?;
    Ok(web::Json(created_user))
}

pub async fn get_user(
    user_services: web::Data<dyn UserService>,
    params: web::Query<QueryParamsImpl>,
) -> Result<web::Json<ResultPaging<User>>, ApiError> {
    let users = user_services.users(&params.into_inner()).await?;
    Ok(web::Json(users))
}

pub async fn get_user_by_id(
    user_services: web::Data<dyn UserService>,
    user_id: web::Path<String>,
) -> Result<web::Json<User>, ApiError> {
    let user = user_services.find_by_id(&user_id).await?;
    Ok(web::Json(user))
}

pub async fn update_user(
    user_services: web::Data<dyn UserService>,
    user_id: web::Path<String>,
    params: web::Json<UpdateUserRequest>,
    identity: UserIdentity,
) -> Result<web::Json<User>, ApiError> {
    let now = Utc::now().naive_utc();
    let updator_user = UserUpdate {
        email: params.email.to_string(),
        first_name: params.first_name.to_string(),
        last_name: params.last_name.to_string(),
        updated_at: now,
        updated_by: identity.user_id.clone(),
    };
    println!("Update user {:?}", updator_user);
    let user = user_services.update_by_id(&user_id, &updator_user).await?;
    Ok(web::Json(user))
}

pub async fn delete_user(
    user_services: web::Data<dyn UserService>,
    user_id: web::Path<String>,
) -> Result<HttpResponse, ApiError> {
    user_services.delete_by_id(&user_id).await?;
    Ok(HttpResponse::NoContent().finish())
}
