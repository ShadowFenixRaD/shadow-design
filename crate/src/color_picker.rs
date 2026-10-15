use dioxus::prelude::*;
use crate::a11y::{use_unique_id, ControlSize};
use crate::icon::{Icon, IconKind};
use crate::util::cx;

/// Supported output formatting modes for colors.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ColorFormat {
    /// Standard hexadecimal string (e.g. `#8b5cf6` or `#8b5cf6ff`).
    #[default]
    Hex,
    /// Functional RGB/RGBA string (e.g. `rgb(139, 92, 246)` or `rgba(139, 92, 246, 0.8)`).
    Rgb,
    /// Functional HSL/HSLA string (e.g. `hsl(258, 90%, 66%)` or `hsla(258, 90%, 66%, 0.8)`).
    Hsl,
}

/// Representation of an RGBA color.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rgba {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: f32,
}

impl Default for Rgba {
    fn default() -> Self {
        Self {
            r: 139,
            g: 92,
            b: 246,
            a: 1.0,
        }
    }
}

/// Representation of an HSLA color.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Hsla {
    /// Hue in degrees `0.0..=360.0`.
    pub h: f32,
    /// Saturation in percentage `0.0..=100.0`.
    pub s: f32,
    /// Lightness in percentage `0.0..=100.0`.
    pub l: f32,
    /// Alpha in fraction `0.0..=1.0`.
    pub a: f32,
}

impl Default for Hsla {
    fn default() -> Self {
        Self {
            h: 258.0,
            s: 90.0,
            l: 66.0,
            a: 1.0,
        }
    }
}

/// Parses a hex color string with or without leading '#'.
#[must_use]
pub fn parse_hex(input: &str) -> Option<Rgba> {
    let clean = input.trim().trim_start_matches('#');
    match clean.len() {
        3 => {
            let r = u8::from_str_radix(&clean[0..1].repeat(2), 16).ok()?;
            let g = u8::from_str_radix(&clean[1..2].repeat(2), 16).ok()?;
            let b = u8::from_str_radix(&clean[2..3].repeat(2), 16).ok()?;
            Some(Rgba { r, g, b, a: 1.0 })
        }
        4 => {
            let r = u8::from_str_radix(&clean[0..1].repeat(2), 16).ok()?;
            let g = u8::from_str_radix(&clean[1..2].repeat(2), 16).ok()?;
            let b = u8::from_str_radix(&clean[2..3].repeat(2), 16).ok()?;
            let a_val = u8::from_str_radix(&clean[3..4].repeat(2), 16).ok()?;
            Some(Rgba {
                r,
                g,
                b,
                a: (a_val as f32) / 255.0,
            })
        }
        6 => {
            let r = u8::from_str_radix(&clean[0..2], 16).ok()?;
            let g = u8::from_str_radix(&clean[2..4], 16).ok()?;
            let b = u8::from_str_radix(&clean[4..6], 16).ok()?;
            Some(Rgba { r, g, b, a: 1.0 })
        }
        8 => {
            let r = u8::from_str_radix(&clean[0..2], 16).ok()?;
            let g = u8::from_str_radix(&clean[2..4], 16).ok()?;
            let b = u8::from_str_radix(&clean[4..6], 16).ok()?;
            let a_val = u8::from_str_radix(&clean[6..8], 16).ok()?;
            Some(Rgba {
                r,
                g,
                b,
                a: (a_val as f32) / 255.0,
            })
        }
        _ => None,
    }
}

/// Parses an RGB or RGBA string.
#[must_use]
pub fn parse_rgb(input: &str) -> Option<Rgba> {
    let clean = input.trim();
    if !(clean.starts_with("rgb(") || clean.starts_with("rgba(")) || !clean.ends_with(')') {
        return None;
    }
    let inner = clean
        .trim_start_matches("rgba(")
        .trim_start_matches("rgb(")
        .trim_end_matches(')');
    let parts: Vec<&str> = inner.split(',').map(str::trim).collect();
    if parts.len() < 3 {
        return None;
    }
    let r = parts[0].parse::<u8>().ok()?;
    let g = parts[1].parse::<u8>().ok()?;
    let b = parts[2].parse::<u8>().ok()?;
    let a = if parts.len() >= 4 {
        parts[3].parse::<f32>().ok()?.clamp(0.0, 1.0)
    } else {
        1.0
    };
    Some(Rgba { r, g, b, a })
}

