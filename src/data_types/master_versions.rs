use super::Pagination;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct MasterVersions {
    pub pagination: Pagination,
    #[serde(default)]
    pub versions: Vec<MasterVersionsItem>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MasterVersionsItem {
    pub id: isize,
    pub label: String,
    pub country: String,
    pub title: String,
    #[serde(default)]
    pub major_formats: Vec<String>,
    pub released: String,
    pub status: String,
    pub resource_url: String,
    pub thumb: String,
    pub stats: MasterVersionsItemStatsCommAndUser,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MasterVersionsItemStatsCommAndUser {
    pub community: MasterVersionsItemStats,
    pub user: Option<MasterVersionsItemStats>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MasterVersionsItemStats {
    pub in_collection: usize,
    pub in_wantlist: usize,
}
