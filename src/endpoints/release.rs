use super::Endpoint;
use crate::Error;
use reqwest::Url;

pub struct Release;

impl<'de> Endpoint<'de> for Release {
    type Parameters = isize;
    type ReturnType = crate::data_types::Release;

    fn build_url(base: &Url, params: Self::Parameters) -> Result<Url, Error> {
        base.join(&format!("/releases/{params}"))
            .map_err(|_| Error::UrlError)
    }
}

#[cfg(test)]
mod tests {
    use super::Release;
    use crate::{Auth, Client};

    #[test]
    fn basic() {
        let id = 27651927;
        let _data = Client::builder()
            .build()
            .unwrap()
            .get::<Release>(id)
            .unwrap();
    }

    #[test]
    fn with_auth() {
        let id = 27651927;
        let pat = std::env::var("discogs-pat")
            .expect("expected personal access token in env var `discogs-pat`");

        let data = Client::builder()
            .auth(Auth::Token(pat))
            .build()
            .unwrap()
            .get::<Release>(id)
            .unwrap();

        assert!(!data.images[0].resource_url.is_empty());
    }
}
