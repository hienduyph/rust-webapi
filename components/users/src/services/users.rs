use crate::entity;
use rwebapi_core::CommonError;
use std::sync::Arc;

pub trait UserService: Send + Sync {
    fn users(&self) -> Result<Vec<entity::User>, CommonError>;
}

pub struct UserServiceImpl {
    pub user_repo: Arc<dyn entity::UserRepo>,
}

impl UserService for UserServiceImpl {
    fn users(&self) -> Result<Vec<entity::User>, CommonError> {
        let users = self
            .user_repo
            .as_ref()
            .get_all()
            .map_err(|e| -> CommonError { e.into() })?;
        Ok(users)
    }
}
