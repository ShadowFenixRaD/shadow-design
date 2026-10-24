//! Theme engine (ADR-0003: runtime CSS tokens are the only theming mechanism).
//!
//! A [`Theme`] is data: a preset name, one accent hex ([`PrimaryColor`]), the
//! surface hexes, a [`RadiusScale`], a [`GlowMode`] and an [`ElevationStyle`].
//! [`ThemeProvider`] turns it into CSS custom properties on a wrapper element:
//!
//! * classes `sd-theme sd-radius-{sharp|medium|soft} sd-glow-{off|subtle|expressive}`;
//! * inline `--sd-accent`, `--sd-accent-rgb`, `--sd-accent-strong`, `--sd-accent-deep`,
//!   `--sd-accent-soft`, `--sd-accent-border`, `--sd-accent-fg`, `--sd-focus-ring`,
//!   `--sd-void`, `--sd-surface-1..3` and `--sd-glow-strength`.
//!
//! Every token default lives in `crate/scss/_tokens.scss` (documented in
//! `docs/design/tokens.md`), so components look identical with and without the
//! `theme` feature. The engine never emits per-preset or utility classes; the
//! string fields of [`Theme`] hold generic `sd-*` class names (or `""`), and
//! [`ComponentOverride`] remains the supported way to swap a component's classes.

use dioxus::prelude::*;
use std::collections::HashMap;

#[cfg(feature = "alert")]
use crate::alert::AlertVariant;
#[cfg(feature = "badge")]
use crate::badge::BadgeVariant;
#[cfg(feature = "button")]
use crate::button::ButtonVariant;
#[cfg(feature = "card")]
use crate::card::CardShadow;

pub use crate::util::{hex_to_rgb, is_valid_hex_color, parse_hex_rgba};

/// Default accent (Ultraviolet).
const DEFAULT_ACCENT: &str = "#a855f7";
/// Default surface hexes (`--sd-void`, `--sd-surface-1..3`).
const DEFAULT_SURFACES: [&str; 4] = ["#030305", "#09090d", "#111118", "#181824"];

fn sanitize_css_prefix(raw: &str) -> String {
    let sanitized: String = raw
        .to_lowercase()
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() { c } else { '-' })
        .collect();
    let trimmed = sanitized
        .split('-')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("-");
    if trimmed.is_empty() {
        "custom".to_string()
    } else {
        trimmed
    }
}

fn safe_hex<'a>(candidate: &'a str, fallback: &'a str) -> &'a str {
    if is_valid_hex_color(candidate) {
        candidate
    } else {
        fallback
    }
}

// ------------------------------------------------------------------------------
// Accent derivation
// ------------------------------------------------------------------------------

/// The full accent token set derived from one hex colour (`docs/design/tokens.md`).
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct AccentTokens {
    /// `--sd-accent`: the base colour as `#rrggbb`.
    pub accent: String,
    /// `--sd-accent-rgb` channels.
    pub rgb: (u8, u8, u8),
    /// `--sd-accent-strong`: base mixed with 35% white (accent text on dark).
    pub strong: String,
    /// `--sd-accent-deep`: base mixed with 40% black (bottom of the primary fill).
    pub deep: String,
    /// `--sd-accent-soft`: base at alpha .14.
    pub soft: String,
    /// `--sd-accent-border`: base at alpha .45.
    pub border: String,
    /// `--sd-accent-fg`: `#000000` if the relative luminance of base > 0.55, else `#ffffff`.
    pub fg: String,
    /// `--sd-focus-ring`: equal to `strong`.
    pub focus_ring: String,
}

fn to_hex((r, g, b): (u8, u8, u8)) -> String {
    format!("#{r:02x}{g:02x}{b:02x}")
}

fn mix_channel(c: u8, target: u8, amount: f64) -> u8 {
    let c = f64::from(c);
    let t = f64::from(target);
    // Values stay in 0..=255, so the cast cannot truncate.
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let v = (c + (t - c) * amount).round().clamp(0.0, 255.0) as u8;
    v
}

fn mix((r, g, b): (u8, u8, u8), target: u8, amount: f64) -> (u8, u8, u8) {
    (
        mix_channel(r, target, amount),
        mix_channel(g, target, amount),
        mix_channel(b, target, amount),
    )
}

/// WCAG relative luminance of an sRGB colour (0.0 – 1.0).
#[must_use]
pub fn relative_luminance((r, g, b): (u8, u8, u8)) -> f64 {
    fn lin(c: u8) -> f64 {
        let c = f64::from(c) / 255.0;
        if c <= 0.04045 {
            c / 12.92
        } else {
            ((c + 0.055) / 1.055).powf(2.4)
        }
    }
    0.2126 * lin(r) + 0.7152 * lin(g) + 0.0722 * lin(b)
}

impl AccentTokens {
    /// Derives the accent set from a hex colour. Invalid input falls back to the
    /// Ultraviolet default; any alpha channel is ignored.
    #[must_use]
    pub fn from_hex(hex: &str) -> Self {
        let rgb = hex_to_rgb(safe_hex(hex, DEFAULT_ACCENT))
            .or_else(|| hex_to_rgb(DEFAULT_ACCENT))
            .unwrap_or((168, 85, 247));
        let (r, g, b) = rgb;
        let strong = to_hex(mix(rgb, 255, 0.35));
        let fg = if relative_luminance(rgb) > 0.55 {
            "#000000"
        } else {
            "#ffffff"
        };
        Self {
            accent: to_hex(rgb),
            rgb,
            focus_ring: strong.clone(),
            strong,
            deep: to_hex(mix(rgb, 0, 0.40)),
            soft: format!("rgba({r}, {g}, {b}, 0.14)"),
            border: format!("rgba({r}, {g}, {b}, 0.45)"),
            fg: fg.to_string(),
        }
    }
}

// ------------------------------------------------------------------------------
// Scales
// ------------------------------------------------------------------------------

