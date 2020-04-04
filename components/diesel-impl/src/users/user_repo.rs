use chrono::NaiveDateTime;
use diesel::prelude::*;
use std::sync::Arc;

use crate::infra;
use crate::schema::users;
use rwebapi_users::{RepoError, RepoResult, User, UserRepo};

#[derive(Queryable, Insertable, AsChangeset)]
#[table_name = "users"]
struct UserDiesel {
    id: String,
    first_name: String,
    last_name: String,
    email: String,
    password: String,
    created_by: String,
    created_at: NaiveDateTime,
    updated_by: String,
    updated_at: NaiveDateTime,
}

impl Into<User> for UserDiesel {
    fn into(self) -> User {
        User {
            id: self.id,
            first_name: self.first_name,
            last_name: self.last_name,
            email: self.email,
            password: self.password,
            created_at: self.created_at,
            created_by: self.created_by,
            updated_at: self.updated_at,
            updated_by: self.updated_by,
        }
    }
}

impl From<User> for UserDiesel {
    fn from(u: User) -> Self {
        UserDiesel {
            id: u.id,
            first_name: u.first_name,
            last_name: u.last_name,
            email: u.email,
            password: u.password,
            created_at: u.created_at,
            created_by: u.created_by,
            updated_at: u.updated_at,
            updated_by: u.updated_by,
        }
    }
}

pub struct UserDieselImpl {
    pool: Arc<infra::DBConn>,
}

impl UserDieselImpl {
    pub fn new(db: Arc<infra::DBConn>) -> Self {
        UserDieselImpl { pool: db }
    }
}

impl UserRepo for UserDieselImpl {
    fn get_all(&self) -> RepoResult<Vec<User>> {
        use crate::schema::users::dsl::users;
        let conn = self.pool.as_ref().get()?;
        let all_users = users.load::<UserDiesel>(&conn)?;
        Ok(all_users.into_iter().map(|v| v.into()).collect())
    }

    fn find(&self, user_id: uuid::Uuid) -> RepoResult<User> {
        use crate::schema::users::dsl::{id, users};
        let conn = self.pool.as_ref().get()?;
        users
            .filter(id.eq(user_id.to_string()))
            .first::<UserDiesel>(&conn)
            .map_err(|e| RepoError {
                message: format!("Exec Error: {}", e),
            })
            .map(|v| v.into())
    }

    fn find_by_auth(&self, user_email: &str, user_password: &str) -> RepoResult<User> {
        use crate::schema::users::dsl::{email, password, users};
        let conn = self.pool.get()?;
        users
            .filter(email.eq(user_email.to_string()))
            .filter(password.eq(user_password.to_string()))
            .first::<UserDiesel>(&conn)
            .map_err(|e| RepoError {
                message: format!("Invalid login: _{}", e),
            })
            .map(|v| v.into())
    }

    fn create(&self, new_user: &User) -> RepoResult<User> {
        let u: UserDiesel = UserDiesel::from(new_user.clone());
        use crate::schema::users::dsl::users;
        let conn = self.pool.get()?;
        diesel::insert_into(users).values(u).execute(&conn)?;
        Ok(new_user.clone())
    }

    fn update(&self, update_user: &User) -> RepoResult<User> {
        let u = UserDiesel::from(update_user.clone());
        use crate::schema::users::dsl::{id, users};
        let conn = self.pool.get()?;
        diesel::update(users)
            .filter(id.eq(update_user.id.clone()))
            .set(u)
            .execute(&conn)?;
        match uuid::Uuid::parse_str(&update_user.id) {
            Ok(v) => self.find(v),
            Err(e) => Err(RepoError {
                message: e.to_string(),
            }),
        }
    }

    fn delete(&self, uuid: uuid::Uuid) -> RepoResult<()> {
        use crate::schema::users::dsl::{id, users};
        let conn = self.pool.get()?;
        diesel::delete(users)
            .filter(id.eq(uuid.to_string()))
            .execute(&conn)?;
        Ok(())
    }
}

#[cfg(test)]
pub mod test {
    use super::*;
    fn test_insert() {}
}
