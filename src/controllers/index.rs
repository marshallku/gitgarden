use axum::{
    extract::{Query, State},
    response::IntoResponse,
};
use chrono::Datelike;
use reqwest::StatusCode;
use serde::Deserialize;

use crate::{
    env::state::AppState,
    services::render_farm::render_farm_service,
    utils::{extractor::ExtractFullOrigin, http::get_cache_header},
};

#[derive(Deserialize)]
pub struct Options {
    user_name: String,
    year: Option<i32>,
}

pub async fn get(
    Query(Options { user_name, year }): Query<Options>,
    State(state): State<AppState>,
    ExtractFullOrigin(origin): ExtractFullOrigin,
) -> impl IntoResponse {
    let mut headers = get_cache_header("1h");
    let year = year.unwrap_or_else(|| chrono::Local::now().year());

    headers.insert("Content-Type", "image/svg+xml".parse().unwrap());

    let rendered_svg = render_farm_service(&user_name, year, state).await;

    match rendered_svg {
        Ok(svg) => {
            let (width, height, image) = svg;

            headers.insert("Content-Length", image.len().to_string().parse().unwrap());
            headers.insert(
                "og:image",
                format!("{}/?user_name={}&year={}", origin, user_name, year)
                    .parse()
                    .unwrap(),
            );
            headers.insert("og:image:type", "image/svg+xml".parse().unwrap());
            headers.insert("og:image:width", width.to_string().parse().unwrap());
            headers.insert("og:image:height", height.to_string().parse().unwrap());

            (StatusCode::OK, headers, image)
        }
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
