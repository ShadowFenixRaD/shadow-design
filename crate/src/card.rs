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
    let theme = crate::theme::use_theme();
    #[cfg(feature = "theme")]
    let radius_class = theme().radius_scale.md();
    #[cfg(feature = "theme")]
    let glow_class = match glow {
        Some(true) => match theme().primary_color {
            crate::theme::PrimaryColor::Violet => "sd-card--glow-violet",
            crate::theme::PrimaryColor::Obsidian => "sd-card--glow-obsidian",
            crate::theme::PrimaryColor::Crimson => "sd-card--glow-crimson",
            crate::theme::PrimaryColor::Cyan => "sd-card--glow-cyan",
            crate::theme::PrimaryColor::Emerald => "sd-card--glow-emerald",
        },
        Some(false) => "",
        None => {
            if hover_glow
                || (theme().glow_mode != crate::theme::GlowMode::Off
                    && variant == CardVariant::Interactive)
            {
                "sd-card--hover-glow"
            } else {
                ""
            }
        }
    };

    #[cfg(not(feature = "theme"))]
    let radius_class = "";
    #[cfg(not(feature = "theme"))]
    let glow_class = if hover_glow || glow == Some(true) {
        "sd-card--hover-glow"
    } else {
        ""
    };

    rsx! {
        div {
            class: "sd-card {variant_class} {shadow_class} {radius_class} {glow_class} {overflow_class} {class}",
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
