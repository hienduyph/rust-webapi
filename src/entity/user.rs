use chrono::{NaiveDateTime, Utc};

use super::repo::RepoResult;
use crate::config::Name;

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub struct NewUser {
    pub id: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub created_by: String,
    pub updated_by: String,
}

#[derive(Debug, Clone)]
pub struct UpdateUser {
    pub id: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub updated_by: String,
}

pub trait UserRepo {
    fn get_all() -> RepoResult<Vec<User>>;

    fn find(user_id: uuid::Uuid) -> RepoResult<User>;

    fn find_by_auth(email: &str, password: &str) -> RepoResult<User>;

    fn create(user: &User) -> RepoResult<User>;

    fn update(update_user: &UpdateUser) -> RepoResult<User>;

    fn delete(user_id: uuid::Uuid) -> RepoResult<()>;
}

impl From<NewUser> for User {
    fn from(user: NewUser) -> Self {
        let now = Utc::now().naive_utc();
        User {
            id: user.id,
            first_name: user.first_name,
            last_name: user.last_name,
            email: user.email,
            // TODO: hash funtion
            password: "".to_owned(),
            created_by: user.created_by,
            created_at: now,
            updated_by: user.updated_by,
            updated_at: now,
        }
    }
}
