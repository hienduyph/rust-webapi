use rwebapi_users::RepoError;

#[derive(Debug)]
pub struct DieselRepoError(RepoError);

impl DieselRepoError {
    pub fn into_inner(self) -> RepoError {
        self.0
    }
}

/// Convert PoolErrors to ApiErrors
impl From<r2d2::Error> for DieselRepoError {
    fn from(error: r2d2::Error) -> DieselRepoError {
        DieselRepoError(RepoError {
            message: error.to_string(),
        })
    }
}

impl From<diesel::result::Error> for DieselRepoError {
    fn from(error: diesel::result::Error) -> DieselRepoError {
        DieselRepoError(RepoError {
            message: error.to_string(),
        })
    }
}

impl<T: std::fmt::Debug> From<crate::async_pool::AsyncPoolError<T>> for DieselRepoError {
    fn from(error: crate::async_pool::AsyncPoolError<T>) -> DieselRepoError {
        DieselRepoError(RepoError {
            message: error.to_string(),
        })
    }
}
