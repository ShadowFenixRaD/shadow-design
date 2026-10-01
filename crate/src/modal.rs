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
    let backdrop = theme().modal_backdrop.clone();
    #[cfg(feature = "theme")]
    let dialog = theme().modal_dialog.clone();
    #[cfg(feature = "theme")]
    let header = theme().modal_header.clone();
    #[cfg(feature = "theme")]
    let close = theme().modal_close.clone();
    #[cfg(feature = "theme")]
    let glow_class = match glow {
        Some(true) => theme().glow_modal.clone(),
        Some(false) => String::new(),
        None => {
            if theme().glow_mode != crate::theme::GlowMode::Off {
                theme().glow_modal.clone()
            } else {
                String::new()
            }
        }
    };

    #[cfg(not(feature = "theme"))]
    let backdrop = "fixed inset-0 z-[80] flex items-center justify-center p-4 bg-black/80 backdrop-blur-md transition-all".to_string();
    #[cfg(not(feature = "theme"))]
    let dialog = "bg-[#0d0d14] border border-white/15 rounded-2xl max-w-lg w-full shadow-2xl overflow-hidden specular-top".to_string();
    #[cfg(not(feature = "theme"))]
    let header =
        "flex items-center justify-between px-6 py-4 border-b border-white/[0.08] bg-[#12121c]"
            .to_string();
    #[cfg(not(feature = "theme"))]
    let close = "text-zinc-400 hover:text-white text-sm p-1.5 rounded-lg hover:bg-white/[0.08] transition-colors cursor-pointer".to_string();
    #[cfg(not(feature = "theme"))]
    let glow_class = if glow == Some(true) {
        "shadow-[0_0_40px_rgba(147,51,234,0.3)]".to_string()
    } else {
        String::new()
    };

    #[cfg(feature = "animation")]
    let anim_class = crate::animation::Animations::MODAL_ENTER;
    #[cfg(not(feature = "animation"))]
    let anim_class = "animate-in fade-in zoom-in-95 duration-150";

    rsx! {
        div {
            class: "{backdrop}",
            onclick: move |_| *is_open.write() = false,
            div {
                class: "{dialog} z-[85] {glow_class} {anim_class} {class}",
                onclick: move |e| e.stop_propagation(),
                div {
                    class: "{header}",
                    h3 { class: "text-sm sm:text-base font-bold tracking-tight text-white", "{title.as_deref().unwrap_or_default()}" }
                    button {
                        class: "{close}",
                        onclick: move |_| *is_open.write() = false,
                        Icon { kind: IconKind::X, size: 16 }
                    }
                }
                div { class: "p-6", {children} }
            }
        }
    }
}
