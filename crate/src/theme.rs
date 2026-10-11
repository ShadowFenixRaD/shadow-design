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

/// Palette di colori primari in stile Mantine / Material-UI, con supporto a colori Custom estendibili.
#[derive(Clone, PartialEq, Debug, Default)]
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

    pub fn hex(&self) -> &str {
        match self {
            PrimaryColor::Violet => "#3b0764",
            PrimaryColor::Obsidian => "#ffffff",
            PrimaryColor::Crimson => "#ff1744",
            PrimaryColor::Cyan => "#00f0ff",
            PrimaryColor::Emerald => "#00ff9d",
            PrimaryColor::Custom { hex, .. } => hex.as_str(),
        }
    }

    /// Ricava automaticamente il prefisso o slug per le classi CSS (es. "cartoon", "manga-yellow").
    pub fn css_prefix(&self) -> String {
        match self {
            PrimaryColor::Violet => "violet".to_string(),
            PrimaryColor::Obsidian => "obsidian".to_string(),
            PrimaryColor::Crimson => "crimson".to_string(),
            PrimaryColor::Cyan => "cyan".to_string(),
            PrimaryColor::Emerald => "emerald".to_string(),
            PrimaryColor::Custom { name, css_prefix, .. } => {
                if let Some(prefix) = css_prefix {
                    prefix.clone()
                } else {
                    name.to_lowercase()
                        .chars()
                        .map(|c| if c.is_alphanumeric() { c } else { '-' })
                        .collect::<String>()
                        .trim_matches('-')
                        .to_string()
                }
            }
        }
    }

    pub fn custom(name: impl Into<String>, hex: impl Into<String>) -> Self {
        Self::Custom {
            name: name.into(),
            hex: hex.into(),
            css_prefix: None,
        }
    }

    pub fn custom_with_prefix(
        name: impl Into<String>,
        hex: impl Into<String>,
        css_prefix: impl Into<String>,
    ) -> Self {
        Self::Custom {
            name: name.into(),
            hex: hex.into(),
            css_prefix: Some(css_prefix.into()),
        }
    }
}

/// Scala globale per i raggi di curvatura (Border Radius).
#[derive(Clone, Copy, PartialEq, Debug, Default)]
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
pub enum GlowMode {
    #[default]
    Subtle,
    Expressive,
    Off,
}

/// Stile di elevazione delle ombre (Livello 1): Neon Glow, Hard Block cartoon, Soft o Flat.
#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub enum ElevationStyle {
    #[default]
    NeonGlow,     // Specular neon glow sfumato (Cyberpunk / Ultraviolet)
    HardBlock,    // Ombre piene offset senza blur (Cartoon / Manga Pop / Neo-brutalist)
    SoftElevated, // Ombre material soft
    Flat,         // Senza ombre
}

/// Personalizzazione specifica per un componente (Livello 2 - Component Level).
#[derive(Clone, PartialEq, Debug, Default)]
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

