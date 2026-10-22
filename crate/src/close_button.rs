use crate::icon::{Icon, IconKind};
use dioxus::prelude::*;

/// Predefined sizes for [`CloseButton`].
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum CloseButtonSize {
    Sm,
    #[default]
    Md,
    Lg,
}

impl CloseButtonSize {
    #[must_use]
    pub fn class_name(&self) -> &'static str {
        match self {
            Self::Sm => "sd-close-btn--sm",
            Self::Md => "sd-close-btn--md",
            Self::Lg => "sd-close-btn--lg",
        }
    }

    #[must_use]
    pub fn icon_size(&self) -> u32 {
        match self {
            Self::Sm => 14,
            Self::Md => 16,
            Self::Lg => 20,
        }
    }
}

/// Visual style variants for [`CloseButton`].
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum CloseButtonVariant {
    #[default]
    Subtle,
    Filled,
    Transparent,
}

impl CloseButtonVariant {
    #[must_use]
    pub fn class_name(&self) -> &'static str {
        match self {
            Self::Subtle => "sd-close-btn--subtle",
            Self::Filled => "sd-close-btn--filled",
            Self::Transparent => "sd-close-btn--transparent",
        }
    }
}

/// Standardized 'X' dismiss button for modals, alerts, notifications, and floating panels.
#[component]
pub fn CloseButton(
    /// Button size. Defaults to [`CloseButtonSize::Md`].
    #[props(default)]
    size: CloseButtonSize,
    /// Visual variant. Defaults to [`CloseButtonVariant::Subtle`].
    #[props(default)]
    variant: CloseButtonVariant,
    /// Accessible label and title attribute. Defaults to "Chiudi".
    #[props(default = "Chiudi".to_string())]
    aria_label: String,
    /// Whether the button is disabled. Defaults to false.
    #[props(default = false)]
    disabled: bool,
    /// Click event callback.
    #[props(default)]
    onclick: Option<EventHandler<MouseEvent>>,
    /// Additional CSS classes.
    #[props(default)]
    class: String,
) -> Element {
    let size_class = size.class_name();
    let variant_class = variant.class_name();
    let icon_sz = size.icon_size();

    let handle_click = move |e: MouseEvent| {
        if disabled {
            return;
        }
        if let Some(h) = onclick {
            h.call(e);
        }
    };

    rsx! {
        button {
            r#type: "button",
            class: "sd-close-btn {size_class} {variant_class} {class}",
            title: "{aria_label}",
            "aria-label": "{aria_label}",
            disabled: disabled,
            onclick: handle_click,
            Icon { kind: IconKind::X, size: icon_sz }
        }
    }
}
