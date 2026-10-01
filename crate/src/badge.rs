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
            BadgeVariant::Default => "bg-zinc-900 text-zinc-300 border-purple-950/70",
            BadgeVariant::Primary => "bg-purple-950/80 text-purple-300 border-purple-800/70 shadow-[0_0_12px_rgba(107,33,168,0.25)]",
            BadgeVariant::Success => "bg-emerald-950/70 text-emerald-400 border-emerald-800/60",
            BadgeVariant::Warning => "bg-amber-950/70 text-amber-400 border-amber-800/60",
            BadgeVariant::Danger => "bg-rose-950/70 text-rose-400 border-rose-800/60",
            BadgeVariant::Outline => "bg-transparent text-purple-300 border-purple-800/60",
        }
    }
}

#[component]
pub fn Badge(
    label: String,
    #[props(default)] variant: BadgeVariant,
    #[props(default)] pill: bool,
    #[props(default)] class: String,
) -> Element {
    let radius = if pill { "rounded-full" } else { "rounded-md" };
    #[cfg(feature = "theme")]
    let theme = crate::theme::use_theme();
    #[cfg(feature = "theme")]
    let variant_classes = theme().badge_variant_classes(variant).to_string();
    #[cfg(not(feature = "theme"))]
    let variant_classes = variant.classes();

    rsx! {
        span {
            class: "inline-flex items-center px-2.5 py-0.5 text-xs font-medium border {radius} {variant_classes} {class}",
            "{label}"
        }
    }
}
