use crate::Error;
use serde::Deserialize;
use std::ops::Deref;

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
    pub(crate) fn new(resp: reqwest::blocking::Response) -> Result<Self, Error> {
        let b: *mut [u8] = Box::leak(Box::from(resp.bytes()?.as_ref())) as *mut [u8];
        let data = serde_json::from_slice(unsafe { b.as_ref().unwrap() })?;
        Ok(Self { b, data })
    }
}
