#![allow(non_snake_case)]

use chrono::Duration;
use dioxus::prelude::*;
use dioxus_router::use_router;
use serde::{Deserialize, Serialize};
use uchat_domain::Caption;
use web_sys::HtmlInputElement;

use crate::{prelude::*, util};

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct PageState {
    pub caption: String,
    pub image: Option<String>,
}

impl PageState {
    pub fn can_submit(&self) -> bool {
        (self.caption.is_empty() || Caption::new(&self.caption).is_ok()) && self.image.is_some()
    }
}

#[inline_props]
pub fn ImageInput(cx: Scope, page_state: UseRef<PageState>) -> Element {
    let toaster = use_toaster(cx);

    let image_oninput = async_handler!(&cx, [page_state, toaster], move |_| async move {
        use gloo_file::{futures::read_as_data_url, File};
        use wasm_bindgen::JsCast;

        let el = util::document()
            .get_element_by_id("image-input")
            .unwrap()
            .unchecked_into::<HtmlInputElement>();

        let file: File = el.files().unwrap().get(0).unwrap().into();

        match read_as_data_url(&file).await {
            Ok(data) => page_state.with_mut(|state| state.image = Some(data)),
            Err(e) => toaster.write().error(
                format!("Error loading file: {e}"),
                chrono::Duration::seconds(5),
            ),
        }
    });

    cx.render(rsx! {
        div {
            label {
                r#for: "image-input",
                "Upload image"
            },
            input {
                class: "w-full",
                id: "image-input",
                r#type: "file",
                accept: "image/*",
                oninput: image_oninput,
            }
        }
    })
}

#[inline_props]
pub fn ImagePreview(cx: Scope, page_state: UseRef<PageState>) -> Element {
    let image_data = page_state.read().clone().image;
    let Preview = if let Some(ref image) = image_data {
        rsx! {
            img {
                class: "
                    max-w-[calc(var(--content-max-width)/2)]
                    max-h-[40vh]
                ",
                src: "{image}"
            }
        }
    } else {
        rsx! { div { "no image uploaded" } }
    };

    cx.render(rsx! {
        div {
            class: "flex flex-row justify-center",
            Preview
        }
    })
}

#[inline_props]
pub fn CaptionInput(cx: Scope, page_state: UseRef<PageState>) -> Element {
    let char_count = || page_state.read().caption.len();
    const MAX_CHARS: usize = Caption::MAX_CHARS;

    let wrong_len = maybe_class!(
        "err-text-color",
        page_state.read().caption.len() > MAX_CHARS
    );

    cx.render(rsx! {
        div {
            label {
                r#for: "caption",
                div {
                    class: "flex flex-row justify-between",
                    span {  "Caption (optional)" },
                    span {
                        class: "text-right {wrong_len}",
                        "{char_count()}/{MAX_CHARS}"
                    }
                },

                input {
                    class: "input-field",
                    id: "caption",
                    value: "{page_state.read().caption}",
                    oninput:  move |ev|  {
                        page_state.with_mut(|state| state.caption = ev.data.value.clone())
                    },
                }
            }
        }
    })
}

pub fn NewImage(cx: Scope) -> Element {
    let api_client = ApiClient::global();
    let page_state = use_ref(cx, || PageState::default());
    let submit_btn_style = maybe_class!("btn-disabled", !page_state.read().can_submit());
    let router = use_router(cx);
    let toaster = use_toaster(cx);

    let form_on_submit = async_handler!(
        &cx,
        [api_client, page_state, router, toaster],
        move |_| async move {
            use uchat_endpoint::post::{Image, ImageKind, NewPost, NewPostOk, NewPostOptions};

            let request = NewPost {
                content: Image {
                    caption: {
                        let caption = &page_state.read().caption;
                        if caption.is_empty() {
                            None
                        } else {
                            Caption::new(caption).ok()
                        }
                    },
                    kind: {
                        let image = &page_state.read().image;
                        ImageKind::DataUrl(image.clone().unwrap())
                    },
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
            title: "New Image",
            AppbarImgButton {
                click_handler: move |_| router.replace_route(page::POST_NEW_CHAT, None, None),
                img: "/static/icons/icon-messages.svg",
                label: "Chat",
                title: "Post a new chat"
            },
            AppbarImgButton {
                click_handler: |_|(),
                img: "/static/icons/icon-image.svg",
                label: "Image",
                title: "Post a new image",
                disabled: true,
                append_class: appbar::BUTTON_SELECTED,
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

            ImageInput { page_state: page_state.clone() },
            ImagePreview { page_state: page_state.clone() },
            CaptionInput { page_state: page_state.clone() },

            button {
                class: "btn {submit_btn_style}",
                r#type: "submit",
                disabled: !page_state.read().can_submit(),
                "Post"
            }
        }
    })
}
