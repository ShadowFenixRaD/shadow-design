use crate::icon::{Icon, IconKind};
use dioxus::prelude::*;

#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub enum DrawerPosition {
    Left,
    #[default]
    Right,
}

#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub enum DrawerSize {
    Sm,
    #[default]
    Md,
    Lg,
}

impl DrawerSize {
    pub fn scss_class(&self) -> &'static str {
        match self {
            DrawerSize::Sm => "sd-drawer-panel--sm",
            DrawerSize::Md => "sd-drawer-panel--md",
            DrawerSize::Lg => "sd-drawer-panel--lg",
        }
    }
}

#[component]
pub fn Drawer(
    is_open: Signal<bool>,
    #[props(default)] title: Option<String>,
    #[props(default)] position: DrawerPosition,
    #[props(default)] size: DrawerSize,
    #[props(default)] glow: Option<bool>,
    #[props(default)] class: String,
    children: Element,
) -> Element {
    if !*is_open.read() {
        return rsx! {};
    }

    #[cfg(feature = "theme")]
    let theme = crate::theme::use_theme();
    #[cfg(feature = "theme")]
    let has_glow = theme().is_component_glow_enabled("drawer", glow);
    #[cfg(not(feature = "theme"))]
    let has_glow = glow.unwrap_or(false);

    let glow_class = if has_glow {
        "sd-drawer-panel--glow"
    } else {
        ""
    };

    #[cfg(feature = "animation")]
    let anim_class = match position {
        DrawerPosition::Left => crate::animation::Animations::DRAWER_LEFT,
        DrawerPosition::Right => crate::animation::Animations::DRAWER_RIGHT,
    };
    #[cfg(not(feature = "animation"))]
    let anim_class = match position {
        DrawerPosition::Left => "animate-drawer-slide-left",
        DrawerPosition::Right => "animate-drawer-slide-right",
    };

    let pos_class = match position {
        DrawerPosition::Left => "sd-drawer-panel--left",
        DrawerPosition::Right => "sd-drawer-panel--right",
    };
    let size_class = size.scss_class();

    rsx! {
        div {
            class: "sd-drawer-backdrop",
            onclick: move |_| *is_open.write() = false,
            div {
                class: "sd-drawer-panel {pos_class} {size_class} {glow_class} {anim_class} {class}",
                onclick: move |e| e.stop_propagation(),
                div { class: "sd-drawer__header",
                    h3 { class: "sd-drawer__title",
                        "{title.as_deref().unwrap_or_default()}"
                    }
                    button {
                        r#type: "button",
                        class: "sd-drawer__close",
                        onclick: move |_| *is_open.write() = false,
                        Icon { kind: IconKind::X, size: 16 }
                    }
                }
                div { class: "sd-drawer__body",
                    {children}
                }
            }
        }
    }
}
