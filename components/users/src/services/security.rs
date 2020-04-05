use argon2rs::argon2i_simple;
use async_trait::async_trait;

use rwebapi_core::CommonError;

#[async_trait]
pub trait UserSecurityService: Send + Sync {
    async fn hash(&self, input: &str) -> Result<String, CommonError>;
}

pub struct UserSecurityServiceImpl {
    pub salt: String,
}

#[async_trait]
impl UserSecurityService for UserSecurityServiceImpl {
    async fn hash(&self, input: &str) -> Result<String, CommonError> {
        let result = argon2i_simple(&input, &self.salt)
            .iter()
            .map(|b| format!("{:02x}", b))
            .collect();
        Ok(result)
    }
}

