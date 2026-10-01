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
            PrimaryColor::Violet => "#9333ea",
            PrimaryColor::Obsidian => "#71717a",
            PrimaryColor::Crimson => "#e11d48",
            PrimaryColor::Cyan => "#06b6d4",
            PrimaryColor::Emerald => "#10b981",
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
            RadiusScale::Sharp => "rounded-[2px]",
            RadiusScale::Medium => "rounded-md",
            RadiusScale::Soft => "rounded-lg",
        }
    }

    pub fn md(&self) -> &'static str {
        match self {
            RadiusScale::Sharp => "rounded-[3px]",
            RadiusScale::Medium => "rounded-lg",
            RadiusScale::Soft => "rounded-xl",
        }
    }

    pub fn lg(&self) -> &'static str {
        match self {
            RadiusScale::Sharp => "rounded-[4px]",
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
                "shadow-none".to_string(),
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

        let is_exp = mode == GlowMode::Expressive;

        match color {
            PrimaryColor::Violet => {
                if is_exp {
                    (
                        "shadow-[0_0_30px_rgba(147,51,234,0.35)]".to_string(),
                        "shadow-[0_0_24px_rgba(147,51,234,0.5)]".to_string(),
                        "shadow-[0_0_35px_rgba(147,51,234,0.28)]".to_string(),
                        "shadow-[0_0_20px_rgba(147,51,234,0.35)]".to_string(),
                        "shadow-[0_0_15px_rgba(147,51,234,0.4)]".to_string(),
                        "shadow-[0_0_60px_rgba(147,51,234,0.4)]".to_string(),
                        "shadow-[0_0_25px_rgba(147,51,234,0.3)]".to_string(),
                        "shadow-[0_0_30px_rgba(147,51,234,0.35)]".to_string(),
                        "shadow-[0_0_28px_rgba(147,51,234,0.35)]".to_string(),
                        "shadow-[0_0_16px_rgba(147,51,234,0.5)]".to_string(),
                        "shadow-[0_0_14px_rgba(147,51,234,0.5)]".to_string(),
                    )
                } else {
                    (
                        "shadow-[0_0_18px_rgba(147,51,234,0.2)]".to_string(),
                        "shadow-[0_0_16px_rgba(147,51,234,0.35)]".to_string(),
                        "shadow-[0_0_22px_rgba(147,51,234,0.16)]".to_string(),
                        "shadow-[0_0_14px_rgba(147,51,234,0.22)]".to_string(),
                        "shadow-[0_0_10px_rgba(147,51,234,0.25)]".to_string(),
                        "shadow-[0_0_40px_rgba(147,51,234,0.25)]".to_string(),
                        "shadow-[0_0_18px_rgba(147,51,234,0.18)]".to_string(),
                        "shadow-[0_0_22px_rgba(147,51,234,0.22)]".to_string(),
                        "shadow-[0_0_20px_rgba(147,51,234,0.22)]".to_string(),
                        "shadow-[0_0_12px_rgba(147,51,234,0.35)]".to_string(),
                        "shadow-[0_0_10px_rgba(147,51,234,0.35)]".to_string(),
                    )
                }
            }
            PrimaryColor::Obsidian => {
                if is_exp {
                    (
                        "shadow-[0_0_25px_rgba(255,255,255,0.15)]".to_string(),
                        "shadow-[0_0_18px_rgba(255,255,255,0.2)]".to_string(),
                        "shadow-[0_0_25px_rgba(255,255,255,0.12)]".to_string(),
                        "shadow-[0_0_15px_rgba(255,255,255,0.18)]".to_string(),
                        "shadow-[0_0_10px_rgba(255,255,255,0.2)]".to_string(),
                        "shadow-[0_0_45px_rgba(255,255,255,0.15)]".to_string(),
                        "shadow-[0_0_20px_rgba(255,255,255,0.1)]".to_string(),
                        "shadow-[0_0_20px_rgba(255,255,255,0.15)]".to_string(),
                        "shadow-[0_0_20px_rgba(255,255,255,0.15)]".to_string(),
                        "shadow-[0_0_12px_rgba(255,255,255,0.2)]".to_string(),
                        "shadow-[0_0_10px_rgba(255,255,255,0.2)]".to_string(),
                    )
                } else {
                    (
                        "shadow-[0_0_15px_rgba(255,255,255,0.06)]".to_string(),
                        "shadow-[0_0_12px_rgba(255,255,255,0.1)]".to_string(),
                        "shadow-[0_0_16px_rgba(255,255,255,0.06)]".to_string(),
                        "shadow-[0_0_10px_rgba(255,255,255,0.08)]".to_string(),
                        "shadow-[0_0_8px_rgba(255,255,255,0.1)]".to_string(),
                        "shadow-[0_0_30px_rgba(255,255,255,0.08)]".to_string(),
                        "shadow-[0_0_12px_rgba(255,255,255,0.06)]".to_string(),
                        "shadow-[0_0_14px_rgba(255,255,255,0.08)]".to_string(),
                        "shadow-[0_0_12px_rgba(255,255,255,0.08)]".to_string(),
                        "shadow-[0_0_8px_rgba(255,255,255,0.1)]".to_string(),
                        "shadow-[0_0_7px_rgba(255,255,255,0.1)]".to_string(),
                    )
                }
            }
            PrimaryColor::Crimson => {
                let (r, a1, a2) = (
                    "244,63,94",
                    if is_exp { "0.35" } else { "0.2" },
                    if is_exp { "0.5" } else { "0.35" },
                );
                (
                    format!("shadow-[0_0_20px_rgba({},{})]", r, a1),
                    format!("shadow-[0_0_18px_rgba({},{})]", r, a2),
                    format!("shadow-[0_0_25px_rgba({},{})]", r, a1),
                    format!("shadow-[0_0_15px_rgba({},{})]", r, a1),
                    format!("shadow-[0_0_12px_rgba({},{})]", r, a2),
                    format!("shadow-[0_0_45px_rgba({},{})]", r, a1),
                    format!("shadow-[0_0_20px_rgba({},{})]", r, a1),
                    format!("shadow-[0_0_24px_rgba({},{})]", r, a1),
                    format!("shadow-[0_0_22px_rgba({},{})]", r, a1),
                    format!("shadow-[0_0_12px_rgba({},{})]", r, a2),
                    format!("shadow-[0_0_10px_rgba({},{})]", r, a2),
                )
            }
            PrimaryColor::Cyan => {
                let (r, a1, a2) = (
                    "6,182,212",
                    if is_exp { "0.35" } else { "0.2" },
                    if is_exp { "0.5" } else { "0.35" },
                );
                (
                    format!("shadow-[0_0_20px_rgba({},{})]", r, a1),
                    format!("shadow-[0_0_18px_rgba({},{})]", r, a2),
                    format!("shadow-[0_0_25px_rgba({},{})]", r, a1),
                    format!("shadow-[0_0_15px_rgba({},{})]", r, a1),
                    format!("shadow-[0_0_12px_rgba({},{})]", r, a2),
                    format!("shadow-[0_0_45px_rgba({},{})]", r, a1),
                    format!("shadow-[0_0_20px_rgba({},{})]", r, a1),
                    format!("shadow-[0_0_24px_rgba({},{})]", r, a1),
                    format!("shadow-[0_0_22px_rgba({},{})]", r, a1),
                    format!("shadow-[0_0_12px_rgba({},{})]", r, a2),
                    format!("shadow-[0_0_10px_rgba({},{})]", r, a2),
                )
            }
            PrimaryColor::Emerald => {
                let (r, a1, a2) = (
                    "16,185,129",
                    if is_exp { "0.35" } else { "0.2" },
                    if is_exp { "0.5" } else { "0.35" },
                );
                (
                    format!("shadow-[0_0_20px_rgba({},{})]", r, a1),
                    format!("shadow-[0_0_18px_rgba({},{})]", r, a2),
                    format!("shadow-[0_0_25px_rgba({},{})]", r, a1),
                    format!("shadow-[0_0_15px_rgba({},{})]", r, a1),
                    format!("shadow-[0_0_12px_rgba({},{})]", r, a2),
                    format!("shadow-[0_0_45px_rgba({},{})]", r, a1),
                    format!("shadow-[0_0_20px_rgba({},{})]", r, a1),
                    format!("shadow-[0_0_24px_rgba({},{})]", r, a1),
                    format!("shadow-[0_0_22px_rgba({},{})]", r, a1),
                    format!("shadow-[0_0_12px_rgba({},{})]", r, a2),
                    format!("shadow-[0_0_10px_rgba({},{})]", r, a2),
                )
            }
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

            surface_0: "bg-[#030305]".to_string(),
            surface_1: "bg-[#09090d]".to_string(),
            surface_2: "bg-[#111118]".to_string(),
            surface_3: "bg-[#181824]".to_string(),

            root: "min-h-screen flex flex-col bg-[#030305] text-zinc-100 selection:bg-purple-900/50 selection:text-white".to_string(),
            header: "border-b border-white/[0.08] bg-[#030305]/85 backdrop-blur-md shadow-[0_4px_30px_rgba(0,0,0,0.8)] specular-top".to_string(),
            sidebar: "border-r border-white/[0.06] bg-[#030305]".to_string(),
            sidebar_link: "flex items-center gap-2.5 px-3 py-2 rounded-lg text-xs font-medium text-zinc-400 hover:text-zinc-100 hover:bg-white/[0.04] transition-all cursor-pointer select-none".to_string(),
            sidebar_link_active: "bg-purple-950/40 text-purple-200 font-semibold border border-purple-500/30 shadow-[0_0_15px_rgba(147,51,234,0.15)]".to_string(),
            content: "bg-[#030305] text-zinc-100".to_string(),
            surface: "bg-[#09090d] border border-white/[0.08] shadow-[inset_0_1px_0_0_rgba(255,255,255,0.06)]".to_string(),
            panel: "bg-[#111118] border border-white/[0.1] shadow-2xl shadow-black/80".to_string(),
            banner: "mb-6 p-4 rounded-xl bg-[#09090d] border border-purple-500/25 shadow-lg shadow-purple-950/20".to_string(),

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

            button_primary: "bg-purple-600 hover:bg-purple-500 active:bg-purple-700 text-white shadow-md shadow-purple-950/50 border border-purple-400/40 hover:shadow-[0_0_15px_rgba(147,51,234,0.35)] shadow-[inset_0_1px_0_0_rgba(255,255,255,0.2)] active:scale-[0.98]".to_string(),
            button_secondary: "bg-[#121218] hover:bg-[#1a1a24] active:bg-[#0e0e14] text-zinc-200 border border-white/[0.08] hover:border-white/20 hover:text-white shadow-sm shadow-[inset_0_1px_0_0_rgba(255,255,255,0.05)] active:scale-[0.98]".to_string(),
            button_outline: "bg-transparent hover:bg-purple-950/30 text-purple-300 border border-purple-500/30 hover:border-purple-400/60 hover:text-purple-100 active:scale-[0.98]".to_string(),
            button_ghost: "bg-transparent hover:bg-white/[0.06] text-zinc-300 hover:text-white active:scale-[0.98]".to_string(),
            button_danger: "bg-rose-600 hover:bg-rose-500 active:bg-rose-700 text-white shadow-md shadow-rose-950/50 border border-rose-400/40 shadow-[inset_0_1px_0_0_rgba(255,255,255,0.2)] active:scale-[0.98]".to_string(),
            button_success: "bg-emerald-600 hover:bg-emerald-500 active:bg-emerald-700 text-white shadow-md shadow-emerald-950/50 border border-emerald-400/40 shadow-[inset_0_1px_0_0_rgba(255,255,255,0.2)] active:scale-[0.98]".to_string(),

            card_base: "bg-[#09090d] border border-white/[0.08] text-zinc-100 rounded-xl hover:border-white/15 transition-all shadow-[inset_0_1px_0_0_rgba(255,255,255,0.06)]".to_string(),
            card_header: "px-6 py-4 border-b border-white/[0.06] bg-[#0c0c12]/80".to_string(),
            card_footer: "px-6 py-3 bg-[#07070a]/90 border-t border-white/[0.06] text-xs text-zinc-400".to_string(),
            card_shadow_none: "shadow-none".to_string(),
            card_shadow_sm: "shadow-sm shadow-black".to_string(),
            card_shadow_md: "shadow-lg shadow-black/80 hover:shadow-purple-950/30 transition-shadow".to_string(),
            card_shadow_lg: "shadow-xl shadow-black/90 hover:shadow-purple-950/50 transition-shadow".to_string(),
            card_shadow_xl: "shadow-2xl shadow-purple-950/40 border-purple-500/20".to_string(),

            input_base: "px-3.5 py-2 bg-[#060608] text-zinc-100 rounded-lg border focus:outline-none focus:ring-2 transition-all placeholder:text-zinc-600 disabled:opacity-50 disabled:cursor-not-allowed text-xs sm:text-sm shadow-[inset_0_1px_2px_rgba(0,0,0,0.5)]".to_string(),
            input_normal_border: "border-white/[0.08] focus:border-purple-500/60 focus:ring-purple-600/20 hover:border-white/15".to_string(),
            input_error_border: "border-rose-500/70 focus:border-rose-400 focus:ring-rose-500/20".to_string(),

            badge_default: "bg-[#14141c] text-zinc-300 border-white/[0.08]".to_string(),
            badge_primary: "bg-purple-950/50 text-purple-300 border-purple-500/30 shadow-[0_0_10px_rgba(147,51,234,0.15)]".to_string(),
            badge_success: "bg-emerald-950/50 text-emerald-300 border-emerald-500/30".to_string(),
            badge_warning: "bg-amber-950/50 text-amber-300 border-amber-500/30".to_string(),
            badge_danger: "bg-rose-950/50 text-rose-300 border-rose-500/30".to_string(),
            badge_outline: "bg-transparent text-purple-300 border-purple-500/30".to_string(),

            navbar_base: "w-full bg-[#030305]/90 backdrop-blur-md border-b border-white/[0.08] shadow-[0_4px_25px_rgba(0,0,0,0.9)] px-6 py-3.5 sticky top-0 z-30".to_string(),

            modal_backdrop: "fixed inset-0 z-[80] flex items-center justify-center p-4 bg-black/80 backdrop-blur-md transition-all".to_string(),
            modal_dialog: "bg-[#0d0d14] border border-white/15 rounded-2xl max-w-lg w-full shadow-2xl shadow-purple-950/50 overflow-hidden specular-top".to_string(),
            modal_header: "flex items-center justify-between px-6 py-4 border-b border-white/[0.08] bg-[#12121c]".to_string(),
            modal_close: "text-zinc-400 hover:text-white text-sm p-1.5 rounded-lg hover:bg-white/[0.08] transition-colors cursor-pointer".to_string(),

            alert_info: "bg-[#0e0e1a] text-purple-100 border-purple-500/30 border-l-4 border-l-purple-500 shadow-lg shadow-purple-950/20".to_string(),
            alert_success: "bg-[#0b1410] text-emerald-100 border-emerald-500/30 border-l-4 border-l-emerald-500 shadow-lg shadow-black/40".to_string(),
            alert_warning: "bg-[#141208] text-amber-100 border-amber-500/30 border-l-4 border-l-amber-500 shadow-lg shadow-black/40".to_string(),
            alert_danger: "bg-[#170a0e] text-rose-100 border-rose-500/30 border-l-4 border-l-rose-500 shadow-lg shadow-black/40".to_string(),
            alert_title_info: "text-purple-300 font-semibold".to_string(),
            alert_title_success: "text-emerald-300 font-semibold".to_string(),
            alert_title_warning: "text-amber-300 font-semibold".to_string(),
            alert_title_danger: "text-rose-300 font-semibold".to_string(),
        }
    }

    /// Preset Obsidian Titanium (Monocromatico rigoroso con accenti grafite e metallo).
    pub fn obsidian() -> Self {
        let mut t = Self::shadow();
        t.name = "Obsidian Titanium".to_string();
        t.primary_color = PrimaryColor::Obsidian;
        t.sidebar_link_active =
            "bg-zinc-800 text-white font-semibold border border-zinc-600 shadow-md".to_string();
        t.border_accent = "border-zinc-500/40".to_string();
        t.text_accent = "text-zinc-300".to_string();
        t.text_accent_bright = "text-white".to_string();
        t.button_primary = "bg-zinc-100 hover:bg-white text-zinc-950 font-semibold shadow-md border border-white active:scale-[0.98]".to_string();
        t.badge_primary = "bg-zinc-800 text-zinc-100 border-zinc-600".to_string();
        t.input_normal_border =
            "border-white/[0.08] focus:border-zinc-400 focus:ring-zinc-400/20".to_string();
        t.banner = "mb-6 p-4 rounded-xl bg-[#09090d] border border-zinc-700 shadow-md".to_string();

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
        t.sidebar_link_active = "bg-rose-950/40 text-rose-200 font-semibold border border-rose-500/30 shadow-[0_0_15px_rgba(225,29,72,0.15)]".to_string();
        t.border_accent = "border-rose-500/40".to_string();
        t.text_accent = "text-rose-400".to_string();
        t.text_accent_bright = "text-rose-300".to_string();
        t.button_primary = "bg-rose-600 hover:bg-rose-500 active:bg-rose-700 text-white shadow-md shadow-rose-950/50 border border-rose-400/40 shadow-[inset_0_1px_0_0_rgba(255,255,255,0.2)] active:scale-[0.98]".to_string();
        t.badge_primary = "bg-rose-950/50 text-rose-300 border-rose-500/30".to_string();
        t.input_normal_border =
            "border-white/[0.08] focus:border-rose-500/60 focus:ring-rose-600/20".to_string();
        t.banner = "mb-6 p-4 rounded-xl bg-[#09090d] border border-rose-500/30 shadow-lg shadow-rose-950/20".to_string();

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
        t.sidebar_link_active = "bg-cyan-950/40 text-cyan-200 font-semibold border border-cyan-500/30 shadow-[0_0_15px_rgba(6,182,212,0.15)]".to_string();
        t.border_accent = "border-cyan-500/40".to_string();
        t.text_accent = "text-cyan-400".to_string();
        t.text_accent_bright = "text-cyan-300".to_string();
        t.button_primary = "bg-cyan-600 hover:bg-cyan-500 active:bg-cyan-700 text-black font-semibold shadow-md shadow-cyan-950/50 border border-cyan-300/40 shadow-[inset_0_1px_0_0_rgba(255,255,255,0.3)] active:scale-[0.98]".to_string();
        t.badge_primary = "bg-cyan-950/50 text-cyan-300 border-cyan-500/30".to_string();
        t.input_normal_border =
            "border-white/[0.08] focus:border-cyan-500/60 focus:ring-cyan-600/20".to_string();
        t.banner = "mb-6 p-4 rounded-xl bg-[#09090d] border border-cyan-500/30 shadow-lg shadow-cyan-950/20".to_string();

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
        t.sidebar_link_active = "bg-emerald-950/40 text-emerald-200 font-semibold border border-emerald-500/30 shadow-[0_0_15px_rgba(16,185,129,0.15)]".to_string();
        t.border_accent = "border-emerald-500/40".to_string();
        t.text_accent = "text-emerald-400".to_string();
        t.text_accent_bright = "text-emerald-300".to_string();
        t.button_primary = "bg-emerald-600 hover:bg-emerald-500 active:bg-emerald-700 text-white shadow-md shadow-emerald-950/50 border border-emerald-400/40 shadow-[inset_0_1px_0_0_rgba(255,255,255,0.2)] active:scale-[0.98]".to_string();
        t.badge_primary = "bg-emerald-950/50 text-emerald-300 border-emerald-500/30".to_string();
        t.input_normal_border =
            "border-white/[0.08] focus:border-emerald-500/60 focus:ring-emerald-600/20".to_string();
        t.banner = "mb-6 p-4 rounded-xl bg-[#09090d] border border-emerald-500/30 shadow-lg shadow-emerald-950/20".to_string();

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
