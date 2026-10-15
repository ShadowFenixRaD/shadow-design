use dioxus::prelude::*;
use crate::a11y::ControlSize;

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

    let mut h = if max == r {
        (g - b) / delta + (if g < b { 6.0 } else { 0.0 })
    } else if max == g {
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
