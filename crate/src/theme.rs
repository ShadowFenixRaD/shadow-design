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

/// Verifica se una stringa è un colore esadecimale CSS valido (#rgb, #rgba, #rrggbb, #rrggbbaa).
pub fn is_valid_hex_color(hex: &str) -> bool {
    let s = hex.trim();
    if !s.starts_with('#') {
        return false;
    }
    let body = &s[1..];
    (body.len() == 3 || body.len() == 4 || body.len() == 6 || body.len() == 8)
        && body.chars().all(|c| c.is_ascii_hexdigit())
}

/// Converte un colore hex valido in tuple (r, g, b). Se non valido, restituisce None.
pub fn hex_to_rgb(hex: &str) -> Option<(u8, u8, u8)> {
    if !is_valid_hex_color(hex) {
        return None;
    }
    let s = hex.trim().trim_start_matches('#');
    match s.len() {
        3 | 4 => {
            let r = u8::from_str_radix(&s[0..1].repeat(2), 16).ok()?;
            let g = u8::from_str_radix(&s[1..2].repeat(2), 16).ok()?;
            let b = u8::from_str_radix(&s[2..3].repeat(2), 16).ok()?;
            Some((r, g, b))
        }
        6 | 8 => {
            let r = u8::from_str_radix(&s[0..2], 16).ok()?;
            let g = u8::from_str_radix(&s[2..4], 16).ok()?;
            let b = u8::from_str_radix(&s[4..6], 16).ok()?;
            Some((r, g, b))
        }
        _ => None,
    }
}

/// Palette di colori primari in stile Mantine / Material-UI, con supporto a colori Custom estendibili.
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum PrimaryColor {
    #[default]
    Violet,
    Obsidian,
    Crimson,
    Cyan,
    Emerald,
    Custom {
        name: String,
        hex: String,
        css_prefix: Option<String>,
    },
}

impl PrimaryColor {
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

    /// Restituisce il valore hex primario. Per Violet restituisce l'accento vivido #a855f7 (VQA-22).
    pub fn hex(&self) -> &str {
        self.accent_hex()
    }

    /// Restituisce il colore vivido di accento per swatch e variabili CSS (VQA-22).
    pub fn accent_hex(&self) -> &str {
        match self {
            PrimaryColor::Violet => "#a855f7",
            PrimaryColor::Obsidian => "#ffffff",
            PrimaryColor::Crimson => "#ff1744",
            PrimaryColor::Cyan => "#00f0ff",
            PrimaryColor::Emerald => "#00ff9d",
            PrimaryColor::Custom { hex, .. } => hex.as_str(),
        }
    }

    /// Restituisce il colore d'ombra o tonalità profonda (VQA-22).
    pub fn shadow_hex(&self) -> &str {
        match self {
            PrimaryColor::Violet => "#3b0764",
            PrimaryColor::Obsidian => "#27272a",
            PrimaryColor::Crimson => "#4c0519",
            PrimaryColor::Cyan => "#083344",
            PrimaryColor::Emerald => "#022c22",
            PrimaryColor::Custom { hex, .. } => hex.as_str(),
        }
    }

    /// Ricava automaticamente il prefisso o slug per le classi CSS (es. "custom", "cartoon").
    /// Se css_prefix è None, mappa a "custom" (RS-02). Se Some(p), sanitizza a caratteri alfanumerici e trattini.
    pub fn css_prefix(&self) -> String {
        match self {
            PrimaryColor::Violet => "violet".to_string(),
            PrimaryColor::Obsidian => "obsidian".to_string(),
            PrimaryColor::Crimson => "crimson".to_string(),
            PrimaryColor::Cyan => "cyan".to_string(),
            PrimaryColor::Emerald => "emerald".to_string(),
            PrimaryColor::Custom { css_prefix, .. } => match css_prefix {
                None => "custom".to_string(),
                Some(p) => {
                    let sanitized: String = p
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
            },
        }
    }

    pub fn custom(name: impl Into<String>, hex: impl Into<String>) -> Self {
        let h = hex.into();
        let valid_hex = if is_valid_hex_color(&h) {
            h
        } else {
            "#a855f7".to_string()
        };
        Self::Custom {
            name: name.into(),
            hex: valid_hex,
            css_prefix: None,
        }
    }

    pub fn custom_with_prefix(
        name: impl Into<String>,
        hex: impl Into<String>,
        css_prefix: impl Into<String>,
    ) -> Self {
        let h = hex.into();
        let valid_hex = if is_valid_hex_color(&h) {
            h
        } else {
            "#a855f7".to_string()
        };
        let p = css_prefix.into();
        let sanitized: String = p
            .to_lowercase()
            .chars()
            .map(|c| if c.is_ascii_alphanumeric() { c } else { '-' })
            .collect();
        let trimmed = sanitized
            .split('-')
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>()
            .join("-");
        let final_prefix = if trimmed.is_empty() {
            "custom".to_string()
        } else {
            trimmed
        };
        Self::Custom {
            name: name.into(),
            hex: valid_hex,
            css_prefix: Some(final_prefix),
        }
    }
}

/// Scala globale per i raggi di curvatura (Border Radius).
#[derive(Clone, Copy, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum RadiusScale {
    Sharp,
    #[default]
    Medium,
    Soft,
}

