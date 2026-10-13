use dioxus::prelude::*;

/// Dimensioni disponibili per la barra di avanzamento.
#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub enum ProgressSize {
    Sm,
    #[default]
    Md,
    Lg,
}

impl ProgressSize {
    pub fn scss_class(&self) -> &'static str {
        match self {
            ProgressSize::Sm => "sd-progress--sm",
            ProgressSize::Md => "sd-progress--md",
            ProgressSize::Lg => "sd-progress--lg",
        }
    }
}

/// Varianti cromatiche semantiche per `Progress`.
#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub enum ProgressVariant {
    #[default]
    Primary,
    Success,
    Warning,
    Danger,
}

impl ProgressVariant {
    pub fn scss_class(&self) -> &'static str {
        match self {
            ProgressVariant::Primary => "sd-progress--primary",
            ProgressVariant::Success => "sd-progress--success",
            ProgressVariant::Warning => "sd-progress--warning",
            ProgressVariant::Danger => "sd-progress--danger",
        }
    }
}

/// Componente ProgressBar / Progress per visualizzare avanzamento upload, transfer chunk e scansioni beacon.
#[component]
pub fn ProgressBar(
    #[props(default = 0.0)] value: f32,
    #[props(default)] size: ProgressSize,
    #[props(default)] variant: ProgressVariant,
    #[props(default)] indeterminate: bool,
    #[props(default)] show_label: bool,
    #[props(default)] label: Option<String>,
    #[props(default)] class: String,
) -> Element {
    let clamped_val = value.clamp(0.0, 100.0);
    let size_class = size.scss_class();
    let variant_class = variant.scss_class();
    let indeterminate_class = if indeterminate { "sd-progress--indeterminate" } else { "" };

    #[cfg(feature = "theme")]
    let theme_val = crate::theme::use_theme()();
    #[cfg(feature = "theme")]
    let base_override = theme_val.resolve_component_base("progress").unwrap_or("");
    #[cfg(feature = "theme")]
    let class_modifiers = theme_val.resolve_class_modifiers("progress", &class);

    #[cfg(not(feature = "theme"))]
    let base_override = "";
    #[cfg(not(feature = "theme"))]
    let class_modifiers = String::new();

    let display_label = if let Some(lbl) = label {
        lbl
    } else if show_label && !indeterminate {
        format!("{:.0}%", clamped_val)
    } else {
        String::new()
    };

    let bar_width = if indeterminate {
        "100%".to_string()
    } else {
        format!("{:.1}%", clamped_val)
    };

    rsx! {
        div {
            class: "sd-progress-wrapper {class}",
            if !display_label.is_empty() {
                div {
                    class: "sd-progress__header",
                    span { class: "sd-progress__label", "{display_label}" }
                }
            }
            div {
                class: "sd-progress {size_class} {variant_class} {indeterminate_class} {base_override} {class_modifiers}",
                role: "progressbar",
                aria_valuenow: "{clamped_val}",
                aria_valuemin: "0",
                aria_valuemax: "100",

                div {
                    class: "sd-progress__bar",
                    style: "width: {bar_width};",
                }
            }
        }
    }
}

/// Alias per `ProgressBar` per convenienza.
pub use ProgressBar as Progress;
