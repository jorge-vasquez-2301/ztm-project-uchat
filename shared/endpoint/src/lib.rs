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

load_dotenv::load_dotenv!();

pub mod app_url {
    use std::str::FromStr;

    use url::Url;

    pub const API_URL: &str = std::env!("API_URL");

    pub fn domain_and(fragment: &str) -> Url {
        Url::from_str(API_URL)
            .and_then(|url| url.join(fragment))
            .unwrap()
    }

    pub mod user_content {
        pub const ROOT: &str = "usercontent/";
        pub const IMAGES: &str = "img/";
    }
}

// public routes
route!("/account/create" => user::CreateUser);
route!("/account/login" => user::Login);

// authorized routes
route!("/post/new" => post::NewPost);
route!("/post/react" => post::React);
route!("/post/bookmark" => post::Bookmark);
route!("/post/boost" => post::Boost);
route!("/post/vote" => post::Vote);
route!("/posts/trending" => post::TrendingPosts);
route!("/posts/home" => post::HomePosts);
route!("/posts/liked" => post::LikedPosts);
route!("/posts/bookmarked" => post::BookmarkedPosts);
