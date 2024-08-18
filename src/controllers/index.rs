use axum::{
    extract::{Query, State},
    response::IntoResponse,
};
use chrono::Datelike;
use reqwest::StatusCode;
use serde::Deserialize;

use crate::{
    env::state::AppState, services::render_farm::render_farm_service, utils::http::get_cache_header,
};

#[derive(Deserialize)]
pub struct Options {
    user_name: String,
    year: Option<i32>,
}

pub async fn get(
    Query(Options { user_name, year }): Query<Options>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    let mut headers = get_cache_header("1h");

    headers.insert("Content-Type", "image/svg+xml".parse().unwrap());

    let rendered_svg = render_farm_service(
        &user_name,
        year.unwrap_or_else(|| chrono::Local::now().year()),
        state,
    )
    .await;

    match rendered_svg {
        Ok(svg) => (StatusCode::OK, headers, svg),
        Err(e) => {
            eprintln!("Error: {}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                headers,
                "Internal Server Error".to_string(),
            );
        }
    }
}