/// Struttura centrale del tema per Shadow Design ispirata a Mantine e Material-UI.
/// Supporta personalizzazione gerarchica su 3 livelli.
#[derive(Clone, PartialEq, Debug)]
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
                _ => "shadow".to_string(),
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

    /// Genera la stringa di CSS Custom Properties da iniettare dinamicamente a runtime.
    pub fn to_css_variables(&self) -> String {
        format!(
            ":root {{ \
                --sd-primary-color: {primary}; \
                --sd-surface-0: {s0}; \
                --sd-surface-1: {s1}; \
                --sd-surface-2: {s2}; \
                --sd-surface-3: {s3}; \
            }}",
            primary = self.primary_color.hex(),
            s0 = self.surface_0,
            s1 = self.surface_1,
            s2 = self.surface_2,
            s3 = self.surface_3,
        )
    }

    /// Calcola i valori di glow per ogni componente in base al colore primario e alla modalità.
    fn calculate_glows(
        color: &PrimaryColor,
        mode: GlowMode,
    ) -> (
        String,
        String,
        String,
        String,
        String,
        String,
        String,
        String,
        String,
        String,
        String,
    ) {
        if mode == GlowMode::Off {
            return (
                String::new(),
                String::new(),
                String::new(),
                String::new(),
                String::new(),
                String::new(),
                String::new(),
                String::new(),
                String::new(),
                String::new(),
                String::new(),
            );
        }

        match color {
            PrimaryColor::Violet => (
                "glow-violet".to_string(),
                "sd-btn--glow".to_string(),
                "sd-card--hover-glow".to_string(),
                "".to_string(),
                "sd-badge--glow".to_string(),
                "sd-modal-dialog--glow".to_string(),
                "sd-alert--glow-info".to_string(),
                "sd-dropdown-menu--glow".to_string(),
                "sd-toast--glow-info".to_string(),
                "sd-switch__track--glow".to_string(),
                "sd-checkbox__box--glow".to_string(),
            ),
            PrimaryColor::Obsidian => (
                "".to_string(),
                "sd-btn--glow-obsidian".to_string(),
                "sd-card--hover-glow".to_string(),
                "".to_string(),
                "sd-badge--glow".to_string(),
                "sd-modal-dialog--glow".to_string(),
                "sd-alert--glow-info".to_string(),
                "sd-dropdown-menu--glow".to_string(),
                "sd-toast--glow-info".to_string(),
                "sd-switch__track--glow-obsidian".to_string(),
                "sd-checkbox__box--glow".to_string(),
            ),
            PrimaryColor::Crimson => (
                "glow-crimson".to_string(),
                "sd-btn--glow-crimson".to_string(),
                "sd-card--hover-glow".to_string(),
                "".to_string(),
                "sd-badge--glow".to_string(),
                "sd-modal-dialog--glow".to_string(),
                "sd-alert--glow-danger".to_string(),
                "sd-dropdown-menu--glow".to_string(),
                "sd-toast--glow-danger".to_string(),
                "sd-switch__track--glow-crimson".to_string(),
                "sd-checkbox__box--glow".to_string(),
            ),
            PrimaryColor::Cyan => (
                "glow-cyan".to_string(),
                "sd-btn--glow-cyan".to_string(),
                "sd-card--hover-glow".to_string(),
                "".to_string(),
                "sd-badge--glow".to_string(),
                "sd-modal-dialog--glow".to_string(),
                "sd-alert--glow-info".to_string(),
                "sd-dropdown-menu--glow".to_string(),
                "sd-toast--glow-info".to_string(),
                "sd-switch__track--glow-cyan".to_string(),
                "sd-checkbox__box--glow".to_string(),
            ),
            PrimaryColor::Emerald => (
                "glow-emerald".to_string(),
                "sd-btn--glow-emerald".to_string(),
                "sd-card--hover-glow".to_string(),
                "".to_string(),
                "sd-badge--glow".to_string(),
                "sd-modal-dialog--glow".to_string(),
                "sd-alert--glow-success".to_string(),
                "sd-dropdown-menu--glow".to_string(),
                "sd-toast--glow-success".to_string(),
                "sd-switch__track--glow-emerald".to_string(),
                "sd-checkbox__box--glow".to_string(),
            ),
            PrimaryColor::Custom { .. } => {
                let prefix = color.css_prefix();
                (
                    format!("glow-{}", prefix),
                    format!("sd-btn--glow-{}", prefix),
                    format!("sd-card--glow-{}", prefix),
                    "".to_string(),
                    format!("sd-badge--glow-{}", prefix),
                    format!("sd-modal-dialog--glow-{}", prefix),
                    "sd-alert--glow-info".to_string(),
                    "sd-dropdown-menu--glow".to_string(),
                    "sd-toast--glow-info".to_string(),
                    format!("sd-switch__track--glow-{}", prefix),
                    "sd-checkbox__box--glow".to_string(),
                )
            }
        }
    }

    /// Tema di default Shadow Design (Ultraviolet Obsidian: Nero profondo con accenti viola).
    pub fn shadow() -> Self {
        let (g, gb, gc, gi, gbd, gm, ga, gd, gn, gs, gcb) =
            Self::calculate_glows(&PrimaryColor::Violet, GlowMode::Subtle);

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
            glow: g,

            glow_button: gb,
            glow_card: gc,
            glow_input: gi,
            glow_badge: gbd,
            glow_modal: gm,
            glow_alert: ga,
            glow_dropdown: gd,
            glow_notification: gn,
            glow_switch: gs,
            glow_checkbox: gcb,

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
            input_normal_border: "".to_string(),
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
        t.primary_color = PrimaryColor::Obsidian;
        t.sidebar_link_active = "sd-sidebar-link--active-obsidian".to_string();
        t.border_accent = "border-zinc-500/40".to_string();
        t.text_accent = "text-zinc-300".to_string();
        t.text_accent_bright = "text-white".to_string();
        t.button_primary = "sd-btn--primary-obsidian".to_string();
        t.badge_primary = "sd-badge--default".to_string();
        t.banner = "sd-banner sd-banner--obsidian".to_string();

        let (g, gb, gc, gi, gbd, gm, ga, gd, gn, gs, gcb) =
            Self::calculate_glows(&PrimaryColor::Obsidian, t.glow_mode);
        t.glow = g;
        t.glow_button = gb;
        t.glow_card = gc;
        t.glow_input = gi;
        t.glow_badge = gbd;
        t.glow_modal = gm;
        t.glow_alert = ga;
        t.glow_dropdown = gd;
        t.glow_notification = gn;
        t.glow_switch = gs;
        t.glow_checkbox = gcb;
        t
    }

    /// Preset Crimson Phoenix.
    pub fn crimson() -> Self {
        let mut t = Self::shadow();
        t.name = "Crimson Phoenix".to_string();
        t.primary_color = PrimaryColor::Crimson;
        t.sidebar_link_active = "sd-sidebar-link--active-crimson".to_string();
        t.border_accent = "border-rose-500/40".to_string();
        t.text_accent = "text-rose-400".to_string();
        t.text_accent_bright = "text-rose-300".to_string();
        t.button_primary = "sd-btn--primary-crimson".to_string();
        t.badge_primary = "sd-badge--danger".to_string();
        t.banner = "sd-banner sd-banner--crimson".to_string();

        let (g, gb, gc, gi, gbd, gm, ga, gd, gn, gs, gcb) =
            Self::calculate_glows(&PrimaryColor::Crimson, t.glow_mode);
        t.glow = g;
        t.glow_button = gb;
        t.glow_card = gc;
        t.glow_input = gi;
        t.glow_badge = gbd;
        t.glow_modal = gm;
        t.glow_alert = ga;
        t.glow_dropdown = gd;
        t.glow_notification = gn;
        t.glow_switch = gs;
        t.glow_checkbox = gcb;
        t
    }

    /// Preset Cyber Cyan.
    pub fn cyan() -> Self {
        let mut t = Self::shadow();
        t.name = "Cyber Cyan".to_string();
        t.primary_color = PrimaryColor::Cyan;
        t.sidebar_link_active = "sd-sidebar-link--active-cyan".to_string();
        t.border_accent = "border-cyan-500/40".to_string();
        t.text_accent = "text-cyan-400".to_string();
        t.text_accent_bright = "text-cyan-300".to_string();
        t.button_primary = "sd-btn--primary-cyan".to_string();
        t.badge_primary = "sd-badge--primary".to_string();
        t.banner = "sd-banner sd-banner--cyan".to_string();

        let (g, gb, gc, gi, gbd, gm, ga, gd, gn, gs, gcb) =
            Self::calculate_glows(&PrimaryColor::Cyan, t.glow_mode);
        t.glow = g;
        t.glow_button = gb;
        t.glow_card = gc;
        t.glow_input = gi;
        t.glow_badge = gbd;
        t.glow_modal = gm;
        t.glow_alert = ga;
        t.glow_dropdown = gd;
        t.glow_notification = gn;
        t.glow_switch = gs;
        t.glow_checkbox = gcb;
        t
    }

    /// Preset Emerald Matrix.
    pub fn emerald() -> Self {
        let mut t = Self::shadow();
        t.name = "Emerald Matrix".to_string();
        t.primary_color = PrimaryColor::Emerald;
        t.sidebar_link_active = "sd-sidebar-link--active-emerald".to_string();
        t.border_accent = "border-emerald-500/40".to_string();
        t.text_accent = "text-emerald-400".to_string();
        t.text_accent_bright = "text-emerald-300".to_string();
        t.button_primary = "sd-btn--primary-emerald".to_string();
        t.badge_primary = "sd-badge--success".to_string();
        t.banner = "sd-banner sd-banner--emerald".to_string();

        let (g, gb, gc, gi, gbd, gm, ga, gd, gn, gs, gcb) =
            Self::calculate_glows(&PrimaryColor::Emerald, t.glow_mode);
        t.glow = g;
        t.glow_button = gb;
        t.glow_card = gc;
        t.glow_input = gi;
        t.glow_badge = gbd;
        t.glow_modal = gm;
        t.glow_alert = ga;
        t.glow_dropdown = gd;
        t.glow_notification = gn;
        t.glow_switch = gs;
        t.glow_checkbox = gcb;
        t
    }

    /// Nuovo Preset Nativo: Cartoon Pop / Manga Neo-Brutalist.
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

        t.root = "sd-root--cartoon".to_string();
        t.header = "sd-header--cartoon".to_string();
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

        t
    }

    /// Applica un colore primario aggiornando tutti i token cromatici associati.
    pub fn apply_primary_color(&mut self, color: PrimaryColor) {
        let current_radius = self.radius_scale;
        let current_elevation = self.elevation_style;
        let current_glow = self.glow_mode;
        let mut updated = match &color {
            PrimaryColor::Violet => Self::shadow(),
            PrimaryColor::Obsidian => Self::obsidian(),
            PrimaryColor::Crimson => Self::crimson(),
            PrimaryColor::Cyan => Self::cyan(),
            PrimaryColor::Emerald => Self::emerald(),
            PrimaryColor::Custom { .. } => {
                let mut custom_t = Self::shadow();
                custom_t.name = color.name().to_string();
                custom_t.primary_color = color.clone();
                let prefix = color.css_prefix();
                custom_t.button_primary = format!("sd-btn--primary-{}", prefix);
                custom_t
            }
        };
        updated.radius_scale = current_radius;
        updated.elevation_style = current_elevation;
        updated.apply_glow_mode(current_glow);
        *self = updated;
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
        let (g, gb, gc, gi, gbd, gm, ga, gd, gn, gs, gcb) =
            Self::calculate_glows(&self.primary_color, glow);
        self.glow = g;
        self.glow_button = gb;
        self.glow_card = gc;
        self.glow_input = gi;
        self.glow_badge = gbd;
        self.glow_modal = gm;
        self.glow_alert = ga;
        self.glow_dropdown = gd;
        self.glow_notification = gn;
        self.glow_switch = gs;
        self.glow_checkbox = gcb;
    }

    /// Restituisce la classe di glow configurata per un dato componente.
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

    // Helper per componenti
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
}

/// Builder avanzato per costruire temi in modo fluido ed ergonomico.
#[derive(Clone, Default)]
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
        self.theme.surface_0 = val.into();
        self
    }

    pub fn surface_1(mut self, val: impl Into<String>) -> Self {
        self.theme.surface_1 = val.into();
        self
    }

    pub fn surface_2(mut self, val: impl Into<String>) -> Self {
        self.theme.surface_2 = val.into();
        self
    }

    pub fn surface_3(mut self, val: impl Into<String>) -> Self {
        self.theme.surface_3 = val.into();
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

/// Provider per iniettare il tema reattivo nel Context Dioxus.
#[component]
pub fn ThemeProvider(#[props(default)] theme: Option<Signal<Theme>>, children: Element) -> Element {
    let current = theme.unwrap_or_else(|| use_signal(Theme::default));
    use_context_provider(|| current);
    rsx! { {children} }
}

/// Hook per accedere e modificare il tema attivo.
pub fn use_theme() -> Signal<Theme> {
    match try_use_context::<Signal<Theme>>() {
        Some(s) => s,
        None => use_context_provider(|| Signal::new(Theme::default())),
    }
}
