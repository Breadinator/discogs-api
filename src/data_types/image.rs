use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct ImageDimensions {
    pub height: usize,
    pub width: usize,
}

#[derive(Debug, Clone, Deserialize)]
pub enum ImageType {
    #[serde(rename = "primary")]
    Primary,
    #[serde(rename = "secondary")]
    Secondary,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Image {
    #[serde(flatten)]
    pub dimensions: ImageDimensions,
    pub resource_url: String,
    pub r#type: ImageType,
    pub uri: String,
    pub uri150: String,
}
