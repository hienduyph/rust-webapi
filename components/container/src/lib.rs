use rwebapi_diesel_impl;
use rwebapi_users::*;
use std::sync::Arc;

pub struct UserContainer {
    pub user_service: Box<dyn UserService>,
}

impl UserContainer {
    pub fn new() -> Self {
        let pool = Arc::new(rwebapi_diesel_impl::db_pool());
        let user_repo = Arc::new(rwebapi_diesel_impl::UserDieselImpl::new(pool));

        // init service
        let password_salt: &str = "aslkdjclkasjdfklq";
        let user_security: Arc<dyn UserSecurityService> = Arc::new(UserSecurityServiceImpl {
            salt: password_salt.to_string(),
        });
        let user_service = Box::new(UserServiceImpl {
            user_repo,
            user_security,
        });
        UserContainer { user_service }
    }
}

impl Default for UserContainer {
    fn default() -> Self {
        Self::new()
    }
}
