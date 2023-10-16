use crate::endpoints::Endpoint;

pub const API_BASE: &str = "https://api.discogs.com";

#[inline]
pub fn get<'de, E: Endpoint<'de>>(
    params: E::Parameters,
) -> Result<reqwest::blocking::Response, reqwest::Error> {
    reqwest::blocking::ClientBuilder::new()
        .user_agent("Mozilla/5.0 (Linux; Android 10; K) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/117.0.0.0 Mobile Safari/537.3")
        .build()?
        .get(format!("{API_BASE}{0}", E::get_endpoint(params)))
        .send()
}

#[inline]
pub fn get_with_auth<'de, E: Endpoint<'de>>(
    params: E::Parameters,
    personal_access_token: &'_ str,
) -> Result<reqwest::blocking::Response, reqwest::Error> {
    reqwest::blocking::ClientBuilder::new()
        .user_agent("Mozilla/5.0 (Linux; Android 10; K) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/117.0.0.0 Mobile Safari/537.3")
        .build()?
        .get(format!("{API_BASE}{0}", E::get_endpoint_with_auth(params, personal_access_token)))
        .send()
}
