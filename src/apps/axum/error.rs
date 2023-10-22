use axum::{
    response::{IntoResponse, Response},
    Json,
};

use crate::core::CommonError;

#[derive(Debug)]
pub struct ApiError(CommonError);

/// Convert PoolErrors to ApiErrors
impl From<CommonError> for ApiError {
    fn from(error: CommonError) -> ApiError {
        ApiError(error)
    }
}

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        (axum::http::StatusCode::BAD_REQUEST, Json(self.0)).into_response()
    }
}
