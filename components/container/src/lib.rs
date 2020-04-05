use rwebapi_diesel_impl;
use rwebapi_users::*;
use std::sync::Arc;

pub struct UserContainer {
    pub user_service: Arc<Box<dyn UserService>>,
    pub user_auth_service: Arc<Box<dyn UserAuthService>>,
    pub user_security_service: Arc<Box<dyn UserSecurityService>>,
}

impl UserContainer {
    pub fn new() -> Self {
        let pool = Arc::new(rwebapi_diesel_impl::db_pool());
        let user_repo_v: Box<dyn UserRepo> =
            Box::new(rwebapi_diesel_impl::UserDieselImpl::new(pool));
        let user_repo = Arc::new(user_repo_v);

        // init service
        let user_security_service: Arc<Box<dyn UserSecurityService>> =
            Arc::new(Box::new(UserSecurityServiceImpl {
                salt: "aslkdjclkasjdfklq".to_string(),
                jwt_key: "calsdkjfalkjclkajsdflkjw83712".to_string(),
            }));

        let user_service: Arc<Box<dyn UserService>> = Arc::new(Box::new(UserServiceImpl {
            user_repo: user_repo.clone(),
            user_security: user_security_service.clone(),
        }));
        let user_auth_service: Arc<Box<dyn UserAuthService>> =
            Arc::new(Box::new(UserAuthServiceImpl {
                user_repo: user_repo.clone(),
                user_security: user_security_service.clone(),
            }));

        UserContainer {
            user_service,
            user_auth_service,
            user_security_service,
        }
    }
}

impl Default for UserContainer {
    fn default() -> Self {
        Self::new()
    }
}
