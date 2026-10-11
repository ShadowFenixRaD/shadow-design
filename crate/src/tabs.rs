use crate::icon::{Icon, IconKind};
use dioxus::prelude::*;

#[component]
pub fn TabList(#[props(default)] class: String, children: Element) -> Element {
    #[cfg(feature = "theme")]
    let theme_val = crate::theme::use_theme()();
    #[cfg(feature = "theme")]
    let base_override = theme_val.resolve_component_base("tabs").unwrap_or("");
    #[cfg(feature = "theme")]
    let class_modifiers = theme_val.resolve_class_modifiers("tabs", &class);

    #[cfg(not(feature = "theme"))]
    let base_override = "";
    #[cfg(not(feature = "theme"))]
    let class_modifiers = String::new();

    rsx! {
        div {
            class: "sd-tabs-list {base_override} {class_modifiers} {class}",
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

    #[cfg(feature = "theme")]
    let theme_val = crate::theme::use_theme()();
    #[cfg(feature = "theme")]
    let base_override = theme_val.resolve_component_base("tab").unwrap_or("");
    #[cfg(feature = "theme")]
    let class_modifiers = theme_val.resolve_class_modifiers("tab", &class);

    #[cfg(not(feature = "theme"))]
    let base_override = "";
    #[cfg(not(feature = "theme"))]
    let class_modifiers = String::new();

    rsx! {
        button {
            r#type: "button",
            class: "sd-tab {base_override} {active_class} {class_modifiers} {class}",
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

    #[cfg(feature = "theme")]
    let theme_val = crate::theme::use_theme()();
    #[cfg(feature = "theme")]
    let base_override = theme_val.resolve_component_base("tab-panel").unwrap_or("");
    #[cfg(feature = "theme")]
    let class_modifiers = theme_val.resolve_class_modifiers("tab-panel", &class);

    #[cfg(not(feature = "theme"))]
    let base_override = "";
    #[cfg(not(feature = "theme"))]
    let class_modifiers = String::new();

    rsx! {
        div {
            class: "sd-tab-panel {base_override} {class_modifiers} {class}",
            {children}
        }
    }
}
