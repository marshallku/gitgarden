use axum::{extract::Query, response::IntoResponse};
use serde::Deserialize;

use crate::services::index::index_service;

#[derive(Deserialize)]
pub struct Options {
    user_name: String,
    year: i32,
}

pub async fn get(Query(Options { user_name, year }): Query<Options>) -> impl IntoResponse {
    index_service(user_name, year).await
}