/// Flexible color parser trying Hex and RGB.
#[must_use]
pub fn parse_color(input: &str) -> Rgba {
    parse_hex(input)
        .or_else(|| parse_rgb(input))
        .unwrap_or_default()
}

/// Converts RGBA to Hex string.
#[must_use]
pub fn rgba_to_hex(rgba: &Rgba, with_alpha: bool) -> String {
    if with_alpha {
        let alpha_byte = (rgba.a.clamp(0.0, 1.0) * 255.0).round() as u8;
        format!("#{:02x}{:02x}{:02x}{:02x}", rgba.r, rgba.g, rgba.b, alpha_byte)
    } else {
        format!("#{:02x}{:02x}{:02x}", rgba.r, rgba.g, rgba.b)
    }
}

/// Converts RGBA to HSL.
#[must_use]
pub fn rgba_to_hsl(rgba: &Rgba) -> Hsla {
    let r = rgba.r as f32 / 255.0;
    let g = rgba.g as f32 / 255.0;
    let b = rgba.b as f32 / 255.0;

    let max = r.max(g).max(b);
    let min = r.min(g).min(b);
    let delta = max - min;

    let l = (max + min) / 2.0;

    if delta == 0.0 {
        return Hsla {
            h: 0.0,
            s: 0.0,
            l: l * 100.0,
            a: rgba.a,
        };
    }

    let s = if l < 0.5 {
        delta / (max + min)
    } else {
        delta / (2.0 - max - min)
    };

    let mut h = if (max - r).abs() < f32::EPSILON {
        (g - b) / delta + (if g < b { 6.0 } else { 0.0 })
    } else if (max - g).abs() < f32::EPSILON {
        (b - r) / delta + 2.0
    } else {
        (r - g) / delta + 4.0
    };
    h *= 60.0;

    Hsla {
        h: h.clamp(0.0, 360.0),
        s: (s * 100.0).clamp(0.0, 100.0),
        l: (l * 100.0).clamp(0.0, 100.0),
        a: rgba.a,
    }
}

/// Converts HSLA to RGBA.
#[must_use]
pub fn hsl_to_rgba(hsla: &Hsla) -> Rgba {
    let h = hsla.h % 360.0;
    let s = hsla.s.clamp(0.0, 100.0) / 100.0;
    let l = hsla.l.clamp(0.0, 100.0) / 100.0;

    let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
    let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
    let m = l - c / 2.0;

    let (r_prime, g_prime, b_prime) = if h < 60.0 {
        (c, x, 0.0)
    } else if h < 120.0 {
        (x, c, 0.0)
    } else if h < 180.0 {
        (0.0, c, x)
    } else if h < 240.0 {
        (0.0, x, c)
    } else if h < 300.0 {
        (x, 0.0, c)
    } else {
        (c, 0.0, x)
    };

    Rgba {
        r: ((r_prime + m) * 255.0).round().clamp(0.0, 255.0) as u8,
        g: ((g_prime + m) * 255.0).round().clamp(0.0, 255.0) as u8,
        b: ((b_prime + m) * 255.0).round().clamp(0.0, 255.0) as u8,
        a: hsla.a.clamp(0.0, 1.0),
    }
}

/// Formats a color according to the specified [`ColorFormat`].
#[must_use]
pub fn format_color(rgba: &Rgba, format: ColorFormat, with_alpha: bool) -> String {
    match format {
        ColorFormat::Hex => rgba_to_hex(rgba, with_alpha),
        ColorFormat::Rgb => {
            if with_alpha {
                format!("rgba({}, {}, {}, {:.2})", rgba.r, rgba.g, rgba.b, rgba.a)
            } else {
                format!("rgb({}, {}, {})", rgba.r, rgba.g, rgba.b)
            }
        }
        ColorFormat::Hsl => {
            let hsl = rgba_to_hsl(rgba);
            if with_alpha {
                format!(
                    "hsla({:.0}, {:.0}%, {:.0}%, {:.2})",
                    hsl.h, hsl.s, hsl.l, hsl.a
                )
            } else {
                format!("hsl({:.0}, {:.0}%, {:.0}%)", hsl.h, hsl.s, hsl.l)
            }
        }
    }
}

