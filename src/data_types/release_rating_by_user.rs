use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct ReleaseRatingByUser {
    pub username: String,
    pub release_id: isize,
    pub rating: u8,
}
