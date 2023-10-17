use super::{Auth, ClientBuilder, Params};

pub struct Client {
    pub(super) client: reqwest::blocking::Client,
    pub(super) auth: Option<Auth>,
}

impl Client {
    #[must_use]
    pub fn builder() -> ClientBuilder {
        ClientBuilder::new()
    }

    pub(crate) fn get(
        &self,
        base: &str,
        endpoint: &str,
        mut params: Params,
    ) -> Result<reqwest::blocking::Response, reqwest::Error> {
        if let Some(Auth::Token(token)) = &self.auth {}

        let built_params: String = params.into();
        let mut url = String::with_capacity(base.len() + endpoint.len() + built_params.len());
        url.push_str(base);
        url.push_str(endpoint);
        url.push_str(&built_params);

        self.client.get(url).send()
    }
}
