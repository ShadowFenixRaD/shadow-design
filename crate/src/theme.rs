use dioxus::prelude::*;

#[cfg(feature = "button")]
use crate::button::ButtonVariant;
#[cfg(feature = "card")]
use crate::card::CardShadow;
#[cfg(feature = "badge")]
use crate::badge::BadgeVariant;
#[cfg(feature = "alert")]
use crate::alert::AlertVariant;

/// Struttura centrale del tema per Shadow Design.
/// Permette di personalizzare, modificare o sovrascrivere tutte le classi
/// dell'intera libreria e dell'applicazione in un unico punto.
#[derive(Clone, PartialEq, Debug)]
pub struct Theme {
    pub name: String,

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
    /// Tema di default Shadow Design (Sfondo nero assoluto 0 0 0 e viola scuro intenso shadowfenix).
    pub fn shadow() -> Self {
        Self {
            name: "Shadow (Default)".to_string(),

            // Layout
            root: "min-h-screen flex flex-col bg-black text-zinc-100 selection:bg-purple-800 selection:text-white".to_string(),
            header: "border-b border-purple-950/60 bg-black/90 backdrop-blur-md shadow-[0_4px_30px_rgba(0,0,0,0.8)]".to_string(),
            sidebar: "border-r border-purple-950/40 bg-black".to_string(),
            sidebar_link: "flex items-center gap-2.5 px-3 py-2 rounded-lg text-sm font-medium text-zinc-400 hover:text-purple-200 hover:bg-purple-950/30 transition-all".to_string(),
            sidebar_link_active: "bg-purple-950/60 text-purple-300 font-semibold border border-purple-800/60 shadow-[0_0_15px_rgba(107,33,168,0.25)]".to_string(),
            content: "bg-black text-zinc-100".to_string(),
            surface: "bg-[#08080a] border border-purple-950/60".to_string(),
            panel: "bg-[#0d0d14] border border-purple-900/60 shadow-xl shadow-purple-950/20".to_string(),
            banner: "mb-6 p-4 rounded-xl bg-purple-950/25 border border-purple-900/50 shadow-md shadow-purple-950/20".to_string(),

            // Accenti
            border: "border-purple-950/60".to_string(),
            border_subtle: "border-purple-950/40".to_string(),
            border_accent: "border-purple-700/60".to_string(),
            text_accent: "text-purple-400".to_string(),
            text_accent_bright: "text-purple-300".to_string(),
            glow: "shadow-[0_0_20px_rgba(107,33,168,0.3)]".to_string(),

            // Button
            button_primary: "bg-purple-800 hover:bg-purple-700 active:bg-purple-900 text-white shadow-lg shadow-purple-950/70 border border-purple-600/70 hover:border-purple-400 hover:shadow-purple-900/60".to_string(),
            button_secondary: "bg-zinc-950 hover:bg-zinc-900 text-zinc-200 shadow-md border border-purple-950/80 hover:border-purple-800/60 hover:text-white".to_string(),
            button_outline: "bg-transparent hover:bg-purple-950/30 text-purple-300 border border-purple-800/60 hover:border-purple-500 hover:text-purple-100".to_string(),
            button_ghost: "bg-transparent hover:bg-purple-950/40 text-zinc-300 hover:text-purple-200".to_string(),
            button_danger: "bg-rose-700 hover:bg-rose-600 text-white shadow-md shadow-rose-950/50 border border-rose-600".to_string(),
            button_success: "bg-emerald-700 hover:bg-emerald-600 text-white shadow-md shadow-emerald-950/50 border border-emerald-600".to_string(),

            // Card
            card_base: "bg-[#08080a] border border-purple-950/60 text-zinc-100 rounded-xl overflow-hidden hover:border-purple-900/50 transition-all".to_string(),
            card_header: "px-6 py-4 border-b border-purple-950/50 bg-[#0d0d12]/90".to_string(),
            card_footer: "px-6 py-3 bg-[#050507]/90 border-t border-purple-950/50 text-sm text-zinc-400".to_string(),
            card_shadow_none: "shadow-none".to_string(),
            card_shadow_sm: "shadow-sm shadow-black".to_string(),
            card_shadow_md: "shadow-lg shadow-black/80 hover:shadow-purple-950/40 transition-shadow".to_string(),
            card_shadow_lg: "shadow-xl shadow-black/90 hover:shadow-purple-950/60 transition-shadow".to_string(),
            card_shadow_xl: "shadow-2xl shadow-purple-950/50 border-purple-800/40".to_string(),

            // TextInput
            input_base: "px-3.5 py-2 bg-black text-zinc-100 rounded-lg border focus:outline-none focus:ring-2 transition-all placeholder:text-zinc-600 disabled:opacity-50 disabled:cursor-not-allowed text-sm shadow-inner".to_string(),
            input_normal_border: "border-zinc-800 focus:border-purple-500 focus:ring-purple-600/30".to_string(),
            input_error_border: "border-rose-500 focus:border-rose-400 focus:ring-rose-500/20".to_string(),

            // Badge
            badge_default: "bg-zinc-900 text-zinc-300 border-purple-950/70".to_string(),
            badge_primary: "bg-purple-950/80 text-purple-300 border-purple-800/70 shadow-[0_0_12px_rgba(107,33,168,0.25)]".to_string(),
            badge_success: "bg-emerald-950/70 text-emerald-400 border-emerald-800/60".to_string(),
            badge_warning: "bg-amber-950/70 text-amber-400 border-amber-800/60".to_string(),
            badge_danger: "bg-rose-950/70 text-rose-400 border-rose-800/60".to_string(),
            badge_outline: "bg-transparent text-purple-300 border-purple-800/60".to_string(),

            // Navbar
            navbar_base: "w-full bg-black/90 backdrop-blur-md border-b border-purple-950/60 shadow-[0_4px_25px_rgba(0,0,0,0.9)] px-6 py-3.5 sticky top-0 z-40".to_string(),

            // Modal
            modal_backdrop: "fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/85 backdrop-blur-md transition-all".to_string(),
            modal_dialog: "bg-[#08080c] border border-purple-900/60 rounded-2xl max-w-lg w-full shadow-2xl shadow-purple-950/60 overflow-hidden".to_string(),
            modal_header: "flex items-center justify-between px-6 py-4 border-b border-purple-950/60 bg-[#0d0d14]/80".to_string(),
            modal_close: "text-zinc-400 hover:text-white text-lg font-bold p-1 rounded-md hover:bg-purple-950/50 transition-colors cursor-pointer".to_string(),

            // Alert
            alert_info: "bg-purple-950/40 text-purple-100 border-purple-900/60 border-l-4 border-l-purple-600 shadow-lg shadow-purple-950/30".to_string(),
            alert_success: "bg-emerald-950/40 text-emerald-100 border-emerald-900/60 border-l-4 border-l-emerald-500 shadow-lg shadow-black/40".to_string(),
            alert_warning: "bg-amber-950/40 text-amber-100 border-amber-900/60 border-l-4 border-l-amber-500 shadow-lg shadow-black/40".to_string(),
            alert_danger: "bg-rose-950/40 text-rose-100 border-rose-900/60 border-l-4 border-l-rose-500 shadow-lg shadow-black/40".to_string(),
            alert_title_info: "text-purple-300".to_string(),
            alert_title_success: "text-emerald-300".to_string(),
            alert_title_warning: "text-amber-300".to_string(),
            alert_title_danger: "text-rose-300".to_string(),
        }
    }

