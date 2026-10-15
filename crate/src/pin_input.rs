use dioxus::prelude::*;
use crate::a11y::ControlSize;
use crate::util::cx;

/// Visual size of individual PIN/OTP input cells.
pub type PinInputSize = ControlSize;

/// Mode for input characters accepted by [`PinInput`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PinInputType {
    /// Accepts all alphanumeric characters (letters and numbers).
    #[default]
    Alphanumeric,
    /// Accepts numeric characters only ('0'..='9').
    Numeric,
}

/// Properties for configuring the [`PinInput`] / OTP component.
#[derive(Props, Clone, PartialEq)]
pub struct PinInputProps {
    /// Reactive string signal binding the full PIN value.
    pub value: Signal<String>,
    /// Number of individual digit or character cells to display (typically 4 or 6).
    #[props(default = 4)]
    pub length: usize,
    /// Whether to obscure the entered characters with a mask dot (password style).
    #[props(default = false)]
    pub mask: bool,
    /// Whether only numeric digits (0-9) are allowed.
    #[props(default = false)]
    pub numeric_only: bool,
    /// Whether the inputs are disabled and non-interactive.
    #[props(default = false)]
    pub disabled: bool,
    /// Whether the component is in an error state.
    #[props(default = false)]
    pub error: bool,
    /// Visual size of the input boxes (Sm, Md, Lg).
    #[props(default = ControlSize::Md)]
    pub size: ControlSize,
    /// Custom placeholder character or symbol for empty cells (e.g. "○" or "•").
    #[props(default)]
    pub placeholder: Option<String>,
    /// Optional custom gap override between input cells.
    #[props(default)]
    pub gap: Option<String>,
    /// Whether to automatically focus the first cell on mount.
    #[props(default = false)]
    pub auto_focus: bool,
    /// Accessibility label for the group of inputs.
    #[props(default)]
    pub aria_label: Option<String>,
    /// Optional additional CSS classes applied to the root container.
    #[props(default)]
    pub class: Option<String>,
    /// Callback triggered when all `length` digits/characters are filled.
    #[props(default)]
    pub on_complete: Option<EventHandler<String>>,
    /// Callback triggered whenever the combined PIN value changes.
    #[props(default)]
    pub on_change: Option<EventHandler<String>>,
}

/// Helper function to sanitize and fit a raw string to the PinInput constraints.
#[must_use]
pub fn sanitize_pin_value(raw: &str, length: usize, numeric_only: bool) -> String {
    let filtered: String = if numeric_only {
        raw.chars().filter(|c| c.is_ascii_digit()).collect()
    } else {
        raw.chars().filter(|c| c.is_alphanumeric()).collect()
    };
    filtered.chars().take(length).collect()
}
