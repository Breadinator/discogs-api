use crate::Error;
use reqwest::Url;
use serde::Deserialize;

pub mod community_rating;
pub mod master;
pub mod master_versions;
pub mod release;
pub mod release_rating_by_user;
pub mod release_stats;

pub trait Endpoint<'de> {
    type Parameters;
    type ReturnType: Deserialize<'de>;

    fn build_url(base: &Url, params: Self::Parameters) -> Result<Url, Error>;
}