impl RadiusScale {
    pub fn name(&self) -> &'static str {
        match self {
            RadiusScale::Sharp => "Sharp (2px)",
            RadiusScale::Medium => "Medium (8px)",
            RadiusScale::Soft => "Soft (14px)",
        }
    }

    pub fn sm(&self) -> &'static str {
        match self {
            RadiusScale::Sharp => "sd-radius-sharp",
            RadiusScale::Medium => "rounded-md",
            RadiusScale::Soft => "rounded-lg",
        }
    }

    pub fn md(&self) -> &'static str {
        match self {
            RadiusScale::Sharp => "sd-radius-sharp",
            RadiusScale::Medium => "rounded-lg",
            RadiusScale::Soft => "rounded-xl",
        }
    }

    pub fn lg(&self) -> &'static str {
        match self {
            RadiusScale::Sharp => "sd-radius-sharp",
            RadiusScale::Medium => "rounded-xl",
            RadiusScale::Soft => "rounded-2xl",
        }
    }
}

/// Intensità del micro-glow ottico radiale.
#[derive(Clone, Copy, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum GlowMode {
    #[default]
    Subtle,
    Expressive,
    Off,
}

/// Stile di elevazione delle ombre (Livello 1): Neon Glow, Hard Block cartoon, Soft o Flat.
#[derive(Clone, Copy, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ElevationStyle {
    #[default]
    NeonGlow,     // Specular neon glow sfumato (Cyberpunk / Ultraviolet)
    HardBlock,    // Ombre piene offset senza blur (Cartoon / Manga Pop / Neo-brutalist)
    SoftElevated, // Ombre material soft
    Flat,         // Senza ombre
}

/// Personalizzazione specifica per un componente (Livello 2 - Component Level).
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
pub struct ComponentOverride {
    /// Classe base aggiuntiva o sostitutiva
    pub base_class: Option<String>,
    /// Mappa delle varianti (es. "primary" -> "my-primary-btn", "danger" -> "my-danger-btn")
    pub variant_classes: HashMap<String, String>,
    /// Mappa delle taglie (es. "sm" -> "my-sm-btn")
    pub size_classes: HashMap<String, String>,
    /// Override dell'ombra o elevazione per questo componente
    pub shadow_class: Option<String>,
    /// Modificatori condizionali per classi specifiche
    pub class_modifiers: HashMap<String, String>,
}

impl ComponentOverride {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_base_class(mut self, base: impl Into<String>) -> Self {
        self.base_class = Some(base.into());
        self
    }

    pub fn with_variant(mut self, variant: impl Into<String>, class: impl Into<String>) -> Self {
        self.variant_classes.insert(variant.into(), class.into());
        self
    }

    pub fn with_size(mut self, size: impl Into<String>, class: impl Into<String>) -> Self {
        self.size_classes.insert(size.into(), class.into());
        self
    }

    pub fn with_shadow(mut self, shadow: impl Into<String>) -> Self {
        self.shadow_class = Some(shadow.into());
        self
    }

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

#[derive(Clone, Debug, Default)]
struct GlowTokens {
    glow: String,
    button: String,
    card: String,
    input: String,
    badge: String,
    modal: String,
    alert: String,
    dropdown: String,
    notification: String,
    switch: String,
    checkbox: String,
    select: String,
    tooltip: String,
    drawer: String,
}

#[derive(Clone, Debug)]
struct ColorTokens {
    sidebar_link_active: String,
    border_accent: String,
    text_accent: String,
    text_accent_bright: String,
    button_primary: String,
    badge_primary: String,
    banner: String,
}

/// Struttura centrale del tema per Shadow Design ispirata a Mantine e Material-UI.
/// Supporta personalizzazione gerarchica su 3 livelli.
#[derive(Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
pub struct Theme {
    pub name: String,
    pub primary_color: PrimaryColor,
    pub radius_scale: RadiusScale,
    pub elevation_style: ElevationStyle,
    pub glow_mode: GlowMode,

    // Token di Superficie a 4 Livelli (Void, Base, Elevated, Overlay)
    pub surface_0: String,
    pub surface_1: String,
    pub surface_2: String,
    pub surface_3: String,

    // Layout e Struttura
    pub root: String,
    pub header: String,
    pub sidebar: String,
    pub sidebar_link: String,
    pub sidebar_link_active: String,
    pub content: String,
    pub surface: String,
    pub panel: String,
    pub banner: String,

    // Accenti e Bordi
    pub border: String,
    pub border_subtle: String,
    pub border_accent: String,
    pub text_accent: String,
    pub text_accent_bright: String,
    pub glow: String,

    // Glow specifici per componente (Livello 1 fallback)
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

    // Classi dei componenti (Livello 1 fallback)
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

    // Livello 2: Registro di personalizzazione per componente e per classe
    pub components: HashMap<String, ComponentOverride>,
    pub class_overrides: HashMap<String, String>,
}

impl Default for Theme {
    fn default() -> Self {
        Self::shadow()
    }
}

impl Theme {
    /// Inizia la costruzione di un tema personalizzato con builder fluido.
    pub fn builder() -> ThemeBuilder {
        ThemeBuilder::default()
    }

