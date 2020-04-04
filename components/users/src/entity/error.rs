use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct CommonError {
    pub message: String,
    pub code: u32,
}

impl std::fmt::Display for CommonError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error: {}, Code: {}", self.message, self.code)
    }
}

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