    /// Preset alternativo: Obsidian Monochrome (Nero profondo con sfumature argento e grafite).
    pub fn obsidian() -> Self {
        let mut t = Self::shadow();
        t.name = "Obsidian Stealth".to_string();
        t.header = "border-b border-zinc-800 bg-black/95 backdrop-blur-md shadow-2xl".to_string();
        t.sidebar = "border-r border-zinc-900 bg-black".to_string();
        t.sidebar_link_active = "bg-zinc-900 text-white font-semibold border border-zinc-700 shadow-md".to_string();
        t.border = "border-zinc-800".to_string();
        t.border_subtle = "border-zinc-900".to_string();
        t.border_accent = "border-zinc-600".to_string();
        t.text_accent = "text-zinc-300".to_string();
        t.text_accent_bright = "text-white".to_string();
        t.glow = "shadow-[0_0_20px_rgba(255,255,255,0.08)]".to_string();
        t.button_primary = "bg-zinc-100 hover:bg-white text-black font-semibold shadow-lg border border-white".to_string();
        t.card_base = "bg-[#060606] border border-zinc-800/80 text-zinc-100 rounded-xl overflow-hidden hover:border-zinc-700 transition-all".to_string();
        t.card_header = "px-6 py-4 border-b border-zinc-900 bg-zinc-950/90".to_string();
        t.card_footer = "px-6 py-3 bg-black border-t border-zinc-900 text-sm text-zinc-400".to_string();
        t.badge_primary = "bg-zinc-800 text-zinc-200 border-zinc-600".to_string();
        t.input_normal_border = "border-zinc-800 focus:border-zinc-400 focus:ring-zinc-400/20".to_string();
        t.banner = "mb-6 p-4 rounded-xl bg-zinc-950 border border-zinc-800 shadow-md".to_string();
        t
    }