    /// Risolve la classe variante tenendo conto della gerarchia:
    /// Livello 2 (Component Override) -> Livello 1 (Global Theme Fallback).
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

    /// Risolve la classe taglia per un componente tenendo conto di Livello 2 -> Livello 1.
    pub fn resolve_size_class(&self, component: &str, size: &str, fallback_class: &str) -> String {
        if let Some(comp_override) = self.components.get(component) {
            if let Some(custom) = comp_override.size_classes.get(size) {
                return custom.clone();
            }
        }
        fallback_class.to_string()
    }

    /// Restituisce la classe base opzionale definita per il componente (Livello 2).
    pub fn resolve_component_base(&self, component: &str) -> Option<&str> {
        self.components
            .get(component)
            .and_then(|c| c.base_class.as_deref())
    }

    /// Risolve modificatori condizionali per classi specifiche (Livello 2 per classi).
    pub fn resolve_class_modifiers(&self, component: &str, instance_classes: &str) -> String {
        let mut modifiers = Vec::new();

        // Controllo sui modificatori del componente specifico
        if let Some(comp) = self.components.get(component) {
            for word in instance_classes.split_whitespace() {
                if let Some(m) = comp.class_modifiers.get(word) {
                    modifiers.push(m.as_str());
                }
            }
        }

        // Controllo sugli override globali per classe
        for word in instance_classes.split_whitespace() {
            if let Some(m) = self.class_overrides.get(word) {
                modifiers.push(m.as_str());
            }
        }

        modifiers.join(" ")
    }

    /// Risolve la classe di ombra / glow in base all'ElevationStyle e alla configurazione del componente.
    pub fn resolve_elevation_class(&self, component: &str, override_glow: Option<bool>) -> String {
        if override_glow == Some(false) {
            return String::new();
        }

        // Se il componente ha un override esplicito di ombra/glow a Livello 2
        if let Some(comp) = self.components.get(component) {
            if let Some(custom_shadow) = &comp.shadow_class {
                return custom_shadow.clone();
            }
        }

        match self.elevation_style {
            ElevationStyle::Flat => String::new(),
            ElevationStyle::HardBlock => match component {
                "button" => "sd-comic-shadow-sm".to_string(),
                "card" => "sd-comic-shadow-lg".to_string(),
                "modal" => "sd-comic-shadow-xl".to_string(),
                "dropdown" => "sd-comic-shadow-md".to_string(),
                "badge" => "sd-comic-shadow-xs".to_string(),
                "alert" => "sd-comic-shadow-md".to_string(),
                _ => "sd-comic-shadow-sm".to_string(),
            },
            ElevationStyle::SoftElevated => match component {
                "button" => "shadow-sm".to_string(),
                "card" => "shadow-md".to_string(),
                "modal" => "shadow-xl".to_string(),
                _ => "shadow-md".to_string(),
            },
            ElevationStyle::NeonGlow => {
                if self.is_component_glow_enabled(component, override_glow) {
                    self.component_glow(component).to_string()
                } else {
                    String::new()
                }
            }
        }
    }

    /// Genera la stringa di CSS Custom Properties da iniettare dinamicamente a runtime (DS-10, SC-15).
    pub fn to_css_variables(&self) -> String {
        let safe_primary = if is_valid_hex_color(self.primary_color.hex()) {
            self.primary_color.hex()
        } else {
            "#a855f7"
        };
        let (r, g, b) = hex_to_rgb(safe_primary).unwrap_or((168, 85, 247));
        let safe_s0 = if is_valid_hex_color(&self.surface_0) {
            &self.surface_0
        } else {
            "#030305"
        };
        let safe_s1 = if is_valid_hex_color(&self.surface_1) {
            &self.surface_1
        } else {
            "#09090d"
        };
        let safe_s2 = if is_valid_hex_color(&self.surface_2) {
            &self.surface_2
        } else {
            "#111118"
        };
        let safe_s3 = if is_valid_hex_color(&self.surface_3) {
            &self.surface_3
        } else {
            "#181824"
        };
        let glow_strength = match self.glow_mode {
            GlowMode::Off => "0",
            GlowMode::Subtle => "0.7",
            GlowMode::Expressive => "1.0",
        };

        format!(
            ":root {{ \
                --sd-primary-color: {safe_primary}; \
                --sd-accent: {safe_primary}; \
                --sd-accent-rgb: {r}, {g}, {b}; \
                --sd-surface-0: {safe_s0}; \
                --sd-surface-1: {safe_s1}; \
                --sd-surface-2: {safe_s2}; \
                --sd-surface-3: {safe_s3}; \
                --sd-glow-strength: {glow_strength}; \
            }}"
        )
    }