/// Accent colour of a theme. Presets are data: a name plus one hex.
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum PrimaryColor {
    /// Ultraviolet `#a855f7` (default).
    #[default]
    Violet,
    /// Obsidian `#f4f4f5` (light accent, dark text on fills).
    Obsidian,
    /// Crimson `#ff1744`.
    Crimson,
    /// Cyan `#00e5ff`.
    Cyan,
    /// Emerald `#00ff9d`.
    Emerald,
    /// Any hex colour; it goes through the same derivation as the presets.
    Custom {
        name: String,
        hex: String,
        /// Free-form identifier. No CSS class depends on it.
        css_prefix: Option<String>,
    },
}

impl PrimaryColor {
    #[must_use]
    pub fn name(&self) -> &str {
        match self {
            PrimaryColor::Violet => "Violet",
            PrimaryColor::Obsidian => "Obsidian",
            PrimaryColor::Crimson => "Crimson",
            PrimaryColor::Cyan => "Cyan",
            PrimaryColor::Emerald => "Emerald",
            PrimaryColor::Custom { name, .. } => name.as_str(),
        }
    }

    /// The base accent hex (same as [`PrimaryColor::accent_hex`]).
    #[must_use]
    pub fn hex(&self) -> &str {
        self.accent_hex()
    }

    /// The base accent hex written to `--sd-accent`.
    #[must_use]
    pub fn accent_hex(&self) -> &str {
        match self {
            PrimaryColor::Violet => "#a855f7",
            PrimaryColor::Obsidian => "#f4f4f5",
            PrimaryColor::Crimson => "#ff1744",
            PrimaryColor::Cyan => "#00e5ff",
            PrimaryColor::Emerald => "#00ff9d",
            PrimaryColor::Custom { hex, .. } => hex.as_str(),
        }
    }

    /// The full derived accent token set (see [`AccentTokens`]).
    #[must_use]
    pub fn accent_tokens(&self) -> AccentTokens {
        AccentTokens::from_hex(self.accent_hex())
    }

    /// Deprecated: use `accent_tokens().deep`. Kept for API compatibility; returns
    /// the base hex for `Custom`.
    #[must_use]
    pub fn shadow_hex(&self) -> &str {
        match self {
            PrimaryColor::Violet => "#653394",
            PrimaryColor::Obsidian => "#929293",
            PrimaryColor::Crimson => "#990e29",
            PrimaryColor::Cyan => "#008999",
            PrimaryColor::Emerald => "#00995e",
            PrimaryColor::Custom { hex, .. } => hex.as_str(),
        }
    }

    /// A sanitised identifier (`violet`, `crimson`, …, or the custom prefix,
    /// default `custom`). Informational only: no CSS class is generated from it,
    /// so any prefix works.
    #[must_use]
    pub fn css_prefix(&self) -> String {
        match self {
            PrimaryColor::Violet => "violet".to_string(),
            PrimaryColor::Obsidian => "obsidian".to_string(),
            PrimaryColor::Crimson => "crimson".to_string(),
            PrimaryColor::Cyan => "cyan".to_string(),
            PrimaryColor::Emerald => "emerald".to_string(),
            PrimaryColor::Custom { css_prefix, .. } => match css_prefix {
                None => "custom".to_string(),
                Some(p) => sanitize_css_prefix(p),
            },
        }
    }

    /// A custom accent. Invalid hex input falls back to Ultraviolet.
    pub fn custom(name: impl Into<String>, hex: impl Into<String>) -> Self {
        let h = hex.into();
        let valid_hex = if is_valid_hex_color(&h) {
            h
        } else {
            DEFAULT_ACCENT.to_string()
        };
        Self::Custom {
            name: name.into(),
            hex: valid_hex,
            css_prefix: None,
        }
    }

    /// A custom accent with an identifier (sanitised to `[a-z0-9-]`).
    pub fn custom_with_prefix(
        name: impl Into<String>,
        hex: impl Into<String>,
        css_prefix: impl Into<String>,
    ) -> Self {
        let h = hex.into();
        let valid_hex = if is_valid_hex_color(&h) {
            h
        } else {
            DEFAULT_ACCENT.to_string()
        };
        let final_prefix = sanitize_css_prefix(&css_prefix.into());
        Self::Custom {
            name: name.into(),
            hex: valid_hex,
            css_prefix: Some(final_prefix),
        }
    }
}

/// Global corner-radius scale. Applied by `ThemeProvider` as one class
/// (`sd-radius-sharp|medium|soft`) that sets the `--sd-radius-*` tokens.
#[derive(Clone, Copy, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum RadiusScale {
    Sharp,
    #[default]
    Medium,
    Soft,
}

impl RadiusScale {
    #[must_use]
    pub fn name(&self) -> &'static str {
        match self {
            RadiusScale::Sharp => "Sharp (2px)",
            RadiusScale::Medium => "Medium (8px)",
            RadiusScale::Soft => "Soft (12px)",
        }
    }

    /// The provider class that selects this scale.
    #[must_use]
    pub fn class(&self) -> &'static str {
        match self {
            RadiusScale::Sharp => "sd-radius-sharp",
            RadiusScale::Medium => "sd-radius-medium",
            RadiusScale::Soft => "sd-radius-soft",
        }
    }

    /// Deprecated: always `""`. Radius is applied by the provider class
    /// ([`RadiusScale::class`]) through the `--sd-radius-*` tokens.
    #[must_use]
    pub fn sm(&self) -> &'static str {
        ""
    }

    /// Deprecated: always `""`. See [`RadiusScale::sm`].
    #[must_use]
    pub fn md(&self) -> &'static str {
        ""
    }

    /// Deprecated: always `""`. See [`RadiusScale::sm`].
    #[must_use]
    pub fn lg(&self) -> &'static str {
        ""
    }
}

/// Global glow intensity, written as `--sd-glow-strength`.
#[derive(Clone, Copy, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum GlowMode {
    /// 0.6 (default).
    #[default]
    Subtle,
    /// 1.0.
    Expressive,
    /// 0.
    Off,
}

impl GlowMode {
    /// The `--sd-glow-strength` value.
    #[must_use]
    pub fn strength(&self) -> &'static str {
        match self {
            GlowMode::Off => "0",
            GlowMode::Subtle => "0.6",
            GlowMode::Expressive => "1",
        }
    }

    /// The provider class (`sd-glow-off|subtle|expressive`).
    #[must_use]
    pub fn class(&self) -> &'static str {
        match self {
            GlowMode::Off => "sd-glow-off",
            GlowMode::Subtle => "sd-glow-subtle",
            GlowMode::Expressive => "sd-glow-expressive",
        }
    }
}

