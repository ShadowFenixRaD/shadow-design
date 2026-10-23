use dioxus::prelude::*;

use crate::icon::{Icon, IconKind};

/// Helper per calcolare il valore clampato e arrotondato in base al conteggio e alle frazioni.
#[must_use]
pub fn clamp_rating_value(val: f64, count: usize, fractions: usize) -> f64 {
    let step = if fractions == 2 { 0.5 } else { 1.0 };
    let clamped = val.clamp(0.0, count as f64);
    let steps = (clamped / step).round();
    steps * step
}

/// Componente di valutazione `Rating` a stelle o icone vettoriali con supporto per
/// mezze valutazioni (frazioni 0.5), anteprima hover dinamica e accessibilità da tastiera.
#[component]
pub fn Rating(
    value: Signal<f64>,
    #[props(default = 5)] count: usize,
    #[props(default = 1)] fractions: usize,
    #[props(default)] readonly: bool,
    #[props(default)] disabled: bool,
    #[props(default)] icon: Option<IconKind>,
    #[props(default = 22)] size: u32,
    #[props(default)] label: Option<String>,
    #[props(default)] id: Option<String>,
    #[props(default)] class: String,
    #[props(default)] on_change: Option<EventHandler<f64>>,
) -> Element {
    let rating_id = crate::a11y::use_unique_id("sd-rating", id);
    let mut hover_val: Signal<Option<f64>> = use_signal(|| None);

    let active_val = *value.read();
    let display_val = hover_val().unwrap_or(active_val);

    #[cfg(feature = "theme")]
    let theme = crate::theme::use_theme();
    #[cfg(feature = "theme")]
    let theme_val = theme.read();
    #[cfg(feature = "theme")]
    let base_override = theme_val.resolve_component_base("rating").unwrap_or("");
    #[cfg(feature = "theme")]
    let class_modifiers = theme_val.resolve_class_modifiers("rating", &class);
    #[cfg(feature = "theme")]
    let active_color_class = match theme_val.primary_color {
        crate::theme::PrimaryColor::Violet => "sd-rating--violet",
        crate::theme::PrimaryColor::Obsidian => "sd-rating--obsidian",
        crate::theme::PrimaryColor::Crimson => "sd-rating--crimson",
        crate::theme::PrimaryColor::Cyan => "sd-rating--cyan",
        crate::theme::PrimaryColor::Emerald => "sd-rating--emerald",
        _ => "sd-rating--violet",
    };

    #[cfg(not(feature = "theme"))]
    let base_override = "";
    #[cfg(not(feature = "theme"))]
    let class_modifiers = String::new();
    #[cfg(not(feature = "theme"))]
    let active_color_class = "sd-rating--violet";

    let disabled_class = if disabled { "sd-rating--disabled" } else { "" };
    let readonly_class = if readonly { "sd-rating--readonly" } else { "" };

    let step = if fractions == 2 { 0.5 } else { 1.0 };

    let do_select = move |val: f64| {
        if !readonly && !disabled {
            let clamped = val.clamp(0.0, count as f64);
            *value.write() = clamped;
            if let Some(handler) = &on_change {
                handler.call(clamped);
            }
        }
    };

    rsx! {
        div {
            class: "sd-rating-wrapper {class_modifiers} {class}",
            if let Some(lbl) = &label {
                label {
                    r#for: "{rating_id}",
                    class: "sd-rating-label",
                    "{lbl}"
                }
            }

            div {
                id: "{rating_id}",
                role: "slider",
                "aria-label": label.as_deref().unwrap_or("Valutazione"),
                "aria-valuenow": "{active_val}",
                "aria-valuemin": "0",
                "aria-valuemax": "{count}",
                "aria-readonly": if readonly { "true" } else { "false" },
                tabindex: if disabled || readonly { "-1" } else { "0" },
                class: "sd-rating {active_color_class} {disabled_class} {readonly_class} {base_override}",
                onmouseleave: move |_| {
                    if !readonly && !disabled {
                        hover_val.set(None);
                    }
                },
                onkeydown: {
                    let mut do_select = do_select;
                    move |e| {
                        if !readonly && !disabled {
                            let curr = *value.read();
                            match e.key() {
                                Key::ArrowRight | Key::ArrowUp => {
                                    e.prevent_default();
                                    do_select((curr + step).min(count as f64));
                                }
                                Key::ArrowLeft | Key::ArrowDown => {
                                    e.prevent_default();
                                    do_select((curr - step).max(0.0));
                                }
                                _ => {}
                            }
                        }
                    }
                },

                for i in 1..=count {
                    {
                        let star_idx = i as f64;
                        let is_full = display_val >= star_idx;
                        let is_half = !is_full && fractions == 2 && display_val >= (star_idx - 0.5);

                        let fill_color = if is_full {
                            "currentColor".to_string()
                        } else if is_half {
                            format!("url(#{rating_id}-grad-{i})")
                        } else {
                            "rgba(255, 255, 255, 0.04)".to_string()
                        };

                        let fill_state = if is_full {
                            "sd-rating__item--full"
                        } else if is_half {
                            "sd-rating__item--half"
                        } else {
                            "sd-rating__item--empty"
                        };

                        rsx! {
                            div {
                                key: "{i}",
                                class: "sd-rating__item {fill_state}",
                                style: "width: {size}px; height: {size}px;",

                                // Interactivity overlay for fractions (left half / right half)
                                if !readonly && !disabled {
                                    if fractions == 2 {
                                        div {
                                            class: "sd-rating__hitbox sd-rating__hitbox--left",
                                            onmouseenter: move |_| hover_val.set(Some(star_idx - 0.5)),
                                            onclick: {
                                                let mut do_select = do_select;
                                                move |e| {
                                                    e.stop_propagation();
                                                    do_select(star_idx - 0.5);
                                                }
                                            },
                                        }
                                        div {
                                            class: "sd-rating__hitbox sd-rating__hitbox--right",
                                            onmouseenter: move |_| hover_val.set(Some(star_idx)),
                                            onclick: {
                                                let mut do_select = do_select;
                                                move |e| {
                                                    e.stop_propagation();
                                                    do_select(star_idx);
                                                }
                                            },
                                        }
                                    } else {
                                        div {
                                            class: "sd-rating__hitbox sd-rating__hitbox--full",
                                            onmouseenter: move |_| hover_val.set(Some(star_idx)),
                                            onclick: {
                                                let mut do_select = do_select;
                                                move |e| {
                                                    e.stop_propagation();
                                                    do_select(star_idx);
                                                }
                                            },
                                        }
                                    }
                                }

                                // Visual Symbol (Custom icon or native geometric star)
                                if let Some(custom_ic) = icon {
                                    Icon { kind: custom_ic, size }
                                } else {
                                    svg {
                                        view_box: "0 0 24 24",
                                        class: "sd-rating__star-svg",
                                        defs {
                                            linearGradient {
                                                id: "{rating_id}-grad-{i}",
                                                x1: "0%", y1: "0%", x2: "100%", y2: "0%",
                                                stop { offset: "50%", stop_color: "currentColor" }
                                                stop { offset: "50%", stop_color: "transparent" }
                                            }
                                        }
                                        polygon {
                                            points: "12,2 15.09,8.26 22,9.27 17,14.14 18.18,21.02 12,17.77 5.82,21.02 7,14.14 2,9.27 8.91,8.26",
                                            fill: "{fill_color}",
                                            stroke: "currentColor",
                                            stroke_width: "1.5",
                                            stroke_linejoin: "round",
                                        }
                                    }
                                }
                            }
                        }
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
    fn test_clamp_rating_whole_stars() {
        assert_eq!(clamp_rating_value(3.2, 5, 1), 3.0);
        assert_eq!(clamp_rating_value(3.7, 5, 1), 4.0);
        assert_eq!(clamp_rating_value(-1.0, 5, 1), 0.0);
        assert_eq!(clamp_rating_value(6.0, 5, 1), 5.0);
    }

    #[test]
    fn test_clamp_rating_half_stars() {
        assert_eq!(clamp_rating_value(3.2, 5, 2), 3.0);
        assert_eq!(clamp_rating_value(3.4, 5, 2), 3.5);
        assert_eq!(clamp_rating_value(3.8, 5, 2), 4.0);
    }
}
