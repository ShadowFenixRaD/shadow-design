use dioxus::prelude::*;

#[cfg(feature = "icon")]
use crate::icon::{RenderSvgIcon, SvgIcon};

/// Dimensione del SegmentedControl.
#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub enum SegmentedControlSize {
    Sm,
    #[default]
    Md,
    Lg,
}

impl SegmentedControlSize {
    pub fn scss_class(&self) -> &'static str {
        match self {
            SegmentedControlSize::Sm => "sd-segmented-control--sm",
            SegmentedControlSize::Md => "sd-segmented-control--md",
            SegmentedControlSize::Lg => "sd-segmented-control--lg",
        }
    }
}

/// Opzione per il componente `SegmentedControl`.
#[derive(Clone, PartialEq, Debug)]
pub struct SegmentOption {
    pub value: String,
    pub label: String,
    #[cfg(feature = "icon")]
    pub icon: Option<SvgIcon>,
    pub disabled: bool,
}

impl SegmentOption {
    pub fn new(value: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            label: label.into(),
            #[cfg(feature = "icon")]
            icon: None,
            disabled: false,
        }
    }

    #[cfg(feature = "icon")]
    pub fn with_icon(mut self, icon: SvgIcon) -> Self {
        self.icon = Some(icon);
        self
    }

    pub fn with_disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

/// Selettore a segmenti orizzontale tattile ad alta ergonomia con micro-glow del tema attivo.
#[component]
pub fn SegmentedControl(
    options: Vec<SegmentOption>,
    value: Signal<String>,
    #[props(default)] size: SegmentedControlSize,
    #[props(default)] full_width: bool,
    #[props(default)] disabled: bool,
    #[props(default)] on_change: Option<EventHandler<String>>,
    #[props(default)] class: String,
) -> Element {
    let size_class = size.scss_class();
    let full_width_class = if full_width { "sd-segmented-control--full-width" } else { "" };
    let disabled_class = if disabled { "sd-segmented-control--disabled" } else { "" };

    #[cfg(feature = "theme")]
    let theme_val = crate::theme::use_theme()();
    #[cfg(feature = "theme")]
    let base_override = theme_val.resolve_component_base("segmented_control").unwrap_or("");
    #[cfg(feature = "theme")]
    let class_modifiers = theme_val.resolve_class_modifiers("segmented_control", &class);
    #[cfg(feature = "theme")]
    let radius = theme_val.radius_scale.sm();

    #[cfg(not(feature = "theme"))]
    let base_override = "";
    #[cfg(not(feature = "theme"))]
    let class_modifiers = String::new();
    #[cfg(not(feature = "theme"))]
    let radius = "";

    let current_val = value();

    rsx! {
        div {
            class: "sd-segmented-control {size_class} {full_width_class} {disabled_class} {radius} {base_override} {class_modifiers} {class}",
            role: "radiogroup",

            for option in options {
                {
                    let is_active = option.value == current_val;
                    let opt_val = option.value.clone();
                    let is_opt_disabled = disabled || option.disabled;
                    let active_class = if is_active { "sd-segmented-control__item--active" } else { "" };
                    let opt_disabled_class = if is_opt_disabled { "sd-segmented-control__item--disabled" } else { "" };

                    rsx! {
                        button {
                            key: "{option.value}",
                            r#type: "button",
                            role: "radio",
                            aria_checked: "{is_active}",
                            disabled: is_opt_disabled,
                            class: "sd-segmented-control__item {active_class} {opt_disabled_class}",
                            onclick: {
                                let opt_val_click = opt_val.clone();
                                move |_| {
                                    if !is_opt_disabled {
                                        *value.write() = opt_val_click.clone();
                                        if let Some(handler) = &on_change {
                                            handler.call(opt_val_click.clone());
                                        }
                                    }
                                }
                            },

                            if let Some(opt_icon) = option.icon {
                                RenderSvgIcon {
                                    icon: opt_icon,
                                    size: 14,
                                    class: "sd-segmented-control__icon".to_string(),
                                }
                            }

                            span {
                                class: "sd-segmented-control__label",
                                "{option.label}"
                            }
                        }
                    }
                }
            }
        }
    }
}
