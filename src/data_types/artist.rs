use super::Image;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Member {
    pub id: isize,
    pub name: String,
    pub resource_url: String,
    pub active: bool,
    /// None if not authenticated
    pub thumbnail_url: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Artist {
    pub name: String,
    pub id: isize,
    pub resource_url: String,
    pub uri: String,
    pub releases_url: String,
    #[serde(default)]
    pub images: Vec<Image>,
    pub realname: Option<String>,
    pub profile: String,
    #[serde(default)]
    pub urls: Vec<String>,
    pub namevariations: Option<Vec<String>>,
    pub members: Option<Vec<Member>>,
    pub data_quality: String,
}
