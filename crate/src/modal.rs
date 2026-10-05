use crate::icon::{Icon, IconKind};
use dioxus::prelude::*;

#[component]
pub fn Modal(
    is_open: Signal<bool>,
    #[props(default)] title: Option<String>,
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
    let glow_class = match glow {
        Some(true) => "sd-modal-dialog--glow",
        Some(false) => "",
        None => {
            if theme().glow_mode != crate::theme::GlowMode::Off {
                "sd-modal-dialog--glow"
            } else {
                ""
            }
        }
    };

    #[cfg(not(feature = "theme"))]
    let glow_class = if glow == Some(true) {
        "sd-modal-dialog--glow"
    } else {
        ""
    };

    #[cfg(feature = "animation")]
    let anim_class = crate::animation::Animations::MODAL_ENTER;
    #[cfg(not(feature = "animation"))]
    let anim_class = "animate-modal-enter";

    rsx! {
        div {
            class: "sd-modal-backdrop",
            onclick: move |_| *is_open.write() = false,
            div {
                class: "sd-modal-dialog {glow_class} {anim_class} {class}",
                onclick: move |e| e.stop_propagation(),
                div {
                    class: "sd-modal-dialog__header",
                    h3 { class: "sd-modal-dialog__title", "{title.as_deref().unwrap_or_default()}" }
                    button {
                        class: "sd-modal-dialog__close",
                        onclick: move |_| *is_open.write() = false,
                        Icon { kind: IconKind::X, size: 16 }
                    }
                }
                div { class: "sd-modal-dialog__body", {children} }
            }
        }
    }
}
