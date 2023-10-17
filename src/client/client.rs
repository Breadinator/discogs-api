use super::{Auth, ClientBuilder};
use crate::{endpoints::Endpoint, Error, ParsedResponse};
use reqwest::Url;

pub struct Client {
    pub(super) client: reqwest::blocking::Client,
    pub(super) auth: Option<Auth>,
    pub(super) url_base: Url,
}

impl Client {
    #[must_use]
    pub fn builder() -> ClientBuilder {
        ClientBuilder::new()
    }

    pub fn get<'de, E: Endpoint<'de>>(
        &self,
        params: E::Parameters,
    ) -> Result<ParsedResponse<E::ReturnType>, Error> {
        let mut url = E::build_url(&self.url_base, params)?;

        if let Some(Auth::Token(token)) = &self.auth {
            url.query_pairs_mut().append_pair("token", token);
        }

        self.client
            .get(url)
            .send()
            .map_err(Error::NetError)
            .and_then(ParsedResponse::new)
    }
}
