use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct NamedResource {
    pub id: isize,
    pub name: String,
    pub resource_url: String,
}
