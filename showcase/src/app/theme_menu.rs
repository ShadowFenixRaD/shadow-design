//! Theme controls of the chrome: swatches, radius/glow segments and the header popover.
use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::Route;

/// The five presets with their display names (same order everywhere in the showcase).
pub const PRESETS: [(PrimaryColor, &str); 5] = [
    (PrimaryColor::Violet, "Ultraviolet"),
    (PrimaryColor::Obsidian, "Obsidian"),
    (PrimaryColor::Crimson, "Crimson"),
    (PrimaryColor::Cyan, "Cyan"),
    (PrimaryColor::Emerald, "Emerald"),
];

/// Five accent dots. Each dot is a library `Button` (ghost, icon only) carrying a
/// `.sc-dot` span whose background is the preset hex through an inline style
/// built from `PrimaryColor::hex()` (a runtime value, not a literal).
#[component]
pub fn ThemeSwatches(#[props(default)] compact: bool) -> Element {
    let mut theme = use_theme();
    let current = theme().primary_color;
    let class = if compact { "sc-swatches sc-swatches--compact" } else { "sc-swatches" };
    rsx! {
        div { class: "{class}", role: "group", aria_label: "Colore primario",
            for (color , name) in PRESETS {
                Tooltip { key: "{name}", content: name.to_string(), position: TooltipPosition::Bottom,
                    Button {
                        variant: ButtonVariant::Ghost,
                        size: ButtonSize::Sm,
                        aria_label: Some(format!("Accento {name}")),
                        aria_pressed: Some(current == color),
                        class: if current == color { "sc-swatch sc-swatch--active".to_string() } else { "sc-swatch".to_string() },
                        onclick: {
                            let color = color.clone();
                            move |_| theme.write().apply_primary_color(color.clone())
                        },
                        span { class: "sc-dot", style: "background-color: {color.hex()};" }
                    }
                }
            }
        }
    }
}

/// Radius scale and glow mode as two segmented controls, synced with the global theme.
#[component]
pub fn ThemeControls() -> Element {
    let mut theme = use_theme();
    let mut radius = use_signal(|| radius_key(theme.peek().radius_scale).to_string());
    let mut glow = use_signal(|| glow_key(theme.peek().glow_mode).to_string());
    use_effect(move || {
        let t = theme();
        if radius.peek().as_str() != radius_key(t.radius_scale) {
            radius.set(radius_key(t.radius_scale).to_string());
        }
        if glow.peek().as_str() != glow_key(t.glow_mode) {
            glow.set(glow_key(t.glow_mode).to_string());
        }
    });
    rsx! {
        div { class: "sc-theme-controls",
            div { class: "sc-knob",
                span { class: "sc-eyebrow", "Raggio" }
                SegmentedControl {
                    size: SegmentedControlSize::Sm,
                    full_width: true,
                    options: vec![
                        SegmentOption::new("sharp", "Sharp"),
                        SegmentOption::new("medium", "Medium"),
                        SegmentOption::new("soft", "Soft"),
                    ],
                    value: radius,
                    on_change: move |v: String| theme.write().apply_radius_scale(radius_from_key(&v)),
                }
            }
            div { class: "sc-knob",
                span { class: "sc-eyebrow", "Glow" }
                SegmentedControl {
                    size: SegmentedControlSize::Sm,
                    full_width: true,
                    options: vec![
                        SegmentOption::new("off", "Off"),
                        SegmentOption::new("subtle", "Subtle"),
                        SegmentOption::new("expressive", "Expressive"),
                    ],
                    value: glow,
                    on_change: move |v: String| theme.write().apply_glow_mode(glow_from_key(&v)),
                }
            }
        }
    }
}

/// Header button: current accent dot + "Tema", opens the full controls in a Popover.
#[component]
pub fn ThemeMenuButton() -> Element {
    let mut opened = use_signal(|| false);
    let navigator = use_navigator();
    rsx! {
        Popover { opened, position: PopoverPosition::BottomEnd, with_arrow: false,
            PopoverTrigger {
                Button {
                    variant: ButtonVariant::Ghost,
                    size: ButtonSize::Sm,
                    aria_label: Some("Apri le opzioni del tema".to_string()),
                    span { class: "sc-dot sc-dot--accent" }
                    span { class: "sc-hide-below-md", "Tema" }
                }
            }
            PopoverDropdown { class: "sc-theme-menu".to_string(),
                span { class: "sc-eyebrow", "Accento" }
                ThemeSwatches {}
                ThemeControls {}
                Button {
                    variant: ButtonVariant::Outline,
                    size: ButtonSize::Sm,
                    full_width: true,
                    left_icon: Some(rsx! {
                        Icon { kind: IconKind::Palette, size: 14 }
                    }),
                    onclick: move |_| {
                        opened.set(false);
                        navigator.push(Route::ThemeBuilderPage {});
                    },
                    "Theme Builder"
                }
            }
        }
    }
}

pub fn radius_key(r: RadiusScale) -> &'static str {
    match r {
        RadiusScale::Sharp => "sharp",
        RadiusScale::Medium => "medium",
        RadiusScale::Soft => "soft",
    }
}

pub fn radius_from_key(key: &str) -> RadiusScale {
    match key {
        "sharp" => RadiusScale::Sharp,
        "soft" => RadiusScale::Soft,
        _ => RadiusScale::Medium,
    }
}

pub fn glow_key(g: GlowMode) -> &'static str {
    match g {
        GlowMode::Off => "off",
        GlowMode::Subtle => "subtle",
        GlowMode::Expressive => "expressive",
    }
}

pub fn glow_from_key(key: &str) -> GlowMode {
    match key {
        "off" => GlowMode::Off,
        "expressive" => GlowMode::Expressive,
        _ => GlowMode::Subtle,
    }
}

/// Display name of a preset accent ("Ultraviolet" for `Violet`), or the custom name.
pub fn preset_name(color: &PrimaryColor) -> String {
    PRESETS
        .iter()
        .find(|(c, _)| c == color)
        .map(|(_, n)| (*n).to_string())
        .unwrap_or_else(|| color.name().to_string())
}
