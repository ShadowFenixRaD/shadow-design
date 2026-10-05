use dioxus::prelude::*;

#[component]
pub fn Navbar(brand: Element, #[props(default)] class: String, children: Element) -> Element {
    rsx! {
        nav {
            class: "sd-navbar {class}",
            div {
                class: "sd-navbar__inner",
                div { class: "sd-navbar__brand", {brand} }
                div { class: "sd-navbar__actions", {children} }
            }
        }
    }
}
