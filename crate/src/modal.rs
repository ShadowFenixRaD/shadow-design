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

    rsx! {
        div {
            class: "fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/70 backdrop-blur-sm transition-all",
            div {
                class: "bg-zinc-900 border border-zinc-800 rounded-2xl max-w-lg w-full shadow-2xl overflow-hidden {class}",
                div {
                    class: "flex items-center justify-between px-6 py-4 border-b border-zinc-800",
                    h3 { class: "text-lg font-bold text-white", "{title.as_deref().unwrap_or_default()}" }
                    button {
                        class: "text-zinc-400 hover:text-white text-lg font-bold p-1 rounded-md hover:bg-zinc-800 transition-colors cursor-pointer",
                        onclick: move |_| *is_open.write() = false,
                        "✕"
                    }
                }
                div { class: "p-6", {children} }
            }
        }
    }
}
