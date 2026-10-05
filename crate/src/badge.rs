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
    let variant_class = variant.scss_class();
    let pill_class = if pill { "sd-badge--pill" } else { "" };

    #[cfg(feature = "theme")]
    let theme = crate::theme::use_theme();
    #[cfg(feature = "theme")]
    let radius = if pill {
        ""
    } else {
        theme().radius_scale.sm()
    };
    #[cfg(feature = "theme")]
    let glow_class = match glow {
        Some(true) => "sd-badge--glow",
        Some(false) => "",
        None => {
            if variant == BadgeVariant::Primary && theme().glow_mode != crate::theme::GlowMode::Off
            {
                "sd-badge--glow"
            } else {
                ""
            }
        }
    };

    #[cfg(not(feature = "theme"))]
    let radius = "";
    #[cfg(not(feature = "theme"))]
    let glow_class = if glow == Some(true) || (glow.is_none() && variant == BadgeVariant::Primary) {
        "sd-badge--glow"
    } else {
        ""
    };

    let dot_kind = variant.dot_color_class();

    rsx! {
        span {
            class: "sd-badge {variant_class} {pill_class} {radius} {glow_class} {class}",
            if dot {
                span { class: "sd-badge__dot-container",
                    span { class: "sd-badge__dot-ping sd-badge__dot-ping--{dot_kind}" }
                    span { class: "sd-badge__dot sd-badge__dot--{dot_kind}" }
                }
            }
            "{label}"
        }
    }
}
