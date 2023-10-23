use super::Endpoint;
use crate::{data_types::SortOrder, Error};

pub struct ArtistReleases;

#[derive(Debug, Clone, Default)]
pub struct ArtistReleasesParams {
    pub artist_id: isize,
    /// Seemes to be `year`, `title`, or `format`
    pub sort: Option<String>,
    pub sort_order: Option<SortOrder>,
}

impl Endpoint<'_> for ArtistReleases {
    type Parameters = ArtistReleasesParams;
    type ReturnType = crate::data_types::ArtistReleases;

    fn build_url(
        base: &reqwest::Url,
        params: Self::Parameters,
    ) -> Result<reqwest::Url, crate::Error> {
        let mut out = base
            .join(&format!("/artists/{}/releases", params.artist_id))
            .map_err(|_| Error::UrlError)?;

        // idk if preallocating is better here
        // i also tried a match stmt over a tuple of the two options
        // seems trivially different, but could be interesting to measure
        let mut pairs = Vec::with_capacity(
            usize::from(params.sort.is_some()) + usize::from(params.sort_order.is_some()),
        );
        if let Some(sort) = params.sort.as_deref() {
            pairs.push(("sort", sort));
        }
        if let Some(sort_order) = params.sort_order.as_ref() {
            pairs.push(("sort_order", sort_order.as_ref()));
        }
        out.query_pairs_mut().extend_pairs(pairs);

        Ok(out)
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::prelude::*;

    #[test]
    fn basic() {
        let artist_id = 3946211;
        let params = ArtistReleasesParams {
            artist_id,
            ..Default::default()
        };

        let client = Client::default();
        let data = client.get::<ArtistReleases>(params).unwrap();

        assert_eq!(data.pagination.page, 1);
        assert!(!data.releases.is_empty());
        assert!(!data.releases[0].title.is_empty());
    }

    #[test]
    fn with_auth() {
        let artist_id = 2445772;
        let params = ArtistReleasesParams {
            artist_id,
            ..Default::default()
        };
        let pat = std::env::var("discogs-pat")
            .expect("expected personal access token in env var `discogs-pat`");

        let client = Client::builder().auth(Auth::Token(pat)).build().unwrap();
        let data = client.get::<ArtistReleases>(params).unwrap();

        assert!(!data.releases[0].thumb.is_empty());
        assert!(data.releases[0].stats.is_some());
    }
}
