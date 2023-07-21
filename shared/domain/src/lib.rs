#[cfg(feature = "query")]
#[macro_use]
extern crate diesel_derive_newtype;

pub mod ids;
mod user;

pub use user::*;