    /// Preset alternativo: Crimson Shadow (Nero assoluto con fiamme cremisi della fenice).
    pub fn crimson() -> Self {
        let mut t = Self::shadow();
        t.name = "Crimson Fenix".to_string();
        t.header = "border-b border-rose-950/60 bg-black/90 backdrop-blur-md shadow-[0_4px_30px_rgba(0,0,0,0.8)]".to_string();
        t.sidebar = "border-r border-rose-950/40 bg-black".to_string();
        t.sidebar_link_active = "bg-rose-950/60 text-rose-300 font-semibold border border-rose-800/60 shadow-[0_0_15px_rgba(225,29,72,0.25)]".to_string();
        t.border = "border-rose-950/60".to_string();
        t.border_subtle = "border-rose-950/40".to_string();
        t.border_accent = "border-rose-700/60".to_string();
        t.text_accent = "text-rose-400".to_string();
        t.text_accent_bright = "text-rose-300".to_string();
        t.glow = "shadow-[0_0_20px_rgba(225,29,72,0.3)]".to_string();
        t.button_primary = "bg-rose-800 hover:bg-rose-700 active:bg-rose-900 text-white shadow-lg shadow-rose-950/70 border border-rose-600/70 hover:border-rose-400 hover:shadow-rose-900/60".to_string();
        t.card_base = "bg-[#0a0607] border border-rose-950/60 text-zinc-100 rounded-xl overflow-hidden hover:border-rose-900/50 transition-all".to_string();
        t.card_header = "px-6 py-4 border-b border-rose-950/50 bg-[#12080a]/90".to_string();
        t.card_footer = "px-6 py-3 bg-[#070405]/90 border-t border-rose-950/50 text-sm text-zinc-400".to_string();
        t.badge_primary = "bg-rose-950/80 text-rose-300 border-rose-800/70 shadow-[0_0_12px_rgba(225,29,72,0.25)]".to_string();
        t.input_normal_border = "border-zinc-800 focus:border-rose-500 focus:ring-rose-600/30".to_string();
        t.banner = "mb-6 p-4 rounded-xl bg-rose-950/25 border border-rose-900/50 shadow-md shadow-rose-950/20".to_string();
        t
    }

    // Builder methods per personalizzazione rapida
    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = name.into();
        self
    }

    pub fn with_root(mut self, root: impl Into<String>) -> Self {
        self.root = root.into();
        self
    }

    pub fn with_header(mut self, header: impl Into<String>) -> Self {
        self.header = header.into();
        self
    }

    pub fn with_sidebar(mut self, sidebar: impl Into<String>) -> Self {
        self.sidebar = sidebar.into();
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
pub fn ThemeProvider(
    #[props(default)] theme: Option<Signal<Theme>>,
    children: Element,
) -> Element {
    let current = theme.unwrap_or_else(|| use_signal(Theme::default));
    use_context_provider(|| current);
    rsx! { {children} }
}

/// Hook per accedere e modificare il tema attivo.
/// Se nessun ThemeProvider è presente, ne inizializza uno locale predefinito senza andare in panic.
pub fn use_theme() -> Signal<Theme> {
    match try_use_context::<Signal<Theme>>() {
        Some(s) => s,
        None => {
            use_context_provider(|| Signal::new(Theme::default()))
        }
    }
}
