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
    #[props(default)] avatar: Option<Element>,
    #[props(default)] variant: CardVariant,
    #[props(default)] shadow: CardShadow,
    #[props(default)] hover_glow: bool,
    #[props(default)] glow: Option<bool>,
    #[props(default = false)] overflow_hidden: bool,
    #[props(default)] class: String,
    #[props(default)] progress: Option<f32>,
    #[props(default)] buttons: Option<Element>,
    #[props(default)] image: Option<String>,
    #[props(default)] image_alt: Option<String>,
    #[props(default)] image_height: Option<String>,
    #[props(default)] on_click: Option<EventHandler<MouseEvent>>,
    #[props(default)] footer: Option<Element>,
    #[props(default)] children: Element,
) -> Element {
    let variant_class = variant.scss_class();
    let shadow_class = shadow.scss_class();
    let overflow_class = if overflow_hidden {
        "sd-card--overflow-hidden"
    } else {
        ""
    };

    let has_avatar = avatar.is_some();
    let has_title_or_sub = title.is_some() || subtitle.is_some();
    let header_layout_class = if has_avatar && has_title_or_sub {
        "sd-card__header--with-avatar"
    } else if has_avatar {
        "sd-card__header--avatar-only"
    } else if has_title_or_sub {
        "sd-card__header--title-only"
    } else {
        ""
    };

    let alt_str = image_alt.as_deref().unwrap_or_default();

    let img_style = image_height
        .as_deref()
        .map(|h| format!("height: {h};"))
        .unwrap_or_default();

    let progress_element = if let Some(prog_val) = progress {
        #[cfg(feature = "progress")]
        rsx! {
            div {
                class: "sd-card__progress",
                crate::progress::ProgressBar {
                    value: prog_val,
                    size: crate::progress::ProgressSize::Sm,
                }
            }
        }
        #[cfg(not(feature = "progress"))]
        rsx! {
            div {
                class: "sd-card__progress",
                div {
                    class: "sd-card__progress-track",
                    div {
                        class: "sd-card__progress-fill",
                        style: "width: {prog_val.clamp(0.0, 100.0)}%;",
                    }
                }
            }
        }
    } else {
        rsx! {}
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
            onclick: move |evt| {
                if let Some(h) = &on_click {
                    h.call(evt);
                }
            },

            // Intestazione con Avatar, Titolo e Sottotitolo
            if has_avatar || has_title_or_sub {
                div {
                    class: "sd-card__header {header_layout_class}",
                    if let Some(av) = avatar {
                        div { class: "sd-card__avatar", {av} }
                    }
                    if has_title_or_sub {
                        div {
                            class: "sd-card__header-text",
                            if let Some(t) = title {
                                h3 { class: "sd-card__title", "{t}" }
                            }
                            if let Some(s) = subtitle {
                                p { class: "sd-card__subtitle", "{s}" }
                            }
                        }
                    }
                }
            }

            // Corpo principale della card
            div {
                class: "sd-card__body",
                {children}
            }

            // Sezione Progress Bar (avanzamento)
            {progress_element}

            // Sezione Bottoni / Azioni dedicata in basso
            if let Some(btns) = buttons {
                div {
                    class: "sd-card__buttons",
                    {btns}
                }
            }

            // Sezione Immagine in basso (occupa tutto il fondo seguendo i bordi)
            if let Some(img_url) = image {
                div {
                    class: "sd-card__image-container",
                    img {
                        src: "{img_url}",
                        alt: "{alt_str}",
                        class: "sd-card__image",
                        style: "{img_style}",
                    }
                }
            }

            // Footer a piè di pagina
            if let Some(f) = footer {
                div {
                    class: "sd-card__footer",
                    {f}
                }
            }
        }
    }
}
