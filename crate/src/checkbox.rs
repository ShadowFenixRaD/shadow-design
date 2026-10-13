use crate::icon::{Icon, IconKind};
use dioxus::prelude::*;

#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub enum CheckboxSize {
    Sm,
    #[default]
    Md,
    Lg,
}

impl CheckboxSize {
    #[must_use]
    pub fn scss_class(&self) -> &'static str {
        match self {
            CheckboxSize::Sm => "sd-checkbox__box--sm",
            CheckboxSize::Md => "sd-checkbox__box--md",
            CheckboxSize::Lg => "sd-checkbox__box--lg",
        }
    }

    #[must_use]
    pub fn icon_size(&self) -> u32 {
        match self {
            CheckboxSize::Sm => 11,
            CheckboxSize::Md => 13,
            CheckboxSize::Lg => 16,
        }
    }
}

#[component]
pub fn Checkbox(
    checked: Signal<bool>,
    #[props(default)] label: Option<String>,
    #[props(default)] description: Option<String>,
    #[props(default)] disabled: bool,
    #[props(default)] error: Option<String>,
    #[props(default)] indeterminate: bool,
    #[props(default)] size: CheckboxSize,
    #[props(default)] glow: Option<bool>,
    #[props(default)] id: Option<String>,
    #[props(default)] class: String,
    #[props(default)] on_change: Option<EventHandler<bool>>,
) -> Element {
    let is_checked = *checked.read();
    let input_id = crate::a11y::use_unique_id("sd-checkbox", id);
    let desc_id = if description.is_some() {
        Some(format!("{}-desc", input_id))
    } else {
        None
    };
    let err_id = if error.is_some() {
        Some(format!("{}-err", input_id))
    } else {
        None
    };
    let described_by = match (&desc_id, &err_id) {
        (Some(d), Some(e)) => Some(format!("{} {}", d, e)),
        (Some(d), None) => Some(d.clone()),
        (None, Some(e)) => Some(e.clone()),
        (None, None) => None,
    };

    #[cfg(feature = "theme")]
    let theme = crate::theme::use_theme();
    #[cfg(feature = "theme")]
    let theme_val = theme.read();
    #[cfg(feature = "theme")]
    let active_color_class = match &theme_val.primary_color {
        crate::theme::PrimaryColor::Violet => "sd-checkbox__box--checked".to_string(),
        crate::theme::PrimaryColor::Obsidian => "sd-checkbox__box--checked-obsidian".to_string(),
        crate::theme::PrimaryColor::Crimson => "sd-checkbox__box--checked-crimson".to_string(),
        crate::theme::PrimaryColor::Cyan => "sd-checkbox__box--checked-cyan".to_string(),
        crate::theme::PrimaryColor::Emerald => "sd-checkbox__box--checked-emerald".to_string(),
        crate::theme::PrimaryColor::Custom { .. } => {
            format!(
                "sd-checkbox__box--checked-{}",
                theme_val.primary_color.css_prefix()
            )
        }
    };
    #[cfg(feature = "theme")]
    let glow_class = if is_checked || indeterminate {
        theme_val.resolve_elevation_class("checkbox", glow)
    } else {
        String::new()
    };
    #[cfg(feature = "theme")]
    let base_override = theme_val.resolve_component_base("checkbox").unwrap_or("");
    #[cfg(feature = "theme")]
    let class_modifiers = theme_val.resolve_class_modifiers("checkbox", &class);

    #[cfg(not(feature = "theme"))]
    let base_override = "";
    #[cfg(not(feature = "theme"))]
    let class_modifiers = String::new();
    #[cfg(not(feature = "theme"))]
    let active_color_class = "sd-checkbox__box--checked".to_string();
    #[cfg(not(feature = "theme"))]
    let glow_class = if glow == Some(true) || (glow.is_none() && (is_checked || indeterminate)) {
        "sd-checkbox__box--glow".to_string()
    } else {
        String::new()
    };

    let checked_class = if is_checked || indeterminate {
        active_color_class.as_str()
    } else {
        ""
    };

    let error_class = if error.is_some() {
        "sd-checkbox__box--error"
    } else {
        ""
    };

    let box_size = size.scss_class();
    let disabled_class = if disabled {
        "sd-checkbox--disabled"
    } else {
        ""
    };

    rsx! {
        label {
            class: "sd-checkbox {base_override} {disabled_class} {class_modifiers} {class}",
            r#for: "{input_id}",
            input {
                r#type: "checkbox",
                class: "sd-visually-hidden",
                id: "{input_id}",
                checked: is_checked,
                disabled: disabled,
                "aria-checked": if indeterminate { "mixed" } else if is_checked { "true" } else { "false" },
                "aria-invalid": if error.is_some() { "true" } else { "false" },
                "aria-describedby": described_by.as_deref().unwrap_or(""),
                onchange: move |e| {
                    if !disabled {
                        let next = if indeterminate { true } else { e.checked() };
                        *checked.write() = next;
                        if let Some(handler) = on_change.as_ref() {
                            handler.call(next);
                        }
                    }
                },
            }
            span {
                class: "sd-checkbox__box {box_size} {checked_class} {glow_class} {error_class}",
                "aria-hidden": "true",
                if indeterminate {
                    span { style: "display: block; width: 10px; height: 2px; background: currentColor; border-radius: 9999px;" }
                } else if is_checked {
                    Icon { kind: IconKind::Check, size: size.icon_size() }
                }
            }
            if label.is_some() || description.is_some() || error.is_some() {
                span { class: "sd-checkbox__label",
                    if let Some(lbl) = label.as_ref() {
                        span { class: "sd-checkbox__title", "{lbl}" }
                    }
                    if let Some(desc) = description.as_ref() {
                        span {
                            id: desc_id.as_deref().unwrap_or(""),
                            class: "sd-checkbox__description",
                            "{desc}"
                        }
                    }
                    if let Some(err) = error.as_ref() {
                        span {
                            id: err_id.as_deref().unwrap_or(""),
                            class: "sd-checkbox__error-text",
                            "{err}"
                        }
                    }
                }
            }
        }
    }
}