    /// Calcola i valori di glow per ogni componente in base al colore primario e alla modalità (DS-13, DS-12, DS-02).
    fn calculate_glows(color: &PrimaryColor, mode: GlowMode) -> GlowTokens {
        if mode == GlowMode::Off {
            return GlowTokens::default();
        }

        let prefix = color.css_prefix();
        let is_subtle = mode == GlowMode::Subtle;

        match color {
            PrimaryColor::Violet => GlowTokens {
                glow: if is_subtle {
                    String::new()
                } else {
                    "glow-violet".to_string()
                },
                button: "sd-btn--glow".to_string(),
                card: if is_subtle {
                    String::new()
                } else {
                    "sd-card--hover-glow".to_string()
                },
                input: String::new(),
                badge: if is_subtle {
                    String::new()
                } else {
                    "sd-badge--glow".to_string()
                },
                modal: if is_subtle {
                    String::new()
                } else {
                    "sd-modal-dialog--glow".to_string()
                },
                alert: if is_subtle {
                    String::new()
                } else {
                    "sd-alert--glow-info".to_string()
                },
                dropdown: if is_subtle {
                    String::new()
                } else {
                    "sd-dropdown-menu--glow".to_string()
                },
                notification: if is_subtle {
                    String::new()
                } else {
                    "sd-toast--glow-info".to_string()
                },
                switch: "sd-switch__track--glow".to_string(),
                checkbox: "sd-checkbox__box--glow".to_string(),
                select: if is_subtle {
                    String::new()
                } else {
                    "sd-select-trigger--glow".to_string()
                },
                tooltip: if is_subtle {
                    String::new()
                } else {
                    "sd-tooltip__bubble--glow".to_string()
                },
                drawer: if is_subtle {
                    String::new()
                } else {
                    "sd-drawer-panel--glow".to_string()
                },
            },
            PrimaryColor::Obsidian => GlowTokens {
                glow: String::new(),
                button: "sd-btn--glow-obsidian".to_string(),
                card: if is_subtle {
                    String::new()
                } else {
                    "sd-card--hover-glow".to_string()
                },
                input: String::new(),
                badge: if is_subtle {
                    String::new()
                } else {
                    "sd-badge--glow".to_string()
                },
                modal: if is_subtle {
                    String::new()
                } else {
                    "sd-modal-dialog--glow".to_string()
                },
                alert: String::new(),
                dropdown: if is_subtle {
                    String::new()
                } else {
                    "sd-dropdown-menu--glow".to_string()
                },
                notification: String::new(),
                switch: "sd-switch__track--glow-obsidian".to_string(),
                checkbox: "sd-checkbox__box--glow".to_string(),
                select: String::new(),
                tooltip: String::new(),
                drawer: String::new(),
            },
            PrimaryColor::Crimson => GlowTokens {
                glow: if is_subtle {
                    String::new()
                } else {
                    "glow-crimson".to_string()
                },
                button: "sd-btn--glow-crimson".to_string(),
                card: if is_subtle {
                    String::new()
                } else {
                    "sd-card--hover-glow".to_string()
                },
                input: String::new(),
                badge: if is_subtle {
                    String::new()
                } else {
                    "sd-badge--glow".to_string()
                },
                modal: if is_subtle {
                    String::new()
                } else {
                    "sd-modal-dialog--glow".to_string()
                },
                alert: if is_subtle {
                    String::new()
                } else {
                    "sd-alert--glow-danger".to_string()
                },
                dropdown: if is_subtle {
                    String::new()
                } else {
                    "sd-dropdown-menu--glow".to_string()
                },
                notification: if is_subtle {
                    String::new()
                } else {
                    "sd-toast--glow-danger".to_string()
                },
                switch: "sd-switch__track--glow-crimson".to_string(),
                checkbox: "sd-checkbox__box--glow".to_string(),
                select: if is_subtle {
                    String::new()
                } else {
                    "sd-select-trigger--glow".to_string()
                },
                tooltip: if is_subtle {
                    String::new()
                } else {
                    "sd-tooltip__bubble--glow".to_string()
                },
                drawer: if is_subtle {
                    String::new()
                } else {
                    "sd-drawer-panel--glow".to_string()
                },
            },
            PrimaryColor::Cyan => GlowTokens {
                glow: if is_subtle {
                    String::new()
                } else {
                    "glow-cyan".to_string()
                },
                button: "sd-btn--glow-cyan".to_string(),
                card: if is_subtle {
                    String::new()
                } else {
                    "sd-card--hover-glow".to_string()
                },
                input: String::new(),
                badge: if is_subtle {
                    String::new()
                } else {
                    "sd-badge--glow".to_string()
                },
                modal: if is_subtle {
                    String::new()
                } else {
                    "sd-modal-dialog--glow".to_string()
                },
                alert: String::new(),
                dropdown: if is_subtle {
                    String::new()
                } else {
                    "sd-dropdown-menu--glow".to_string()
                },
                notification: String::new(),
                switch: "sd-switch__track--glow-cyan".to_string(),
                checkbox: "sd-checkbox__box--glow".to_string(),
                select: if is_subtle {
                    String::new()
                } else {
                    "sd-select-trigger--glow".to_string()
                },
                tooltip: if is_subtle {
                    String::new()
                } else {
                    "sd-tooltip__bubble--glow".to_string()
                },
                drawer: if is_subtle {
                    String::new()
                } else {
                    "sd-drawer-panel--glow".to_string()
                },
            },
            PrimaryColor::Emerald => GlowTokens {
                glow: if is_subtle {
                    String::new()
                } else {
                    "glow-emerald".to_string()
                },
                button: "sd-btn--glow-emerald".to_string(),
                card: if is_subtle {
                    String::new()
                } else {
                    "sd-card--hover-glow".to_string()
                },
                input: String::new(),
                badge: if is_subtle {
                    String::new()
                } else {
                    "sd-badge--glow".to_string()
                },
                modal: if is_subtle {
                    String::new()
                } else {
                    "sd-modal-dialog--glow".to_string()
                },
                alert: if is_subtle {
                    String::new()
                } else {
                    "sd-alert--glow-success".to_string()
                },
                dropdown: if is_subtle {
                    String::new()
                } else {
                    "sd-dropdown-menu--glow".to_string()
                },
                notification: if is_subtle {
                    String::new()
                } else {
                    "sd-toast--glow-success".to_string()
                },
                switch: "sd-switch__track--glow-emerald".to_string(),
                checkbox: "sd-checkbox__box--glow".to_string(),
                select: if is_subtle {
                    String::new()
                } else {
                    "sd-select-trigger--glow".to_string()
                },
                tooltip: if is_subtle {
                    String::new()
                } else {
                    "sd-tooltip__bubble--glow".to_string()
                },
                drawer: if is_subtle {
                    String::new()
                } else {
                    "sd-drawer-panel--glow".to_string()
                },
            },
            PrimaryColor::Custom { .. } => GlowTokens {
                glow: if is_subtle {
                    String::new()
                } else {
                    format!("glow-{}", prefix)
                },
                button: format!("sd-btn--glow-{}", prefix),
                card: if is_subtle {
                    String::new()
                } else {
                    format!("sd-card--glow-{}", prefix)
                },
                input: String::new(),
                badge: if is_subtle {
                    String::new()
                } else {
                    format!("sd-badge--glow-{}", prefix)
                },
                modal: if is_subtle {
                    String::new()
                } else {
                    format!("sd-modal-dialog--glow-{}", prefix)
                },
                alert: String::new(),
                dropdown: if is_subtle {
                    String::new()
                } else {
                    "sd-dropdown-menu--glow".to_string()
                },
                notification: String::new(),
                switch: format!("sd-switch__track--glow-{}", prefix),
                checkbox: "sd-checkbox__box--glow".to_string(),
                select: if is_subtle {
                    String::new()
                } else {
                    "sd-select-trigger--glow".to_string()
                },
                tooltip: if is_subtle {
                    String::new()
                } else {
                    "sd-tooltip__bubble--glow".to_string()
                },
                drawer: if is_subtle {
                    String::new()
                } else {
                    "sd-drawer-panel--glow".to_string()
                },
            },
        }
    }

