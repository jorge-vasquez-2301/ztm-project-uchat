#![allow(non_snake_case)]

use chrono::Duration;
use dioxus::prelude::*;

use crate::{elements::PublicPostEntry, prelude::*};

pub fn Trending(cx: Scope) -> Element {
    let api_client = ApiClient::global();
    let router = use_router(cx);
    let toaster = use_toaster(cx);
    let post_manager = use_post_manager(cx);

    let _fetch_trending_posts = {
        to_owned![api_client, toaster, post_manager];
        use_future(cx, (), |_| async move {
            use uchat_endpoint::post::{TrendingPosts, TrendingPostsOk};
            toaster
                .write()
                .info("Retrieving trending posts", Duration::seconds(3));
            post_manager.write().clear();
            let response = fetch_json!(<TrendingPostsOk>, api_client, TrendingPosts);
            match response {
                Ok(res) => post_manager.write().populate(res.posts.into_iter()),
                Err(e) => toaster.write().error(
                    format!("Failed to retrieve posts: {e}"),
                    Duration::seconds(3),
                ),
            }
        })
    };

    let TrendingPosts = post_manager
        .read()
        .posts
        .iter()
        .map(|(&id, _)| {
            rsx! {
                div {
                    PublicPostEntry  { post_id: id }
                }
            }
        })
        .collect::<Vec<_>>();

    cx.render(rsx! {
        Appbar  {
           title: "Trending Posts",
           AppbarImgButton {
               click_handler: move |_| router.pop_route(),
               img: "/static/icons/icon-back.svg",
               label: "Back",
               title: "Go to the previous page"
           }
        },
        TrendingPosts.into_iter()
    })
}
