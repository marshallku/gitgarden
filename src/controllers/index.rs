use axum::response::IntoResponse;

pub async fn get() -> impl IntoResponse {
    "Hello, world!"
}
