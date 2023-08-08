#![allow(non_snake_case)]

use chrono::Duration;
use dioxus::prelude::*;
use dioxus_router::use_router;
use serde::{Deserialize, Serialize};
use uchat_domain::{Headline, Message};

use crate::{fetch_json, prelude::*};

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct PageState {
    pub message: String,
    pub headline: String,
}

impl PageState {
    pub fn can_submit(&self) -> bool {
        Message::new(&self.message).is_ok()
            && (self.headline.is_empty() || Headline::new(&self.headline).is_ok())
    }
}

#[inline_props]
pub fn MessageInput(cx: Scope, page_state: UseRef<PageState>) -> Element {
    let char_count = || page_state.read().message.len();
    const MAX_CHARS: usize = Message::MAX_CHARS;

    let wrong_len = maybe_class!(
        "err-text-color",
        page_state.read().message.len() > MAX_CHARS || page_state.read().message.is_empty()
    );

    cx.render(rsx! {
        div {
            label {
                r#for: "message",
                div {
                    class: "flex flex-row justify-between",
                    span {  "Message" },
                    span {
                        class: "text-right {wrong_len}",
                        "{char_count()}/{MAX_CHARS}"
                    }
                },

                textarea {
                    class: "input-field",
                    id: "message",
                    rows: 5,
                    value: "{page_state.read().message}",
                    oninput:  move |ev|  {
                        page_state.with_mut(|state| state.message = ev.data.value.clone())
                    },
                }
            }
        }
    })
}

#[inline_props]
pub fn HeadlineInput(cx: Scope, page_state: UseRef<PageState>) -> Element {
    let char_count = || page_state.read().headline.len();
    const MAX_CHARS: usize = Headline::MAX_CHARS;

    let wrong_len = maybe_class!(
        "err-text-color",
        page_state.read().headline.len() > MAX_CHARS
    );

    cx.render(rsx! {
        div {
            label {
                r#for: "headline",
                div {
                    class: "flex flex-row justify-between",
                    span {  "Headline" },
                    span {
                        class: "text-right {wrong_len}",
                        "{char_count()}/{MAX_CHARS}"
                    }
                },

                input {
                    class: "input-field",
                    id: "headline",
                    value: "{page_state.read().headline}",
                    oninput:  move |ev|  {
                        page_state.with_mut(|state| state.headline = ev.data.value.clone())
                    },
                }
            }
        }
    })
}

pub fn NewChat(cx: Scope) -> Element {
    let api_client = ApiClient::global();
    let page_state = use_ref(cx, || PageState::default());
    let submit_btn_style = maybe_class!("btn-disabled", !page_state.read().can_submit());
    let router = use_router(cx);
    let toaster = use_toaster(cx);

    let form_on_submit = async_handler!(
        &cx,
        [api_client, page_state, router, toaster],
        move |_| async move {
            use uchat_endpoint::post::{Chat, NewPost, NewPostOk, NewPostOptions};

            let request = NewPost {
                content: Chat {
                    headline: {
                        let headline = &page_state.read().headline;
                        if headline.is_empty() {
                            None
                        } else {
                            Headline::new(headline).ok()
                        }
                    },
                    message: Message::new(&page_state.read().message).unwrap(),
                }
                .into(),
                options: NewPostOptions::default(),
            };

            let response = fetch_json!(<NewPostOk>, api_client, request);
            match response {
                Ok(_) => {
                    toaster.write().success("Posted", Duration::seconds(3));
                    router.replace_route(page::HOME, None, None);
                }
                Err(e) => {
                    toaster
                        .write()
                        .error(format!("Post failed: {e}"), Duration::seconds(3));
                }
            }
        }
    );

    cx.render(rsx! {
        Appbar  {
            title: "New Chat",
            AppbarImgButton {
                click_handler: |_|(),
                img: "/static/icons/icon-messages.svg",
                label: "Chat",
                title: "Post a new chat",
                disabled: true,
                append_class: appbar::BUTTON_SELECTED,
            },
            AppbarImgButton {
                click_handler: move |_| router.replace_route(page::POST_NEW_IMAGE, None, None),
                img: "/static/icons/icon-image.svg",
                label: "Image",
                title: "Post a new image"
            },
            AppbarImgButton {
                click_handler: move |_| router.replace_route(page::POST_NEW_POLL, None, None),
                img: "/static/icons/icon-poll.svg",
                label: "Poll",
                title: "Post a new poll"
            },
            AppbarImgButton {
                click_handler: move |_| router.pop_route(),
                img: "/static/icons/icon-back.svg",
                label: "Back",
                title: "Go to the previous page"
            }
        },
        form {
            class: "flex flex-col gap-4",
            onsubmit: form_on_submit,
            prevent_default: "onsubmit",

            MessageInput { page_state: page_state.clone() },
            HeadlineInput { page_state: page_state.clone() },

            button {
                class: "btn {submit_btn_style}",
                r#type: "submit",
                disabled: !page_state.read().can_submit(),
                "Post"
            }
        }
    })
}
