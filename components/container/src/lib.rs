use rwebapi_diesel_impl;
use rwebapi_users::*;
use std::sync::Arc;

pub struct UserContainer {
    pub user_service: Box<dyn UserService>,
}

impl UserContainer {
    pub fn new() -> Self {
        let pool = Arc::new(rwebapi_diesel_impl::infra::db_pool());
        let user_repo = Arc::new(rwebapi_diesel_impl::users::UserDieselImpl::new(pool));
        let svc = Box::new(UserServiceImpl { user_repo });
        UserContainer { user_service: svc }
    }
}

impl Default for UserContainer {
    fn default() -> Self {
        Self::new()
    }
}
