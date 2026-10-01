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
    pub fn width_class(&self) -> &'static str {
        match self {
            DrawerSize::Sm => "w-72 sm:w-80",
            DrawerSize::Md => "w-80 sm:w-96",
            DrawerSize::Lg => "w-96 sm:w-[32rem]",
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
        match position {
            DrawerPosition::Left => "shadow-[0_0_40px_rgba(168,85,247,0.25)]",
            DrawerPosition::Right => "shadow-[0_0_40px_rgba(168,85,247,0.25)]",
        }
    } else {
        "shadow-2xl"
    };

    #[cfg(feature = "animation")]
    let anim_class = match position {
        DrawerPosition::Left => crate::animation::Animations::DRAWER_LEFT,
        DrawerPosition::Right => crate::animation::Animations::DRAWER_RIGHT,
    };
    #[cfg(not(feature = "animation"))]
    let anim_class = match position {
        DrawerPosition::Left => "animate-in slide-in-from-left duration-200",
        DrawerPosition::Right => "animate-in slide-in-from-right duration-200",
    };

    let pos_class = match position {
        DrawerPosition::Left => "inset-y-0 left-0 border-r border-white/10",
        DrawerPosition::Right => "inset-y-0 right-0 border-l border-white/10",
    };
    let width = size.width_class();

    rsx! {
        div {
            class: "fixed inset-0 z-[65] bg-black/80 backdrop-blur-sm transition-all",
            onclick: move |_| *is_open.write() = false,
            div {
                class: "fixed {pos_class} {width} bg-[#0a0a0f] {glow_class} {anim_class} flex flex-col z-[70] overflow-hidden {class}",
                onclick: move |e| e.stop_propagation(),
                // Drawer Header
                div { class: "flex items-center justify-between px-6 py-4 border-b border-white/[0.08] bg-[#0e0e14]",
                    h3 { class: "text-sm sm:text-base font-bold text-white tracking-tight",
                        "{title.as_deref().unwrap_or_default()}"
                    }
                    button {
                        r#type: "button",
                        class: "p-1.5 text-zinc-400 hover:text-white rounded-lg hover:bg-white/[0.08] transition-colors cursor-pointer",
                        onclick: move |_| *is_open.write() = false,
                        Icon { kind: IconKind::X, size: 16 }
                    }
                }
                // Drawer Body
                div { class: "flex-1 p-6 overflow-y-auto",
                    {children}
                }
            }
        }
    }
}
