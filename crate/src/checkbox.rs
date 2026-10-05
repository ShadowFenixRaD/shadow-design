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
    pub fn scss_class(&self) -> &'static str {
        match self {
            CheckboxSize::Sm => "sd-checkbox__box--sm",
            CheckboxSize::Md => "sd-checkbox__box--md",
            CheckboxSize::Lg => "sd-checkbox__box--lg",
        }
    }

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
    #[props(default)] class: String,
) -> Element {
    let is_checked = *checked.read();

    #[cfg(feature = "theme")]
    let theme = crate::theme::use_theme();
    #[cfg(feature = "theme")]
    let active_color_class = match theme().primary_color {
        crate::theme::PrimaryColor::Violet => "sd-checkbox__box--checked",
        crate::theme::PrimaryColor::Obsidian => "sd-checkbox__box--checked-obsidian",
        crate::theme::PrimaryColor::Crimson => "sd-checkbox__box--checked-crimson",
        crate::theme::PrimaryColor::Cyan => "sd-checkbox__box--checked-cyan",
        crate::theme::PrimaryColor::Emerald => "sd-checkbox__box--checked-emerald",
    };
    #[cfg(feature = "theme")]
    let glow_class = match glow {
        Some(true) => "sd-checkbox__box--glow",
        Some(false) => "",
        None => {
            if theme().glow_mode != crate::theme::GlowMode::Off && (is_checked || indeterminate) {
                "sd-checkbox__box--glow"
            } else {
                ""
            }
        }
    };

    #[cfg(not(feature = "theme"))]
    let active_color_class = "sd-checkbox__box--checked";
    #[cfg(not(feature = "theme"))]
    let glow_class = if is_checked || indeterminate {
        "sd-checkbox__box--glow"
    } else {
        ""
    };

    let checked_class = if is_checked || indeterminate {
        active_color_class
    } else {
        ""
    };

    let error_class = if error.is_some() {
        "sd-checkbox__box--error"
    } else {
        ""
    };

    let box_size = size.scss_class();
    let is_disabled = disabled;
    let disabled_class = if is_disabled {
        "sd-checkbox--disabled"
    } else {
        ""
    };

    rsx! {
        div {
            class: "sd-checkbox {disabled_class} {class}",
            onclick: move |_| {
                if !is_disabled {
                    let next = !*checked.read();
                    *checked.write() = next;
                }
            },
            div {
                class: "sd-checkbox__box {box_size} {checked_class} {glow_class} {error_class}",
                if indeterminate {
                    span { style: "display: block; width: 10px; height: 2px; background: currentColor; border-radius: 9999px;" }
                } else if is_checked {
                    Icon { kind: IconKind::Check, size: size.icon_size() }
                }
            }
            if label.is_some() || description.is_some() || error.is_some() {
                div { class: "sd-checkbox__label",
                    if let Some(lbl) = label {
                        span { class: "sd-checkbox__title", "{lbl}" }
                    }
                    if let Some(desc) = description {
                        span { class: "sd-checkbox__description", "{desc}" }
                    }
                    if let Some(err) = error {
                        span { class: "sd-checkbox__error-text", "{err}" }
                    }
                }
            }
        }
    }
}
