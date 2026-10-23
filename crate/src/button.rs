use dioxus::prelude::*;
use crate::icon::{Icon, IconKind};

#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub enum ButtonVariant {
    #[default]
    Primary,
    Secondary,
    Outline,
    Ghost,
    Danger,
    Success,
}

impl ButtonVariant {
    #[must_use]
    pub fn scss_class(&self) -> &'static str {
        match self {
            ButtonVariant::Primary => "sd-btn--primary",
            ButtonVariant::Secondary => "sd-btn--secondary",
            ButtonVariant::Outline => "sd-btn--outline",
            ButtonVariant::Ghost => "sd-btn--ghost",
            ButtonVariant::Danger => "sd-btn--danger",
            ButtonVariant::Success => "sd-btn--success",
        }
    }

    #[must_use]
    pub fn name(&self) -> &'static str {
        match self {
            ButtonVariant::Primary => "primary",
            ButtonVariant::Secondary => "secondary",
            ButtonVariant::Outline => "outline",
            ButtonVariant::Ghost => "ghost",
            ButtonVariant::Danger => "danger",
            ButtonVariant::Success => "success",
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub enum ButtonSize {
    Xs,
    Sm,
    #[default]
    Md,
    Lg,
    Xl,
}

impl ButtonSize {
    #[must_use]
    pub fn scss_class(&self) -> &'static str {
        match self {
            ButtonSize::Xs => "sd-btn--xs",
            ButtonSize::Sm => "sd-btn--sm",
            ButtonSize::Md => "sd-btn--md",
            ButtonSize::Lg => "sd-btn--lg",
            ButtonSize::Xl => "sd-btn--xl",
        }
    }

    #[must_use]
    pub fn name(&self) -> &'static str {
        match self {
            ButtonSize::Xs => "xs",
            ButtonSize::Sm => "sm",
            ButtonSize::Md => "md",
            ButtonSize::Lg => "lg",
            ButtonSize::Xl => "xl",
        }
    }
}

#[component]
pub fn Button(
    #[props(default)] variant: ButtonVariant,
    #[props(default)] size: ButtonSize,
    #[props(default)] disabled: bool,
    #[props(default)] loading: bool,
    #[props(default)] loading_label: Option<String>,
    #[props(default)] full_width: bool,
    #[props(default)] glow: Option<bool>,
    #[props(default)] left_icon: Option<Element>,
    #[props(default)] right_icon: Option<Element>,
    #[props(default)] class: String,
    #[props(default)] aria_label: Option<String>,
    #[props(default)] aria_pressed: Option<bool>,
    #[props(default)] button_type: Option<String>,
    onclick: Option<EventHandler<MouseEvent>>,
    children: Element,
) -> Element {
    let size_class = size.scss_class();
    let full_class = if full_width { "sd-btn--full" } else { "" };
    let loading_class = if loading { "sd-btn--loading" } else { "" };

    // DS-03: By default (glow is None), only filled emphasis variants glow
    let should_glow_by_default = matches!(
        variant,
        ButtonVariant::Primary | ButtonVariant::Danger | ButtonVariant::Success
    );
    #[cfg(feature = "theme")]
    let effective_glow = match glow {
        Some(g) => Some(g),
        None => {
            if should_glow_by_default {
                None
            } else {
                Some(false)
            }
        }
    };

    #[cfg(feature = "theme")]
    let theme = crate::theme::use_theme();
    #[cfg(feature = "theme")]
    let theme_val = theme.read();

    // RS-05: Use theme_val.button_variant_classes(variant) as fallback
    #[cfg(feature = "theme")]
    let fallback_variant = theme_val.button_variant_classes(variant).to_string();

    #[cfg(feature = "theme")]
    let variant_class =
        theme_val.resolve_variant_class("button", variant.name(), &fallback_variant);

    #[cfg(feature = "theme")]
    let base_override = theme_val.resolve_component_base("button").unwrap_or("");

    #[cfg(feature = "theme")]
    let class_modifiers = theme_val.resolve_class_modifiers("button", &class);

    #[cfg(feature = "theme")]
    let radius_class = match size {
        ButtonSize::Xs | ButtonSize::Sm => theme_val.radius_scale.sm(),
        ButtonSize::Md => theme_val.radius_scale.md(),
        ButtonSize::Lg | ButtonSize::Xl => theme_val.radius_scale.lg(),
    };

    #[cfg(feature = "theme")]
    let glow_class = theme_val.resolve_elevation_class("button", effective_glow);

    #[cfg(not(feature = "theme"))]
    let base_override = "";
    #[cfg(not(feature = "theme"))]
    let class_modifiers = String::new();
    #[cfg(not(feature = "theme"))]
    let variant_class = variant.scss_class().to_string();
    #[cfg(not(feature = "theme"))]
    let radius_class = "";
    #[cfg(not(feature = "theme"))]
    let glow_class = if glow == Some(true) || (glow.is_none() && should_glow_by_default) {
        "sd-btn--glow".to_string()
    } else {
        String::new()
    };

    let locale = crate::locale::use_locale();
    let effective_loading_label = loading_label.unwrap_or(locale.loading);

    let btn_type = button_type.unwrap_or_else(|| "button".to_string());

    rsx! {
        button {
            r#type: "{btn_type}",
            class: "sd-btn {base_override} {variant_class} {size_class} {full_class} {loading_class} {radius_class} {glow_class} {class_modifiers} {class}",
            disabled: disabled,
            aria_label: aria_label,
            aria_pressed: aria_pressed.map(|p| p.to_string()),
            "aria-disabled": if disabled || loading { "true" } else { "false" },
            "aria-busy": if loading { "true" } else { "false" },
            onclick: move |evt| {
                if !disabled && !loading {
                    if let Some(handler) = onclick.as_ref() {
                        handler.call(evt);
                    }
                }
            },
            if loading {
                Icon {
                    kind: IconKind::Spinner,
                    size: 16,
                    class: "sd-spinner",
                }
                span { class: "sd-visually-hidden", "{effective_loading_label}" }
            } else {
                if let Some(licon) = left_icon {
                    span { class: "sd-btn__icon sd-btn__icon--left", {licon} }
                }
                span { class: "sd-btn__text", {children} }
                if let Some(ricon) = right_icon {
                    span { class: "sd-btn__icon sd-btn__icon--right", {ricon} }
                }
            }
        }
    }
}
