use crate::icon::{Icon, IconKind};
use dioxus::prelude::*;

pub type CheckboxSize = crate::a11y::ControlSize;

pub trait CheckboxSizeExt {
    fn scss_class(&self) -> &'static str;
    fn icon_size(&self) -> u32;
}

impl CheckboxSizeExt for CheckboxSize {
    fn scss_class(&self) -> &'static str {
        match self {
            CheckboxSize::Sm => "sd-checkbox__box--sm",
            CheckboxSize::Md => "sd-checkbox__box--md",
            CheckboxSize::Lg => "sd-checkbox__box--lg",
        }
    }
    fn icon_size(&self) -> u32 {
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

    let is_active = is_checked || indeterminate;

    #[cfg(feature = "theme")]
    let theme = crate::theme::use_theme();
    #[cfg(feature = "theme")]
    let theme_val = theme.read();
    #[cfg(feature = "theme")]
    let glow_class = if is_active {
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
    let glow_class = if is_active && glow == Some(true) {
        "sd-checkbox__box--glow".to_string()
    } else {
        String::new()
    };
    let checked_class = if is_active {
        "sd-checkbox__box--checked"
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
            "aria-disabled": if disabled { "true" } else { "false" },
            input {
                r#type: "checkbox",
                class: "sd-checkbox__input",
                id: "{input_id}",
                checked: is_checked,
                disabled: disabled,
                "aria-checked": if indeterminate { "mixed" } else if is_checked { "true" } else { "false" },
                "aria-invalid": if error.is_some() { "true" } else { "false" },
                "aria-describedby": described_by.as_deref(),
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
                    span { class: "sd-checkbox__indeterminate" }
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
                            id: desc_id.as_deref(),
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
