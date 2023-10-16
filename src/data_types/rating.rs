use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Rating {
    pub count: usize,
    pub average: f32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ReleaseRating {
    pub release_id: isize,
    pub rating: Rating,
}
