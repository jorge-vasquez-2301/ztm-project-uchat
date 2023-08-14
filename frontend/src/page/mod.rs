mod edit_profile;
mod home;
mod login;
mod new_post;
mod register;
mod route;
mod trending;
mod view_profile;

pub use edit_profile::EditProfile;
pub use home::{Home, HomeBookmarked, HomeLiked};
pub use login::Login;
pub use new_post::*;
pub use register::Register;
pub use route::*;
pub use trending::Trending;
pub use view_profile::ViewProfile;
