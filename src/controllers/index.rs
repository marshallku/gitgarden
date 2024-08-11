use axum::{extract::Query, response::IntoResponse, Json};
use serde::Deserialize;

use crate::api::contributions::get_daily_commits;

#[derive(Deserialize)]
pub struct Options {
    user_name: String,
    year: i32,
}

pub async fn get(Query(Options { user_name, year }): Query<Options>) -> impl IntoResponse {
    let response = get_daily_commits(&user_name, year).await.unwrap();
    Json(response)
}
