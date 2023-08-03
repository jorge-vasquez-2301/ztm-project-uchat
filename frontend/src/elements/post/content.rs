#![allow(non_snake_case)]

use dioxus::prelude::*;
use uchat_domain::ids::PostId;
use uchat_endpoint::post::{ImageKind, PublicPost};

#[inline_props]
pub fn Content<'a>(cx: Scope<'a>, post: &'a PublicPost) -> Element<'a> {
    cx.render(rsx! {
        div {
            match &post.content {
                uchat_endpoint::post::Content::Chat(content) => rsx! { Chat { post_id: post.id, content: content } },
                uchat_endpoint::post::Content::Image(content) => rsx! { Image { post_id: post.id, content: content } },
            }
        }
    })
}

#[inline_props]
pub fn Chat<'a>(
    cx: Scope<'a>,
    post_id: PostId,
    content: &'a uchat_endpoint::post::Chat,
) -> Element<'a> {
    let Headline = content.headline.as_ref().map(|headline| {
        rsx! {
            div {
                class: "font-bold",
                "{headline.as_ref()}"
            }
        }
    });

    cx.render(rsx! {
        div {
            Headline,
            p { "{content.message.as_ref()}" }
        }
    })
}

#[inline_props]
pub fn Image<'a>(
    cx: Scope<'a>,
    post_id: PostId,
    content: &'a uchat_endpoint::post::Image,
) -> Element<'a> {
    let ImageKind::Url(ref url) = content.kind else {
        return cx.render(rsx! { "image not found" });
    };

    let Caption = content
        .caption
        .as_ref()
        .map(|caption| rsx! { figcaption { em { "{caption.as_ref()}" } } });

    cx.render(rsx! {
        figure {
            class: "flex flex-col gap2",
            Caption,
            img {
                class: "w-full object-contain max-h-[80vh]",
                src: "{url}"
            }
        }
    })
}
