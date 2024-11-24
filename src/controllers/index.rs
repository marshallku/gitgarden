use axum::{
    extract::{Query, State},
    response::IntoResponse,
};
use chrono::Datelike;
use reqwest::StatusCode;
use serde::Deserialize;

use crate::{
    env::state::AppState,
    services::{render_farm::render_farm_service, render_page::render_page_service},
    utils::{extractor::ExtractFullOrigin, http::get_cache_header},
};

#[derive(Deserialize)]
pub struct Options {
    user_name: Option<String>,
    year: Option<i32>,
}

pub async fn get(
    Query(Options { user_name, year }): Query<Options>,
    State(state): State<AppState>,
    ExtractFullOrigin(origin): ExtractFullOrigin,
) -> impl IntoResponse {
    if user_name.is_none() {
        let mut headers = get_cache_header("1h");
        let rendered_page = render_page_service().await;

        headers.insert("Content-Type", "text/html".parse().unwrap());

        return (StatusCode::OK, headers, rendered_page);
    }

    let user_name = user_name.unwrap();
    let year = year.unwrap_or_else(|| chrono::Local::now().year());
    let rendered_svg = render_farm_service(&user_name, year, state).await;

    match rendered_svg {
        Ok(svg) => {
            let mut headers = get_cache_header("1h");

            headers.insert("Content-Type", "image/svg+xml".parse().unwrap());
            headers.insert(
                "Accept-Ch",
                "UA, UA-Mobile, Save-Data, RTT".parse().unwrap(),
            );
            headers.insert("Referrer-Policy", "same-origin".parse().unwrap());
            headers.insert(
                "Content-Disposition",
                format!("inline; filename=\"{}-gitgarden-{}.svg\"", user_name, year)
                    .parse()
                    .unwrap(),
            );
            headers.insert("Access-Control-Allow-Origin", origin.parse().unwrap());
            headers.insert("X-Content-Type-Options", "nosniff".parse().unwrap());

            return (StatusCode::OK, headers, svg);
        }
        Err(e) => {
            let mut headers = get_cache_header("0");

            headers.insert("Access-Control-Allow-Origin", origin.parse().unwrap());
            headers.insert("X-Content-Type-Options", "nosniff".parse().unwrap());

            return (StatusCode::INTERNAL_SERVER_ERROR, headers, e.to_string());
        }
    }
}
