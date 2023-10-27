mod artist;
mod artist_releases;
mod community_rating;
mod label;
mod master;
mod master_versions;
mod release;
mod release_rating_by_user;
mod release_stats;

pub use artist::*;
pub use artist_releases::*;
pub use community_rating::*;
pub use label::*;
pub use master::*;
pub use master_versions::*;
pub use release::*;
pub use release_rating_by_user::*;
pub use release_stats::*;

pub trait Endpoint<'de> {
    type Parameters;
    type ReturnType: serde::Deserialize<'de>;

    fn build_url(
        base: &reqwest::Url,
        params: Self::Parameters,
    ) -> Result<reqwest::Url, crate::Error>;
}
