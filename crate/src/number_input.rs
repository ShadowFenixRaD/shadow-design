use dioxus::prelude::*;

#[cfg(feature = "icon")]
use crate::icon::{Icon, IconKind};

/// Utility helper to format and clamp a numeric value according to bounds and precision.
#[must_use]
pub fn format_and_clamp_number(
    val: f64,
    min: Option<f64>,
    max: Option<f64>,
    precision: usize,
) -> f64 {
    let mut clamped = val;
    if let Some(min_v) = min {
        if clamped < min_v {
            clamped = min_v;
        }
    }
    if let Some(max_v) = max {
        if clamped > max_v {
            clamped = max_v;
        }
    }
    let factor = 10f64.powi(precision as i32);
    (clamped * factor).round() / factor
}

/// Format a number with given precision string representation.
#[must_use]
pub fn format_number_string(val: f64, precision: usize) -> String {
    if precision == 0 {
        format!("{:.0}", val.round())
    } else {
        format!("{val:.precision$}")
    }
}

/// Controllo numerico `NumberInput` / `SpinButton` con pulsanti stepper, precisione decimale,
/// limiti min/max, supporto per prefisso/suffisso e navigazione da tastiera.
#[component]
pub fn NumberInput(
    value: Signal<f64>,
    #[props(default = 1.0)] step: f64,
    #[props(default)] min: Option<f64>,
    #[props(default)] max: Option<f64>,
    #[props(default = 0)] precision: usize,
    #[props(default)] prefix: Option<String>,
    #[props(default)] suffix: Option<String>,
    #[props(default)] label: Option<String>,
    #[props(default)] placeholder: Option<String>,
    #[props(default)] help_text: Option<String>,
    #[props(default)] error: Option<String>,
    #[props(default)] disabled: bool,
    #[props(default)] glow: Option<bool>,
    #[props(default)] id: Option<String>,
    #[props(default)] class: String,
    #[props(default)] on_change: Option<EventHandler<f64>>,
) -> Element {
    let input_id = crate::a11y::use_unique_id("sd-number-input", id);
    let help_id = if help_text.is_some() {
        Some(format!("{input_id}-help"))
    } else {
        None
    };
    let err_id = if error.is_some() {
        Some(format!("{input_id}-err"))
    } else {
        None
    };
    let described_by = match (&help_id, &err_id) {
        (Some(h), Some(e)) => Some(format!("{h} {e}")),
        (Some(h), None) => Some(h.clone()),
        (None, Some(e)) => Some(e.clone()),
        (None, None) => None,
    };

    #[cfg(feature = "theme")]
    let theme = crate::theme::use_theme();
    #[cfg(feature = "theme")]
    let theme_val = theme.read();
    #[cfg(feature = "theme")]
    let radius_class = theme_val.radius_scale.md();
    #[cfg(feature = "theme")]
    let base_override = theme_val.resolve_component_base("number_input").unwrap_or("");
    #[cfg(feature = "theme")]
    let class_modifiers = theme_val.resolve_class_modifiers("number_input", &class);
    #[cfg(feature = "theme")]
    let glow_class = theme_val.resolve_elevation_class("number_input", glow);
    #[cfg(feature = "theme")]
    let error_class = if error.is_some() {
        theme_val.input_border_class(true)
    } else {
        theme_val.input_border_class(false)
    };

    #[cfg(not(feature = "theme"))]
    let base_override = "";
    #[cfg(not(feature = "theme"))]
    let class_modifiers = String::new();
    #[cfg(not(feature = "theme"))]
    let radius_class = "";
    #[cfg(not(feature = "theme"))]
    let glow_class = if glow.unwrap_or(false) {
        "sd-number-input--glow".to_string()
    } else {
        String::new()
    };
    #[cfg(not(feature = "theme"))]
    let error_class = if error.is_some() {
        "sd-number-input--error"
    } else {
        ""
    };

    let current_val = *value.read();
    let display_str = format_number_string(current_val, precision);

    let can_increment = !disabled && max.is_none_or(|m| current_val + step <= m + 1e-9);
    let can_decrement = !disabled && min.is_none_or(|m| current_val - step >= m - 1e-9);

    let do_update = move |new_val: f64| {
        let clamped = format_and_clamp_number(new_val, min, max, precision);
        let mut value = value;
        *value.write() = clamped;
        if let Some(handler) = on_change.as_ref() {
            handler.call(clamped);
        }
    };

    let do_increment = move || {
        if can_increment {
            let current = *value.read();
            do_update(current + step);
        }
    };

    let do_decrement = move || {
        if can_decrement {
            let current = *value.read();
            do_update(current - step);
        }
    };

    let disabled_class = if disabled { "sd-number-input--disabled" } else { "" };
    let up_disabled_class = if !can_increment { "sd-number-input__stepper--disabled" } else { "" };
    let down_disabled_class = if !can_decrement { "sd-number-input__stepper--disabled" } else { "" };

    let is_err = error.is_some();

    let mut is_focused = use_signal(|| false);
    let mut raw_text = use_signal(|| display_str.clone());

    rsx! {
        div {
            class: "sd-number-input-wrapper {class_modifiers} {class}",
            if let Some(lbl) = &label {
                label {
                    r#for: "{input_id}",
                    class: "sd-number-input-label",
                    "{lbl}"
                }
            }

            div {
                class: "sd-number-input-container {base_override} {error_class} {radius_class} {glow_class} {disabled_class}",

                if let Some(pref) = &prefix {
                    span { class: "sd-number-input__prefix", "{pref}" }
                }

                input {
                    r#type: "text",
                    id: "{input_id}",
                    role: "spinbutton",
                    "aria-valuenow": "{current_val}",
                    "aria-valuemin": min.map(|v| v.to_string()),
                    "aria-valuemax": max.map(|v| v.to_string()),
                    "aria-invalid": if is_err { "true" } else { "false" },
                    "aria-describedby": described_by.as_deref(),
                    disabled,
                    placeholder: placeholder.as_deref(),
                    class: "sd-number-input__field",
                    value: if is_focused() { raw_text() } else { display_str.clone() },
                    onfocus: move |_| {
                        is_focused.set(true);
                        raw_text.set(display_str.clone());
                    },
                    onblur: move |_| {
                        is_focused.set(false);
                        let text = raw_text();
                        if let Ok(parsed) = text.trim().parse::<f64>() {
                            do_update(parsed);
                        } else if text.trim().is_empty() {
                            let fallback = min.unwrap_or(0.0);
                            do_update(fallback);
                        }
                    },
                    oninput: move |e| {
                        let text = e.value();
                        raw_text.set(text.clone());
                        if let Ok(parsed) = text.trim().parse::<f64>() {
                            let clamped = parsed.clamp(min.unwrap_or(f64::NEG_INFINITY), max.unwrap_or(f64::INFINITY));
                            *value.write() = clamped;
                            if let Some(handler) = on_change.as_ref() {
                                handler.call(clamped);
                            }
                        }
                    },
                    onkeydown: move |e| {
                        match e.key() {
                            Key::ArrowUp => {
                                e.prevent_default();
                                do_increment();
                            }
                            Key::ArrowDown => {
                                e.prevent_default();
                                do_decrement();
                            }
                            _ => {}
                        }
                    },
                }

                if let Some(suff) = &suffix {
                    span { class: "sd-number-input__suffix", "{suff}" }
                }

                // Steppers Vertical Column (+ / -)
                div {
                    class: "sd-number-input__steppers",
                    button {
                        r#type: "button",
                        tabindex: "-1",
                        disabled: !can_increment,
                        "aria-label": "Incrementa",
                        class: "sd-number-input__stepper sd-number-input__stepper--up {up_disabled_class}",
                        onclick: move |e| {
                            e.stop_propagation();
                            do_increment();
                        },
                        Icon { kind: IconKind::ChevronUp, size: 11 }
                    }
                    button {
                        r#type: "button",
                        tabindex: "-1",
                        disabled: !can_decrement,
                        "aria-label": "Decrementa",
                        class: "sd-number-input__stepper sd-number-input__stepper--down {down_disabled_class}",
                        onclick: move |e| {
                            e.stop_propagation();
                            do_decrement();
                        },
                        Icon { kind: IconKind::ChevronDown, size: 11 }
                    }
                }
            }

            if let Some(err) = &error {
                p {
                    id: err_id.as_deref().unwrap_or(""),
                    class: "sd-number-input-error",
                    "{err}"
                }
            } else if let Some(help) = &help_text {
                p {
                    id: help_id.as_deref().unwrap_or(""),
                    class: "sd-number-input-help",
                    "{help}"
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_and_clamp_within_bounds() {
        let val = format_and_clamp_number(5.456, Some(0.0), Some(10.0), 2);
        assert!((val - 5.46).abs() < 1e-6);
    }

    #[test]
    fn test_format_and_clamp_under_min() {
        let val = format_and_clamp_number(-5.0, Some(0.0), Some(10.0), 1);
        assert!((val - 0.0).abs() < 1e-6);
    }

    #[test]
    fn test_format_and_clamp_over_max() {
        let val = format_and_clamp_number(15.0, Some(0.0), Some(10.0), 0);
        assert!((val - 10.0).abs() < 1e-6);
    }

    #[test]
    fn test_format_number_string_precision() {
        assert_eq!(format_number_string(42.0, 0), "42");
        assert_eq!(format_number_string(42.1234, 2), "42.12");
        assert_eq!(format_number_string(42.5, 1), "42.5");
    }
}
