use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::Serialize;

#[derive(Serialize)]
pub struct HealthResponse {
    status: &'static str,
}

pub async fn get() -> impl IntoResponse {
    (StatusCode::OK, Json(HealthResponse { status: "healthy" }))
}
