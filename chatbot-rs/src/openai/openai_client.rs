use miette::{Context, IntoDiagnostic, Result};
use reqwest::header::{HeaderValue, AUTHORIZATION};

pub static APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);

#[derive(Debug, Clone)]
pub struct Config {
    api_key: String,
}

pub struct Client(
    pub reqwest::Client
);

impl Config {
    pub fn from_env() -> Result<Self> {
        let api_key = std::env::var("OPENAI_API_KEY")
            .into_diagnostic()
            .wrap_err("Could not find OPENAI_API_KEY env var")?;

        Ok(Self { api_key })
    }

    pub fn client(&self) -> Result<Client> {
        let mut headers = reqwest::header::HeaderMap::new();

        let value = format!("Bearer {}", self.api_key);
        let mut value = HeaderValue::from_str(&value)
            .into_diagnostic()
            .wrap_err("Could not create header value")?;
        value.set_sensitive(true);

        headers.insert(AUTHORIZATION, value);

        let client = reqwest::Client::builder()
            .user_agent(APP_USER_AGENT)
            .default_headers(headers)
            .build()
            .into_diagnostic()
            .wrap_err("Could not build reqwest client")?;

        Ok(Client(client))
    }
}