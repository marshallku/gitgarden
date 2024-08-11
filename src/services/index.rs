use axum::{response::IntoResponse, Json};

use crate::api::contributions::get_daily_commits;

pub async fn index_service(user_name: String, year: i32) -> impl IntoResponse {
    let response = get_daily_commits(&user_name, year).await.unwrap();
    Json(response)
}
