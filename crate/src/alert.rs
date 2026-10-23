use crate::icon::{Icon, IconKind};
use crate::locale::use_locale;
use crate::a11y::AriaHeading;
use crate::close_button::{CloseButton, CloseButtonSize, CloseButtonVariant};
use dioxus::prelude::*;

/// Severity variants for system alerts.
#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub enum AlertVariant {
    #[default]
    Info,
    Success,
    Warning,
    Danger,
}

impl AlertVariant {
    #[must_use]
    pub fn scss_class(&self) -> &'static str {
        match self {
            AlertVariant::Info => "sd-alert--info",
            AlertVariant::Success => "sd-alert--success",
            AlertVariant::Warning => "sd-alert--warning",
            AlertVariant::Danger => "sd-alert--danger",
        }
    }

    #[must_use]
    pub fn title_class(&self) -> &'static str {
        match self {
            AlertVariant::Info => "sd-alert__title--info",
            AlertVariant::Success => "sd-alert__title--success",
            AlertVariant::Warning => "sd-alert__title--warning",
            AlertVariant::Danger => "sd-alert__title--danger",
        }
    }

    #[must_use]
    pub fn icon_color_class(&self) -> &'static str {
        match self {
            AlertVariant::Info => "sd-alert__icon--info",
            AlertVariant::Success => "sd-alert__icon--success",
            AlertVariant::Warning => "sd-alert__icon--warning",
            AlertVariant::Danger => "sd-alert__icon--danger",
        }
    }

    #[must_use]
    pub fn glow_class(&self) -> &'static str {
        match self {
            AlertVariant::Info => "sd-alert--glow-info",
            AlertVariant::Success => "sd-alert--glow-success",
            AlertVariant::Warning => "sd-alert--glow-warning",
            AlertVariant::Danger => "sd-alert--glow-danger",
        }
    }

    #[must_use]
    pub fn icon_kind(&self) -> IconKind {
        match self {
            AlertVariant::Info => IconKind::Info,
            AlertVariant::Success => IconKind::Check,
            AlertVariant::Warning => IconKind::AlertTriangle,
            AlertVariant::Danger => IconKind::AlertCircle,
        }
    }

    #[must_use]
    pub fn name(&self) -> &'static str {
        match self {
            AlertVariant::Info => "info",
            AlertVariant::Success => "success",
            AlertVariant::Warning => "warning",
            AlertVariant::Danger => "danger",
        }
    }

    #[must_use]
    pub fn default_prefix(&self) -> &'static str {
        match self {
            AlertVariant::Info => "Info",
            AlertVariant::Success => "Success",
            AlertVariant::Warning => "Warning",
            AlertVariant::Danger => "Error",
        }
    }
}

/// Alert notification box component with ARIA live region support and variant semantic glows.
#[component]
pub fn Alert(
    /// Optional unique identifier for the alert.
    #[props(default)]
    id: Option<String>,
    /// Optional header title displayed at the top of the alert box.
    #[props(default)]
    title: Option<String>,
    /// Severity level determining colors, icons, and live region semantics.
    #[props(default)]
    variant: AlertVariant,
    /// Whether the alert acts as a dynamic live region. Defaults to true.
    #[props(default = true)]
    live: bool,
    /// Overrides the visually-hidden severity prefix text announced by screen readers.
    #[props(default)]
    severity_label: Option<String>,
    /// Heading level for the title (e.g. 2 for h2, 3 for h3, 4 for h4). None renders a styled div.
    #[props(default)]
    heading_level: Option<u8>,
    /// Accessible label for the close button.
    #[props(default)]
    close_label: Option<String>,
    /// Additional CSS or SCSS classes.
    #[props(default)]
    class: String,
    /// Perimetric optical glow coordinated with the active theme.
    #[props(default)]
    glow: Option<bool>,
    /// Optional callback invoked when the user clicks the dismiss button.
    on_close: Option<EventHandler<MouseEvent>>,
    /// Alert body message contents.
    children: Element,
) -> Element {
    let locale = use_locale();
    let alert_id = crate::a11y::use_unique_id("sd-alert", id);
    let title_id = format!("{alert_id}-title");

    let title_variant_class = variant.title_class();
    let icon_color_class = variant.icon_color_class();

    #[cfg(feature = "theme")]
    let theme_val = crate::theme::use_theme()();
    #[cfg(feature = "theme")]
    let radius_class = theme_val.radius_scale.md();
    #[cfg(feature = "theme")]
    let base_override = theme_val.resolve_component_base("alert").unwrap_or("");
    #[cfg(feature = "theme")]
    let class_modifiers = theme_val.resolve_class_modifiers("alert", &class);
    #[cfg(feature = "theme")]
    let variant_class = theme_val.resolve_variant_class("alert", variant.name(), variant.scss_class());
    #[cfg(feature = "theme")]
    let glow_class = if theme_val.is_component_glow_enabled("alert", glow) {
        variant.glow_class().to_string()
    } else {
        String::new()
    };

    #[cfg(not(feature = "theme"))]
    let base_override = "";
    #[cfg(not(feature = "theme"))]
    let class_modifiers = String::new();
    #[cfg(not(feature = "theme"))]
    let variant_class = variant.scss_class().to_string();
    #[cfg(not(feature = "theme"))]
    let radius_class = "";
    #[cfg(not(feature = "theme"))]
    let glow_class = if glow.unwrap_or(false) {
        variant.glow_class().to_string()
    } else {
        String::new()
    };

    let icon_kind = variant.icon_kind();
    let role = if live {
        match variant {
            AlertVariant::Danger | AlertVariant::Warning => Some("alert"),
            AlertVariant::Info | AlertVariant::Success => Some("status"),
        }
    } else {
        None
    };

    let aria_live = if live {
        match variant {
            AlertVariant::Danger | AlertVariant::Warning => Some("assertive"),
            AlertVariant::Info | AlertVariant::Success => Some("polite"),
        }
    } else {
        None
    };

    let prefix_text = severity_label.as_deref().unwrap_or_else(|| variant.default_prefix());
    let close_btn_label = close_label.unwrap_or_else(|| locale.close.clone());

    rsx! {
        div {
            role: role,
            "aria-live": aria_live,
            "aria-labelledby": if title.is_some() { Some(title_id.as_str()) } else { None },
            class: "sd-alert {base_override} {variant_class} {glow_class} {radius_class} {class_modifiers} {class}",
            span {
                class: "sd-alert__icon {icon_color_class}",
                "aria-hidden": "true",
                Icon { kind: icon_kind, size: 18 }
            }
            div { class: "sd-alert__content",
                span { class: "sd-visually-hidden", "{prefix_text}: " }
                if let Some(t) = &title {
                    AriaHeading {
                        level: heading_level,
                        id: title_id.clone(),
                        class: format!("sd-alert__title {title_variant_class}"),
                        "{t}"
                    }
                }
                div { class: "sd-alert__body", {children} }
            }
            if let Some(close_handler) = on_close {
                CloseButton {
                    size: CloseButtonSize::Sm,
                    variant: CloseButtonVariant::Subtle,
                    aria_label: close_btn_label,
                    class: "sd-alert__close",
                    onclick: move |e| close_handler.call(e),
                }
            }
        }
    }
}