    fn color_tokens(color: &PrimaryColor) -> ColorTokens {
        match color {
            PrimaryColor::Violet => ColorTokens {
                sidebar_link_active: "sd-sidebar-link--active".to_string(),
                border_accent: "border-purple-500/40".to_string(),
                text_accent: "text-purple-400".to_string(),
                text_accent_bright: "text-purple-300".to_string(),
                button_primary: "sd-btn--primary".to_string(),
                badge_primary: "sd-badge--primary".to_string(),
                banner: "sd-banner".to_string(),
            },
            PrimaryColor::Obsidian => ColorTokens {
                sidebar_link_active: "sd-sidebar-link--active-obsidian".to_string(),
                border_accent: "border-zinc-500/40".to_string(),
                text_accent: "text-zinc-300".to_string(),
                text_accent_bright: "text-white".to_string(),
                button_primary: "sd-btn--primary-obsidian".to_string(),
                badge_primary: "sd-badge--default".to_string(),
                banner: "sd-banner sd-banner--obsidian".to_string(),
            },
            PrimaryColor::Crimson => ColorTokens {
                sidebar_link_active: "sd-sidebar-link--active-crimson".to_string(),
                border_accent: "border-rose-500/40".to_string(),
                text_accent: "text-rose-400".to_string(),
                text_accent_bright: "text-rose-300".to_string(),
                button_primary: "sd-btn--primary-crimson".to_string(),
                badge_primary: "sd-badge--danger".to_string(),
                banner: "sd-banner sd-banner--crimson".to_string(),
            },
            PrimaryColor::Cyan => ColorTokens {
                sidebar_link_active: "sd-sidebar-link--active-cyan".to_string(),
                border_accent: "border-cyan-500/40".to_string(),
                text_accent: "text-cyan-400".to_string(),
                text_accent_bright: "text-cyan-300".to_string(),
                button_primary: "sd-btn--primary-cyan".to_string(),
                badge_primary: "sd-badge--primary".to_string(),
                banner: "sd-banner sd-banner--cyan".to_string(),
            },
            PrimaryColor::Emerald => ColorTokens {
                sidebar_link_active: "sd-sidebar-link--active-emerald".to_string(),
                border_accent: "border-emerald-500/40".to_string(),
                text_accent: "text-emerald-400".to_string(),
                text_accent_bright: "text-emerald-300".to_string(),
                button_primary: "sd-btn--primary-emerald".to_string(),
                badge_primary: "sd-badge--success".to_string(),
                banner: "sd-banner sd-banner--emerald".to_string(),
            },
            PrimaryColor::Custom { .. } => {
                let prefix = color.css_prefix();
                ColorTokens {
                    sidebar_link_active: "sd-sidebar-link--active".to_string(),
                    border_accent: "border-purple-500/40".to_string(),
                    text_accent: "text-purple-400".to_string(),
                    text_accent_bright: "text-purple-300".to_string(),
                    button_primary: format!("sd-btn--primary-{}", prefix),
                    badge_primary: "sd-badge--primary".to_string(),
                    banner: "sd-banner".to_string(),
                }
            }
        }
    }

