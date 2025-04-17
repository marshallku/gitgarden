use std::{fs::File, io::Write};

use axum::serve;
use controllers::app::app;
use env::state::AppState;
use tokio::{net::TcpListener, signal};
use tower_http::trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer};
use tracing::{info, Level};
use utils::log::trace_layer_on_request;

mod api;
mod constants;
mod controllers;
mod env;
mod render;
mod services;
mod utils;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();

    let state = AppState::from_env();
    let address = format!("{}:{}", state.host, state.port);
    let app = app()
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
                .on_response(DefaultOnResponse::new().level(Level::INFO))
                .on_request(trace_layer_on_request),
        )
        .with_state(state);
    let listener = TcpListener::bind(address.as_str()).await.unwrap();

    info!("Listening on http://{}", address);

    serve(listener, app.into_make_service())
        .with_graceful_shutdown(handle_shutdown())
        .await
        .unwrap();
}

async fn handle_shutdown() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    // Create file
    let mut file = File::create("shutdown.txt").unwrap();
    file.write_all(b"Shutdown signal received").unwrap();
}
