use reqwest::Url;

use crate::Error;

use super::Endpoint;

pub struct ReleaseRatingByUser;

#[derive(Debug, Clone, Copy)]
pub struct Params<'a> {
    pub release_id: isize,
    pub username: &'a str,
}

impl<'de> Endpoint<'de> for ReleaseRatingByUser {
    type Parameters = Params<'de>;
    type ReturnType = serde_json::Value;

    fn build_url(base: &Url, params: Self::Parameters) -> Result<Url, Error> {
        base.join(&format!(
            "/releases/{}/rating/{}",
            params.release_id, params.username
        ))
        .map_err(|_| Error::UrlError)
    }
}

#[cfg(test)]
mod tests {
    use crate::Client;

    #[test]
    fn empty_username() {
        let params = super::Params {
            release_id: 27651927,
            username: "",
        };
        let resp = Client::builder()
            .build()
            .unwrap()
            .get::<super::ReleaseRatingByUser>(params)
            .unwrap();
        dbg![resp];
    }
}
