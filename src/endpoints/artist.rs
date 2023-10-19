use crate::Error;

use super::Endpoint;

pub struct Artist;

impl Endpoint<'_> for Artist {
    type Parameters = isize;
    type ReturnType = crate::data_types::Artist;

    fn build_url(base: &reqwest::Url, params: Self::Parameters) -> Result<reqwest::Url, Error> {
        base.join(&format!("/artists/{params}"))
            .map_err(|_| Error::UrlError)
    }
}

#[cfg(test)]
mod tests {
    use super::Artist;
    use crate::{Auth, Client};

    #[test]
    fn basic() {
        let id = 8298093;
        let data = Client::builder()
            .build()
            .unwrap()
            .get::<Artist>(id)
            .unwrap();
        assert_eq!(data.id, id);
        assert_eq!(data.name.as_str(), "Meenoi");
    }

    #[test]
    fn with_auth() {
        let id = 5210284;
        let pat = std::env::var("discogs-pat")
            .expect("expected personal access token in env var `discogs-pat`");

        let data = Client::builder()
            .auth(Auth::Token(pat))
            .build()
            .unwrap()
            .get::<Artist>(id)
            .unwrap();

        assert_eq!(data.name.as_str(), "BLACKPINK");
        assert_eq!(data.id, id);
        assert_eq!(data.realname.as_ref().unwrap().as_str(), "블랙핑크");
        assert!(!data.urls.is_empty());

        let member_ids = [6771863, 6771866, 6771868, 6771905];
        for member in data.members.as_ref().unwrap() {
            assert!(
                member_ids.contains(&member.id),
                "found unexpected member id, `{}`",
                member.id
            );
        }

        assert!(!data.images[0].resource_url.is_empty());
    }
}