/// Shadow style. `Flat` behaves like [`GlowMode::Off`] for glow.
#[derive(Clone, Copy, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ElevationStyle {
    /// Lit Glass shadows with the subtle accent glow (default).
    #[default]
    NeonGlow,
    /// Lit Glass shadows, glow strength forced to 0.
    Flat,
}

/// Per-component class customisation (level 2). The supported way for consumers
/// to swap a component's classes; library defaults never depend on it.
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
pub struct ComponentOverride {
    /// Extra or replacement base class.
    pub base_class: Option<String>,
    /// Variant classes (e.g. "primary" -> "my-primary-btn").
    pub variant_classes: HashMap<String, String>,
    /// Size classes (e.g. "sm" -> "my-sm-btn").
    pub size_classes: HashMap<String, String>,
    /// Shadow/glow class override for this component.
    pub shadow_class: Option<String>,
    /// Conditional modifiers keyed by an instance class.
    pub class_modifiers: HashMap<String, String>,
}

impl ComponentOverride {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn with_base_class(mut self, base: impl Into<String>) -> Self {
        self.base_class = Some(base.into());
        self
    }

    #[must_use]
    pub fn with_variant(mut self, variant: impl Into<String>, class: impl Into<String>) -> Self {
        self.variant_classes.insert(variant.into(), class.into());
        self
    }

    #[must_use]
    pub fn with_size(mut self, size: impl Into<String>, class: impl Into<String>) -> Self {
        self.size_classes.insert(size.into(), class.into());
        self
    }

    #[must_use]
    pub fn with_shadow(mut self, shadow: impl Into<String>) -> Self {
        self.shadow_class = Some(shadow.into());
        self
    }

    #[must_use]
    pub fn with_class_modifier(
        mut self,
        target_class: impl Into<String>,
        modifier: impl Into<String>,
    ) -> Self {
        self.class_modifiers
            .insert(target_class.into(), modifier.into());
        self
    }
}

// ------------------------------------------------------------------------------
// Theme
// ------------------------------------------------------------------------------

/// The theme: data that `ThemeProvider` writes as CSS variables.
///
/// The string fields are generic `sd-*` class names (identical for every preset)
/// or `""` where nothing is needed; they are kept for API compatibility and may
/// be replaced by consumers. Fields documented as deprecated are always `""`.
#[derive(Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
pub struct Theme {
    pub name: String,
    pub primary_color: PrimaryColor,
    pub radius_scale: RadiusScale,
    pub elevation_style: ElevationStyle,
    pub glow_mode: GlowMode,

    /// `--sd-void` (page background).
    pub surface_0: String,
    /// `--sd-surface-1` (containers).
    pub surface_1: String,
    /// `--sd-surface-2` (floating layers).
    pub surface_2: String,
    /// `--sd-surface-3` (hover inside floating layers, secondary button top).
    pub surface_3: String,

    // Application shell classes (_theme_presets.scss).
    pub root: String,
    pub header: String,
    pub sidebar: String,
    pub sidebar_link: String,
    pub sidebar_link_active: String,
    pub content: String,
    pub surface: String,
    pub panel: String,
    pub banner: String,

    /// Deprecated, always `""`.
    pub border: String,
    /// Deprecated, always `""`.
    pub border_subtle: String,
    /// Deprecated, always `""`.
    pub border_accent: String,
    /// `sd-text-accent` (colour `--sd-accent-strong`).
    pub text_accent: String,
    /// `sd-text-accent` (colour `--sd-accent-strong`).
    pub text_accent_bright: String,
    /// Deprecated, always `""`.
    pub glow: String,

    // Glow modifier classes per component, emitted only for an explicit
    // `glow: true` prop (see `resolve_elevation_class`). Intensity comes from
    // `--sd-glow-strength`, never from the class.
    pub glow_button: String,
    pub glow_card: String,
    pub glow_input: String,
    pub glow_badge: String,
    pub glow_modal: String,
    pub glow_alert: String,
    pub glow_dropdown: String,
    pub glow_notification: String,
    pub glow_switch: String,
    pub glow_checkbox: String,
    pub glow_select: String,
    pub glow_tooltip: String,
    pub glow_drawer: String,

    // Component classes (level 1 fallback).
    pub button_primary: String,
    pub button_secondary: String,
    pub button_outline: String,
    pub button_ghost: String,
    pub button_danger: String,
    pub button_success: String,

    pub card_base: String,
    pub card_header: String,
    pub card_footer: String,
    pub card_shadow_none: String,
    pub card_shadow_sm: String,
    pub card_shadow_md: String,
    pub card_shadow_lg: String,
    pub card_shadow_xl: String,

    pub input_base: String,
    pub input_normal_border: String,
    pub input_error_border: String,

    pub badge_default: String,
    pub badge_primary: String,
    pub badge_success: String,
    pub badge_warning: String,
    pub badge_danger: String,
    pub badge_outline: String,

    pub navbar_base: String,

    pub modal_backdrop: String,
    pub modal_dialog: String,
    pub modal_header: String,
    pub modal_close: String,

    pub alert_info: String,
    pub alert_success: String,
    pub alert_warning: String,
    pub alert_danger: String,
    pub alert_title_info: String,
    pub alert_title_success: String,
    pub alert_title_warning: String,
    pub alert_title_danger: String,

    // Level 2: per-component and per-class overrides.
    pub components: HashMap<String, ComponentOverride>,
    pub class_overrides: HashMap<String, String>,
}

impl Default for Theme {
    fn default() -> Self {
        Self::shadow()
    }
}

impl Theme {
    /// Starts a fluent theme builder.
    #[must_use]
    pub fn builder() -> ThemeBuilder {
        ThemeBuilder::default()
    }

