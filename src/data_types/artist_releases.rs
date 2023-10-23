use super::{Pagination, Release};
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct ArtistReleases {
    pub pagination: Pagination,
    pub releases: Vec<Release>,
}
