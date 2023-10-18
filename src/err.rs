use crate::ParsedResponse;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct ErrorResponse {
    pub message: String,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0}")]
    NetError(#[from] reqwest::Error),
    #[error("{0}")]
    ParseError(#[from] serde_json::Error),
    #[error("{0:?}")]
    DiscogsError(ParsedResponse<ErrorResponse>),
    #[error("failed to build url")]
    UrlError,
}
