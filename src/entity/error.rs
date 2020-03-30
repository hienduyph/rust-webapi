pub struct RepoError {
    pub message: String,
}

/// Convert PoolErrors to ApiErrors
impl From<diesel::r2d2::PoolError> for RepoError {
    fn from(error: diesel::r2d2::PoolError) -> RepoError {
        RepoError {
            message: error.to_string(),
        }
    }
}

impl From<diesel::result::Error> for RepoError {
    fn from(error: diesel::result::Error) -> RepoError {
        RepoError {
            message: error.to_string(),
        }
    }
}
