#![allow(non_snake_case)]

use crate::prelude::*;
use dioxus::prelude::*;

pub const BUTTON_SELECTED: &str = "border-slate-600";

#[derive(Props)]
pub struct AppbarImgButtonProps<'a, F>
where
    F: Fn(Event<MouseData>),
{
    append_class: Option<&'a str>,
    click_handler: Option<F>,
    disabled: Option<bool>,
    img: &'a str,
    label: &'a str,
    title: Option<&'a str>,
}

#[derive(Props)]
pub struct AppbarProps<'a> {
    title: &'a str,
    children: Element<'a>,
}

pub fn AppbarImgButton<'a, F>(cx: Scope<'a, AppbarImgButtonProps<'a, F>>) -> Element
where
    F: Fn(Event<MouseData>),
{
    let append_class = cx.props.append_class.unwrap_or("");

    cx.render(rsx! {
        button {
            class: "
                flex flex-col w-10 h-14
                justify-end items-center
                border-b-4 {append_class}
            ",
            disabled: cx.props.disabled.unwrap_or_default(),
            onclick: |ev| {
                if let Some(callback) = &cx.props.click_handler {
                    callback(ev);
                }
            },
            title: cx.props.title,
            img {
                class: "w-6 h-6",
                src: "{cx.props.img}"
            },
            span {
                class: "text-sm",
                "{cx.props.label}"
            }
        }
    })
}

pub fn Appbar<'a>(cx: Scope<'a, AppbarProps<'a>>) -> Element {
    cx.render(rsx! {
        div {
            class: "
                max-w-[var(--content-max-width)] h-[var(--appbar-height)]
                fixed top-0 right-0 left-0 mx-auto z-50
                bg-slate-200
            ",
            div {
                class: "flex flex-row gap-8 items-center w-full pr-5 h-full",
                div {
                    class: "cursor-pointer",
                    onclick: |_|(),
                    img {
                        class: "profile-portrait",
                        src: ""
                    }
                },
                div {
                    class: "text-xl font-bold mr-auto",
                    "{cx.props.title}"
                },
                &cx.props.children
            }
        }
    })
}
