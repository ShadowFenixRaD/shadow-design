use crate::icon::{Icon, IconKind};
use dioxus::prelude::*;

#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub enum DropdownPlacement {
    #[default]
    BottomRight,
    BottomLeft,
    TopRight,
    TopLeft,
}

impl DropdownPlacement {
    pub fn scss_class(&self) -> &'static str {
        match self {
            DropdownPlacement::BottomRight => "sd-dropdown-menu--bottom-right",
            DropdownPlacement::BottomLeft => "sd-dropdown-menu--bottom-left",
            DropdownPlacement::TopRight => "sd-dropdown-menu--top-right",
            DropdownPlacement::TopLeft => "sd-dropdown-menu--top-left",
        }
    }
}

#[component]
pub fn Dropdown(
    is_open: Signal<bool>,
    trigger: Element,
    #[props(default)] placement: DropdownPlacement,
    #[props(default)] glow: Option<bool>,
    #[props(default)] class: String,
    children: Element,
) -> Element {
    let open = *is_open.read();

    #[cfg(feature = "theme")]
    let theme = crate::theme::use_theme();
    #[cfg(feature = "theme")]
    let glow_class = match glow {
        Some(true) => "sd-dropdown-menu--glow",
        Some(false) => "",
        None => {
            if theme().glow_mode != crate::theme::GlowMode::Off {
                "sd-dropdown-menu--glow"
            } else {
                ""
            }
        }
    };

    #[cfg(not(feature = "theme"))]
    let glow_class = if glow == Some(true) {
        "sd-dropdown-menu--glow"
    } else {
        ""
    };

    let open_container_class = if open { "sd-dropdown-container--open" } else { "" };
    let placement_class = placement.scss_class();

    #[cfg(feature = "animation")]
    let anim_class = crate::animation::Animations::DROPDOWN_ENTER;
    #[cfg(not(feature = "animation"))]
    let anim_class = "animate-dropdown-enter";

    rsx! {
        div { class: "sd-dropdown-container {open_container_class} {class}",
            div {
                onclick: move |_| {
                    let next = !*is_open.read();
                    *is_open.write() = next;
                },
                {trigger}
            }
            if open {
                div {
                    class: "sd-dropdown-backdrop",
                    onclick: move |e| {
                        e.stop_propagation();
                        *is_open.write() = false;
                    },
                }
                div {
                    class: "sd-dropdown-menu {placement_class} {glow_class} {anim_class}",
                    onclick: move |e| e.stop_propagation(),
                    {children}
                }
            }
        }
    }
}

#[component]
pub fn DropdownItem(
    #[props(default)] icon: Option<IconKind>,
    #[props(default)] danger: bool,
    #[props(default)] class: String,
    onclick: Option<EventHandler<MouseEvent>>,
    children: Element,
) -> Element {
    let danger_class = if danger { "sd-dropdown-item--danger" } else { "" };

    rsx! {
        button {
            r#type: "button",
            class: "sd-dropdown-item {danger_class} {class}",
            onclick: move |e| {
                if let Some(h) = onclick.as_ref() {
                    h.call(e);
                }
            },
            if let Some(i) = icon {
                Icon { kind: i, size: 14 }
            }
            {children}
        }
    }
}

#[component]
pub fn DropdownDivider() -> Element {
    rsx! {
        div { class: "sd-dropdown-divider" }
    }
}
