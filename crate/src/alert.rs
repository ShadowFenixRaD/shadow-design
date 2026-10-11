use crate::icon::{Icon, IconKind};
use dioxus::prelude::*;

#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub enum AlertVariant {
    #[default]
    Info,
    Success,
    Warning,
    Danger,
}

impl AlertVariant {
    pub fn scss_class(&self) -> &'static str {
        match self {
            AlertVariant::Info => "sd-alert--info",
            AlertVariant::Success => "sd-alert--success",
            AlertVariant::Warning => "sd-alert--warning",
            AlertVariant::Danger => "sd-alert--danger",
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            AlertVariant::Info => "info",
            AlertVariant::Success => "success",
            AlertVariant::Warning => "warning",
            AlertVariant::Danger => "danger",
        }
    }

    pub fn glow_class(&self) -> &'static str {
        match self {
            AlertVariant::Info => "sd-alert--glow-info",
            AlertVariant::Success => "sd-alert--glow-success",
            AlertVariant::Warning => "sd-alert--glow-warning",
            AlertVariant::Danger => "sd-alert--glow-danger",
        }
    }

    pub fn title_class(&self) -> &'static str {
        match self {
            AlertVariant::Info => "sd-alert__title--info",
            AlertVariant::Success => "sd-alert__title--success",
            AlertVariant::Warning => "sd-alert__title--warning",
            AlertVariant::Danger => "sd-alert__title--danger",
        }
    }

    pub fn icon_kind(&self) -> IconKind {
        match self {
            AlertVariant::Info => IconKind::Info,
            AlertVariant::Success => IconKind::CheckCircle,
            AlertVariant::Warning => IconKind::AlertTriangle,
            AlertVariant::Danger => IconKind::XCircle,
        }
    }

    pub fn icon_color_class(&self) -> &'static str {
        match self {
            AlertVariant::Info => "sd-alert__icon--info",
            AlertVariant::Success => "sd-alert__icon--success",
            AlertVariant::Warning => "sd-alert__icon--warning",
            AlertVariant::Danger => "sd-alert__icon--danger",
        }
    }
}

#[component]
pub fn Alert(
    #[props(default)] title: Option<String>,
    #[props(default)] variant: AlertVariant,
    #[props(default)] class: String,
    #[props(default)] glow: Option<bool>,
    on_close: Option<EventHandler<MouseEvent>>,
    children: Element,
) -> Element {
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
    let glow_class = theme_val.resolve_elevation_class("alert", glow);

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

    rsx! {
        div {
            class: "sd-alert {base_override} {variant_class} {glow_class} {radius_class} {class_modifiers} {class}",
            span { class: "sd-alert__icon {icon_color_class}",
                Icon { kind: icon_kind, size: 18 }
            }
            div { class: "sd-alert__content",
                if let Some(t) = title {
                    h4 { class: "sd-alert__title {title_variant_class}", "{t}" }
                }
                div { class: "sd-alert__body", {children} }
            }
            if let Some(close_handler) = on_close {
                button {
                    r#type: "button",
                    class: "sd-alert__close",
                    onclick: move |e| close_handler.call(e),
                    Icon { kind: IconKind::X, size: 14 }
                }
            }
        }
    }
}
