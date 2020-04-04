use rwebapi_core::CommonError;

pub struct RepoError {
    pub message: String,
}

impl Into<CommonError> for RepoError {
    fn into(self) -> CommonError {
        CommonError {
            message: self.message,
            code: 1,
        }
    }
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
