use axum::extract::FromRef;
use deadpool_redis::Pool;

use crate::{container::UserContainer, users::UserService};

#[derive(Clone, FromRef)]
pub struct AppState {
    pub user_c: UserContainer,
    pub redis_pool: Pool,
}
