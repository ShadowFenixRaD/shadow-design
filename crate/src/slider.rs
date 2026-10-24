use dioxus::prelude::*;
use crate::tooltip::{Tooltip, TooltipPosition};

/// Tacca o punto di riferimento graduato su uno `Slider` o `RangeSlider`.
#[derive(Clone, PartialEq, Debug)]
pub struct SliderMark {
    pub value: f64,
    pub label: Option<String>,
}

impl SliderMark {
    #[must_use]
    pub fn new(value: f64, label: impl Into<String>) -> Self {
        Self {
            value,
            label: Some(label.into()),
        }
    }

    #[must_use]
    pub fn point(value: f64) -> Self {
        Self { value, label: None }
    }
}

/// Helper per calcolare la percentuale normalizzata [0.0, 100.0] di un valore tra min e max.
#[must_use]
pub fn calc_percentage(val: f64, min: f64, max: f64) -> f64 {
    if (max - min).abs() < 1e-9 {
        return 0.0;
    }
    ((val - min) / (max - min) * 100.0).clamp(0.0, 100.0)
}

/// Helper per allineare un valore allo step specificato.
#[must_use]
pub fn snap_to_step(val: f64, min: f64, max: f64, step: f64) -> f64 {
    if step <= 0.0 {
        return val.clamp(min, max);
    }
    let steps = ((val - min) / step).round();
    let snapped = min + steps * step;
    snapped.clamp(min, max)
}

