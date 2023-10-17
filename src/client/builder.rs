use super::{Auth, Client};
use reqwest::Url;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum BuildError {
    #[error("{0}")]
    ReqwestError(#[from] reqwest::Error),
    #[error("couldn't parse base url")]
    UrlParseError,
}

#[derive(Default)]
pub struct ClientBuilder {
    auth: Option<Auth>,
    client: Option<reqwest::blocking::Client>,
    user_agent: Option<String>,
    base_url: Option<Url>,
}

impl ClientBuilder {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(self) -> Result<Client, BuildError> {
        let client = if let Some(client) = self.client {
            client
        } else {
            reqwest::blocking::Client::builder()
                .user_agent(self.user_agent.unwrap_or_else(|| String::from("Mozilla/5.0 (Linux; Android 10; K) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/117.0.0.0 Mobile Safari/537.3")))
                .build()?
        };

        let url_base = if let Some(url_base) = self.base_url {
            url_base
        } else if let Ok(url_base) = Url::parse("https://api.discogs.com") {
            url_base
        } else {
            return Err(BuildError::UrlParseError);
        };

        Ok(Client {
            client,
            auth: self.auth,
            url_base,
        })
    }

    #[must_use]
    pub fn auth(mut self, auth: Auth) -> Self {
        self.auth = Some(auth);
        self
    }

    #[must_use]
    pub fn client(mut self, client: reqwest::blocking::Client) -> Self {
        self.client = Some(client);
        self
    }

    #[must_use]
    pub fn user_agent(mut self, user_agent: String) -> Self {
        self.user_agent = Some(user_agent);
        self
    }
}
