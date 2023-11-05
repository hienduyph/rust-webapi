use async_trait::async_trait;

use crate::core::{QueryParams, RepoResult, ResultPaging};

use super::entity::{User, UserRepo, UserUpdate};

pub struct UserSqlxRepoImpl {
    pool: crate::infra::DBConn,
}

impl UserSqlxRepoImpl {
    pub fn new(pool: crate::infra::DBConn) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepo for UserSqlxRepoImpl {
    async fn get_all(&self, params: &dyn QueryParams) -> RepoResult<ResultPaging<User>> {
        let count = sqlx::query!("SELECT COUNT(*) AS count FROM users")
            .fetch_one(&self.pool.clone())
            .await
            .unwrap()
            .count;
        let limit = params.limit();
        let offset = params.offset();
        let users = sqlx::query_as!(
            User,
            r#"SELECT * FROM users ORDER BY id LIMIT ? OFFSET ?"#,
            limit,
            offset,
        )
        .fetch_all(&self.pool.clone())
        .await
        .unwrap();
        return Ok(ResultPaging {
            total: count as i64,
            items: users,
        });
    }

    async fn find(&self, user_id: &str) -> RepoResult<User> {
        let user = sqlx::query_as!(User, "SELECT * FROM users WHERE id = ?", user_id)
            .fetch_one(&self.pool.clone())
            .await
            .unwrap();
        Ok(user)
    }

    async fn find_by_email(&self, email: &str) -> RepoResult<User> {
        let user = sqlx::query_as!(User, "SELECT * FROM users WHERE email = ?", email)
            .fetch_one(&self.pool.clone())
            .await
            .unwrap();
        return Ok(user);
    }

    async fn create(&self, user: &User) -> RepoResult<User> {
        panic!("impl")
    }

    async fn update(&self, id: &str, update_user: &UserUpdate) -> RepoResult<User> {
        panic!("impl")
    }

    async fn delete(&self, user_id: &str) -> RepoResult<()> {
        panic!("impl")
    }
}
