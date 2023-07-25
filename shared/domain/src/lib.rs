#[cfg(feature = "query")]
#[macro_use]
extern crate diesel_derive_newtype;

pub mod ids;
mod post;
mod user;

pub use post::*;
pub use user::*;

pub trait UserFacingError {
    fn formatted_error(&self) -> &'static str;
}