    /// Variant class: level 2 override, else `fallback_class`.
    #[must_use]
    pub fn resolve_variant_class(
        &self,
        component: &str,
        variant: &str,
        fallback_class: &str,
    ) -> String {
        if let Some(comp_override) = self.components.get(component) {
            if let Some(custom) = comp_override.variant_classes.get(variant) {
                return custom.clone();
            }
        }
        fallback_class.to_string()
    }

    /// Size class: level 2 override, else `fallback_class`.
    #[must_use]
    pub fn resolve_size_class(&self, component: &str, size: &str, fallback_class: &str) -> String {
        if let Some(comp_override) = self.components.get(component) {
            if let Some(custom) = comp_override.size_classes.get(size) {
                return custom.clone();
            }
        }
        fallback_class.to_string()
    }

    /// Optional level 2 base class of a component.
    #[must_use]
    pub fn resolve_component_base(&self, component: &str) -> Option<&str> {
        self.components
            .get(component)
            .and_then(|c| c.base_class.as_deref())
    }

    /// Conditional modifiers for the classes of one instance (level 2).
    #[must_use]
    pub fn resolve_class_modifiers(&self, component: &str, instance_classes: &str) -> String {
        let mut modifiers = Vec::new();

        if let Some(comp) = self.components.get(component) {
            for word in instance_classes.split_whitespace() {
                if let Some(m) = comp.class_modifiers.get(word) {
                    modifiers.push(m.as_str());
                }
            }
        }

        for word in instance_classes.split_whitespace() {
            if let Some(m) = self.class_overrides.get(word) {
                modifiers.push(m.as_str());
            }
        }

        modifiers.join(" ")
    }

    /// Shadow/glow modifier class for a component.
    ///
    /// * `Some(false)` → `""`.
    /// * a level 2 `shadow_class` override → that class.
    /// * `Some(true)` → the component's `--glow` class (`component_glow`, or
    ///   `sd-<component>--glow` by convention for components without a field).
    ///   Always emitted; its intensity comes from `--sd-glow-strength`, so
    ///   `GlowMode::Off` / `ElevationStyle::Flat` simply render it at 0.
    /// * `None` → `""`: default glow (primary button, focus, selected, value fills)
    ///   is part of the component's SCSS recipe, not a class.
    #[must_use]
    pub fn resolve_elevation_class(&self, component: &str, override_glow: Option<bool>) -> String {
        if override_glow == Some(false) {
            return String::new();
        }
        if let Some(comp) = self.components.get(component) {
            if let Some(custom_shadow) = &comp.shadow_class {
                return custom_shadow.clone();
            }
        }
        if override_glow == Some(true) {
            let class = self.component_glow(component);
            if class.is_empty() {
                return format!("sd-{}--glow", component.replace('_', "-"));
            }
            return class.to_string();
        }
        String::new()
    }

