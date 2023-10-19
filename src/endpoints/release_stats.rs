use reqwest::Url;

use crate::Error;

use super::Endpoint;

pub struct ReleaseStats;

impl<'de> Endpoint<'de> for ReleaseStats {
    type Parameters = isize;
    type ReturnType = crate::data_types::ReleaseStats;

    fn build_url(base: &Url, params: Self::Parameters) -> Result<Url, Error> {
        base.join(&format!("/releases/{params}/stats"))
            .map_err(|_| Error::UrlError)
    }
}

#[cfg(test)]
mod tests {
    use super::ReleaseStats;
    use crate::Client;

    #[test]
    fn basic() {
        let id = 27736512;
        let data = dbg![Client::builder()
            .build()
            .unwrap()
            .get::<ReleaseStats>(id)
            .unwrap()];
        assert!(!data.is_offensive);
    }
}