    /// Tema di default Shadow Design (Ultraviolet Obsidian: Nero profondo con accenti viola).
    pub fn shadow() -> Self {
        let glows = Self::calculate_glows(&PrimaryColor::Violet, GlowMode::Subtle);

        Self {
            name: "Ultraviolet Shadow".to_string(),
            primary_color: PrimaryColor::Violet,
            radius_scale: RadiusScale::Medium,
            elevation_style: ElevationStyle::NeonGlow,
            glow_mode: GlowMode::Subtle,

            surface_0: "#030305".to_string(),
            surface_1: "#09090d".to_string(),
            surface_2: "#111118".to_string(),
            surface_3: "#181824".to_string(),

            root: "sd-root".to_string(),
            header: "sd-header".to_string(),
            sidebar: "sd-sidebar".to_string(),
            sidebar_link: "sd-sidebar-link".to_string(),
            sidebar_link_active: "sd-sidebar-link--active".to_string(),
            content: "sd-content".to_string(),
            surface: "sd-surface".to_string(),
            panel: "sd-panel".to_string(),
            banner: "sd-banner".to_string(),

            border: "border-purple-500/25".to_string(),
            border_subtle: "border-purple-500/10".to_string(),
            border_accent: "border-purple-500/40".to_string(),
            text_accent: "text-purple-400".to_string(),
            text_accent_bright: "text-purple-300".to_string(),
            glow: glows.glow,

            glow_button: glows.button,
            glow_card: glows.card,
            glow_input: glows.input,
            glow_badge: glows.badge,
            glow_modal: glows.modal,
            glow_alert: glows.alert,
            glow_dropdown: glows.dropdown,
            glow_notification: glows.notification,
            glow_switch: glows.switch,
            glow_checkbox: glows.checkbox,
            glow_select: glows.select,
            glow_tooltip: glows.tooltip,
            glow_drawer: glows.drawer,

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

    /// Preset Obsidian Titanium (Monocromatico con accenti grafite).
    pub fn obsidian() -> Self {
        let mut t = Self::shadow();
        t.name = "Obsidian Titanium".to_string();
        t.apply_primary_color(PrimaryColor::Obsidian);
        t
    }

    /// Preset Crimson Phoenix.
    pub fn crimson() -> Self {
        let mut t = Self::shadow();
        t.name = "Crimson Phoenix".to_string();
        t.apply_primary_color(PrimaryColor::Crimson);
        t
    }

    /// Preset Cyber Cyan.
    pub fn cyan() -> Self {
        let mut t = Self::shadow();
        t.name = "Cyber Cyan".to_string();
        t.apply_primary_color(PrimaryColor::Cyan);
        t
    }

    /// Preset Emerald Matrix.
    pub fn emerald() -> Self {
        let mut t = Self::shadow();
        t.name = "Emerald Matrix".to_string();
        t.apply_primary_color(PrimaryColor::Emerald);
        t
    }

    /// Nuovo Preset Nativo: Cartoon Pop / Manga Neo-Brutalist (DS-09).
    pub fn cartoon() -> Self {
        let mut t = Self::shadow();
        t.name = "Cartoon Pop".to_string();
        t.primary_color = PrimaryColor::custom_with_prefix("Cartoon Yellow", "#FFCC00", "cartoon");
        t.radius_scale = RadiusScale::Sharp;
        t.elevation_style = ElevationStyle::HardBlock;
        t.glow_mode = GlowMode::Off;

        t.surface_0 = "#FBFAF3".to_string();
        t.surface_1 = "#FFFFFF".to_string();
        t.surface_2 = "#F4F4F4".to_string();
        t.surface_3 = "#EAE6DF".to_string();

        t.root = "sd-root sd-root--cartoon".to_string();
        t.header = "sd-header sd-header--cartoon".to_string();
        t.border = "border-3 border-black".to_string();
        t.border_accent = "border-4 border-black".to_string();
        t.text_accent = "text-[#FF3333]".to_string();
        t.text_accent_bright = "text-[#FFCC00]".to_string();

        t.button_primary = "sd-btn--primary-cartoon".to_string();
        t.button_secondary = "sd-btn--secondary-cartoon".to_string();
        t.card_base = "sd-card--comic".to_string();
        t.badge_primary = "sd-badge--primary-cartoon".to_string();

        // Livello 2: Configurazione dei componenti
        let mut btn_override = ComponentOverride::new();
        btn_override.base_class = Some("sd-btn--comic".to_string());
        btn_override
            .variant_classes
            .insert("primary".to_string(), "sd-btn--primary-cartoon".to_string());
        btn_override
            .variant_classes
            .insert("secondary".to_string(), "sd-btn--secondary-cartoon".to_string());
        t.components.insert("button".to_string(), btn_override);

        let mut card_override = ComponentOverride::new();
        card_override.base_class = Some("sd-card--comic".to_string());
        t.components.insert("card".to_string(), card_override);

        let mut badge_override = ComponentOverride::new();
        badge_override.base_class = Some("sd-badge--comic".to_string());
        t.components.insert("badge".to_string(), badge_override);

        t.apply_glow_mode(GlowMode::Off);
        t
    }

    /// Applica un colore primario aggiornando solo i token cromatici senza distruggere
    /// nome, superfici, componenti e configurazioni esistenti (RS-06).
    pub fn apply_primary_color(&mut self, color: PrimaryColor) {
        let tokens = Self::color_tokens(&color);
        self.primary_color = color;
        self.sidebar_link_active = tokens.sidebar_link_active;
        self.border_accent = tokens.border_accent;
        self.text_accent = tokens.text_accent;
        self.text_accent_bright = tokens.text_accent_bright;
        self.button_primary = tokens.button_primary;
        self.badge_primary = tokens.badge_primary;
        self.banner = tokens.banner;
        self.apply_glow_mode(self.glow_mode);
    }

    /// Applica una scala globale del raggio di curvatura.
    pub fn apply_radius_scale(&mut self, radius: RadiusScale) {
        self.radius_scale = radius;
    }

    /// Applica uno stile di elevazione globale.
    pub fn apply_elevation_style(&mut self, elevation: ElevationStyle) {
        self.elevation_style = elevation;
    }

    /// Applica una modalità di intensità per il micro-glow, ricalcolando tutti i componenti.
    pub fn apply_glow_mode(&mut self, glow: GlowMode) {
        self.glow_mode = glow;
        let glows = Self::calculate_glows(&self.primary_color, glow);
        self.glow = glows.glow;
        self.glow_button = glows.button;
        self.glow_card = glows.card;
        self.glow_input = glows.input;
        self.glow_badge = glows.badge;
        self.glow_modal = glows.modal;
        self.glow_alert = glows.alert;
        self.glow_dropdown = glows.dropdown;
        self.glow_notification = glows.notification;
        self.glow_switch = glows.switch;
        self.glow_checkbox = glows.checkbox;
        self.glow_select = glows.select;
        self.glow_tooltip = glows.tooltip;
        self.glow_drawer = glows.drawer;
    }

    /// Restituisce la classe di glow configurata per un dato componente (DS-12).
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

    /// Risolve se il glow è attivo per un componente tenendo conto dell'override opzionale.
    pub fn is_component_glow_enabled(&self, component: &str, override_glow: Option<bool>) -> bool {
        match override_glow {
            Some(enabled) => enabled,
            None => !self.component_glow(component).is_empty(),
        }
    }

    // Builder methods fluenti retrocompatibili
    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = name.into();
        self
    }

    pub fn with_primary(mut self, button_primary: impl Into<String>) -> Self {
        self.button_primary = button_primary.into();
        self
    }

    pub fn with_card_base(mut self, card_base: impl Into<String>) -> Self {
        self.card_base = card_base.into();
        self
    }

    pub fn with_glow(mut self, glow: impl Into<String>) -> Self {
        self.glow = glow.into();
        self
    }

    pub fn with_elevation(mut self, elevation: ElevationStyle) -> Self {
        self.elevation_style = elevation;
        self
    }

    pub fn with_component_override(
        mut self,
        component: impl Into<String>,
        component_override: ComponentOverride,
    ) -> Self {
        self.components.insert(component.into(), component_override);
        self
    }

    pub fn with_class_override(
        mut self,
        target_class: impl Into<String>,
        replacement_or_extra: impl Into<String>,
    ) -> Self {
        self.class_overrides
            .insert(target_class.into(), replacement_or_extra.into());
        self
    }

    // Helper per componenti (RS-05)
    #[cfg(feature = "button")]
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
    pub fn alert_variant_classes(&self, variant: AlertVariant) -> &str {
        match variant {
            AlertVariant::Info => &self.alert_info,
            AlertVariant::Success => &self.alert_success,
            AlertVariant::Warning => &self.alert_warning,
            AlertVariant::Danger => &self.alert_danger,
        }
    }

    #[cfg(feature = "alert")]
    pub fn alert_title_classes(&self, variant: AlertVariant) -> &str {
        match variant {
            AlertVariant::Info => &self.alert_title_info,
            AlertVariant::Success => &self.alert_title_success,
            AlertVariant::Warning => &self.alert_title_warning,
            AlertVariant::Danger => &self.alert_title_danger,
        }
    }

    pub fn input_border_class(&self, is_error: bool) -> &str {
        if is_error {
            &self.input_error_border
        } else {
            &self.input_normal_border
        }
    }
}

/// Builder avanzato per costruire temi in modo fluido ed ergonomico.
#[derive(Clone, Debug, PartialEq, Default)]
pub struct ThemeBuilder {
    theme: Theme,
}

impl ThemeBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.theme.name = name.into();
        self
    }

