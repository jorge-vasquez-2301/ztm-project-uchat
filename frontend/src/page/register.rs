#![allow(non_snake_case)]

use dioxus::prelude::{rsx, Element, Scope};

pub fn Register(cx: Scope) -> Element {
    cx.render(rsx! {"Registration page"})
}
