use std::sync::Arc;

use async_trait::async_trait;

use rwebapi_core::CommonError;

use super::security::UserSecurityService;
use crate::entity::{User, UserRepo};

#[async_trait]
pub trait UserAuthService: Sync + Send {
    async fn login(&self, email: &str, password: &str) -> Result<(User, String), CommonError>;
}

pub struct UserAuthServiceImpl {
    pub user_repo: Arc<dyn UserRepo>,
    pub user_security: Arc<dyn UserSecurityService>,
}

#[async_trait]
impl UserAuthService for UserAuthServiceImpl {
    async fn login(&self, email: &str, password: &str) -> Result<(User, String), CommonError> {
        let user = self
            .user_repo
            .find_by_email(email)
            .await
            .map_err(|e| -> CommonError { e.into() })?;
        // compare auth
        let is_valid = self
            .user_security
            .verify_hash(&user.password, password)
            .await?;
        if !is_valid {
            return Err(CommonError {
                message: "InvalidPassword".to_string(),
                code: 401,
            });
        }
        let token = self.user_security.token_generator(&user).await?;
        Ok((user, token))
    }
}
