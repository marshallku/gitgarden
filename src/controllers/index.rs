use axum::{extract::Query, http::HeaderMap, response::IntoResponse};
use reqwest::StatusCode;
use serde::Deserialize;

use crate::services::index::index_service;

#[derive(Deserialize)]
pub struct Options {
    user_name: String,
    year: i32,
}

pub async fn get(Query(Options { user_name, year }): Query<Options>) -> impl IntoResponse {
    let mut headers = HeaderMap::new();

    headers.insert("Content-Type", "image/svg+xml".parse().unwrap());
    headers.insert("Cache-Control", "no-cache".parse().unwrap());
    headers.insert("Pragma", "no-cache".parse().unwrap());
    headers.insert("Expires", "0".parse().unwrap());

    let response = index_service(user_name, year).await;

    (StatusCode::OK, headers, response)
}
