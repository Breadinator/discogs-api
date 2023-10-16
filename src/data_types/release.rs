use super::{Image, NamedResource, Track, Video};
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Release {
    pub id: isize,
    pub resource_url: String,
    pub uri: String,
    #[serde(default)]
    pub artists: Vec<NamedResource>,
    #[serde(default)]
    pub labels: Vec<NamedResource>,
    #[serde(default)]
    pub companies: Vec<NamedResource>,
    pub master_id: isize,
    pub master_url: String,
    pub title: String,
    pub country: String,
    pub released: String,
    pub released_formatted: String,
    pub notes: String,
    #[serde(default)]
    pub videos: Vec<Video>,
    #[serde(default)]
    pub genres: Vec<String>,
    #[serde(default)]
    pub styles: Vec<String>,
    #[serde(default)]
    pub tracklist: Vec<Track>,
    #[serde(default)]
    pub images: Vec<Image>,
    pub thumb: String,
}
