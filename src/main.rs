use axum::serve;
use env::state::AppState;
use routes::app::app;
use tokio::net::TcpListener;
use tower_http::trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer};
use tracing::{info, Level};
use utils::log::trace_layer_on_request;

mod env;
mod routes;
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

    serve(listener, app.into_make_service()).await.unwrap();
}
