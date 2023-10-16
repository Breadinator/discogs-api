use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Pagination {
    pub page: usize,
    pub pages: usize,
    pub per_page: usize,
    pub urls: PaginationUrls,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PaginationUrls {
    pub first: Option<String>,
    pub last: Option<String>,
    pub prev: Option<String>,
    pub next: Option<String>,
}
