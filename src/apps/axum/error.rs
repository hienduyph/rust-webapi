use axum::{
    response::{IntoResponse, Response},
    Json,
};

use crate::core::CommonError;

#[derive(Debug)]
pub struct ApiError(CommonError);

impl ApiError {
    pub fn bad_request(cause: String) -> Self {
        ApiError(CommonError {
            message: cause,
            code: 400,
        })
    }
    pub fn forbidden(cause: String) -> Self {
        ApiError(CommonError {
            message: cause,
            code: 403,
        })
    }
    pub fn unauthorized(cause: String) -> Self {
        ApiError(CommonError {
            message: cause,
            code: 401,
        })
    }
}

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
        (
            axum::http::StatusCode::from_u16(self.0.code as u16).unwrap(),
            Json(self.0),
        )
            .into_response()
    }
}