/// Default theme swatch colors matching Shadow Design palette.
#[must_use]
pub fn default_color_swatches() -> Vec<String> {
    vec![
        "#a855f7".to_string(), // Vivid Violet (Primary)
        "#6366f1".to_string(), // Indigo
        "#06b6d4".to_string(), // Cyan
        "#10b981".to_string(), // Emerald
        "#f43f5e".to_string(), // Crimson
        "#f59e0b".to_string(), // Amber
        "#ec4899".to_string(), // Fuchsia
        "#71717a".to_string(), // Obsidian Zinc
    ]
}

/// Properties for configuring the [`ColorPicker`] component.
#[derive(Props, Clone, PartialEq)]
pub struct ColorPickerProps {
    /// Reactive string signal holding the current color value.
    pub value: Signal<String>,
    /// Format used to write and parse the color value (Hex, Rgb, Hsl).
    #[props(default = ColorFormat::Hex)]
    pub format: ColorFormat,
    /// Whether to show the alpha transparency slider and encode alpha channel.
    #[props(default = false)]
    pub with_alpha: bool,
    /// List of quick color palette swatch hex values displayed below the picker.
    #[props(default = default_color_swatches())]
    pub swatches: Vec<String>,
    /// Size scaling of the picker controls.
    #[props(default = ControlSize::Md)]
    pub size: ControlSize,
    /// Whether the color picker is disabled and non-interactive.
    #[props(default = false)]
    pub disabled: bool,
    /// Optional additional CSS class names for the container.
    #[props(default)]
    pub class: Option<String>,
    /// Callback triggered whenever the color value changes.
    #[props(default)]
    pub on_change: Option<EventHandler<String>>,
}

