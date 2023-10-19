use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Clone, Deserialize)]
pub struct ReleaseStats {
    pub is_offensive: bool,

    /// This `additional_fields` field is here because the api says it's meant to show more
    /// but I can't find any releases that actually do.
    #[serde(flatten)]
    pub additional_fields: HashMap<String, Value>,
}
