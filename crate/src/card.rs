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
    #[must_use]
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
    #[must_use]
    pub fn scss_class(&self) -> &'static str {
        match self {
            CardVariant::Default => "",
            CardVariant::Elevated => "sd-card--elevated",
            CardVariant::Interactive => "sd-card--interactive",
            CardVariant::Metric => "sd-card--metric",
        }
    }
}

/// Card component with support for multiple visual variants, elevations, media, and interactive models.
///
/// ### Accessibility & Interactive Behavior
/// When using `CardVariant::Interactive`, provide either `onclick` or `href`:
/// - When `href` is specified, the title renders as an accessible link (`<a>`) with a stretched link
///   overlay so the entire card is clickable while maintaining valid document outline.
/// - When `onclick` is specified, the card root gains `role="button"`, `tabindex="0"`, and activates
///   on both `Enter` and `Space`.
/// - Note: Interactive child controls (buttons, dropdown triggers, links) nested inside an interactive
///   card should call `evt.stop_propagation()` to prevent triggering the card's parent action.
#[component]
pub fn Card(
    #[props(default)] id: Option<String>,
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
    #[props(default)] onclick: Option<EventHandler<MouseEvent>>,
    #[props(default)] on_click: Option<EventHandler<MouseEvent>>,
    #[props(default)] href: Option<String>,
    #[props(default)] footer: Option<Element>,
    #[props(default)] children: Element,
) -> Element {
    let card_id = use_hook(|| {
        if let Some(user_id) = id {
            user_id
        } else {
            static NEXT_ID: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(1);
            format!("sd-card-{}", NEXT_ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed))
        }
    });
    let title_id = format!("{card_id}-title");

    let effective_onclick = onclick.or(on_click);
    let is_link = href.is_some();
    let is_button = !is_link && effective_onclick.is_some();
    let is_actionable = is_link || is_button;

    let actionable_class = if is_actionable {
        "sd-card--actionable"
    } else {
        ""
    };

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

    let handle_keydown = move |evt: KeyboardEvent| {
        if is_button && (evt.key() == Key::Enter || evt.key() == Key::Character(" ".to_string())) {
            evt.prevent_default();
            let _ = document::eval(r#"
                if (document.activeElement) {
                    document.activeElement.click();
                }
            "#);
        }
    };

    rsx! {
        div {
            id: "{card_id}",
            class: "sd-card {base_override} {variant_class} {actionable_class} {shadow_class} {radius_class} {glow_class} {overflow_class} {class_modifiers} {class}",
            role: if is_button { "button" } else { "" },
            tabindex: if is_button { "0" } else { "" },
            "aria-labelledby": if is_button && title.is_some() { "{title_id}" } else { "" },
            onclick: move |evt| {
                if let Some(h) = &effective_onclick {
                    h.call(evt);
                }
            },
            onkeydown: handle_keydown,

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
                            if let Some(t) = &title {
                                h3 {
                                    id: "{title_id}",
                                    class: "sd-card__title",
                                    if let Some(url) = &href {
                                        a {
                                            href: "{url}",
                                            class: "sd-card__link",
                                            "{t}"
                                        }
                                    } else {
                                        "{t}"
                                    }
                                }
                            }
                            if let Some(s) = &subtitle {
                                p { class: "sd-card__subtitle", "{s}" }
                            }
                        }
                    }
                }
            } else if let Some(url) = &href {
                a {
                    href: "{url}",
                    class: "sd-card__link",
                    "aria-label": "Card link",
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
