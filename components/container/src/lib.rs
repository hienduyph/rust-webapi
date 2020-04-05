use rwebapi_diesel_impl;
use rwebapi_users::*;
use std::sync::Arc;

pub struct UserContainer {
    pub user_service: Box<dyn UserService>,
    pub user_auth_service: Box<dyn UserAuthService>,
}

impl UserContainer {
    pub fn new() -> Self {
        let pool = Arc::new(rwebapi_diesel_impl::db_pool());
        let user_repo = Arc::new(rwebapi_diesel_impl::UserDieselImpl::new(pool));

        // init service
        let user_security: Arc<dyn UserSecurityService> = Arc::new(UserSecurityServiceImpl {
            salt: "aslkdjclkasjdfklq".to_string(),
            jwt_key: "calsdkjfalkjclkajsdflkjw83712".to_string(),
        });
        let user_service = Box::new(UserServiceImpl {
            user_repo: user_repo.clone(),
            user_security: user_security.clone(),
        });
        let user_auth_service = Box::new(UserAuthServiceImpl {
            user_repo: user_repo.clone(),
            user_security: user_security.clone(),
        });
        UserContainer {
            user_service,
            user_auth_service,
        }
    }
}

impl Default for UserContainer {
    fn default() -> Self {
        Self::new()
    }
}
