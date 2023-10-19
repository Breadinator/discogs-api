use super::Endpoint;
use crate::{data_types::Fallible, Error};
use reqwest::Url;

pub struct ReleaseRatingByUser;

#[derive(Debug, Clone, Copy)]
pub struct Params<'a> {
    pub release_id: isize,
    pub username: &'a str,
}

impl<'de> Endpoint<'de> for ReleaseRatingByUser {
    type Parameters = Params<'de>;
    type ReturnType = Fallible<crate::data_types::ReleaseRatingByUser>;

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
    use super::*;
    use crate::Client;

    #[test]
    fn empty_username() {
        let params = Params {
            release_id: 27651927,
            username: "",
        };

        let resp = Client::builder()
            .build()
            .unwrap()
            .get::<ReleaseRatingByUser>(params)
            .unwrap();
        let resp = resp.to_result().expect_err("expected failure");

        assert_eq!(resp.as_str(), "The requested resource was not found.");
    }

    #[test]
    fn with_username() {
        let params = Params {
            release_id: 27651927,
            username: "br3adina7or",
        };

        let resp = Client::builder()
            .build()
            .unwrap()
            .get::<ReleaseRatingByUser>(params)
            .unwrap();
        let resp = resp.to_result().expect("expected ok");
        assert_eq!(resp.release_id, params.release_id);
        assert_eq!(&resp.username, params.username);
    }
}
