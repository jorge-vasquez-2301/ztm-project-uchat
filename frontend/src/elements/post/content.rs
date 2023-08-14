#![allow(non_snake_case)]

use std::collections::HashSet;

use dioxus::prelude::*;
use itertools::Itertools;
use uchat_domain::ids::{PollChoiceId, PostId};
use uchat_endpoint::post::{ImageKind, PublicPost, VoteCast};

use crate::prelude::*;

#[inline_props]
pub fn Content<'a>(cx: Scope<'a>, post: &'a PublicPost) -> Element<'a> {
    cx.render(rsx! {
        div {
            match &post.content {
                uchat_endpoint::post::Content::Chat(content) => rsx! { Chat { post_id: post.id, content: content } },
                uchat_endpoint::post::Content::Image(content) => rsx! { Image { post_id: post.id, content: content } },
                uchat_endpoint::post::Content::Poll(content) => rsx! { Poll { post_id: post.id, content: content } },
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

#[inline_props]
pub fn Poll<'a>(
    cx: Scope<'a>,
    post_id: PostId,
    content: &'a uchat_endpoint::post::Poll,
) -> Element<'a> {
    let toaster = use_toaster(cx);
    let api_client = ApiClient::global();

    let vote_onclick = async_handler!(
        &cx,
        [api_client, toaster],
        move |post_id, choice_id| async move {
            use uchat_endpoint::post::{Vote, VoteOk};

            let request_data = Vote { post_id, choice_id };
            match fetch_json!(<VoteOk>, api_client, request_data) {
                Ok(VoteOk {
                    cast: VoteCast::Yes,
                }) => toaster
                    .write()
                    .success("Vote cast!", chrono::Duration::seconds(3)),
                Ok(VoteOk {
                    cast: VoteCast::AlreadyVoted,
                }) => toaster
                    .write()
                    .success("Already voted", chrono::Duration::seconds(5)),
                Err(e) => toaster.write().error(
                    format!("Failed to cast vote: {e}"),
                    chrono::Duration::seconds(3),
                ),
            }
        }
    );

    let total_votes = content
        .choices
        .iter()
        .map(|choice| choice.num_votes)
        .sum::<i64>();

    let leader_ids = {
        let leaders = content
            .choices
            .iter()
            .max_set_by(|x, y| x.num_votes.cmp(&y.num_votes));
        let ids: HashSet<PollChoiceId> = HashSet::from_iter(leaders.iter().map(|choice| choice.id));
        ids
    };

    let Choices = content.choices.iter().map(|choice| {
        let percent = if total_votes > 0 {
            let percent = (choice.num_votes as f64 / total_votes as f64) * 100.0;
            format!("{percent:.0}%")
        } else {
            "0%".to_string()
        };

        let background_color = if leader_ids.contains(&choice.id) {
            "bg-blue-300"
        } else {
            "bg-neutral-300"
        };

        let foreground_styles = maybe_class!("font-bold", leader_ids.contains(&choice.id));

        rsx! {
            li {
                key: "{choice.id.to_string()}",
                class: "relative p-2 m-2 cursor-pointer grid grid-cols-[3rem_1fr] border rounded border-slate-400",
                onclick: move |_| vote_onclick(*post_id, choice.id),
                div {
                    class: "absolute left-0 {background_color} h-full rounded z-[-1]",
                    style: "width: {percent}"
                },
                div {
                    class: "{foreground_styles}",
                    "{percent}"
                },
                div {
                    class: "{foreground_styles}",
                    "{choice.description.as_ref()}"
                }
            }
        }
    });

    let Headline = rsx! { figcaption { "{content.headline.as_ref()}" } };

    cx.render(rsx! {
        Headline,
        ul {
            Choices.into_iter()
        }
    })
}
