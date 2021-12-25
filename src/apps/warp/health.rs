use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct HealthResponse {
    pub status: String,
    pub version: String,
}

pub async fn health() -> Result<impl warp::Reply, warp::Rejection> {
    let resp = HealthResponse {
        status: "ok".into(),
        version: env!("CARGO_PKG_VERSION").into(),
    };
    Ok(warp::reply::json(&resp))
}
