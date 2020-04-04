use std::sync::Arc;

use async_trait::async_trait;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use futures::future::join_all;

use crate::async_pool;
use crate::errors::DieselRepoError;
use crate::infra;
use crate::schema::users;

use rwebapi_core::{QueryParams, RepoError, RepoResult, ResultPaging};
use rwebapi_users::{User, UserRepo};

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

    async fn total(&self) -> RepoResult<u64> {
        use crate::schema::users::dsl::users;
        let pool = self.pool.clone();
        async_pool::run(move || {
            let conn = pool.get().unwrap();
            users.count().get_result(&conn)
        })
        .await
        .map_err(|v| DieselRepoError::from(v).into_inner())
        .map(|v: i64| v as u64)
    }

    async fn fetch(&self, query: &QueryParams) -> RepoResult<Vec<User>> {
        use crate::schema::users::dsl::users;
        let pool = self.pool.clone();
        let result = async_pool::run(move || {
            let conn = pool.get().unwrap();
            users.load::<UserDiesel>(&conn)
        })
        .await
        .map_err(|v| DieselRepoError::from(v).into_inner())?;
        Ok(result.into_iter().map(|v| -> User { v.into() }).collect())
    }
}

#[async_trait]
impl UserRepo for UserDieselImpl {
    async fn get_all(&self, params: &QueryParams) -> RepoResult<ResultPaging<User>> {
        let total = self.total();
        let users = self.fetch(params);
        Ok(ResultPaging {
            total: total.await?,
            items: users.await?,
        })
    }

    async fn find(&self, user_id: uuid::Uuid) -> RepoResult<User> {
        use crate::schema::users::dsl::{id, users};
        let conn = self
            .pool
            .get()
            .map_err(|v| DieselRepoError::from(v).into_inner())?;
        async_pool::run(move || {
            users
                .filter(id.eq(user_id.to_string()))
                .first::<UserDiesel>(&conn)
        })
        .await
        .map_err(|v| DieselRepoError::from(v).into_inner())
        .map(|v| -> User { v.into() })
    }

    async fn find_by_auth(&self, user_email: &str, user_password: &str) -> RepoResult<User> {
        use crate::schema::users::dsl::{email, password, users};
        let conn = self
            .pool
            .get()
            .map_err(|v| DieselRepoError::from(v).into_inner())?;
        let user_email_u = user_email.to_string();
        let user_password_u = user_password.to_string();
        async_pool::run(move || {
            users
                .filter(email.eq(user_email_u))
                .filter(password.eq(user_password_u))
                .first::<UserDiesel>(&conn)
        })
        .await
        .map_err(|v| DieselRepoError::from(v).into_inner())
        .map(|v| -> User { v.into() })
    }

    async fn create(&self, new_user: &User) -> RepoResult<User> {
        let u: UserDiesel = UserDiesel::from(new_user.clone());
        use crate::schema::users::dsl::users;
        let conn = self
            .pool
            .get()
            .map_err(|v| DieselRepoError::from(v).into_inner())?;
        async_pool::run(move || diesel::insert_into(users).values(u).execute(&conn))
            .await
            .map_err(|v| DieselRepoError::from(v).into_inner())?;
        Ok(new_user.clone())
    }

    async fn update(&self, update_user: &User) -> RepoResult<User> {
        let u = UserDiesel::from(update_user.clone());
        use crate::schema::users::dsl::{id, users};
        let conn = self
            .pool
            .get()
            .map_err(|v| DieselRepoError::from(v).into_inner())?;
        let id_filter = update_user.id.clone();
        async_pool::run(move || {
            diesel::update(users)
                .filter(id.eq(id_filter))
                .set(u)
                .execute(&conn)
        })
        .await
        .map_err(|v| DieselRepoError::from(v).into_inner())?;
        match uuid::Uuid::parse_str(&update_user.id) {
            Ok(v) => self.find(v).await,
            Err(e) => Err(RepoError {
                message: e.to_string(),
            }),
        }
    }

    async fn delete(&self, uuid: uuid::Uuid) -> RepoResult<()> {
        use crate::schema::users::dsl::{id, users};
        let conn = self
            .pool
            .get()
            .map_err(|v| DieselRepoError::from(v).into_inner())?;
        async_pool::run(move || {
            diesel::delete(users)
                .filter(id.eq(uuid.to_string()))
                .execute(&conn)
        })
        .await
        .map_err(|v| DieselRepoError::from(v).into_inner())?;
        Ok(())
    }
}

#[cfg(test)]
pub mod test {
    use super::*;
    fn test_insert() {}
}
