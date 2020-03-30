use chrono::{NaiveDateTime, Utc};

use super::repo::RepoResult;
use crate::repo::schema::users;

#[derive(Queryable, Debug, Clone, Insertable)]
#[table_name = "users"]
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

#[derive(Debug, Clone, AsChangeset)]
#[table_name = "users"]
pub struct UpdateUser {
    pub id: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub updated_by: String,
}

pub trait UserRepo {
    fn get_all(&self) -> RepoResult<Vec<User>>;

    fn find(&self, user_id: uuid::Uuid) -> RepoResult<User>;

    fn find_by_auth(&self, email: &str, password: &str) -> RepoResult<User>;

    fn create(&self, user: &User) -> RepoResult<User>;

    fn update(&self, update_user: &UpdateUser) -> RepoResult<User>;

    fn delete(&self, user_id: uuid::Uuid) -> RepoResult<()>;
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