/// Tactile ColorPicker component with rainbow hue slider, alpha slider,
/// saturation/lightness precision controls, and quick theme palette swatches.
#[component]
pub fn ColorPicker(props: ColorPickerProps) -> Element {
    let current_str = props.value.read().clone();
    let rgba = parse_color(&current_str);
    let hsla = rgba_to_hsl(&rgba);

    #[cfg(feature = "theme")]
    let theme = crate::theme::use_theme();
    #[cfg(feature = "theme")]
    let radius_class = theme.read().radius_scale.md();
    #[cfg(not(feature = "theme"))]
    let radius_class = "rounded-lg";

    let size_modifier = match props.size {
        ControlSize::Sm => "sd-color-picker--sm",
        ControlSize::Md => "sd-color-picker--md",
        ControlSize::Lg => "sd-color-picker--lg",
    };

    let disabled_modifier = if props.disabled { "sd-color-picker--disabled" } else { "" };

    let container_class = cx([
        "sd-color-picker",
        size_modifier,
        disabled_modifier,
        radius_class,
        props.class.as_deref().unwrap_or(""),
    ]);

    let hex_val = rgba_to_hex(&rgba, false);
    let current_css_color = if props.with_alpha {
        format!("rgba({}, {}, {}, {:.2})", rgba.r, rgba.g, rgba.b, rgba.a)
    } else {
        hex_val.clone()
    };

    let mut val_hue = props.value;
    let on_change_hue = props.on_change;
    let format_hue = props.format;
    let alpha_hue = props.with_alpha;

    let mut val_sat = props.value;
    let on_change_sat = props.on_change;
    let format_sat = props.format;
    let alpha_sat = props.with_alpha;

    let mut val_light = props.value;
    let on_change_light = props.on_change;
    let format_light = props.format;
    let alpha_light = props.with_alpha;

    let mut val_alpha = props.value;
    let on_change_alpha = props.on_change;
    let format_alpha = props.format;
    let alpha_alpha = props.with_alpha;

    let mut val_hex_input = props.value;
    let on_change_hex = props.on_change;
    let format_hex_input = props.format;
    let alpha_hex_input = props.with_alpha;

    rsx! {
        div { class: "{container_class}",
            // Top Preview & Direct Code
            div { class: "sd-color-picker__header",
                div {
                    class: "sd-color-picker__preview",
                    style: "background-color: {current_css_color};",
                    title: "Selected color preview",
                }
                div { class: "sd-color-picker__code",
                    input {
                        r#type: "text",
                        class: "sd-color-picker__hex-input",
                        value: "{current_str}",
                        disabled: props.disabled,
                        placeholder: "#000000",
                        oninput: move |evt: FormEvent| {
                            let text = evt.value();
                            if let Some(parsed) = parse_hex(&text).or_else(|| parse_rgb(&text)) {
                                let formatted = format_color(&parsed, format_hex_input, alpha_hex_input);
                                val_hex_input.set(formatted.clone());
                                if let Some(handler) = on_change_hex.as_ref() {
                                    handler.call(formatted);
                                }
                            }
                        },
                    }
                }
            }

            // Sliders Container
            div { class: "sd-color-picker__sliders",
                // Rainbow Hue Slider (0 - 360)
                div { class: "sd-color-picker__slider-row",
                    span { class: "sd-color-picker__slider-label", "H" }
                    input {
                        r#type: "range",
                        class: "sd-color-picker__range sd-color-picker__range--hue",
                        min: "0",
                        max: "360",
                        step: "1",
                        value: "{hsla.h}",
                        disabled: props.disabled,
                        oninput: move |evt: FormEvent| {
                            if let Ok(h) = evt.value().parse::<f32>() {
                                let mut updated = hsla;
                                updated.h = h;
                                let new_rgba = hsl_to_rgba(&updated);
                                let formatted = format_color(&new_rgba, format_hue, alpha_hue);
                                val_hue.set(formatted.clone());
                                if let Some(handler) = on_change_hue.as_ref() {
                                    handler.call(formatted);
                                }
                            }
                        },
                    }
                    span { class: "sd-color-picker__slider-value", "{hsla.h as u32}°" }
                }

                // Saturation Slider (0 - 100%)
                div { class: "sd-color-picker__slider-row",
                    span { class: "sd-color-picker__slider-label", "S" }
                    input {
                        r#type: "range",
                        class: "sd-color-picker__range sd-color-picker__range--sat",
                        min: "0",
                        max: "100",
                        step: "1",
                        value: "{hsla.s}",
                        disabled: props.disabled,
                        oninput: move |evt: FormEvent| {
                            if let Ok(s) = evt.value().parse::<f32>() {
                                let mut updated = hsla;
                                updated.s = s;
                                let new_rgba = hsl_to_rgba(&updated);
                                let formatted = format_color(&new_rgba, format_sat, alpha_sat);
                                val_sat.set(formatted.clone());
                                if let Some(handler) = on_change_sat.as_ref() {
                                    handler.call(formatted);
                                }
                            }
                        },
                    }
                    span { class: "sd-color-picker__slider-value", "{hsla.s as u32}%" }
                }

                // Lightness Slider (0 - 100%)
                div { class: "sd-color-picker__slider-row",
                    span { class: "sd-color-picker__slider-label", "L" }
                    input {
                        r#type: "range",
                        class: "sd-color-picker__range sd-color-picker__range--light",
                        min: "0",
                        max: "100",
                        step: "1",
                        value: "{hsla.l}",
                        disabled: props.disabled,
                        oninput: move |evt: FormEvent| {
                            if let Ok(l) = evt.value().parse::<f32>() {
                                let mut updated = hsla;
                                updated.l = l;
                                let new_rgba = hsl_to_rgba(&updated);
                                let formatted = format_color(&new_rgba, format_light, alpha_light);
                                val_light.set(formatted.clone());
                                if let Some(handler) = on_change_light.as_ref() {
                                    handler.call(formatted);
                                }
                            }
                        },
                    }
                    span { class: "sd-color-picker__slider-value", "{hsla.l as u32}%" }
                }

                // Alpha Slider (0.0 - 1.0)
                if props.with_alpha {
                    div { class: "sd-color-picker__slider-row",
                        span { class: "sd-color-picker__slider-label", "A" }
                        input {
                            r#type: "range",
                            class: "sd-color-picker__range sd-color-picker__range--alpha",
                            min: "0",
                            max: "1",
                            step: "0.01",
                            value: "{rgba.a}",
                            disabled: props.disabled,
                            oninput: move |evt: FormEvent| {
                                if let Ok(a) = evt.value().parse::<f32>() {
                                    let mut updated = rgba;
                                    updated.a = a.clamp(0.0, 1.0);
                                    let formatted = format_color(&updated, format_alpha, alpha_alpha);
                                    val_alpha.set(formatted.clone());
                                    if let Some(handler) = on_change_alpha.as_ref() {
                                        handler.call(formatted);
                                    }
                                }
                            },
                        }
                        span { class: "sd-color-picker__slider-value", "{(rgba.a * 100.0) as u32}%" }
                    }
                }
            }

            // Quick Swatches
            if !props.swatches.is_empty() {
                div { class: "sd-color-picker__swatches",
                    for swatch in &props.swatches {
                        {
                            let sw = swatch.clone();
                            let is_active = current_str.eq_ignore_ascii_case(&sw);
                            let active_class = if is_active { "sd-color-picker__swatch--active" } else { "" };
                            let mut val_sw = props.value;
                            let on_change_sw = props.on_change;
                            let format_sw = props.format;
                            let alpha_sw = props.with_alpha;
                            let sw_click = sw.clone();

                            rsx! {
                                button {
                                    key: "{sw}",
                                    r#type: "button",
                                    class: "sd-color-picker__swatch {active_class}",
                                    style: "background-color: {sw};",
                                    title: "{sw}",
                                    disabled: props.disabled,
                                    onclick: move |_| {
                                        let mut parsed = parse_color(&sw_click);
                                        if !alpha_sw {
                                            parsed.a = 1.0;
                                        }
                                        let formatted = format_color(&parsed, format_sw, alpha_sw);
                                        val_sw.set(formatted.clone());
                                        if let Some(handler) = on_change_sw.as_ref() {
                                            handler.call(formatted);
                                        }
                                    },
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Properties for configuring the [`ColorInput`] form control.
#[derive(Props, Clone, PartialEq)]
pub struct ColorInputProps {
    /// Reactive string signal holding the current color value.
    pub value: Signal<String>,
    /// Format used for displaying and parsing the color (Hex, Rgb, Hsl).
    #[props(default = ColorFormat::Hex)]
    pub format: ColorFormat,
    /// Whether the color includes alpha transparency.
    #[props(default = false)]
    pub with_alpha: bool,
    /// Quick swatches shown in the popover picker.
    #[props(default = default_color_swatches())]
    pub swatches: Vec<String>,
    /// Label text displayed above the input.
    #[props(default)]
    pub label: Option<String>,
    /// Helper or description text displayed below the input.
    #[props(default)]
    pub description: Option<String>,
    /// Error message string. If present, highlights the input in an error state.
    #[props(default)]
    pub error: Option<String>,
    /// Placeholder text for the input when empty.
    #[props(default)]
    pub placeholder: Option<String>,
    /// Whether the input is disabled and non-interactive.
    #[props(default = false)]
    pub disabled: bool,
    /// Whether to display a clear button when the input is not empty.
    #[props(default = false)]
    pub clearable: bool,
    /// Optional additional CSS class names.
    #[props(default)]
    pub class: Option<String>,
    /// Callback triggered whenever the color value changes.
    #[props(default)]
    pub on_change: Option<EventHandler<String>>,
}

/// Form input with an interactive color swatch button and an anchored
/// popover dropdown [`ColorPicker`].
#[component]
pub fn ColorInput(props: ColorInputProps) -> Element {
    let mut is_open = use_signal(|| false);
    let input_id = use_unique_id("color-input", None);

    #[cfg(feature = "theme")]
    let theme = crate::theme::use_theme();
    #[cfg(feature = "theme")]
    let radius_class = theme.read().radius_scale.md();
    #[cfg(not(feature = "theme"))]
    let radius_class = "rounded-lg";

    let current_str = props.value.read().clone();
    let rgba = parse_color(&current_str);
    let swatch_color = if props.with_alpha {
        format!("rgba({}, {}, {}, {:.2})", rgba.r, rgba.g, rgba.b, rgba.a)
    } else {
        rgba_to_hex(&rgba, false)
    };

    let has_error = props.error.is_some();
    let error_modifier = if has_error { "sd-color-input--error" } else { "" };
    let disabled_modifier = if props.disabled { "sd-color-input--disabled" } else { "" };

    let wrapper_class = cx([
        "sd-color-input",
        error_modifier,
        disabled_modifier,
        props.class.as_deref().unwrap_or(""),
    ]);

    let input_class = cx([
        "sd-color-input__field",
        radius_class,
        if has_error { "sd-color-input__field--error" } else { "" },
    ]);

    let mut val_text = props.value;
    let on_change_text = props.on_change;
    let format_text = props.format;
    let alpha_text = props.with_alpha;

    let mut val_clear = props.value;
    let on_change_clear = props.on_change;

    rsx! {
        div { class: "{wrapper_class}",
            if let Some(lbl) = &props.label {
                label {
                    class: "sd-color-input__label",
                    r#for: "{input_id}",
                    "{lbl}"
                }
            }

            div { class: "sd-color-input__container",
                // Leading Color Swatch Trigger Button
                button {
                    r#type: "button",
                    class: "sd-color-input__swatch-btn",
                    style: "background-color: {swatch_color};",
                    disabled: props.disabled,
                    title: "Open color picker",
                    aria_haspopup: "dialog",
                    aria_expanded: if *is_open.read() { "true" } else { "false" },
                    onclick: move |_| {
                        if !props.disabled {
                            let cur = *is_open.read();
                            is_open.set(!cur);
                        }
                    },
                }

                // Text Field showing hex/rgb value
                input {
                    id: "{input_id}",
                    r#type: "text",
                    class: "{input_class}",
                    value: "{current_str}",
                    placeholder: props.placeholder.as_deref().unwrap_or("#a855f7"),
                    disabled: props.disabled,
                    oninput: move |evt: FormEvent| {
                        let text = evt.value();
                        if let Some(parsed) = parse_hex(&text).or_else(|| parse_rgb(&text)) {
                            let formatted = format_color(&parsed, format_text, alpha_text);
                            val_text.set(formatted.clone());
                            if let Some(handler) = on_change_text.as_ref() {
                                handler.call(formatted);
                            }
                        } else {
                            val_text.set(text);
                        }
                    },
                }

                // Clear button
                if props.clearable && !current_str.is_empty() && !props.disabled {
                    button {
                        r#type: "button",
                        class: "sd-color-input__clear-btn",
                        title: "Clear color",
                        onclick: move |_| {
                            val_clear.set(String::new());
                            if let Some(handler) = on_change_clear.as_ref() {
                                handler.call(String::new());
                            }
                        },
                        Icon { kind: IconKind::X, size: 14 }
                    }
                }
            }

            // Description / Error message
            if let Some(err) = &props.error {
                span { class: "sd-color-input__error-text", "{err}" }
            } else if let Some(desc) = &props.description {
                span { class: "sd-color-input__desc-text", "{desc}" }
            }

            // Anchored Popover Dialog
            if *is_open.read() && !props.disabled {
                div {
                    class: "sd-color-input__backdrop",
                    onclick: move |_| is_open.set(false),
                }
                div { class: "sd-color-input__popover",
                    ColorPicker {
                        value: props.value,
                        format: props.format,
                        with_alpha: props.with_alpha,
                        swatches: props.swatches.clone(),
                        on_change: props.on_change,
                    }
                }
            }
        }
    }
}
