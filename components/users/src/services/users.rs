use crate::entity;
use async_trait::async_trait;
use rwebapi_core::CommonError;
use std::sync::Arc;

#[async_trait]
pub trait UserService: Send + Sync {
    async fn users(&self) -> Result<Vec<entity::User>, CommonError>;
}

pub struct UserServiceImpl {
    pub user_repo: Arc<dyn entity::UserRepo>,
}

#[async_trait]
impl UserService for UserServiceImpl {
    async fn users(&self) -> Result<Vec<entity::User>, CommonError> {
        let users = self
            .user_repo
            .as_ref()
            .get_all()
            .await
            .map_err(|e| -> CommonError { e.into() })?;
        Ok(users)
    }
}
