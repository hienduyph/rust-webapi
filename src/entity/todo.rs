use async_trait::async_trait;
use chrono::{naive::NaiveDateDaysIterator, NaiveDateTime, Utc};
use derive_more::derive;
use serde::{Deserialize, Serialize};

use crate::core::{QueryParams, RepoResult, ResultPaging};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Todo {
    pub id: String,
    pub title: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    // ref to user_id
    pub created_by: String,
    pub updated_by: String,
}

#[async_trait]
pub trait TodoRepo: Send + Sync {
    async fn get_all(&self, params: &dyn QueryParams) -> RepoResult<ResultPaging<Todo>>;
}
