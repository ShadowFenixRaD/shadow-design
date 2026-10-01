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
    pub fn menu_classes(&self) -> &'static str {
        match self {
            DropdownPlacement::BottomRight => "top-full right-0 mt-2",
            DropdownPlacement::BottomLeft => "top-full left-0 mt-2",
            DropdownPlacement::TopRight => "bottom-full right-0 mb-2",
            DropdownPlacement::TopLeft => "bottom-full left-0 mb-2",
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
        Some(true) => theme().glow_dropdown.clone(),
        Some(false) => String::new(),
        None => {
            if theme().glow_mode != crate::theme::GlowMode::Off {
                theme().glow_dropdown.clone()
            } else {
                String::new()
            }
        }
    };

    #[cfg(not(feature = "theme"))]
    let glow_class = if glow == Some(true) {
        "shadow-[0_0_25px_rgba(147,51,234,0.25)]"
    } else {
        ""
    };

    let z_container = if open { "z-[999]" } else { "z-10" };
    let pos_class = placement.menu_classes();

    rsx! {
        div { class: "relative inline-block text-left sd-dropdown-container {z_container} {class}",
            div {
                onclick: move |_| {
                    let next = !*is_open.read();
                    *is_open.write() = next;
                },
                {trigger}
            }
            if open {
                // Invisible click-outside backdrop sopra il layout ma sotto il menu
                div {
                    class: "fixed inset-0 z-[995] sd-dropdown-backdrop",
                    onclick: move |_| *is_open.write() = false,
                }
                // Menu panel con z-index 1000 prioritario assoluto
                div {
                    class: "absolute {pos_class} w-56 rounded-xl bg-[#0c0c12] border border-white/10 shadow-2xl p-1.5 z-[1000] sd-dropdown-menu specular-top animate-dropdown-enter {glow_class}",
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
    let text_color = if danger {
        "text-rose-400 hover:text-rose-200 hover:bg-rose-950/40"
    } else {
        "text-zinc-300 hover:text-white hover:bg-white/[0.06]"
    };

    rsx! {
        button {
            r#type: "button",
            class: "w-full flex items-center gap-2.5 px-3 py-2 text-xs font-medium rounded-lg transition-colors cursor-pointer text-left {text_color} {class}",
            onclick: move |e| {
                if let Some(handler) = onclick.as_ref() {
                    handler.call(e);
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
        div { class: "my-1 border-t border-white/[0.06]" }
    }
}
