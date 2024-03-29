#![allow(non_snake_case)]

mod action_bar;
mod content;
mod quick_respond;

use crate::{
    app::POST_MANAGER,
    elements::post::{action_bar::ActionBar, content::Content},
    page::profile_view,
    sync_handler,
};
use dioxus::prelude::*;
use dioxus_router::{use_router, RouterContext};
use fermi::{use_atom_ref, UseAtomRef};
use indexmap::IndexMap;
use uchat_domain::ids::{PostId, UserId};
use uchat_endpoint::post::PublicPost;

pub fn use_post_manager(cx: &ScopeState) -> &UseAtomRef<PostManager> {
    use_atom_ref(cx, POST_MANAGER)
}

#[derive(Default)]
pub struct PostManager {
    pub posts: IndexMap<PostId, PublicPost>,
}

impl PostManager {
    pub fn update<F>(&mut self, id: PostId, mut update_fn: F) -> bool
    where
        F: FnMut(&mut PublicPost),
    {
        match self.posts.get_mut(&id) {
            Some(post) => {
                update_fn(post);
                true
            }
            None => false,
        }
    }

    pub fn populate<T>(&mut self, posts: T)
    where
        T: Iterator<Item = PublicPost>,
    {
        self.clear();
        for post in posts {
            self.posts.insert(post.id, post);
        }
    }

    pub fn clear(&mut self) {
        self.posts.clear();
    }

    pub fn get(&self, post_id: &PostId) -> Option<&PublicPost> {
        self.posts.get(post_id)
    }

    pub fn remove(&mut self, post_id: &PostId) {
        self.posts.remove(post_id);
    }

    pub fn all_to_public<'a, 'b>(&self) -> Vec<LazyNodes<'a, 'b>> {
        self.posts
            .iter()
            .map(|(&id, _)| {
                rsx! {
                    div {
                        PublicPostEntry {
                            post_id: id
                        }
                    }
                }
            })
            .collect()
    }
}

pub fn view_profile_onclick(
    router: &RouterContext,
    user_id: UserId,
) -> impl FnMut(MouseEvent) + '_ {
    sync_handler!([router], move |_| {
        let route = profile_view(user_id);
        router.navigate_to(&route);
    })
}

#[inline_props]
pub fn ProfileImage<'a>(cx: Scope<'a>, post: &'a PublicPost) -> Element<'a> {
    let router = use_router(cx);

    let poster_info = &post.by_user;

    let profile_img_src = poster_info
        .profile_image
        .as_ref()
        .map(|url| url.as_str())
        .unwrap_or_else(|| "");

    cx.render(rsx! {
        div {
            img {
                class: "profile-portrait cursor-pointer",
                onclick: view_profile_onclick(router, post.by_user.id),
                src: "{profile_img_src}"
            }
        }
    })
}

#[inline_props]
pub fn Header<'a>(cx: Scope<'a>, post: &'a PublicPost) -> Element<'a> {
    let (post_date, post_time) = {
        let date = post.time_posted.format("%Y-%m-%d");
        let time = post.time_posted.format("%H-%M-%S");
        (date, time)
    };

    let display_name = post
        .by_user
        .display_name
        .as_ref()
        .map(|name| name.as_ref())
        .unwrap_or_else(|| "");

    let handle = post.by_user.handle.as_str();

    cx.render(rsx! {
        div {
            class: "flex flex-row justify-between",
            div {
                class: "cursor-pointer",
                onclick: |_|(),
                div { "{display_name}" },
                div {
                    class: "font-light",
                    "{handle}"
                }
            },
            div {
                class: "text-right",
                div { "{post_date}" },
                div { "{post_time}" },
            }
        }
    })
}

#[inline_props]
pub fn PublicPostEntry(cx: Scope, post_id: PostId) -> Element {
    let post_manager = use_post_manager(cx);

    let this_post = {
        let post = post_manager.read().get(&post_id).unwrap().clone();
        use_state(cx, || post)
    };

    cx.render(rsx! {
        div {
            key: "{this_post.id.to_string()}",
            class: "grid grid-cols-[50px_1fr] gap-2 mb-4",
            ProfileImage { post: this_post },
            div {
                class: "flex flex-col gap-3",
                Header { post: this_post },
                // reply to
                Content { post: this_post },
                ActionBar { post_id: this_post.id },
                hr {}
            }
        }
    })
}
