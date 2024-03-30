use crate::users::*;
use std::sync::Arc;

#[derive(Clone)]
pub struct UserContainer {
    pub user_service_impl: Arc<UserServiceImpl>,
    pub user_security_service_impl: Arc<UserSecurityServiceImpl>,
    pub user_service: Arc<dyn UserService>,
    pub user_auth_service: Arc<dyn UserAuthService>,
    pub user_security_service: Arc<dyn UserSecurityService>,
}

impl UserContainer {
    pub fn new() -> Self {
        let pool = Arc::new(crate::diesel_impl::db_pool());
        let user_repo: Arc<dyn UserRepo> = Arc::new(crate::diesel_impl::UserDieselImpl::new(pool));

        // init service
        let user_security_service = Arc::new(UserSecurityServiceImpl {
            salt: "aslkdjclkasjdfklq".to_string(),
            jwt_key: "calsdkjfalkjclkajsdflkjw83712".to_string(),
        });

        let user_service = Arc::new(UserServiceImpl {
            user_repo: user_repo.clone(),
            user_security: user_security_service.clone(),
        });
        let user_auth_service: Arc<dyn UserAuthService> = Arc::new(UserAuthServiceImpl {
            user_repo: user_repo.clone(),
            user_security: user_security_service.clone(),
        });

        UserContainer {
            user_service_impl: user_service.clone(),
            user_security_service_impl: user_security_service.clone(),
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
