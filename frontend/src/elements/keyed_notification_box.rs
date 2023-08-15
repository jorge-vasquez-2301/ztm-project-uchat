#![allow(non_snake_case)]

use std::collections::{hash_map::Values, HashMap};

use serde::{Deserialize, Serialize};

use dioxus::prelude::*;

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq)]
pub struct KeyedNotifications {
    pub inner: HashMap<String, String>,
}

impl KeyedNotifications {
    pub fn set<K, V>(&mut self, k: K, v: V)
    where
        K: Into<String>,
        V: Into<String>,
    {
        self.inner.insert(k.into(), v.into());
    }

    pub fn remove<K: AsRef<str>>(&mut self, k: K) {
        self.inner.remove(k.as_ref());
    }

    pub fn messages(&self) -> Values<'_, String, String> {
        self.inner.values()
    }

    pub fn has_messages(&self) -> bool {
        !self.inner.is_empty()
    }
}

#[derive(PartialEq, Props)]
pub struct KeyedNotificationProps<'a> {
    legend: Option<&'a str>,
    notifications: KeyedNotifications,
}

pub fn KeyedNotificationBox<'a>(cx: Scope<'a, KeyedNotificationProps<'a>>) -> Element {
    let notifications = cx.props.notifications.messages().map(|msg| {
        rsx! { li { "{msg}" } }
    });

    let legend = cx.props.legend.unwrap_or("Errors");

    match cx.props.notifications.has_messages() {
        true => cx.render(rsx! {
            fieldset {
                class: "fieldset border-red-300 rounded",
                "{legend}"
                ul {
                    class: "list-disc ml-4",
                    notifications
                }
            }
        }),
        false => None,
    }
}
