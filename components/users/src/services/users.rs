use std::sync::Arc;

use async_trait::async_trait;
use rwebapi_core::{CommonError, QueryParams, ResultPaging};

use crate::entity;

#[async_trait]
pub trait UserService: Send + Sync {
    async fn users(
        &self,
        params: &dyn QueryParams,
    ) -> Result<ResultPaging<entity::User>, CommonError>;
}

pub struct UserServiceImpl {
    pub user_repo: Arc<dyn entity::UserRepo>,
}

#[async_trait]
impl UserService for UserServiceImpl {
    async fn users(
        &self,
        params: &dyn QueryParams,
    ) -> Result<ResultPaging<entity::User>, CommonError> {
        let users = self
            .user_repo
            .as_ref()
            .get_all(params)
            .await
            .map_err(|e| -> CommonError { e.into() })?;
        Ok(users)
    }
}
