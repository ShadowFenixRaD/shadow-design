use crate::icon::{Icon, IconKind};
use dioxus::prelude::*;

#[component]
pub fn TabList(#[props(default)] class: String, children: Element) -> Element {
    rsx! {
        div {
            class: "sd-tabs-list {class}",
            {children}
        }
    }
}

#[component]
pub fn Tab(
    value: String,
    active: Signal<String>,
    #[props(default)] icon: Option<IconKind>,
    #[props(default)] badge: Option<String>,
    #[props(default)] class: String,
    children: Element,
) -> Element {
    let is_active = *active.read() == value;
    let target = value.clone();
    let active_class = if is_active { "sd-tab--active" } else { "" };

    rsx! {
        button {
            r#type: "button",
            class: "sd-tab {active_class} {class}",
            onclick: move |_| *active.write() = target.clone(),
            if let Some(i) = icon {
                span { class: "sd-tab__icon",
                    Icon { kind: i, size: 14 }
                }
            }
            {children}
            if let Some(b) = badge {
                span { class: "sd-tab__badge", "{b}" }
            }
        }
    }
}

#[component]
pub fn TabPanel(
    value: String,
    active: Signal<String>,
    #[props(default)] class: String,
    children: Element,
) -> Element {
    if *active.read() != value {
        return rsx! {};
    }

    rsx! {
        div {
            class: "sd-tab-panel {class}",
            {children}
        }
    }
}
