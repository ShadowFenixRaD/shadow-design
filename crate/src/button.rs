use dioxus::prelude::*;

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
    pub fn scss_class(&self) -> &'static str {
        match self {
            ButtonSize::Xs => "sd-btn--xs",
            ButtonSize::Sm => "sd-btn--sm",
            ButtonSize::Md => "sd-btn--md",
            ButtonSize::Lg => "sd-btn--lg",
            ButtonSize::Xl => "sd-btn--xl",
        }
    }

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

    rsx! {
        button {
            r#type: "button",
            class: "sd-btn {base_override} {variant_class} {size_class} {full_class} {loading_class} {radius_class} {glow_class} {class_modifiers} {class}",
            disabled: disabled,
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
                svg {
                    class: "sd-spinner mr-1.5",
                    xmlns: "http://www.w3.org/2000/svg",
                    fill: "none",
                    view_box: "0 0 24 24",
                    "aria-hidden": "true",
                    "focusable": "false",
                    circle {
                        class: "sd-spinner__track",
                        cx: "12",
                        cy: "12",
                        r: "10",
                        stroke: "currentColor",
                        stroke_width: "4",
                    }
                    path {
                        class: "sd-spinner__arc",
                        fill: "currentColor",
                        d: "M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z",
                    }
                }
                span { class: "sd-visually-hidden", "{effective_loading_label}" }
            } else if let Some(licon) = left_icon {
                span { class: "sd-btn__icon sd-btn__icon--left", {licon} }
            }
            span { class: "sd-btn__text", {children} }
            if !loading {
                if let Some(ricon) = right_icon {
                    span { class: "sd-btn__icon sd-btn__icon--right", {ricon} }
                }
            }
        }
    }
}