    pub fn primary_color(mut self, color: PrimaryColor) -> Self {
        self.theme.apply_primary_color(color);
        self
    }

    pub fn radius_scale(mut self, radius: RadiusScale) -> Self {
        self.theme.apply_radius_scale(radius);
        self
    }

    pub fn elevation_style(mut self, elevation: ElevationStyle) -> Self {
        self.theme.apply_elevation_style(elevation);
        self
    }

    pub fn glow_mode(mut self, glow: GlowMode) -> Self {
        self.theme.apply_glow_mode(glow);
        self
    }

    pub fn surface_0(mut self, val: impl Into<String>) -> Self {
        let s = val.into();
        if is_valid_hex_color(&s) {
            self.theme.surface_0 = s;
        }
        self
    }

    pub fn surface_1(mut self, val: impl Into<String>) -> Self {
        let s = val.into();
        if is_valid_hex_color(&s) {
            self.theme.surface_1 = s;
        }
        self
    }

    pub fn surface_2(mut self, val: impl Into<String>) -> Self {
        let s = val.into();
        if is_valid_hex_color(&s) {
            self.theme.surface_2 = s;
        }
        self
    }

    pub fn surface_3(mut self, val: impl Into<String>) -> Self {
        let s = val.into();
        if is_valid_hex_color(&s) {
            self.theme.surface_3 = s;
        }
        self
    }