    /// The effective `--sd-glow-strength` (0 when `ElevationStyle::Flat`).
    #[must_use]
    pub fn glow_strength(&self) -> &'static str {
        if self.elevation_style == ElevationStyle::Flat {
            GlowMode::Off.strength()
        } else {
            self.glow_mode.strength()
        }
    }

    /// The effective glow provider class (`sd-glow-off` when `ElevationStyle::Flat`).
    #[must_use]
    pub fn glow_class(&self) -> &'static str {
        if self.elevation_style == ElevationStyle::Flat {
            GlowMode::Off.class()
        } else {
            self.glow_mode.class()
        }
    }

    /// Classes of the `ThemeProvider` wrapper: `sd-theme sd-radius-* sd-glow-*`.
    #[must_use]
    pub fn provider_classes(&self) -> String {
        format!(
            "sd-theme {} {}",
            self.radius_scale.class(),
            self.glow_class()
        )
    }

    /// CSS declarations (`--sd-…: …;`) for the accent set, the surfaces and the
    /// glow strength. Inputs are validated: invalid hex values fall back to the
    /// defaults, so the output never contains user-controlled CSS.
    #[must_use]
    pub fn to_inline_css_variables(&self) -> String {
        let a = self.primary_color.accent_tokens();
        let (r, g, b) = a.rgb;
        let void = safe_hex(&self.surface_0, DEFAULT_SURFACES[0]);
        let s1 = safe_hex(&self.surface_1, DEFAULT_SURFACES[1]);
        let s2 = safe_hex(&self.surface_2, DEFAULT_SURFACES[2]);
        let s3 = safe_hex(&self.surface_3, DEFAULT_SURFACES[3]);
        let glow = self.glow_strength();
        format!(
            "--sd-accent: {}; --sd-accent-rgb: {r}, {g}, {b}; --sd-accent-strong: {}; \
             --sd-accent-deep: {}; --sd-accent-soft: {}; --sd-accent-border: {}; \
             --sd-accent-fg: {}; --sd-focus-ring: {}; --sd-void: {void}; \
             --sd-surface-1: {s1}; --sd-surface-2: {s2}; --sd-surface-3: {s3}; \
             --sd-glow-strength: {glow};",
            a.accent, a.strong, a.deep, a.soft, a.border, a.fg, a.focus_ring
        )
    }

    /// The same declarations as [`Theme::to_inline_css_variables`] wrapped in a
    /// `:root { … }` rule, for consumers who want to inject them globally.
    /// `ThemeProvider` does not use it.
    #[must_use]
    pub fn to_css_variables(&self) -> String {
        format!(":root {{ {} }}", self.to_inline_css_variables())
    }

    /// Default theme: Ultraviolet accent on the black void.
    #[must_use]
    pub fn shadow() -> Self {
        Self {
            name: "Ultraviolet Shadow".to_string(),
            primary_color: PrimaryColor::Violet,
            radius_scale: RadiusScale::Medium,
            elevation_style: ElevationStyle::NeonGlow,
            glow_mode: GlowMode::Subtle,

            surface_0: DEFAULT_SURFACES[0].to_string(),
            surface_1: DEFAULT_SURFACES[1].to_string(),
            surface_2: DEFAULT_SURFACES[2].to_string(),
            surface_3: DEFAULT_SURFACES[3].to_string(),

            root: "sd-root".to_string(),
            header: "sd-header".to_string(),
            sidebar: "sd-sidebar".to_string(),
            sidebar_link: "sd-sidebar-link".to_string(),
            sidebar_link_active: "sd-sidebar-link--active".to_string(),
            content: "sd-content".to_string(),
            surface: "sd-surface".to_string(),
            panel: "sd-panel".to_string(),
            banner: "sd-banner".to_string(),

            border: String::new(),
            border_subtle: String::new(),
            border_accent: String::new(),
            text_accent: "sd-text-accent".to_string(),
            text_accent_bright: "sd-text-accent".to_string(),
            glow: String::new(),

            glow_button: "sd-btn--glow".to_string(),
            glow_card: "sd-card--glow".to_string(),
            glow_input: "sd-input--glow".to_string(),
            glow_badge: "sd-badge--glow".to_string(),
            glow_modal: "sd-modal-dialog--glow".to_string(),
            glow_alert: "sd-alert--glow".to_string(),
            glow_dropdown: "sd-dropdown-menu--glow".to_string(),
            glow_notification: "sd-toast--glow".to_string(),
            glow_switch: "sd-switch__track--glow".to_string(),
            glow_checkbox: "sd-checkbox__box--glow".to_string(),
            glow_select: "sd-select-trigger--glow".to_string(),
            glow_tooltip: "sd-tooltip__bubble--glow".to_string(),
            glow_drawer: "sd-drawer-panel--glow".to_string(),

            button_primary: "sd-btn--primary".to_string(),
            button_secondary: "sd-btn--secondary".to_string(),
            button_outline: "sd-btn--outline".to_string(),
            button_ghost: "sd-btn--ghost".to_string(),
            button_danger: "sd-btn--danger".to_string(),
            button_success: "sd-btn--success".to_string(),

            card_base: "sd-card".to_string(),
            card_header: "sd-card__header".to_string(),
            card_footer: "sd-card__footer".to_string(),
            card_shadow_none: "sd-card--shadow-none".to_string(),
            card_shadow_sm: "sd-card--shadow-sm".to_string(),
            card_shadow_md: "sd-card--shadow-md".to_string(),
            card_shadow_lg: "sd-card--shadow-lg".to_string(),
            card_shadow_xl: "sd-card--shadow-xl".to_string(),

            input_base: "sd-input".to_string(),
            input_normal_border: String::new(),
            input_error_border: "sd-input--error".to_string(),

            badge_default: "sd-badge--default".to_string(),
            badge_primary: "sd-badge--primary".to_string(),
            badge_success: "sd-badge--success".to_string(),
            badge_warning: "sd-badge--warning".to_string(),
            badge_danger: "sd-badge--danger".to_string(),
            badge_outline: "sd-badge--outline".to_string(),

            navbar_base: "sd-navbar".to_string(),

            modal_backdrop: "sd-modal-backdrop".to_string(),
            modal_dialog: "sd-modal-dialog".to_string(),
            modal_header: "sd-modal-dialog__header".to_string(),
            modal_close: "sd-modal-dialog__close".to_string(),

            alert_info: "sd-alert--info".to_string(),
            alert_success: "sd-alert--success".to_string(),
            alert_warning: "sd-alert--warning".to_string(),
            alert_danger: "sd-alert--danger".to_string(),
            alert_title_info: "sd-alert__title--info".to_string(),
            alert_title_success: "sd-alert__title--success".to_string(),
            alert_title_warning: "sd-alert__title--warning".to_string(),
            alert_title_danger: "sd-alert__title--danger".to_string(),

            components: HashMap::new(),
            class_overrides: HashMap::new(),
        }
    }

    fn preset(name: &str, color: PrimaryColor) -> Self {
        let mut t = Self::shadow();
        t.name = name.to_string();
        t.apply_primary_color(color);
        t
    }

    /// Obsidian preset (`#f4f4f5`).
    #[must_use]
    pub fn obsidian() -> Self {
        Self::preset("Obsidian Titanium", PrimaryColor::Obsidian)
    }

    /// Crimson preset (`#ff1744`).
    #[must_use]
    pub fn crimson() -> Self {
        Self::preset("Crimson Phoenix", PrimaryColor::Crimson)
    }

    /// Cyan preset (`#00e5ff`).
    #[must_use]
    pub fn cyan() -> Self {
        Self::preset("Cyber Cyan", PrimaryColor::Cyan)
    }

    /// Emerald preset (`#00ff9d`).
    #[must_use]
    pub fn emerald() -> Self {
        Self::preset("Emerald Matrix", PrimaryColor::Emerald)
    }

    /// Sets the accent colour. Names, surfaces and overrides are kept; all
    /// derived tokens are recomputed when the variables are written.
    pub fn apply_primary_color(&mut self, color: PrimaryColor) {
        self.primary_color = color;
    }

    /// Sets the radius scale.
    pub fn apply_radius_scale(&mut self, radius: RadiusScale) {
        self.radius_scale = radius;
    }

    /// Sets the elevation style (`Flat` forces glow strength 0).
    pub fn apply_elevation_style(&mut self, elevation: ElevationStyle) {
        self.elevation_style = elevation;
    }

    /// Sets the glow mode (`--sd-glow-strength`). No classes change.
    pub fn apply_glow_mode(&mut self, glow: GlowMode) {
        self.glow_mode = glow;
    }

    /// The glow modifier class field of a component (`""` if it has none).
    #[must_use]
    pub fn component_glow(&self, component: &str) -> &str {
        match component {
            "button" => &self.glow_button,
            "card" => &self.glow_card,
            "input" => &self.glow_input,
            "badge" => &self.glow_badge,
            "modal" => &self.glow_modal,
            "alert" => &self.glow_alert,
            "dropdown" => &self.glow_dropdown,
            "notification" => &self.glow_notification,
            "switch" => &self.glow_switch,
            "checkbox" => &self.glow_checkbox,
            "select" => &self.glow_select,
            "tooltip" => &self.glow_tooltip,
            "drawer" => &self.glow_drawer,
            _ => &self.glow,
        }
    }

    /// Whether an optional glow modifier is on: only for an explicit
    /// `Some(true)`. Default glow belongs to the SCSS recipe.
    #[must_use]
    pub fn is_component_glow_enabled(&self, _component: &str, override_glow: Option<bool>) -> bool {
        override_glow == Some(true)
    }

    #[must_use]
    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = name.into();
        self
    }

    #[must_use]
    pub fn with_primary(mut self, button_primary: impl Into<String>) -> Self {
        self.button_primary = button_primary.into();
        self
    }

    #[must_use]
    pub fn with_card_base(mut self, card_base: impl Into<String>) -> Self {
        self.card_base = card_base.into();
        self
    }

    #[must_use]
    pub fn with_glow(mut self, glow: impl Into<String>) -> Self {
        self.glow = glow.into();
        self
    }

    #[must_use]
    pub fn with_elevation(mut self, elevation: ElevationStyle) -> Self {
        self.elevation_style = elevation;
        self
    }

    #[must_use]
    pub fn with_component_override(
        mut self,
        component: impl Into<String>,
        component_override: ComponentOverride,
    ) -> Self {
        self.components.insert(component.into(), component_override);
        self
    }

    #[must_use]
    pub fn with_class_override(
        mut self,
        target_class: impl Into<String>,
        replacement_or_extra: impl Into<String>,
    ) -> Self {
        self.class_overrides
            .insert(target_class.into(), replacement_or_extra.into());
        self
    }

    #[cfg(feature = "button")]
    #[must_use]
    pub fn button_variant_classes(&self, variant: ButtonVariant) -> &str {
        match variant {
            ButtonVariant::Primary => &self.button_primary,
            ButtonVariant::Secondary => &self.button_secondary,
            ButtonVariant::Outline => &self.button_outline,
            ButtonVariant::Ghost => &self.button_ghost,
            ButtonVariant::Danger => &self.button_danger,
            ButtonVariant::Success => &self.button_success,
        }
    }

    #[cfg(feature = "card")]
    #[must_use]
    pub fn card_shadow_classes(&self, shadow: CardShadow) -> &str {
        match shadow {
            CardShadow::None => &self.card_shadow_none,
            CardShadow::Sm => &self.card_shadow_sm,
            CardShadow::Md => &self.card_shadow_md,
            CardShadow::Lg => &self.card_shadow_lg,
            CardShadow::Xl => &self.card_shadow_xl,
        }
    }

    #[cfg(feature = "badge")]
    #[must_use]
    pub fn badge_variant_classes(&self, variant: BadgeVariant) -> &str {
        match variant {
            BadgeVariant::Default => &self.badge_default,
            BadgeVariant::Primary => &self.badge_primary,
            BadgeVariant::Success => &self.badge_success,
            BadgeVariant::Warning => &self.badge_warning,
            BadgeVariant::Danger => &self.badge_danger,
            BadgeVariant::Outline => &self.badge_outline,
        }
    }

    #[cfg(feature = "alert")]
    #[must_use]
    pub fn alert_variant_classes(&self, variant: AlertVariant) -> &str {
        match variant {
            AlertVariant::Info => &self.alert_info,
            AlertVariant::Success => &self.alert_success,
            AlertVariant::Warning => &self.alert_warning,
            AlertVariant::Danger => &self.alert_danger,
        }
    }

    #[cfg(feature = "alert")]
    #[must_use]
    pub fn alert_title_classes(&self, variant: AlertVariant) -> &str {
        match variant {
            AlertVariant::Info => &self.alert_title_info,
            AlertVariant::Success => &self.alert_title_success,
            AlertVariant::Warning => &self.alert_title_warning,
            AlertVariant::Danger => &self.alert_title_danger,
        }
    }

    #[must_use]
    pub fn input_border_class(&self, is_error: bool) -> &str {
        if is_error {
            &self.input_error_border
        } else {
            &self.input_normal_border
        }
    }
}

