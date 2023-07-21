#[cfg(feature = "query")]
#[macro_use]
extern crate diesel_derive_newtype;

pub mod ids;
mod user;

pub use user::*;

pub trait UserFacingError {
    fn formatted_error(&self) -> &'static str;
}
