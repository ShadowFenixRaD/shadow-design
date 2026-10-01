use dioxus::prelude::*;

#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub enum BadgeVariant {
    #[default]
    Default,
    Primary,
    Success,
    Warning,
    Danger,
    Outline,
}

impl BadgeVariant {
    pub fn classes(&self) -> &'static str {
        match self {
            BadgeVariant::Default => "bg-[#14141c] text-zinc-300 border-white/[0.08]",
            BadgeVariant::Primary => "bg-purple-950/50 text-purple-300 border-purple-500/30",
            BadgeVariant::Success => "bg-emerald-950/50 text-emerald-300 border-emerald-500/30",
            BadgeVariant::Warning => "bg-amber-950/50 text-amber-300 border-amber-500/30",
            BadgeVariant::Danger => "bg-rose-950/50 text-rose-300 border-rose-500/30",
            BadgeVariant::Outline => "bg-transparent text-purple-300 border-purple-500/30",
        }
    }

    pub fn dot_color(&self) -> &'static str {
        match self {
            BadgeVariant::Default => "bg-zinc-400",
            BadgeVariant::Primary => "bg-purple-400 shadow-[0_0_8px_rgba(192,132,252,0.8)]",
            BadgeVariant::Success => "bg-emerald-400 shadow-[0_0_8px_rgba(52,211,153,0.8)]",
            BadgeVariant::Warning => "bg-amber-400 shadow-[0_0_8px_rgba(251,191,36,0.8)]",
            BadgeVariant::Danger => "bg-rose-400 shadow-[0_0_8px_rgba(251,113,133,0.8)]",
            BadgeVariant::Outline => "bg-purple-400",
        }
    }
}

#[component]
pub fn Badge(
    label: String,
    #[props(default)] variant: BadgeVariant,
    #[props(default)] pill: bool,
    #[props(default)] dot: bool,
    #[props(default)] glow: Option<bool>,
    #[props(default)] class: String,
) -> Element {
    #[cfg(feature = "theme")]
    let theme = crate::theme::use_theme();
    #[cfg(feature = "theme")]
    let radius = if pill {
        "rounded-full"
    } else {
        theme().radius_scale.sm()
    };
    #[cfg(feature = "theme")]
    let variant_classes = theme().badge_variant_classes(variant).to_string();
    #[cfg(feature = "theme")]
    let glow_class = match glow {
        Some(true) => theme().glow_badge.clone(),
        Some(false) => String::new(),
        None => {
            if variant == BadgeVariant::Primary && theme().glow_mode != crate::theme::GlowMode::Off
            {
                theme().glow_badge.clone()
            } else {
                String::new()
            }
        }
    };

    #[cfg(not(feature = "theme"))]
    let radius = if pill { "rounded-full" } else { "rounded-md" };
    #[cfg(not(feature = "theme"))]
    let variant_classes = variant.classes();
    #[cfg(not(feature = "theme"))]
    let glow_class = if glow == Some(true) || (glow.is_none() && variant == BadgeVariant::Primary) {
        "shadow-[0_0_10px_rgba(147,51,234,0.25)]"
    } else {
        ""
    };

    let dot_color = variant.dot_color();

    rsx! {
        span {
            class: "inline-flex items-center gap-1.5 px-2.5 py-0.5 text-[11px] font-medium border select-none {radius} {variant_classes} {glow_class} {class}",
            if dot {
                span { class: "relative flex h-1.5 w-1.5",
                    span { class: "animate-ping absolute inline-flex h-full w-full rounded-full opacity-75 {dot_color}" }
                    span { class: "relative inline-flex rounded-full h-1.5 w-1.5 {dot_color}" }
                }
            }
            "{label}"
        }
    }
}