    /// Personalizzazione Livello 2 per uno specifico componente.
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

    /// Personalizzazione Livello 2 per una specifica classe target.
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

    pub fn build(self) -> Theme {
        self.theme
    }
}

/// Provider per iniettare il tema reattivo nel Context Dioxus (RS-07, DS-02, DS-10).
#[component]
pub fn ThemeProvider(#[props(default)] theme: Option<Signal<Theme>>, children: Element) -> Element {
    let owned = use_signal(Theme::default);
    let active_theme = theme.unwrap_or(owned);
    use_context_provider(|| active_theme);

    let theme_val = active_theme.read();
    let prefix = theme_val.primary_color.css_prefix();
    let radius_class = match theme_val.radius_scale {
        RadiusScale::Sharp => "sd-radius-sharp",
        RadiusScale::Medium => "sd-radius-medium",
        RadiusScale::Soft => "sd-radius-soft",
    };
    let glow_class = match theme_val.glow_mode {
        GlowMode::Off => "sd-glow-off",
        GlowMode::Subtle => "sd-glow-subtle",
        GlowMode::Expressive => "sd-glow-expressive",
    };
    let css_vars = theme_val.to_css_variables();

    rsx! {
        div {
            class: "sd-theme-wrapper sd-theme-{prefix} {radius_class} {glow_class}",
            "data-sd-theme": "{prefix}",
            style: "display: contents;",
            document::Style { "{css_vars}" }
            {children}
        }
    }
}

/// Hook per accedere e modificare il tema attivo (RS-07).
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
    fn test_primary_color_custom_classes() {
        let custom = PrimaryColor::custom("Brand", "#ff8800");
        assert_eq!(custom.css_prefix(), "custom");
        assert_eq!(custom.hex(), "#ff8800");

        let themed = Theme::builder().primary_color(custom).build();
        assert_eq!(themed.button_primary, "sd-btn--primary-custom");
    }

    #[test]
    fn test_primary_color_custom_with_prefix_sanitized() {
        let custom = PrimaryColor::custom_with_prefix("Brand", "#ff8800", "My Brand_123!");
        assert_eq!(custom.css_prefix(), "my-brand-123");
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
    fn test_builder_preserves_overrides_when_changing_color() {
        let theme = Theme::builder()
            .override_component("button", |o| o.with_base_class("my-btn"))
            .primary_color(PrimaryColor::Crimson)
            .build();
        assert_eq!(theme.primary_color, PrimaryColor::Crimson);
        assert_eq!(theme.resolve_component_base("button"), Some("my-btn"));
    }

    #[test]
    fn test_cartoon_apply_primary_color_keeps_components() {
        let mut theme = Theme::cartoon();
        theme.apply_primary_color(PrimaryColor::Crimson);
        assert_eq!(theme.primary_color, PrimaryColor::Crimson);
        assert!(theme.components.contains_key("button"));
        assert_eq!(theme.root, "sd-root sd-root--cartoon");
    }

    #[test]
    fn test_vivid_violet_accent() {
        assert_eq!(PrimaryColor::Violet.hex(), "#a855f7");
        assert_eq!(PrimaryColor::Violet.accent_hex(), "#a855f7");
        assert_eq!(PrimaryColor::Violet.shadow_hex(), "#3b0764");
    }

    #[test]
    fn test_glow_modes_different() {
        let mut theme = Theme::shadow();
        theme.apply_glow_mode(GlowMode::Off);
        assert!(theme.glow_button.is_empty());
        assert!(theme.glow_card.is_empty());

        theme.apply_glow_mode(GlowMode::Subtle);
        assert!(!theme.glow_button.is_empty());
        assert!(
            theme.glow_card.is_empty(),
            "Cards should not glow in Subtle mode"
        );

        theme.apply_glow_mode(GlowMode::Expressive);
        assert!(!theme.glow_button.is_empty());
        assert!(
            !theme.glow_card.is_empty(),
            "Cards should glow in Expressive mode"
        );
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
}
