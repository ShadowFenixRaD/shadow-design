use dioxus::prelude::*;

#[component]
pub fn Navbar(
    brand: Element,
    #[props(default)] class: String,
    children: Element,
) -> Element {
    rsx! {
        nav {
            class: "w-full bg-zinc-950/80 backdrop-blur-md border-b border-zinc-800/80 px-6 py-3.5 sticky top-0 z-40 {class}",
            div {
                class: "max-w-7xl mx-auto flex items-center justify-between gap-4",
                div { class: "flex items-center gap-3", {brand} }
                div { class: "flex items-center gap-4", {children} }
            }
        }
    }
}
