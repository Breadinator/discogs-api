use endpoints::Endpoint;
use serde::Deserialize;
use std::ops::Deref;

pub mod data_types;
pub mod endpoints;

pub(crate) mod requester;

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
}

/// Wrapper for deserialized data.
///
/// # Safety
/// This contains a raw pointer to the underlying bytes of the response used to get the data.
/// However, this is only used on drop to deallocate it (after which there are no valid references
/// to `data`) and cannot be constructed such that it can be invalidated at drop.
pub struct ParsedResponse<T> {
    b: *mut [u8],
    data: T,
}

impl<T: std::fmt::Debug> std::fmt::Debug for ParsedResponse<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{:?}", self.data))
    }
}

impl<T> Drop for ParsedResponse<T> {
    fn drop(&mut self) {
        unsafe {
            drop(Box::from_raw(self.b));
        }
    }
}

impl<T> Deref for ParsedResponse<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<'de, T: Deserialize<'de>> ParsedResponse<T> {
    fn new(resp: reqwest::blocking::Response) -> Result<Self, Error> {
        let b: *mut [u8] = Box::leak(Box::from(resp.bytes()?.as_ref())) as *mut [u8];
        let data = serde_json::from_slice(unsafe { b.as_ref().unwrap() })?;
        Ok(Self { b, data })
    }
}

pub fn get<'de, E: Endpoint<'de>>(
    params: E::Parameters,
) -> Result<ParsedResponse<E::ReturnType>, Error> {
    let resp = requester::get::<'de, E>(params)?;
    if resp.status().is_success() {
        ParsedResponse::new(resp)
    } else {
        Err(Error::DiscogsError(ParsedResponse::new(resp)?))
    }
}

pub fn get_with_auth<'de, E: Endpoint<'de>>(
    params: E::Parameters,
    personal_access_token: &'_ str,
) -> Result<ParsedResponse<E::ReturnType>, Error> {
    let resp = requester::get_with_auth::<E>(params, personal_access_token)?;
    if resp.status().is_success() {
        ParsedResponse::new(resp)
    } else {
        Err(Error::DiscogsError(ParsedResponse::new(resp)?))
    }
}

pub(crate) fn build_params(params: &[(&str, &str)]) -> String {
    if params.is_empty() {
        return String::new();
    }

    let len = params
        .iter()
        .fold(0, |acc, p| acc + 2 + p.0.len() + p.1.len());
    let mut out = String::with_capacity(len);

    let mut params_iter = params.iter();
    let param = params_iter.next().unwrap(); // fine to unwrap because already checked for empty
    out.push('?');
    out.push_str(param.0);
    out.push('=');
    out.push_str(param.1);

    for param in params_iter {
        out.push('&');
        out.push_str(param.0);
        out.push('=');
        out.push_str(param.1);
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_params() {
        let pairs = [("master_id", "3175560"), ("page", "1"), ("per_page", "25")];
        let built = build_params(&pairs);
        assert_eq!(&built, "?master_id=3175560&page=1&per_page=25");
    }
}
