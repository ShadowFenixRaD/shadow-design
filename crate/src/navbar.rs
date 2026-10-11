use dioxus::prelude::*;

#[component]
pub fn Navbar(brand: Element, #[props(default)] class: String, children: Element) -> Element {
    #[cfg(feature = "theme")]
    let theme_val = crate::theme::use_theme()();
    #[cfg(feature = "theme")]
    let base_override = theme_val.resolve_component_base("navbar").unwrap_or("");
    #[cfg(feature = "theme")]
    let class_modifiers = theme_val.resolve_class_modifiers("navbar", &class);

    #[cfg(not(feature = "theme"))]
    let base_override = "";
    #[cfg(not(feature = "theme"))]
    let class_modifiers = String::new();

    rsx! {
        nav {
            class: "sd-navbar {base_override} {class_modifiers} {class}",
            div {
                class: "sd-navbar__inner",
                div { class: "sd-navbar__brand", {brand} }
                div { class: "sd-navbar__actions", {children} }
            }
        }
    }
}
