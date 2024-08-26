#[cfg(test)]
mod tests {

    use axum::{
        body::Body,
        http::{Request, StatusCode},
        routing::get,
        Router,
    };
    use tower::ServiceExt;

    use crate::{controllers::index, env::state::AppState};

    #[tokio::test]
    async fn should_not_response_without_queries() {
        let app: Router<AppState> = Router::new().route("/", get(index::get));
        let state = AppState::from_env();
        let response = app
            .with_state(state)
            .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn should_response_with_queries() {
        let app: Router<AppState> = Router::new().route("/", get(index::get));
        let state = AppState::from_env();
        let response = app
            .with_state(state)
            .oneshot(
                Request::builder()
                    .uri("/?user_name=marshallku&year=2024")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(
            response.headers().get("Content-Type").unwrap(),
            "image/svg+xml"
        );
    }
}
