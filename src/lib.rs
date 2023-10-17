#![warn(clippy::pedantic)]
#![allow(
    clippy::module_name_repetitions,
    clippy::module_inception,
    clippy::unreadable_literal
)]
//
// temp lint allows
#![allow(clippy::missing_errors_doc)]

use serde::Deserialize;
use std::ops::Deref;

pub mod client;
pub mod data_types;
pub mod endpoints;

pub use client::{Auth, Client};

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
