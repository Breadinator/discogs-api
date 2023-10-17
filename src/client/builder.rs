use super::{Auth, Client};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum BuildError {
    #[error("{0}")]
    ReqwestError(#[from] reqwest::Error),
}

#[derive(Default)]
pub struct ClientBuilder {
    auth: Option<Auth>,
    client: Option<reqwest::blocking::Client>,
    user_agent: Option<String>,
}

impl ClientBuilder {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(self) -> Result<Client, BuildError> {
        let client = if let Some(client) = self.client {
            client
        } else if let Some(user_agent) = self.user_agent {
            reqwest::blocking::Client::builder()
                .user_agent(user_agent)
                .build()?
        } else {
            Default::default()
        };

        Ok(Client {
            client,
            auth: self.auth,
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
