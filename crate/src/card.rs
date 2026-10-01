use dioxus::prelude::*;

#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub enum CardShadow {
    None,
    Sm,
    #[default]
    Md,
    Lg,
    Xl,
}

impl CardShadow {
    pub fn classes(&self) -> &'static str {
        match self {
            CardShadow::None => "shadow-none",
            CardShadow::Sm => "shadow-sm shadow-black/60",
            CardShadow::Md => {
                "shadow-lg shadow-black/80 hover:shadow-purple-950/20 transition-shadow"
            }
            CardShadow::Lg => {
                "shadow-xl shadow-black/90 hover:shadow-purple-950/40 transition-shadow"
            }
            CardShadow::Xl => "shadow-2xl shadow-purple-950/30 border-purple-500/20",
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub enum CardVariant {
    #[default]
    Default,
    Elevated,
    Interactive,
    Metric,
}

#[component]
pub fn Card(
    #[props(default)] title: Option<String>,
    #[props(default)] subtitle: Option<String>,
    #[props(default)] variant: CardVariant,
    #[props(default)] shadow: CardShadow,
    #[props(default)] hover_glow: bool,
    #[props(default)] glow: Option<bool>,
    #[props(default = false)] overflow_hidden: bool,
    #[props(default)] class: String,
    #[props(default)] footer: Option<Element>,
    children: Element,
) -> Element {
    #[cfg(feature = "theme")]
    let theme = crate::theme::use_theme();
    #[cfg(feature = "theme")]
    let shadow_class = theme().card_shadow_classes(shadow).to_string();
    #[cfg(feature = "theme")]
    let base_class = match variant {
        CardVariant::Default => "bg-[#09090d] border border-white/[0.08] text-zinc-100 shadow-[inset_0_1px_0_0_rgba(255,255,255,0.06)]".to_string(),
        CardVariant::Elevated => "bg-[#111118] border border-white/[0.1] text-zinc-100 shadow-2xl shadow-black/90 shadow-[inset_0_1px_0_0_rgba(255,255,255,0.08)]".to_string(),
        CardVariant::Interactive => "bg-[#09090d] border border-white/[0.08] hover:border-purple-500/40 text-zinc-100 transition-all cursor-pointer shadow-[inset_0_1px_0_0_rgba(255,255,255,0.06)]".to_string(),
        CardVariant::Metric => "bg-[#09090d] border border-white/[0.08] text-zinc-100 shadow-[inset_0_1px_0_0_rgba(255,255,255,0.06)]".to_string(),
    };
    #[cfg(feature = "theme")]
    let radius_class = theme().radius_scale.lg();
    #[cfg(feature = "theme")]
    let header_class = theme().card_header.clone();
    #[cfg(feature = "theme")]
    let footer_class = theme().card_footer.clone();
    #[cfg(feature = "theme")]
    let glow_class = match glow {
        Some(true) => format!("border-purple-500/40 {}", theme().glow_card),
        Some(false) => String::new(),
        None => {
            if hover_glow
                || (theme().glow_mode != crate::theme::GlowMode::Off
                    && variant == CardVariant::Interactive)
            {
                format!("hover:border-purple-500/40 {}", theme().glow_card)
            } else {
                String::new()
            }
        }
    };

    #[cfg(not(feature = "theme"))]
    let shadow_class = shadow.classes();
    #[cfg(not(feature = "theme"))]
    let base_class = match variant {
        CardVariant::Default => "bg-[#09090d] border border-white/[0.08] text-zinc-100 shadow-[inset_0_1px_0_0_rgba(255,255,255,0.06)]",
        CardVariant::Elevated => "bg-[#111118] border border-white/[0.1] text-zinc-100 shadow-2xl shadow-black/90",
        CardVariant::Interactive => "bg-[#09090d] border border-white/[0.08] hover:border-purple-500/40 text-zinc-100 transition-all cursor-pointer",
        CardVariant::Metric => "bg-[#09090d] border border-white/[0.08] text-zinc-100",
    };
    #[cfg(not(feature = "theme"))]
    let radius_class = "rounded-xl";
    #[cfg(not(feature = "theme"))]
    let header_class = "px-6 py-4 border-b border-white/[0.06] bg-[#0c0c12]/80";
    #[cfg(not(feature = "theme"))]
    let footer_class =
        "px-6 py-3 bg-[#07070a]/90 border-t border-white/[0.06] text-xs text-zinc-400";
    #[cfg(not(feature = "theme"))]
    let glow_class = if hover_glow || glow == Some(true) {
        "hover:border-purple-500/40 hover:shadow-[0_0_25px_rgba(147,51,234,0.2)]".to_string()
    } else {
        String::new()
    };

    let overflow = if overflow_hidden {
        "overflow-hidden"
    } else {
        ""
    };
    let header_corner = if overflow_hidden { "" } else { "rounded-t-xl" };
    let footer_corner = if overflow_hidden { "" } else { "rounded-b-xl" };

    rsx! {
        div {
            class: "{base_class} {radius_class} {shadow_class} {glow_class} {overflow} {class}",
            if title.is_some() || subtitle.is_some() {
                div {
                    class: "{header_class} {header_corner}",
                    if let Some(t) = title {
                        h3 { class: "text-base font-semibold tracking-tight text-white", "{t}" }
                    }
                    if let Some(s) = subtitle {
                        p { class: "text-xs text-zinc-400 mt-0.5", "{s}" }
                    }
                }
            }
            div {
                class: "p-5 sm:p-6",
                {children}
            }
            if let Some(f) = footer {
                div {
                    class: "{footer_class} {footer_corner}",
                    {f}
                }
            }
        }
    }
}
