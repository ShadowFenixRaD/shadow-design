use dioxus::prelude::*;

#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub enum BadgeVariant {
    #[default]
    Default,
    Primary,
    Success,
    Warning,
    Danger,
    Outline,
}

impl BadgeVariant {
    #[must_use]
    pub fn scss_class(&self) -> &'static str {
        match self {
            BadgeVariant::Default => "sd-badge--default",
            BadgeVariant::Primary => "sd-badge--primary",
            BadgeVariant::Success => "sd-badge--success",
            BadgeVariant::Warning => "sd-badge--warning",
            BadgeVariant::Danger => "sd-badge--danger",
            BadgeVariant::Outline => "sd-badge--outline",
        }
    }

    #[must_use]
    pub fn name(&self) -> &'static str {
        match self {
            BadgeVariant::Default => "default",
            BadgeVariant::Primary => "primary",
            BadgeVariant::Success => "success",
            BadgeVariant::Warning => "warning",
            BadgeVariant::Danger => "danger",
            BadgeVariant::Outline => "outline",
        }
    }

    #[must_use]
    pub fn dot_color_class(&self) -> &'static str {
        match self {
            BadgeVariant::Default => "default",
            BadgeVariant::Primary => "primary",
            BadgeVariant::Success => "success",
            BadgeVariant::Warning => "warning",
            BadgeVariant::Danger => "danger",
            BadgeVariant::Outline => "outline",
        }
    }
}

#[component]
pub fn Badge(
    label: String,
    #[props(default)] variant: BadgeVariant,
    #[props(default)] pill: bool,
    #[props(default)] dot: bool,
    #[props(default)] glow: Option<bool>,
    #[props(default)] class: String,
) -> Element {
    let pill_class = if pill { "sd-badge--pill" } else { "" };

    #[cfg(feature = "theme")]
    let theme_val = crate::theme::use_theme()();
    #[cfg(feature = "theme")]
    let base_override = theme_val.resolve_component_base("badge").unwrap_or("");
    #[cfg(feature = "theme")]
    let class_modifiers = theme_val.resolve_class_modifiers("badge", &class);
    #[cfg(feature = "theme")]
    let variant_class =
        theme_val.resolve_variant_class("badge", variant.name(), variant.scss_class());
    #[cfg(feature = "theme")]
    let glow_class = theme_val.resolve_elevation_class("badge", glow);

    #[cfg(not(feature = "theme"))]
    let base_override = "";
    #[cfg(not(feature = "theme"))]
    let class_modifiers = String::new();
    #[cfg(not(feature = "theme"))]
    let variant_class = variant.scss_class().to_string();
    #[cfg(not(feature = "theme"))]
    let glow_class = if glow == Some(true) { "sd-badge--glow" } else { "" };

    rsx! {
        span {
            class: "sd-badge {base_override} {variant_class} {pill_class} {glow_class} {class_modifiers} {class}",
            if dot {
                span { class: "sd-badge__dot-container",
                    span { class: "sd-badge__dot-ping" }
                    span { class: "sd-badge__dot" }
                }
            }
            "{label}"
        }
    }
}
