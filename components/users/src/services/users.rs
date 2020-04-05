use std::sync::Arc;

use async_trait::async_trait;
use rwebapi_core::{CommonError, QueryParams, ResultPaging};

use super::UserSecurityService;
use crate::entity::{User, UserRepo, UserUpdate};

#[async_trait]
pub trait UserService: Send + Sync {
    async fn create(&self, user: &User) -> Result<User, CommonError>;

    async fn users(&self, params: &dyn QueryParams) -> Result<ResultPaging<User>, CommonError>;

    async fn find_by_id(&self, id: &str) -> Result<User, CommonError>;

    async fn update_by_id(&self, id: &str, u: &UserUpdate) -> Result<User, CommonError>;

    async fn delete_by_id(&self, id: &str) -> Result<(), CommonError>;
}

pub struct UserServiceImpl {
    pub user_repo: Arc<dyn UserRepo>,
    pub user_security: Arc<dyn UserSecurityService>,
}

#[async_trait]
impl UserService for UserServiceImpl {
    async fn create(&self, user: &User) -> Result<User, CommonError> {
        let mut cloned = user.clone();

        let hashed_password = self.user_security.hash(&cloned.password).await?;
        cloned.password = hashed_password;
        self.user_repo
            .create(&cloned)
            .await
            .map_err(|e| -> CommonError { e.into() })
    }

    async fn users(&self, params: &dyn QueryParams) -> Result<ResultPaging<User>, CommonError> {
        self.user_repo
            .get_all(params)
            .await
            .map_err(|e| -> CommonError { e.into() })
    }

    async fn find_by_id(&self, id: &str) -> Result<User, CommonError> {
        self.user_repo
            .find(id)
            .await
            .map_err(|e| -> CommonError { e.into() })
    }

    async fn update_by_id(&self, id: &str, u: &UserUpdate) -> Result<User, CommonError> {
        self.user_repo
            .update(id, u)
            .await
            .map_err(|e| -> CommonError { e.into() })
    }

    async fn delete_by_id(&self, id: &str) -> Result<(), CommonError> {
        self.user_repo
            .delete(id)
            .await
            .map_err(|e| -> CommonError { e.into() })
    }
}
