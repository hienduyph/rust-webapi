use rwebapi_core::CommonError;

#[derive(Debug)]
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

