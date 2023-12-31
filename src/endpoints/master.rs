use super::Endpoint;
use crate::Error;
use reqwest::Url;

pub struct Master;

impl<'de> Endpoint<'de> for Master {
    type Parameters = isize;
    type ReturnType = crate::data_types::Master;

    fn build_url(base: &Url, params: Self::Parameters) -> Result<Url, Error> {
        base.join(&format!("/masters/{params}"))
            .map_err(|_| Error::UrlError)
    }
}

#[cfg(test)]
mod tests {
    use crate::{Auth, Client};

    use super::Master;

    #[test]
    fn basic() {
        let id = 3166419;
        let _data = Client::builder()
            .build()
            .unwrap()
            .get::<Master>(id)
            .unwrap();
    }

    #[test]
    fn with_auth() {
        let id = 3166419;
        let pat = std::env::var("discogs-pat")
            .expect("expected personal access token in env var `discogs-pat`");
        let _data = Client::builder()
            .auth(Auth::Token(pat))
            .build()
            .unwrap()
            .get::<Master>(id)
            .unwrap();
    }
}
