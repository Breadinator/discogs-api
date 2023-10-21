use super::{Auth, ClientBuilder};
use crate::{endpoints::Endpoint, Error, ParsedResponse};
use reqwest::Url;
use serde::Deserialize;

pub struct Client {
    pub(super) client: reqwest::blocking::Client,
    pub(super) auth: Option<Auth>,
    pub(super) url_base: Url,
}

impl Client {
    #[must_use]
    #[inline]
    pub fn builder() -> ClientBuilder {
        ClientBuilder::new()
    }

    #[inline]
    pub fn get<'de, E: Endpoint<'de>>(
        &self,
        params: E::Parameters,
    ) -> Result<ParsedResponse<E::ReturnType>, Error> {
        self.get_custom_return_type::<E, E::ReturnType>(params)
    }

    /// This method is here to allow users of this library
    /// to change the type the response is parsed as.
    ///
    /// Use `Client::get` where possible to use the regular types.<br/>
    /// Use [`serde_json::Value`] as `R` to parse any valid response.
    #[inline]
    pub fn get_custom_return_type<'a, 'b, E: Endpoint<'a>, R: Deserialize<'b>>(
        &self,
        params: E::Parameters,
    ) -> Result<ParsedResponse<R>, Error> {
        E::build_url(&self.url_base, params).and_then(|url| self.get_url(url))
    }

    /// This method only modifies the given url in that it will add the user token if applicable
    pub fn get_url<'de, R>(&self, mut url: Url) -> Result<ParsedResponse<R>, Error>
    where
        R: Deserialize<'de>,
    {
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

impl Default for Client {
    /// Can panic. If failable, build manually.
    #[must_use]
    fn default() -> Self {
        Self::builder().build().unwrap()
    }
}
