use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use super::repo::RepoResult;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
    pub created_by: String,
    pub created_at: NaiveDateTime,
    pub updated_by: String,
    pub updated_at: NaiveDateTime,
}

pub trait UserRepo: Send + Sync {
    fn get_all(&self) -> RepoResult<Vec<User>>;

    fn find(&self, user_id: uuid::Uuid) -> RepoResult<User>;

    fn find_by_auth(&self, email: &str, password: &str) -> RepoResult<User>;

    fn create(&self, user: &User) -> RepoResult<User>;

    fn update(&self, update_user: &User) -> RepoResult<User>;

    fn delete(&self, user_id: uuid::Uuid) -> RepoResult<()>;
}
