use super::Endpoint;
use crate::Error;
use reqwest::Url;

pub struct CommunityRating;

impl<'de> Endpoint<'de> for CommunityRating {
    type Parameters = isize;
    type ReturnType = crate::data_types::ReleaseRating;

    fn build_url(base: &Url, params: Self::Parameters) -> Result<Url, Error> {
        base.join(&format!("/releases/{params}/rating"))
            .map_err(|_| Error::UrlError)
    }
}

#[cfg(test)]
mod tests {
    use super::CommunityRating;

    #[test]
    fn basic() {
        let id = 27651927;
        let _data = crate::Client::builder()
            .build()
            .unwrap()
            .get::<CommunityRating>(id)
            .unwrap();
    }
}