/// Fluent builder for [`Theme`].
#[derive(Clone, Debug, PartialEq, Default)]
pub struct ThemeBuilder {
    theme: Theme,
}

impl ThemeBuilder {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.theme.name = name.into();
        self
    }

    #[must_use]
    pub fn primary_color(mut self, color: PrimaryColor) -> Self {
        self.theme.apply_primary_color(color);
        self
    }

    #[must_use]
    pub fn radius_scale(mut self, radius: RadiusScale) -> Self {
        self.theme.apply_radius_scale(radius);
        self
    }

    #[must_use]
    pub fn elevation_style(mut self, elevation: ElevationStyle) -> Self {
        self.theme.apply_elevation_style(elevation);
        self
    }

    #[must_use]
    pub fn glow_mode(mut self, glow: GlowMode) -> Self {
        self.theme.apply_glow_mode(glow);
        self
    }

    /// `--sd-void`; ignored unless a valid hex.
    #[must_use]
    pub fn surface_0(mut self, val: impl Into<String>) -> Self {
        let s = val.into();
        if is_valid_hex_color(&s) {
            self.theme.surface_0 = s;
        }
        self
    }

    /// `--sd-surface-1`; ignored unless a valid hex.
    #[must_use]
    pub fn surface_1(mut self, val: impl Into<String>) -> Self {
        let s = val.into();
        if is_valid_hex_color(&s) {
            self.theme.surface_1 = s;
        }
        self
    }

    /// `--sd-surface-2`; ignored unless a valid hex.
    #[must_use]
    pub fn surface_2(mut self, val: impl Into<String>) -> Self {
        let s = val.into();
        if is_valid_hex_color(&s) {
            self.theme.surface_2 = s;
        }
        self
    }

    /// `--sd-surface-3`; ignored unless a valid hex.
    #[must_use]
    pub fn surface_3(mut self, val: impl Into<String>) -> Self {
        let s = val.into();
        if is_valid_hex_color(&s) {
            self.theme.surface_3 = s;
        }
        self
    }

    /// Level 2 customisation of one component.
    #[must_use]
    pub fn override_component(
        mut self,
        component: impl Into<String>,
        config_fn: impl FnOnce(ComponentOverride) -> ComponentOverride,
    ) -> Self {
        let key = component.into();
        let existing = self.theme.components.remove(&key).unwrap_or_default();
        let updated = config_fn(existing);
        self.theme.components.insert(key, updated);
        self
    }

    /// Level 2 customisation of one target class.
    #[must_use]
    pub fn override_class(
        mut self,
        target_class: impl Into<String>,
        modifier: impl Into<String>,
    ) -> Self {
        self.theme
            .class_overrides
            .insert(target_class.into(), modifier.into());
        self
    }

    #[must_use]
    pub fn build(self) -> Theme {
        self.theme
    }
}

