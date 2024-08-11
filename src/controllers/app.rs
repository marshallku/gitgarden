use axum::{routing::get, Router};

use crate::env::state::AppState;

pub fn app() -> Router<AppState> {
    Router::new().route("/", get(super::index::get))
}
