use dioxus::prelude::*;

/// Forma geometrica del placeholder Skeleton.
#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub enum SkeletonShape {
    Circle,
    Rectangle,
    #[default]
    Text,
}

impl SkeletonShape {
    #[must_use]
    pub fn scss_class(&self) -> &'static str {
        match self {
            SkeletonShape::Circle => "sd-skeleton--circle",
            SkeletonShape::Rectangle => "sd-skeleton--rectangle",
            SkeletonShape::Text => "sd-skeleton--text",
        }
    }
}

/// Componente Skeleton per mostrare placeholder animati durante il caricamento di dati, messaggi o peer.
#[component]
pub fn Skeleton(
    #[props(default)] width: Option<String>,
    #[props(default)] height: Option<String>,
    #[props(default)] shape: SkeletonShape,
    #[props(default = true)] animate: bool,
    #[props(default = 1)] count: usize,
    #[props(default)] class: String,
) -> Element {
    let shape_class = shape.scss_class();
    let anim_class = if animate { "sd-skeleton--animated" } else { "" };

    #[cfg(feature = "theme")]
    let theme_val = crate::theme::use_theme()();
    #[cfg(feature = "theme")]
    let base_override = theme_val.resolve_component_base("skeleton").unwrap_or("");
    #[cfg(feature = "theme")]
    let class_modifiers = theme_val.resolve_class_modifiers("skeleton", &class);

    #[cfg(not(feature = "theme"))]
    let base_override = "";
    #[cfg(not(feature = "theme"))]
    let class_modifiers = String::new();

    let custom_style = match (&width, &height) {
        (Some(w), Some(h)) => format!("width: {w}; height: {h};"),
        (Some(w), None) => format!("width: {w};"),
        (None, Some(h)) => format!("height: {h};"),
        (None, None) => String::new(),
    };

    if count > 1 && shape == SkeletonShape::Text {
        rsx! {
            div {
                class: "sd-skeleton-group {class}",
                for idx in 0..count {
                    {
                        let is_last = idx == count - 1;
                        let line_style = if is_last && width.is_none() {
                            "width: 70%;".to_string()
                        } else {
                            custom_style.clone()
                        };

                        rsx! {
                            div {
                                key: "{idx}",
                                class: "sd-skeleton {shape_class} {anim_class} {base_override} {class_modifiers}",
                                style: "{line_style}",
                                aria_hidden: "true",
                            }
                        }
                    }
                }
            }
        }
    } else {
        rsx! {
            div {
                class: "sd-skeleton {shape_class} {anim_class} {base_override} {class_modifiers} {class}",
                style: "{custom_style}",
                aria_hidden: "true",
            }
        }
    }
}
