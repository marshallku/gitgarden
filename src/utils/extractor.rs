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
                .map(|v| v.to_str().unwrap_or("http"))
                .unwrap_or("http"),
        );
        let host = parts.uri.host().unwrap_or(
            parts
                .headers
                .get("host")
                .or(parts.headers.get("x-forwarded-host"))
                .map(|v| v.to_str().unwrap_or("localhost"))
                .unwrap_or("localhost"),
        );
        let port = if host.contains(':') {
            "".to_string()
        } else {
            format!(
                ":{}",
                parts.uri.port_u16().unwrap_or(
                    parts
                        .headers
                        .get("x-forwarded-port")
                        .map(|v| v.to_str().unwrap_or("80"))
                        .unwrap_or("80")
                        .parse()
                        .unwrap_or(80),
                )
            )
        };

        Ok(ExtractFullOrigin(format!("{}://{}{}", scheme, host, port)))
    }
}
