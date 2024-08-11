#[cfg(test)]
mod tests {

    use axum::{
        body::Body,
        http::{Request, StatusCode},
        routing::get,
        Router,
    };
    use tower::ServiceExt;

    use crate::routes::index;

    #[tokio::test]
    async fn should_response() {
        let app: Router = Router::new().route("/", get(index::get));
        let response = app
            .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }
}
