use crate::icon::{Icon, IconKind};
use dioxus::prelude::*;

#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub enum AlertVariant {
    #[default]
    Info,
    Success,
    Warning,
    Danger,
}

impl AlertVariant {
    pub fn classes(&self) -> &'static str {
        match self {
            AlertVariant::Info => {
                "bg-[#0e0e1a] text-purple-100 border-purple-500/30 border-l-4 border-l-purple-500 shadow-lg shadow-purple-950/20"
            }
            AlertVariant::Success => {
                "bg-[#0b1410] text-emerald-100 border-emerald-500/30 border-l-4 border-l-emerald-500 shadow-lg shadow-black/40"
            }
            AlertVariant::Warning => {
                "bg-[#141208] text-amber-100 border-amber-500/30 border-l-4 border-l-amber-500 shadow-lg shadow-black/40"
            }
            AlertVariant::Danger => {
                "bg-[#170a0e] text-rose-100 border-rose-500/30 border-l-4 border-l-rose-500 shadow-lg shadow-black/40"
            }
        }
    }

    pub fn title_class(&self) -> &'static str {
        match self {
            AlertVariant::Info => "text-purple-300 font-semibold",
            AlertVariant::Success => "text-emerald-300 font-semibold",
            AlertVariant::Warning => "text-amber-300 font-semibold",
            AlertVariant::Danger => "text-rose-300 font-semibold",
        }
    }

    pub fn icon_kind(&self) -> IconKind {
        match self {
            AlertVariant::Info => IconKind::Info,
            AlertVariant::Success => IconKind::CheckCircle,
            AlertVariant::Warning => IconKind::AlertTriangle,
            AlertVariant::Danger => IconKind::XCircle,
        }
    }

    pub fn icon_color(&self) -> &'static str {
        match self {
            AlertVariant::Info => "text-purple-400",
            AlertVariant::Success => "text-emerald-400",
            AlertVariant::Warning => "text-amber-400",
            AlertVariant::Danger => "text-rose-400",
        }
    }
}

#[component]
pub fn Alert(
    #[props(default)] title: Option<String>,
    #[props(default)] variant: AlertVariant,
    #[props(default)] class: String,
    #[props(default)] glow: Option<bool>,
    on_close: Option<EventHandler<MouseEvent>>,
    children: Element,
) -> Element {
    #[cfg(feature = "theme")]
    let theme = crate::theme::use_theme();
    #[cfg(feature = "theme")]
    let variant_classes = theme().alert_variant_classes(variant).to_string();
    #[cfg(feature = "theme")]
    let title_class = theme().alert_title_classes(variant).to_string();
    #[cfg(feature = "theme")]
    let radius_class = theme().radius_scale.md();
    #[cfg(feature = "theme")]
    let glow_class = if theme().is_component_glow_enabled("alert", glow) {
        match variant {
            AlertVariant::Info => "shadow-[0_0_20px_rgba(168,85,247,0.18)]",
            AlertVariant::Success => "shadow-[0_0_20px_rgba(16,185,129,0.18)]",
            AlertVariant::Warning => "shadow-[0_0_20px_rgba(245,158,11,0.18)]",
            AlertVariant::Danger => "shadow-[0_0_20px_rgba(244,63,94,0.22)]",
        }
    } else {
        ""
    };

    #[cfg(not(feature = "theme"))]
    let variant_classes = variant.classes();
    #[cfg(not(feature = "theme"))]
    let title_class = variant.title_class();
    #[cfg(not(feature = "theme"))]
    let radius_class = "rounded-xl";
    #[cfg(not(feature = "theme"))]
    let glow_class = if glow.unwrap_or(false) {
        "shadow-[0_0_20px_rgba(168,85,247,0.18)]"
    } else {
        ""
    };

    let icon_kind = variant.icon_kind();
    let icon_color = variant.icon_color();

    rsx! {
        div {
            class: "p-4 border flex items-start gap-3 shadow-md {radius_class} {variant_classes} {glow_class} {class}",
            span { class: "mt-0.5 shrink-0 {icon_color}",
                Icon { kind: icon_kind, size: 18 }
            }
            div { class: "flex flex-col gap-0.5 flex-1 min-w-0",
                if let Some(t) = title {
                    h4 { class: "text-xs sm:text-sm {title_class}", "{t}" }
                }
                div { class: "text-xs sm:text-sm opacity-90 leading-relaxed text-zinc-300", {children} }
            }
            if let Some(close_handler) = on_close {
                button {
                    r#type: "button",
                    class: "text-zinc-500 hover:text-white p-1 rounded transition-colors cursor-pointer shrink-0 -mr-1 -mt-1",
                    onclick: move |e| close_handler.call(e),
                    Icon { kind: IconKind::X, size: 14 }
                }
            }
        }
    }
}
