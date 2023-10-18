#![warn(clippy::pedantic)]
#![allow(
    clippy::module_name_repetitions,
    clippy::module_inception,
    clippy::unreadable_literal
)]
//
// temp lint allows
#![allow(clippy::missing_errors_doc)]

pub mod client;
pub use client::{Auth, Client};
pub mod data_types;
pub mod endpoints;
mod err;
mod parsed_response;
pub use err::*;
pub use parsed_response::*;

pub mod prelude {
    pub use super::endpoints::*;
    pub use super::{Auth, Client};
}
