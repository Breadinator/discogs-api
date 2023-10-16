use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Track {
    pub position: String,
    pub title: String,
    pub duration: String,
}
