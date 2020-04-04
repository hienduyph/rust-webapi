use serde::{Deserialize, Serialize};

pub type RepoResult<T> = Result<T, super::error::RepoError>;

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryParams {
    pub size: u64,
    pub page: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResultPaging<T> {
    pub total: u64,
    pub items: Vec<T>,
}
