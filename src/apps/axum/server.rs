use axum::{
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};

pub async fn serve() -> std::io::Result<()> {
    // construct di
    let user_component = crate::container::UserContainer::new();

    let app = Router::new()
        .route("/", get(health))
        .route(
            "/auth/login",
            post(super::auth_handler::login).with_state(user_component.user_auth_service),
        )
        .route("/health", get(health));

    let addr = "0.0.0.0:8000".parse().unwrap();
    println!("Listening {:?}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
    Ok(())
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
