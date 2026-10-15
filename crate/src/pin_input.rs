use dioxus::prelude::*;
use crate::a11y::{use_unique_id, ControlSize};
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

/// Tactile, segmented PIN and OTP code input component.
///
/// Supports automatic next-cell focus advance, backspace navigation,
/// full-string paste distribution, numeric-only constraint, and password masking.
#[component]
pub fn PinInput(props: PinInputProps) -> Element {
    let group_id = use_unique_id("pin-input", None);

    #[cfg(feature = "theme")]
    let theme = crate::theme::use_theme();
    #[cfg(feature = "theme")]
    let theme_val = theme.read();
    #[cfg(feature = "theme")]
    let radius_class = theme_val.radius_scale.md();
    #[cfg(not(feature = "theme"))]
    let radius_class = "rounded-lg";

    let size_modifier = match props.size {
        ControlSize::Sm => "sd-pin-input--sm",
        ControlSize::Md => "sd-pin-input--md",
        ControlSize::Lg => "sd-pin-input--lg",
    };

    let error_modifier = if props.error { "sd-pin-input--error" } else { "" };
    let disabled_modifier = if props.disabled { "sd-pin-input--disabled" } else { "" };

    let container_class = cx([
        "sd-pin-input",
        size_modifier,
        error_modifier,
        disabled_modifier,
        props.class.as_deref().unwrap_or(""),
    ]);

    let aria_label = props
        .aria_label
        .clone()
        .unwrap_or_else(|| "PIN code input".to_string());

    let default_ph = props.placeholder.clone().unwrap_or_else(|| "○".to_string());

    let auto_focus = props.auto_focus;
    let disabled = props.disabled;
    let gid_clone = group_id.clone();
    use_effect(move || {
        if auto_focus && !disabled {
            let first_cell = format!("{gid_clone}-cell-0");
            let _ = document::eval(&format!(
                r#"
                const el = document.getElementById("{first_cell}");
                if (el) {{
                    el.focus();
                }}
                "#
            ));
        }
    });

    let current_val = props.value.read().clone();
    let sanitized = sanitize_pin_value(&current_val, props.length, props.numeric_only);
    let chars: Vec<char> = sanitized.chars().collect();

    rsx! {
        div {
            class: "{container_class}",
            role: "group",
            aria_label: "{aria_label}",
            aria_disabled: if props.disabled { "true" } else { "false" },
            style: props.gap.as_ref().map(|g| format!("gap: {g};")),

            for i in 0..props.length {
                {
                    let cell_id = format!("{group_id}-cell-{i}");
                    let cell_char = chars.get(i).copied();
                    let cell_value = cell_char.map(|c| c.to_string()).unwrap_or_default();
                    let is_filled = cell_char.is_some();
                    let filled_class = if is_filled { "sd-pin-input__cell--filled" } else { "" };
                    let cell_class = cx([
                        "sd-pin-input__cell",
                        radius_class,
                        filled_class,
                    ]);

                    let cell_type = if props.mask { "password" } else { "text" };
                    let input_mode = if props.numeric_only { "numeric" } else { "text" };
                    let pattern = if props.numeric_only { "[0-9]*" } else { "[a-zA-Z0-9]*" };

                    let cell_id_click = cell_id.clone();
                    let gid_input = group_id.clone();
                    let chars_input = chars.clone();
                    let on_change_prop = props.on_change;
                    let on_complete_prop = props.on_complete;
                    let gid_keydown = group_id.clone();
                    let chars_keydown = chars.clone();
                    let mut val_input = props.value;
                    let mut val_keydown = props.value;

                    rsx! {
                        input {
                            key: "{cell_id}",
                            id: "{cell_id}",
                            r#type: "{cell_type}",
                            class: "{cell_class}",
                            value: "{cell_value}",
                            placeholder: "{default_ph}",
                            disabled: props.disabled,
                            inputmode: "{input_mode}",
                            pattern: "{pattern}",
                            maxlength: "{props.length}",
                            autocomplete: "one-time-code",
                            aria_label: format!("Digit {} of {}", i + 1, props.length),
                            aria_invalid: if props.error { "true" } else { "false" },

                            onclick: move |_| {
                                let cid = cell_id_click.clone();
                                let _ = document::eval(&format!(
                                    r#"
                                    const el = document.getElementById("{cid}");
                                    if (el) {{
                                        el.select();
                                    }}
                                    "#
                                ));
                            },

                            oninput: move |evt: FormEvent| {
                                let raw = evt.value();
                                let filtered = sanitize_pin_value(&raw, props.length, props.numeric_only);

                                if filtered.is_empty() {
                                    let mut new_chars = chars_input.clone();
                                    if i < new_chars.len() {
                                        new_chars.remove(i);
                                    }
                                    let new_val: String = new_chars.into_iter().collect();
                                    val_input.set(new_val.clone());
                                    if let Some(handler) = on_change_prop.as_ref() {
                                        handler.call(new_val);
                                    }
                                    return;
                                }

                                let in_chars: Vec<char> = filtered.chars().collect();
                                if in_chars.len() == 1 {
                                    let mut new_chars = chars_input.clone();
                                    while new_chars.len() < i {
                                        new_chars.push(' ');
                                    }
                                    if i < new_chars.len() {
                                        new_chars[i] = in_chars[0];
                                    } else {
                                        new_chars.push(in_chars[0]);
                                    }
                                    let new_val: String = new_chars.iter().take(props.length).collect();
                                    val_input.set(new_val.clone());
                                    if let Some(handler) = on_change_prop.as_ref() {
                                        handler.call(new_val.clone());
                                    }
                                    if new_val.chars().count() == props.length {
                                        if let Some(complete) = on_complete_prop.as_ref() {
                                            complete.call(new_val);
                                        }
                                    }
                                    if i + 1 < props.length {
                                        let next_id = format!("{gid_input}-cell-{}", i + 1);
                                        let _ = document::eval(&format!(
                                            r#"
                                            const el = document.getElementById("{next_id}");
                                            if (el) {{
                                                el.focus();
                                                el.select();
                                            }}
                                            "#
                                        ));
                                    }
                                } else {
                                    // Multi-character paste / autofill
                                    let mut final_chars = chars_input.clone();
                                    for (offset, ch) in in_chars.into_iter().enumerate() {
                                        let target = i + offset;
                                        if target < props.length {
                                            if target < final_chars.len() {
                                                final_chars[target] = ch;
                                            } else {
                                                final_chars.push(ch);
                                            }
                                        }
                                    }
                                    let new_val: String = final_chars.iter().take(props.length).collect();
                                    let count = new_val.chars().count();
                                    val_input.set(new_val.clone());
                                    if let Some(handler) = on_change_prop.as_ref() {
                                        handler.call(new_val.clone());
                                    }
                                    if count == props.length {
                                        if let Some(complete) = on_complete_prop.as_ref() {
                                            complete.call(new_val);
                                        }
                                    }
                                    let target_focus = (i + filtered.chars().count()).min(props.length - 1);
                                    let target_id = format!("{gid_input}-cell-{target_focus}");
                                    let _ = document::eval(&format!(
                                        r#"
                                        const el = document.getElementById("{target_id}");
                                        if (el) {{
                                            el.focus();
                                            el.select();
                                        }}
                                        "#
                                    ));
                                }
                            },

                            onkeydown: move |evt: KeyboardEvent| {
                                match evt.key() {
                                    Key::Backspace => {
                                        if cell_value.is_empty() && i > 0 {
                                            let prev_id = format!("{gid_keydown}-cell-{}", i - 1);
                                            let mut new_chars = chars_keydown.clone();
                                            if i - 1 < new_chars.len() {
                                                new_chars.remove(i - 1);
                                                let new_val: String = new_chars.into_iter().collect();
                                                val_keydown.set(new_val.clone());
                                                if let Some(handler) = on_change_prop.as_ref() {
                                                    handler.call(new_val);
                                                }
                                            }
                                            let _ = document::eval(&format!(
                                                r#"
                                                const el = document.getElementById("{prev_id}");
                                                if (el) {{
                                                    el.focus();
                                                    el.select();
                                                }}
                                                "#
                                            ));
                                        } else if !cell_value.is_empty() {
                                            let mut new_chars = chars_keydown.clone();
                                            if i < new_chars.len() {
                                                new_chars.remove(i);
                                                let new_val: String = new_chars.into_iter().collect();
                                                val_keydown.set(new_val.clone());
                                                if let Some(handler) = on_change_prop.as_ref() {
                                                    handler.call(new_val);
                                                }
                                            }
                                        }
                                    }
                                    Key::ArrowLeft => {
                                        if i > 0 {
                                            let prev_id = format!("{gid_keydown}-cell-{}", i - 1);
                                            let _ = document::eval(&format!(
                                                r#"
                                                const el = document.getElementById("{prev_id}");
                                                if (el) {{
                                                    el.focus();
                                                    el.select();
                                                }}
                                                "#
                                            ));
                                        }
                                    }
                                    Key::ArrowRight => {
                                        if i + 1 < props.length {
                                            let next_id = format!("{gid_keydown}-cell-{}", i + 1);
                                            let _ = document::eval(&format!(
                                                r#"
                                                const el = document.getElementById("{next_id}");
                                                if (el) {{
                                                    el.focus();
                                                    el.select();
                                                }}
                                                "#
                                            ));
                                        }
                                    }
                                    _ => {}
                                }
                            },
                        }
                    }
                }
            }
        }
    }
}
