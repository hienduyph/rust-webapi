use crate::users::*;
use std::sync::Arc;

pub struct UserContainer {
    pub user_service: Arc<dyn UserService>,
    pub user_auth_service: Arc<dyn UserAuthService>,
    pub user_security_service: Arc<dyn UserSecurityService>,
}

impl UserContainer {
    pub async fn new() -> Self {
        let sqlx_pool = crate::infra::db_conn().await;
        let sqlx_user_repo: Arc<dyn UserRepo> =
            Arc::new(crate::users::UserSqlxRepoImpl::new(sqlx_pool.clone()));
        let pool = Arc::new(crate::diesel_impl::db_pool());
        let user_repo: Arc<dyn UserRepo> = Arc::new(crate::diesel_impl::UserDieselImpl::new(pool));

        // init service
        let user_security_service: Arc<dyn UserSecurityService> =
            Arc::new(UserSecurityServiceImpl {
                salt: "aslkdjclkasjdfklq".to_string(),
                jwt_key: "calsdkjfalkjclkajsdflkjw83712".to_string(),
            });

        let user_service: Arc<dyn UserService> = Arc::new(UserServiceImpl {
            user_repo: sqlx_user_repo.clone(),
            user_security: user_security_service.clone(),
        });
        let user_auth_service: Arc<dyn UserAuthService> = Arc::new(UserAuthServiceImpl {
            user_repo: user_repo.clone(),
            user_security: user_security_service.clone(),
        });

        UserContainer {
            user_service,
            user_auth_service,
            user_security_service,
        }
    }
}
