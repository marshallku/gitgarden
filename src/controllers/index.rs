use axum::{
    extract::{Query, State},
    http::HeaderMap,
    response::IntoResponse,
};
use chrono::Datelike;
use reqwest::StatusCode;
use serde::Deserialize;

use crate::{env::state::AppState, services::render_farm::render_farm_service};

#[derive(Deserialize)]
pub struct Options {
    user_name: String,
    year: Option<i32>,
}

pub async fn get(
    Query(Options { user_name, year }): Query<Options>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    let mut headers = HeaderMap::new();

    headers.insert("Content-Type", "image/svg+xml".parse().unwrap());
    headers.insert("Cache-Control", "no-cache".parse().unwrap());
    headers.insert("Pragma", "no-cache".parse().unwrap());
    headers.insert("Expires", "0".parse().unwrap());

    let response = render_farm_service(
        user_name,
        year.unwrap_or_else(|| chrono::Local::now().year()),
        state,
    )
    .await;

    (StatusCode::OK, headers, response)
}
