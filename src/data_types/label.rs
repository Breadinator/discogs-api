use super::{Image, NamedResource};
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Label {
    pub id: isize,
    pub name: String,
    pub resource_url: String,
    pub uri: String,
    pub releases_url: String,
    pub images: Vec<Image>,
    pub contact_info: String,
    pub profile: String,
    pub data_quality: String,
    pub urls: Vec<String>,
    pub sublabels: Option<Vec<NamedResource>>,
    pub parent_label: Option<NamedResource>,
}
