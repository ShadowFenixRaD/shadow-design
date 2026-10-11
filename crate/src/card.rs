use dioxus::prelude::*;

#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub enum CardShadow {
    None,
    Sm,
    #[default]
    Md,
    Lg,
    Xl,
}

impl CardShadow {
    pub fn scss_class(&self) -> &'static str {
        match self {
            CardShadow::None => "sd-card--shadow-none",
            CardShadow::Sm => "sd-card--shadow-sm",
            CardShadow::Md => "sd-card--shadow-md",
            CardShadow::Lg => "sd-card--shadow-lg",
            CardShadow::Xl => "sd-card--shadow-xl",
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub enum CardVariant {
    #[default]
    Default,
    Elevated,
    Interactive,
    Metric,
}

impl CardVariant {
    pub fn scss_class(&self) -> &'static str {
        match self {
            CardVariant::Default => "",
            CardVariant::Elevated => "sd-card--elevated",
            CardVariant::Interactive => "sd-card--interactive",
            CardVariant::Metric => "sd-card--metric",
        }
    }
}

#[component]
pub fn Card(
    #[props(default)] title: Option<String>,
    #[props(default)] subtitle: Option<String>,
    #[props(default)] variant: CardVariant,
    #[props(default)] shadow: CardShadow,
    #[props(default)] hover_glow: bool,
    #[props(default)] glow: Option<bool>,
    #[props(default = false)] overflow_hidden: bool,
    #[props(default)] class: String,
    #[props(default)] footer: Option<Element>,
    children: Element,
) -> Element {
    let variant_class = variant.scss_class();
    let shadow_class = shadow.scss_class();
    let overflow_class = if overflow_hidden {
        "sd-card--overflow-hidden"
    } else {
        ""
    };

    #[cfg(feature = "theme")]
    let theme_val = crate::theme::use_theme()();
    #[cfg(feature = "theme")]
    let radius_class = theme_val.radius_scale.md();
    #[cfg(feature = "theme")]
    let base_override = theme_val.resolve_component_base("card").unwrap_or("");
    #[cfg(feature = "theme")]
    let class_modifiers = theme_val.resolve_class_modifiers("card", &class);
    #[cfg(feature = "theme")]
    let glow_class = match glow {
        Some(true) => match &theme_val.primary_color {
            crate::theme::PrimaryColor::Violet => "sd-card--glow-violet".to_string(),
            crate::theme::PrimaryColor::Obsidian => "sd-card--glow-obsidian".to_string(),
            crate::theme::PrimaryColor::Crimson => "sd-card--glow-crimson".to_string(),
            crate::theme::PrimaryColor::Cyan => "sd-card--glow-cyan".to_string(),
            crate::theme::PrimaryColor::Emerald => "sd-card--glow-emerald".to_string(),
            crate::theme::PrimaryColor::Custom { .. } => {
                format!("sd-card--glow-{}", theme_val.primary_color.css_prefix())
            }
        },
        Some(false) => "".to_string(),
        None => {
            if hover_glow
                || (theme_val.glow_mode != crate::theme::GlowMode::Off
                    && variant == CardVariant::Interactive)
            {
                theme_val.resolve_elevation_class("card", None)
            } else {
                "".to_string()
            }
        }
    };

    #[cfg(not(feature = "theme"))]
    let base_override = "";
    #[cfg(not(feature = "theme"))]
    let class_modifiers = String::new();
    #[cfg(not(feature = "theme"))]
    let radius_class = "";
    #[cfg(not(feature = "theme"))]
    let glow_class = if hover_glow || glow == Some(true) {
        "sd-card--hover-glow".to_string()
    } else {
        "".to_string()
    };

    rsx! {
        div {
            class: "sd-card {base_override} {variant_class} {shadow_class} {radius_class} {glow_class} {overflow_class} {class_modifiers} {class}",
            if title.is_some() || subtitle.is_some() {
                div { class: "sd-card__header",
                    if let Some(t) = title {
                        h3 { class: "sd-card__title", "{t}" }
                    }
                    if let Some(s) = subtitle {
                        p { class: "sd-card__subtitle", "{s}" }
                    }
                }
            }
            div { class: "sd-card__body",
                {children}
            }
            if let Some(f) = footer {
                div { class: "sd-card__footer",
                    {f}
                }
            }
        }
    }
}
