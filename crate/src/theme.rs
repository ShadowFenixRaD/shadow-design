use dioxus::prelude::*;

#[cfg(feature = "alert")]
use crate::alert::AlertVariant;
#[cfg(feature = "badge")]
use crate::badge::BadgeVariant;
#[cfg(feature = "button")]
use crate::button::ButtonVariant;
#[cfg(feature = "card")]
use crate::card::CardShadow;

/// Palette di colori primari in stile Mantine.
#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub enum PrimaryColor {
    #[default]
    Violet,
    Obsidian,
    Crimson,
    Cyan,
    Emerald,
}

impl PrimaryColor {
    pub fn name(&self) -> &'static str {
        match self {
            PrimaryColor::Violet => "Violet",
            PrimaryColor::Obsidian => "Obsidian",
            PrimaryColor::Crimson => "Crimson",
            PrimaryColor::Cyan => "Cyan",
            PrimaryColor::Emerald => "Emerald",
        }
    }

    pub fn hex(&self) -> &'static str {
        match self {
            PrimaryColor::Violet => "#3b0764",
            PrimaryColor::Obsidian => "#ffffff",
            PrimaryColor::Crimson => "#ff1744",
            PrimaryColor::Cyan => "#00f0ff",
            PrimaryColor::Emerald => "#00ff9d",
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

/// Intensità del micro-glow ottico.
#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub enum GlowMode {
    #[default]
    Subtle,
    Expressive,
    Off,
}

/// Struttura centrale del tema per Shadow Design ispirata a Mantine / Craft UI.
#[derive(Clone, PartialEq, Debug)]
pub struct Theme {
    pub name: String,
    pub primary_color: PrimaryColor,
    pub radius_scale: RadiusScale,
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

    // Accenti e Bordi Hairline
    pub border: String,
    pub border_subtle: String,
    pub border_accent: String,
    pub text_accent: String,
    pub text_accent_bright: String,
    pub glow: String,

    // Glow e Ombre Specifiche per Componente (Personalizzabili)
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

    // Componente Button
    pub button_primary: String,
    pub button_secondary: String,
    pub button_outline: String,
    pub button_ghost: String,
    pub button_danger: String,
    pub button_success: String,

    // Componente Card
    pub card_base: String,
    pub card_header: String,
    pub card_footer: String,
    pub card_shadow_none: String,
    pub card_shadow_sm: String,
    pub card_shadow_md: String,
    pub card_shadow_lg: String,
    pub card_shadow_xl: String,

    // Componente TextInput
    pub input_base: String,
    pub input_normal_border: String,
    pub input_error_border: String,

    // Componente Badge
    pub badge_default: String,
    pub badge_primary: String,
    pub badge_success: String,
    pub badge_warning: String,
    pub badge_danger: String,
    pub badge_outline: String,

    // Componente Navbar
    pub navbar_base: String,

    // Componente Modal
    pub modal_backdrop: String,
    pub modal_dialog: String,
    pub modal_header: String,
    pub modal_close: String,

    // Componente Alert
    pub alert_info: String,
    pub alert_success: String,
    pub alert_warning: String,
    pub alert_danger: String,
    pub alert_title_info: String,
    pub alert_title_success: String,
    pub alert_title_warning: String,
    pub alert_title_danger: String,
}

impl Default for Theme {
    fn default() -> Self {
        Self::shadow()
    }
}

impl Theme {
    /// Calcola i valori di glow per ogni componente in base al colore primario e alla modalità.
    fn calculate_glows(
        color: PrimaryColor,
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
        }
    }

    /// Tema di default Shadow Design (Ultraviolet Obsidian: Nero profondo con accenti viola e specular highlights).
    pub fn shadow() -> Self {
        let (g, gb, gc, gi, gbd, gm, ga, gd, gn, gs, gcb) =
            Self::calculate_glows(PrimaryColor::Violet, GlowMode::Subtle);

        Self {
            name: "Ultraviolet Shadow".to_string(),
            primary_color: PrimaryColor::Violet,
            radius_scale: RadiusScale::Medium,
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

            border: "border-white/[0.08]".to_string(),
            border_subtle: "border-white/[0.05]".to_string(),
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
        }
    }

    /// Preset Obsidian Titanium (Monocromatico rigoroso con accenti grafite e metallo).
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
            Self::calculate_glows(PrimaryColor::Obsidian, t.glow_mode);
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

    /// Preset Crimson Phoenix (Nero profondo con fiamme cremisi fenice).
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
            Self::calculate_glows(PrimaryColor::Crimson, t.glow_mode);
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

    /// Preset Cyber Cyan (Nero profondo con accenti ciano elettrico ad alto contrasto).
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
            Self::calculate_glows(PrimaryColor::Cyan, t.glow_mode);
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

    /// Preset Emerald Matrix (Nero profondo con verde smeraldo ad altissima nitidezza).
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
            Self::calculate_glows(PrimaryColor::Emerald, t.glow_mode);
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

    /// Applica un colore primario aggiornando tutti i token cromatici associati.
    pub fn apply_primary_color(&mut self, color: PrimaryColor) {
        let current_radius = self.radius_scale;
        let current_glow = self.glow_mode;
        let mut updated = match color {
            PrimaryColor::Violet => Self::shadow(),
            PrimaryColor::Obsidian => Self::obsidian(),
            PrimaryColor::Crimson => Self::crimson(),
            PrimaryColor::Cyan => Self::cyan(),
            PrimaryColor::Emerald => Self::emerald(),
        };
        updated.radius_scale = current_radius;
        updated.apply_glow_mode(current_glow);
        *self = updated;
    }

    /// Applica una scala globale del raggio di curvatura.
    pub fn apply_radius_scale(&mut self, radius: RadiusScale) {
        self.radius_scale = radius;
    }

    /// Applica una modalità di intensità per il micro-glow, ricalcolando tutti i componenti.
    pub fn apply_glow_mode(&mut self, glow: GlowMode) {
        self.glow_mode = glow;
        let (g, gb, gc, gi, gbd, gm, ga, gd, gn, gs, gcb) =
            Self::calculate_glows(self.primary_color, glow);
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

    // Builder methods
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

    pub fn with_component_glow(mut self, component: &str, glow: impl Into<String>) -> Self {
        let val = glow.into();
        match component {
            "button" => self.glow_button = val,
            "card" => self.glow_card = val,
            "input" => self.glow_input = val,
            "badge" => self.glow_badge = val,
            "modal" => self.glow_modal = val,
            "alert" => self.glow_alert = val,
            "dropdown" => self.glow_dropdown = val,
            "notification" => self.glow_notification = val,
            "switch" => self.glow_switch = val,
            "checkbox" => self.glow_checkbox = val,
            _ => self.glow = val,
        }
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
