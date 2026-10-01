use dioxus::prelude::*;

#[component]
pub fn Navbar(
    brand: Element,
    #[props(default)] class: String,
    children: Element,
) -> Element {
    #[cfg(feature = "theme")]
    let theme = crate::theme::use_theme();
    #[cfg(feature = "theme")]
    let nav_base = theme().navbar_base.clone();
    #[cfg(not(feature = "theme"))]
    let nav_base = "w-full bg-black/90 backdrop-blur-md border-b border-purple-950/60 shadow-[0_4px_25px_rgba(0,0,0,0.9)] px-6 py-3.5 sticky top-0 z-40".to_string();

    rsx! {
        nav {
            class: "{nav_base} {class}",
            div {
                class: "max-w-7xl mx-auto flex items-center justify-between gap-4",
                div { class: "flex items-center gap-3", {brand} }
                div { class: "flex items-center gap-4", {children} }
            }
        }
    }
}
