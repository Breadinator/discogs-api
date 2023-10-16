use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Video {
    pub uri: String,
    pub title: String,
    pub description: String,
}
