use std::borrow::Cow;

#[derive(Clone, Debug)]
pub struct Env {
    pub port: u16,
    pub host: Cow<'static, str>,
}

impl Env {
    pub fn new() -> Self {
        let port = match std::env::var("PORT") {
            Ok(port) => port.parse().unwrap_or(41890),
            Err(_) => 41890,
        };
        let host = match std::env::var("HOST") {
            Ok(host) => Cow::Owned(host),
            Err(_) => Cow::Owned("http://localhost/".to_string()),
        };

        Self { port, host }
    }
}
