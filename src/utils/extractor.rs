use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
};

pub struct ExtractFullOrigin(pub String);

#[async_trait]
impl<S> FromRequestParts<S> for ExtractFullOrigin
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let scheme = parts.uri.scheme_str().unwrap_or(
            parts
                .headers
                .get("x-forwarded-proto")
                .map(|v| v.to_str().unwrap())
                .unwrap_or("http"),
        );
        let host = parts.uri.host().unwrap_or(
            parts
                .headers
                .get("host")
                .or(parts.headers.get("x-forwarded-host"))
                .map(|v| v.to_str().unwrap())
                .unwrap_or("localhost"),
        );
        let port = parts.uri.port_u16().unwrap_or(
            parts
                .headers
                .get("x-forwarded-port")
                .map(|v| v.to_str().unwrap())
                .unwrap_or("48092")
                .parse()
                .unwrap(),
        );

        Ok(ExtractFullOrigin(format!("{}://{}:{}", scheme, host, port)))
    }
}
