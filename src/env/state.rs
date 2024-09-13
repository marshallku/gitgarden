use super::app::Env;
use dotenv::dotenv;

#[derive(Clone)]
pub struct AppState {
    pub host: String,
    pub port: u16,
    pub token: String,
}

impl AppState {
    pub fn from_env() -> Self {
        dotenv().ok();

        let env = Env::new();

        Self {
            host: env.host.into_owned(),
            port: env.port,
            token: env.token.into_owned(),
        }
    }
}
