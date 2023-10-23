use super::{Image, MasterVersionsItemStatsCommAndUser, NamedResource, Track, Video};
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Release {
    pub id: isize,
    pub resource_url: String,
    pub uri: Option<String>,
    pub artists: Option<Vec<NamedResource>>,
    pub artist: Option<String>,
    pub labels: Option<Vec<NamedResource>>,
    pub companies: Option<Vec<NamedResource>>,
    pub master_id: Option<isize>,
    pub master_url: Option<String>,
    pub title: String,
    pub country: Option<String>,
    pub released: Option<String>,
    pub released_formatted: Option<String>,
    pub notes: Option<String>,
    pub videos: Option<Vec<Video>>,
    pub genres: Option<Vec<String>>,
    pub styles: Option<Vec<String>>,
    pub tracklist: Option<Vec<Track>>,
    pub images: Option<Vec<Image>>,
    /// An empty string when unauthenticated.
    pub thumb: String,
    pub stats: Option<MasterVersionsItemStatsCommAndUser>,
}
