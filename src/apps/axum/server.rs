use std::sync::Arc;

use super::state::AppState;
use axum::{
    middleware,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use tokio::signal;

use crate::{
    apps::axum::{
        auth_middleware::auth_required,
        user_handler::{users_by_id, users_create, users_list},
    },
    container::UserContainer,
};

pub async fn serve() -> std::io::Result<()> {
    // construct di
    let user_component = UserContainer::new();
    let ctx = Arc::new(AppState {
        user_c: user_component.clone(),
    });

    let user_routes = Router::new()
        .route(
            "/",
            get(users_list).with_state(user_component.user_service.clone()),
        )
        .route(
            "/",
            post(users_create).with_state(user_component.user_service.clone()),
        )
        .route(
            "/{user_id}",
            get(users_by_id).with_state(user_component.user_service.clone()),
        )
        .route_layer(middleware::from_fn_with_state(ctx.clone(), auth_required));

    let app = Router::new()
        .nest("/users", user_routes)
        .route("/health", get(health))
        .route(
            "/auth/login",
            post(super::auth_handler::login).with_state(user_component.user_auth_service.clone()),
        )
        .route("/", get(health))
        .with_state(ctx);

    let addr = "0.0.0.0:8000";
    log::info!("Listening {:?}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    // Run the server with graceful shutdown
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
    Ok(())
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}

async fn health() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok".into(),
        version: env!("CARGO_PKG_VERSION").into(),
    })
}

#[derive(Serialize, Deserialize)]
pub struct HealthResponse {
    pub status: String,
    pub version: String,
}
