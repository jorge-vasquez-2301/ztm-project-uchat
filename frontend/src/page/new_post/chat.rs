#![allow(non_snake_case)]

use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

use crate::maybe_class;

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct PageState {
    pub message: String,
    pub headline: String,
}

#[inline_props]
pub fn MessageInput(cx: Scope, page_state: UseRef<PageState>) -> Element {
    use uchat_domain::Message;

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
    use uchat_domain::Headline;

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
    let page_state = use_ref(cx, || PageState::default());
    cx.render(rsx! {
        form {
            class: "flex flex-col gap-4",
            onsubmit: |_|(),
            prevent_default: "onsubmit",

            MessageInput { page_state: page_state.clone() },
            HeadlineInput { page_state: page_state.clone() },

            button {
                class: "btn",
                r#type: "submit",
                disabled: true,
                "Post"
            }
        }
    })
}