/// Cursore graduato singolo `Slider` con traccia evidenziata, tacche e tooltip fluttuante.
#[component]
pub fn Slider(
    value: Signal<f64>,
    #[props(default = 0.0)] min: f64,
    #[props(default = 100.0)] max: f64,
    #[props(default = 1.0)] step: f64,
    #[props(default)] marks: Vec<SliderMark>,
    #[props(default = true)] show_tooltip: bool,
    #[props(default)] label: Option<String>,
    #[props(default)] disabled: bool,
    #[props(default)] glow: Option<bool>,
    #[props(default)] id: Option<String>,
    #[props(default)] class: String,
    #[props(default)] on_change: Option<EventHandler<f64>>,
) -> Element {
    let slider_id = crate::a11y::use_unique_id("sd-slider", id);
    let mut is_hovered = use_signal(|| false);

    let current_val = *value.read();
    let percent = calc_percentage(current_val, min, max);

    #[cfg(feature = "theme")]
    let theme = crate::theme::use_theme();
    #[cfg(feature = "theme")]
    let theme_val = theme.read();
    #[cfg(feature = "theme")]
    let base_override = theme_val.resolve_component_base("slider").unwrap_or("");
    #[cfg(feature = "theme")]
    let class_modifiers = theme_val.resolve_class_modifiers("slider", &class);
    #[cfg(feature = "theme")]
    let glow_class = theme_val.resolve_elevation_class("slider", glow);
    #[cfg(feature = "theme")]
    let accent_bar_class = match theme_val.primary_color {
        crate::theme::PrimaryColor::Violet => "sd-slider__bar--violet",
        crate::theme::PrimaryColor::Obsidian => "sd-slider__bar--obsidian",
        crate::theme::PrimaryColor::Crimson => "sd-slider__bar--crimson",
        crate::theme::PrimaryColor::Cyan => "sd-slider__bar--cyan",
        crate::theme::PrimaryColor::Emerald => "sd-slider__bar--emerald",
        _ => "sd-slider__bar--violet",
    };

    #[cfg(not(feature = "theme"))]
    let base_override = "";
    #[cfg(not(feature = "theme"))]
    let class_modifiers = String::new();
    #[cfg(not(feature = "theme"))]
    let glow_class = if glow.unwrap_or(false) { "sd-slider--glow".to_string() } else { String::new() };
    #[cfg(not(feature = "theme"))]
    let accent_bar_class = "sd-slider__bar--violet";

    let disabled_class = if disabled { "sd-slider--disabled" } else { "" };

    let mut do_update = move |new_val: f64| {
        let snapped = snap_to_step(new_val, min, max, step);
        *value.write() = snapped;
        if let Some(handler) = on_change {
            handler.call(snapped);
        }
    };

    let tooltip_visible = show_tooltip && !disabled && *is_hovered.read();

    rsx! {
        div {
            class: "sd-slider-wrapper {class_modifiers} {class}",
            if let Some(lbl) = &label {
                div { class: "flex items-center justify-between mb-1.5",
                    label {
                        r#for: "{slider_id}",
                        class: "sd-slider-label",
                        "{lbl}"
                    }
                    span { class: "sd-slider-value-display", "{current_val:.1}" }
                }
            }

            div {
                class: "sd-slider-container {base_override} {glow_class} {disabled_class}",
                onmouseenter: move |_| is_hovered.set(true),
                onmouseleave: move |_| is_hovered.set(false),

                // Native accessible range input overlaid
                input {
                    r#type: "range",
                    id: "{slider_id}",
                    min: "{min}",
                    max: "{max}",
                    step: "{step}",
                    value: "{current_val}",
                    "aria-label": label.as_deref().unwrap_or("Slider"),
                    "aria-valuemin": "{min}",
                    "aria-valuemax": "{max}",
                    "aria-valuenow": "{current_val}",
                    disabled,
                    class: "sd-slider__native",
                    oninput: move |e| {
                        if let Ok(v) = e.value().parse::<f64>() {
                            do_update(v);
                        }
                    },
                }

                // Styled visual track
                div { class: "sd-slider__track",
                    div {
                        class: "sd-slider__bar {accent_bar_class}",
                        style: "width: {percent}%;",
                    }
                }

                // Marks
                if !marks.is_empty() {
                    div { class: "sd-slider__marks",
                        for mark in &marks {
                            {
                                let mark_pct = calc_percentage(mark.value, min, max);
                                let is_active = mark.value <= current_val;
                                let active_mark_class = if is_active { "sd-slider__mark--active" } else { "" };
                                rsx! {
                                    div {
                                        key: "{mark.value}",
                                        class: "sd-slider__mark {active_mark_class}",
                                        style: "left: {mark_pct}%;",
                                        if let Some(mlbl) = &mark.label {
                                            span { class: "sd-slider__mark-label", "{mlbl}" }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                // Visual Thumb with Tooltip
                div {
                    class: "sd-slider__thumb-wrapper",
                    style: "left: {percent}%;",
                    Tooltip {
                        content: format!("{current_val:.1}"),
                        position: TooltipPosition::Top,
                        opened: Some(tooltip_visible),
                        div { class: "sd-slider__thumb" }
                    }
                }
            }
        }
    }
}

/// Cursore a doppio cursore `RangeSlider` (intervallo min..max).
#[component]
pub fn RangeSlider(
    value: Signal<(f64, f64)>,
    #[props(default = 0.0)] min: f64,
    #[props(default = 100.0)] max: f64,
    #[props(default = 1.0)] step: f64,
    #[props(default)] marks: Vec<SliderMark>,
    #[props(default = true)] show_tooltip: bool,
    #[props(default)] label: Option<String>,
    #[props(default)] disabled: bool,
    #[props(default)] glow: Option<bool>,
    #[props(default)] id: Option<String>,
    #[props(default)] class: String,
    #[props(default)] on_change: Option<EventHandler<(f64, f64)>>,
) -> Element {
    let locale = crate::locale::use_locale();
    let slider_id = crate::a11y::use_unique_id("sd-range-slider", id);
    let mut is_hovered = use_signal(|| false);

    let (val_min, val_max) = *value.read();
    let pct_min = calc_percentage(val_min, min, max);
    let pct_max = calc_percentage(val_max, min, max);
    let bar_width = (pct_max - pct_min).max(0.0);

    #[cfg(feature = "theme")]
    let theme = crate::theme::use_theme();
    #[cfg(feature = "theme")]
    let theme_val = theme.read();
    #[cfg(feature = "theme")]
    let base_override = theme_val.resolve_component_base("slider").unwrap_or("");
    #[cfg(feature = "theme")]
    let class_modifiers = theme_val.resolve_class_modifiers("slider", &class);
    #[cfg(feature = "theme")]
    let glow_class = theme_val.resolve_elevation_class("slider", glow);
    #[cfg(feature = "theme")]
    let accent_bar_class = match theme_val.primary_color {
        crate::theme::PrimaryColor::Violet => "sd-slider__bar--violet",
        crate::theme::PrimaryColor::Obsidian => "sd-slider__bar--obsidian",
        crate::theme::PrimaryColor::Crimson => "sd-slider__bar--crimson",
        crate::theme::PrimaryColor::Cyan => "sd-slider__bar--cyan",
        crate::theme::PrimaryColor::Emerald => "sd-slider__bar--emerald",
        _ => "sd-slider__bar--violet",
    };

    #[cfg(not(feature = "theme"))]
    let base_override = "";
    #[cfg(not(feature = "theme"))]
    let class_modifiers = String::new();
    #[cfg(not(feature = "theme"))]
    let glow_class = if glow.unwrap_or(false) { "sd-slider--glow".to_string() } else { String::new() };
    #[cfg(not(feature = "theme"))]
    let accent_bar_class = "sd-slider__bar--violet";

    let disabled_class = if disabled { "sd-slider--disabled" } else { "" };

    let mut update_min = move |new_min: f64| {
        let snapped = snap_to_step(new_min, min, max, step);
        let (_, current_high) = *value.read();
        let safe_low = snapped.min(current_high);
        *value.write() = (safe_low, current_high);
        if let Some(handler) = on_change {
            handler.call((safe_low, current_high));
        }
    };

    let mut update_max = move |new_max: f64| {
        let snapped = snap_to_step(new_max, min, max, step);
        let (current_low, _) = *value.read();
        let safe_high = snapped.max(current_low);
        *value.write() = (current_low, safe_high);
        if let Some(handler) = on_change {
            handler.call((current_low, safe_high));
        }
    };

    let tooltip_visible = show_tooltip && !disabled && *is_hovered.read();

    rsx! {
        div {
            class: "sd-slider-wrapper {class_modifiers} {class}",
            if let Some(lbl) = &label {
                div { class: "flex items-center justify-between mb-1.5",
                    label {
                        r#for: "{slider_id}-min",
                        class: "sd-slider-label",
                        "{lbl}"
                    }
                    span { class: "sd-slider-value-display", "{val_min:.1} - {val_max:.1}" }
                }
            }

            div {
                class: "sd-slider-container {base_override} {glow_class} {disabled_class}",
                onmouseenter: move |_| is_hovered.set(true),
                onmouseleave: move |_| is_hovered.set(false),

                // Native inputs for min and max
                input {
                    r#type: "range",
                    id: "{slider_id}-min",
                    min: "{min}",
                    max: "{max}",
                    step: "{step}",
                    value: "{val_min}",
                    "aria-label": label.as_deref().map(|l| format!("{l} (Min)")).unwrap_or_else(|| locale.slider_min.clone()),
                    "aria-valuemin": "{min}",
                    "aria-valuemax": "{val_max}",
                    "aria-valuenow": "{val_min}",
                    disabled,
                    class: "sd-slider__native sd-slider__native--range",
                    oninput: move |e| {
                        if let Ok(v) = e.value().parse::<f64>() {
                            update_min(v);
                        }
                    },
                }
                input {
                    r#type: "range",
                    id: "{slider_id}-max",
                    min: "{min}",
                    max: "{max}",
                    step: "{step}",
                    value: "{val_max}",
                    "aria-label": label.as_deref().map(|l| format!("{l} (Max)")).unwrap_or_else(|| locale.slider_max.clone()),
                    "aria-valuemin": "{val_min}",
                    "aria-valuemax": "{max}",
                    "aria-valuenow": "{val_max}",
                    disabled,
                    class: "sd-slider__native sd-slider__native--range",
                    oninput: move |e| {
                        if let Ok(v) = e.value().parse::<f64>() {
                            update_max(v);
                        }
                    },
                }

                // Styled visual track
                div { class: "sd-slider__track",
                    div {
                        class: "sd-slider__bar {accent_bar_class}",
                        style: "left: {pct_min}%; width: {bar_width}%;",
                    }
                }

                // Marks
                if !marks.is_empty() {
                    div { class: "sd-slider__marks",
                        for mark in &marks {
                            {
                                let mark_pct = calc_percentage(mark.value, min, max);
                                let is_active = mark.value >= val_min && mark.value <= val_max;
                                let active_mark_class = if is_active { "sd-slider__mark--active" } else { "" };
                                rsx! {
                                    div {
                                        key: "{mark.value}",
                                        class: "sd-slider__mark {active_mark_class}",
                                        style: "left: {mark_pct}%;",
                                        if let Some(mlbl) = &mark.label {
                                            span { class: "sd-slider__mark-label", "{mlbl}" }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                // Thumbs
                div {
                    class: "sd-slider__thumb-wrapper",
                    style: "left: {pct_min}%;",
                    Tooltip {
                        content: format!("{val_min:.1}"),
                        position: TooltipPosition::Top,
                        opened: Some(tooltip_visible),
                        div { class: "sd-slider__thumb" }
                    }
                }
                div {
                    class: "sd-slider__thumb-wrapper",
                    style: "left: {pct_max}%;",
                    Tooltip {
                        content: format!("{val_max:.1}"),
                        position: TooltipPosition::Top,
                        opened: Some(tooltip_visible),
                        div { class: "sd-slider__thumb" }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calc_percentage_bounds() {
        assert_eq!(calc_percentage(0.0, 0.0, 100.0), 0.0);
        assert_eq!(calc_percentage(50.0, 0.0, 100.0), 50.0);
        assert_eq!(calc_percentage(100.0, 0.0, 100.0), 100.0);
        assert_eq!(calc_percentage(-10.0, 0.0, 100.0), 0.0);
        assert_eq!(calc_percentage(150.0, 0.0, 100.0), 100.0);
    }

    #[test]
    fn test_calc_percentage_equal_min_max() {
        assert_eq!(calc_percentage(50.0, 50.0, 50.0), 0.0);
    }

    #[test]
    fn test_snap_to_step() {
        assert_eq!(snap_to_step(4.2, 0.0, 10.0, 1.0), 4.0);
        assert_eq!(snap_to_step(4.7, 0.0, 10.0, 1.0), 5.0);
        assert_eq!(snap_to_step(23.0, 0.0, 100.0, 5.0), 25.0);
        assert_eq!(snap_to_step(105.0, 0.0, 100.0, 10.0), 100.0);
    }

    #[test]
    fn test_slider_marks() {
        let m1 = SliderMark::new(10.0, "Start");
        assert_eq!(m1.value, 10.0);
        assert_eq!(m1.label.as_deref(), Some("Start"));

        let m2 = SliderMark::point(50.0);
        assert_eq!(m2.value, 50.0);
        assert_eq!(m2.label, None);
    }
}
