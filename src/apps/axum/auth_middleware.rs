use std::sync::Arc;

use super::{error::ApiError, state::AppState};
use crate::users::UserIdentity;
use axum::extract::{Request, State};
use axum::http::HeaderMap;
use axum::middleware::Next;
use axum::response::IntoResponse;

pub async fn auth_required(
    State(app_state): State<Arc<AppState>>,
    mut request: Request,
    next: Next,
) -> Result<impl IntoResponse, ApiError> {
    let token = get_token(request.headers())?;
    let payload = app_state
        .user_c
        .user_security_service
        .clone()
        .decode_token(&token)
        .await;
    if payload.is_err() {
        return Err(ApiError::unauthorized("can not decode token".to_string()));
    }
    let user_boxed = app_state
        .user_c
        .user_service
        .clone()
        .find_by_email(&payload.unwrap().email)
        .await;
    if user_boxed.is_err() {
        return Err(ApiError::unauthorized(
            "user for this token not found".to_string(),
        ));
    }
    let user = user_boxed.unwrap();
    let identity = UserIdentity {
        email: user.email,
        user_id: user.id,
    };
    log::info!("got loggedin users: {:?}", identity);
    request.extensions_mut().insert(identity);
    Ok(next.run(request).await)
}

fn get_token(header: &HeaderMap) -> Result<String, ApiError> {
    let header = header.get("authorization");
    match header {
        None => Err(ApiError::unauthorized(
            "missing authorization header".to_string(),
        )),
        Some(v) => {
            let value = v
                .to_str()
                .map_err(|e| ApiError::bad_request(e.to_string()))?;
            let parts: Vec<_> = value.split(' ').collect();
            if parts.len() != 2 || parts[0].to_lowercase() != "bearer" {
                Err(ApiError::unauthorized(
                    "invalid token format, expected Bearer value".to_string(),
                ))
            } else {
                Ok(parts[1].to_owned())
            }
        }
    }
}
