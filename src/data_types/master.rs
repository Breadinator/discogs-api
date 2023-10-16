use super::{Image, NamedResource, Track, Video};
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Master {
    /// Id of this master
    pub id: isize,
    /// Id of the main (usually oldest) release
    pub main_release: isize,
    /// Id of the most recent release
    pub most_recent_release: isize,
    pub resource_url: String,
    pub uri: String,
    pub versions_url: String,
    pub main_release_url: String,
    pub most_recent_release_url: String,
    pub num_for_sale: usize,
    /// `None` if none for sale. Not sure what the currency is (seems to be USD).
    pub lowest_price: Option<f32>,
    #[serde(default)]
    pub images: Vec<Image>,
    #[serde(default)]
    pub genres: Vec<String>,
    #[serde(default)]
    pub styles: Vec<String>,
    pub year: u16, // hmu in the year 65537
    #[serde(default)]
    pub tracklist: Vec<Track>,
    #[serde(default)]
    pub artists: Vec<NamedResource>,
    pub title: String,
    pub data_quality: String,
    #[serde(default)]
    pub videos: Vec<Video>,
}
