#[cfg(test)]
mod tests {

    use axum::{
        body::Body,
        http::{Request, StatusCode},
        routing::get,
        Router,
    };
    use tower::ServiceExt;

    use crate::controllers::index;

    #[tokio::test]
    async fn should_not_response_without_queries() {
        let app: Router = Router::new().route("/", get(index::get));
        let response = app
            .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn should_response_with_queries() {
        let app: Router = Router::new().route("/", get(index::get));
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/?user_name=marshallku&year=2024")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }
}
