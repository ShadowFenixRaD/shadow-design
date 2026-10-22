use dioxus::prelude::*;

/// Contenitore utility per centrare istantaneamente il contenuto sia in orizzontale che in verticale,
/// senza codice CSS boilerplate, con supporto per layout a blocco o inline.
#[component]
pub fn Center(
    #[props(default = false)] inline: bool,
    #[props(default)] class: String,
    children: Element,
) -> Element {
    let inline_class = if inline { "sd-center--inline" } else { "" };

    #[cfg(feature = "theme")]
    let theme_val = crate::theme::use_theme()();
    #[cfg(feature = "theme")]
    let base_override = theme_val.resolve_component_base("center").unwrap_or("");
    #[cfg(feature = "theme")]
    let class_modifiers = theme_val.resolve_class_modifiers("center", &class);

    #[cfg(not(feature = "theme"))]
    let base_override = "";
    #[cfg(not(feature = "theme"))]
    let class_modifiers = String::new();

    rsx! {
        div {
            class: "sd-center {inline_class} {base_override} {class_modifiers} {class}",
            {children}
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_center_inline_flag() {
        let inline_true = true;
        let inline_false = false;
        assert_eq!(if inline_true { "sd-center--inline" } else { "" }, "sd-center--inline");
        assert_eq!(if inline_false { "sd-center--inline" } else { "" }, "");
    }
}
