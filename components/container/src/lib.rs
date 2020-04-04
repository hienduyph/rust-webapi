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
        let user_service = Box::new(UserServiceImpl { user_repo });
        UserContainer { user_service }
    }
}

impl Default for UserContainer {
    fn default() -> Self {
        Self::new()
    }
}
