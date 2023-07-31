use serde::{Deserialize, Serialize};

pub mod post;
pub mod user;

pub trait Endpoint {
    const URL: &'static str;

    fn url(&self) -> &'static str {
        Self::URL
    }
}

macro_rules! route {
    ($url:literal => $request_type:ty) => {
        impl Endpoint for $request_type {
            const URL: &'static str = $url;
        }
    };
}

#[derive(thiserror::Error, Debug, Deserialize, Serialize)]
#[error("{msg}")]
pub struct RequestFailed {
    pub msg: String,
}

// public routes
route!("/account/create" => user::CreateUser);
route!("/account/login" => user::Login);

// authorized routes
route!("/post/new" => post::NewPost);
route!("/post/react" => post::React);
route!("/post/bookmark" => post::Bookmark);
route!("/posts/trending" => post::TrendingPosts);
