use async_trait::async_trait;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::core::{QueryParams, RepoResult, ResultPaging};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    #[serde(skip_serializing)]
    pub password: String,
    pub created_by: String,
    pub created_at: NaiveDateTime,
    pub updated_by: String,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone)]
pub struct UserUpdate {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub updated_by: String,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenPayload {
    pub email: String,
    pub exp: i64,
}

#[async_trait]
pub trait UserRepo: Send + Sync {
    async fn get_all(&self, params: &dyn QueryParams) -> RepoResult<ResultPaging<User>>;

    async fn find(&self, user_id: &str) -> RepoResult<User>;

    async fn find_by_email(&self, email: &str) -> RepoResult<User>;

    async fn create(&self, user: &User) -> RepoResult<User>;

    async fn update(&self, id: &str, update_user: &UserUpdate) -> RepoResult<User>;

    async fn delete(&self, user_id: &str) -> RepoResult<()>;
}
