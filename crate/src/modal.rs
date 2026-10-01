use dioxus::prelude::*;

#[component]
pub fn Modal(
    is_open: Signal<bool>,
    #[props(default)] title: Option<String>,
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

    #[cfg(not(feature = "theme"))]
    let backdrop = "fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/85 backdrop-blur-md transition-all".to_string();
    #[cfg(not(feature = "theme"))]
    let dialog = "bg-[#08080c] border border-purple-900/60 rounded-2xl max-w-lg w-full shadow-2xl shadow-purple-950/60 overflow-hidden".to_string();
    #[cfg(not(feature = "theme"))]
    let header = "flex items-center justify-between px-6 py-4 border-b border-purple-950/60 bg-[#0d0d14]/80".to_string();
    #[cfg(not(feature = "theme"))]
    let close = "text-zinc-400 hover:text-white text-lg font-bold p-1 rounded-md hover:bg-purple-950/50 transition-colors cursor-pointer".to_string();

    rsx! {
        div {
            class: "{backdrop}",
            div {
                class: "{dialog} {class}",
                div {
                    class: "{header}",
                    h3 { class: "text-lg font-bold text-white", "{title.as_deref().unwrap_or_default()}" }
                    button {
                        class: "{close}",
                        onclick: move |_| *is_open.write() = false,
                        "✕"
                    }
                }
                div { class: "p-6", {children} }
            }
        }
    }
}