/// Provides a reactive `Signal<Theme>` to descendants and writes it as CSS
/// variables on a `display: contents` wrapper with the classes
/// `sd-theme sd-radius-{scale} sd-glow-{mode}` (see [`Theme::provider_classes`]
/// and [`Theme::to_inline_css_variables`]). With `persist_key` (and the `serde`
/// feature) the theme is restored from and saved to `localStorage`.
#[component]
pub fn ThemeProvider(
    #[props(default)] theme: Option<Signal<Theme>>,
    #[props(default)] persist_key: Option<String>,
    children: Element,
) -> Element {
    let owned = use_signal(Theme::default);
    #[allow(unused_mut)]
    let mut active_theme = theme.unwrap_or(owned);
    use_context_provider(|| active_theme);

    #[cfg(feature = "serde")]
    {
        let mut is_restored = use_signal(|| persist_key.is_none());
        if let Some(storage_key) = persist_key.clone() {
            use_hook(move || {
                let sk = storage_key.clone();
                spawn(async move {
                    let js = format!(
                        r#"
                        try {{
                            return (typeof window !== "undefined" && window.localStorage)
                                ? (window.localStorage.getItem("{sk}") || "")
                                : "";
                        }} catch(e) {{
                            return "";
                        }}
                        "#
                    );
                    let eval = document::eval(&js);
                    if let Ok(val) = eval.await {
                        if let Some(json_str) = val.as_str() {
                            if !json_str.is_empty() {
                                if let Ok(loaded) = serde_json::from_str::<Theme>(json_str) {
                                    *active_theme.write() = loaded;
                                }
                            }
                        }
                    }
                    is_restored.set(true);
                });
            });

            let save_key = persist_key.unwrap_or_default();
            use_effect(move || {
                if !is_restored() {
                    return;
                }
                let current = active_theme();
                if let Ok(json) = serde_json::to_string(&current) {
                    let sk = save_key.clone();
                    spawn(async move {
                        let js_str = serde_json::to_string(&json).unwrap_or_default();
                        let eval = document::eval(&format!(
                            r#"
                            try {{
                                if (typeof window !== "undefined" && window.localStorage) {{
                                    window.localStorage.setItem("{sk}", {js_str});
                                }}
                            }} catch(e) {{}}
                            "#
                        ));
                        let _ = eval.await;
                    });
                }
            });
        }
    }

    let theme_val = active_theme.read();
    let classes = theme_val.provider_classes();
    let inline_vars = theme_val.to_inline_css_variables();

    // The variables are written inline on the wrapper only (no global `:root`
    // <style>): overlays render inside the provider subtree, and everything
    // outside it (e.g. `body`) uses the `:root` defaults from _tokens.scss.
    // `display: contents` keeps the wrapper out of layout; custom properties
    // still inherit through it.
    rsx! {
        div {
            class: "{classes}",
            style: "display: contents; {inline_vars}",
            {children}
        }
    }
}

