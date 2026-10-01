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
            BadgeVariant::Default => "bg-zinc-800 text-zinc-200 border-zinc-700",
            BadgeVariant::Primary => "bg-blue-950/70 text-blue-400 border-blue-800/60",
            BadgeVariant::Success => "bg-emerald-950/70 text-emerald-400 border-emerald-800/60",
            BadgeVariant::Warning => "bg-amber-950/70 text-amber-400 border-amber-800/60",
            BadgeVariant::Danger => "bg-rose-950/70 text-rose-400 border-rose-800/60",
            BadgeVariant::Outline => "bg-transparent text-zinc-300 border-zinc-600",
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
    let variant_classes = variant.classes();

    rsx! {
        span {
            class: "inline-flex items-center px-2.5 py-0.5 text-xs font-medium border {radius} {variant_classes} {class}",
            "{label}"
        }
    }
}
