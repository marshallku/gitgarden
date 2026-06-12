use std::borrow::Cow;

#[derive(Clone, Debug)]
pub struct Env {
    pub port: u16,
    pub host: Cow<'static, str>,
    pub token: Cow<'static, str>,
}

impl Env {
    pub fn new() -> Self {
        let port = std::env::var("PORT")
            .ok()
            .and_then(|port| port.parse().ok())
            .unwrap_or(18080);
        let host = match std::env::var("HOST") {
            Ok(host) => Cow::Owned(host),
            Err(_) => Cow::Borrowed("127.0.0.1"),
        };
        let token = match std::env::var("GITHUB_TOKEN") {
            Ok(token) => Cow::Owned(token),
            Err(_) => Cow::Borrowed(""),
        };

        Self { port, host, token }
    }
}
