use dioxus::prelude::*;

/// Orientamento della linea separatrice.
#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub enum Orientation {
    #[default]
    Horizontal,
    Vertical,
}

impl Orientation {
    pub fn scss_class(&self) -> &'static str {
        match self {
            Orientation::Horizontal => "sd-divider--horizontal",
            Orientation::Vertical => "sd-divider--vertical",
        }
    }
}

/// Componente Divider: separatore semitrasparente con orientamento e supporto a etichetta centrata.
/// Ideale per date nella chat ("Oggi", "Ieri") o separatori di sezioni nelle impostazioni.
#[component]
pub fn Divider(
    #[props(default)] orientation: Orientation,
    #[props(default)] label: Option<String>,
    #[props(default)] class: String,
) -> Element {
    let orient_class = orientation.scss_class();

    #[cfg(feature = "theme")]
    let theme_val = crate::theme::use_theme()();
    #[cfg(feature = "theme")]
    let base_override = theme_val.resolve_component_base("divider").unwrap_or("");
    #[cfg(feature = "theme")]
    let class_modifiers = theme_val.resolve_class_modifiers("divider", &class);

    #[cfg(not(feature = "theme"))]
    let base_override = "";
    #[cfg(not(feature = "theme"))]
    let class_modifiers = String::new();

    if let Some(lbl) = label {
        rsx! {
            div {
                class: "sd-divider-with-label {orient_class} {base_override} {class_modifiers} {class}",
                role: "separator",
                div { class: "sd-divider__line" }
                span { class: "sd-divider__label", "{lbl}" }
                div { class: "sd-divider__line" }
            }
        }
    } else {
        rsx! {
            div {
                class: "sd-divider {orient_class} {base_override} {class_modifiers} {class}",
                role: "separator",
            }
        }
    }
}