/// Reads (or, with `.write()`, changes) the active theme.
pub fn use_theme() -> Signal<Theme> {
    match try_use_context::<Signal<Theme>>() {
        Some(s) => s,
        None => {
            #[cfg(debug_assertions)]
            {
                static WARNED: std::sync::atomic::AtomicBool =
                    std::sync::atomic::AtomicBool::new(false);
                if !WARNED.swap(true, std::sync::atomic::Ordering::Relaxed) {
                    eprintln!(
                        "Warning: `use_theme()` called without an ancestor `ThemeProvider`. Using shared root context fallback."
                    );
                }
            }
            let rt = dioxus_core::Runtime::current();
            let sig = Signal::new_in_scope(Theme::default(), ScopeId::ROOT);
            rt.provide_context(ScopeId::ROOT, sig)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_accent_derivation_ultraviolet() {
        let a = PrimaryColor::Violet.accent_tokens();
        assert_eq!(a.accent, "#a855f7");
        assert_eq!(a.rgb, (168, 85, 247));
        assert_eq!(a.strong, "#c691fa");
        assert_eq!(a.deep, "#653394");
        assert_eq!(a.soft, "rgba(168, 85, 247, 0.14)");
        assert_eq!(a.border, "rgba(168, 85, 247, 0.45)");
        assert_eq!(a.fg, "#ffffff");
        assert_eq!(a.focus_ring, a.strong);
    }

    #[test]
    fn test_accent_fg_follows_luminance() {
        assert_eq!(PrimaryColor::Obsidian.accent_tokens().fg, "#000000");
        assert_eq!(PrimaryColor::Cyan.accent_tokens().fg, "#000000");
        assert_eq!(PrimaryColor::Emerald.accent_tokens().fg, "#000000");
        assert_eq!(PrimaryColor::Crimson.accent_tokens().fg, "#ffffff");
        assert_eq!(PrimaryColor::custom("Yellow", "#ffee00").accent_tokens().fg, "#000000");
    }

    #[test]
    fn test_preset_hexes() {
        assert_eq!(PrimaryColor::Violet.hex(), "#a855f7");
        assert_eq!(PrimaryColor::Obsidian.hex(), "#f4f4f5");
        assert_eq!(PrimaryColor::Crimson.hex(), "#ff1744");
        assert_eq!(PrimaryColor::Cyan.hex(), "#00e5ff");
        assert_eq!(PrimaryColor::Emerald.hex(), "#00ff9d");
        for c in [
            PrimaryColor::Violet,
            PrimaryColor::Obsidian,
            PrimaryColor::Crimson,
            PrimaryColor::Cyan,
            PrimaryColor::Emerald,
        ] {
            assert_eq!(c.shadow_hex(), c.accent_tokens().deep);
        }
    }

    #[test]
    fn test_custom_color_any_prefix_uses_generic_classes() {
        let custom = PrimaryColor::custom_with_prefix("Brand", "#ff8800", "My Brand_123!");
        assert_eq!(custom.css_prefix(), "my-brand-123");
        let themed = Theme::builder().primary_color(custom).build();
        assert_eq!(themed.button_primary, "sd-btn--primary");
        assert_eq!(themed.provider_classes(), "sd-theme sd-radius-medium sd-glow-subtle");
        let vars = themed.to_inline_css_variables();
        assert!(vars.contains("--sd-accent: #ff8800;"));
        assert!(vars.contains("--sd-accent-rgb: 255, 136, 0;"));
        assert!(!vars.contains("my-brand"));
    }

    #[test]
    fn test_invalid_hex_is_sanitised() {
        let c = PrimaryColor::custom("Bad", "#${alert(1)}");
        assert_eq!(c.hex(), "#a855f7");
        let mut t = Theme::shadow();
        t.primary_color = PrimaryColor::Custom {
            name: "x".into(),
            hex: "red; background: url(x)".into(),
            css_prefix: None,
        };
        t.surface_1 = "}</style>".into();
        let vars = t.to_inline_css_variables();
        assert!(vars.contains("--sd-accent: #a855f7;"));
        assert!(vars.contains("--sd-surface-1: #09090d;"));
        assert!(!vars.contains("url("));
    }

    #[test]
    fn test_hex_color_validation() {
        assert!(is_valid_hex_color("#fff"));
        assert!(is_valid_hex_color("#ffffff"));
        assert!(is_valid_hex_color("#123456aa"));
        assert!(!is_valid_hex_color(""));
        assert!(!is_valid_hex_color("fff"));
        assert!(!is_valid_hex_color("#12"));
        assert!(!is_valid_hex_color("#123456789"));
        assert!(!is_valid_hex_color("#${alert(1)}"));
    }

    #[test]
    fn test_css_variables_share_one_source() {
        let t = Theme::crimson();
        assert_eq!(
            t.to_css_variables(),
            format!(":root {{ {} }}", t.to_inline_css_variables())
        );
    }

    #[test]
    fn test_builder_preserves_overrides_when_changing_color() {
        let theme = Theme::builder()
            .override_component("button", |o| o.with_base_class("my-btn"))
            .primary_color(PrimaryColor::Crimson)
            .build();
        assert_eq!(theme.primary_color, PrimaryColor::Crimson);
        assert_eq!(theme.resolve_component_base("button"), Some("my-btn"));
    }

    #[test]
    fn test_presets_emit_identical_classes() {
        let base = Theme::shadow();
        for t in [Theme::obsidian(), Theme::crimson(), Theme::cyan(), Theme::emerald()] {
            assert_eq!(t.button_primary, base.button_primary);
            assert_eq!(t.badge_primary, base.badge_primary);
            assert_eq!(t.sidebar_link_active, base.sidebar_link_active);
            assert_eq!(t.banner, base.banner);
            assert_eq!(t.glow_button, base.glow_button);
            assert_eq!(t.text_accent, "sd-text-accent");
        }
    }

    #[test]
    fn test_radius_scale_is_provider_class_only() {
        for r in [RadiusScale::Sharp, RadiusScale::Medium, RadiusScale::Soft] {
            assert_eq!(r.sm(), "");
            assert_eq!(r.md(), "");
            assert_eq!(r.lg(), "");
        }
        assert_eq!(RadiusScale::Sharp.class(), "sd-radius-sharp");
        let t = Theme::builder().radius_scale(RadiusScale::Soft).build();
        assert!(t.provider_classes().contains("sd-radius-soft"));
    }

    #[test]
    fn test_glow_strength_and_flat() {
        let mut theme = Theme::shadow();
        assert_eq!(theme.glow_strength(), "0.6");
        theme.apply_glow_mode(GlowMode::Expressive);
        assert_eq!(theme.glow_strength(), "1");
        assert!(theme.to_inline_css_variables().contains("--sd-glow-strength: 1;"));
        theme.apply_glow_mode(GlowMode::Off);
        assert_eq!(theme.glow_strength(), "0");
        theme.apply_glow_mode(GlowMode::Subtle);
        theme.apply_elevation_style(ElevationStyle::Flat);
        assert_eq!(theme.glow_strength(), "0");
        assert!(theme.provider_classes().ends_with("sd-glow-off"));
    }

    #[test]
    fn test_explicit_glow_always_yields_class() {
        for mode in [GlowMode::Off, GlowMode::Subtle, GlowMode::Expressive] {
            let t = Theme::builder().glow_mode(mode).build();
            assert_eq!(t.resolve_elevation_class("button", Some(true)), "sd-btn--glow");
            assert_eq!(t.resolve_elevation_class("card", Some(true)), "sd-card--glow");
            assert_eq!(t.resolve_elevation_class("number_input", Some(true)), "sd-number-input--glow");
            assert_eq!(t.resolve_elevation_class("button", None), "");
            assert_eq!(t.resolve_elevation_class("button", Some(false)), "");
        }
        let t = Theme::builder()
            .override_component("card", |o| o.with_shadow("my-shadow"))
            .build();
        assert_eq!(t.resolve_elevation_class("card", None), "my-shadow");
        assert_eq!(t.resolve_elevation_class("card", Some(false)), "");
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_theme_serde_missing_fields() {
        let json = r#"{"name": "Saved Theme", "primary_color": "Violet"}"#;
        let deserialized: Result<Theme, _> = serde_json::from_str(json);
        assert!(
            deserialized.is_ok(),
            "Theme should deserialize with missing fields: {:?}",
            deserialized.err()
        );
        let theme = deserialized.unwrap();
        assert_eq!(theme.name, "Saved Theme");
        assert_eq!(theme.primary_color, PrimaryColor::Violet);
        assert_eq!(theme.radius_scale, RadiusScale::Medium);
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_removed_elevation_styles_do_not_deserialize() {
        assert!(serde_json::from_str::<ElevationStyle>(r#""HardBlock""#).is_err());
        assert!(serde_json::from_str::<ElevationStyle>(r#""Flat""#).is_ok());
    }
}
