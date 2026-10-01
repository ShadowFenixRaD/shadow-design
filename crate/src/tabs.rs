use crate::icon::{Icon, IconKind};
use dioxus::prelude::*;

#[component]
pub fn TabList(#[props(default)] class: String, children: Element) -> Element {
    rsx! {
        div {
            class: "flex flex-wrap items-center gap-1.5 p-1 rounded-xl bg-[#060608] border border-white/[0.08] shadow-inner {class}",
            {children}
        }
    }
}

#[component]
pub fn Tab(
    value: String,
    active: Signal<String>,
    #[props(default)] icon: Option<IconKind>,
    #[props(default)] badge: Option<String>,
    #[props(default)] class: String,
    children: Element,
) -> Element {
    let is_active = *active.read() == value;
    let target = value.clone();

    #[cfg(feature = "theme")]
    let theme = crate::theme::use_theme();
    #[cfg(feature = "theme")]
    let active_style = format!(
        "bg-purple-950/70 text-purple-200 border-purple-500/40 shadow-sm {}",
        theme().radius_scale.sm()
    );
    #[cfg(feature = "theme")]
    let radius_class = theme().radius_scale.sm();

    #[cfg(not(feature = "theme"))]
    let active_style = "bg-purple-950/70 text-purple-200 border-purple-500/40 shadow-sm rounded-lg";
    #[cfg(not(feature = "theme"))]
    let radius_class = "rounded-lg";

    let inactive_style = format!(
        "text-zinc-400 hover:text-zinc-200 hover:bg-white/[0.04] border-transparent {}",
        radius_class
    );
    let current_style = if is_active {
        active_style
    } else {
        inactive_style
    };

    rsx! {
        button {
            r#type: "button",
            class: "flex items-center gap-2 px-3.5 py-1.5 text-xs font-medium border transition-all duration-150 cursor-pointer select-none active:scale-[0.98] {current_style} {class}",
            onclick: move |_| *active.write() = target.clone(),
            if let Some(i) = icon {
                Icon { kind: i, size: 14 }
            }
            {children}
            if let Some(b) = badge {
                span { class: "ml-1 px-1.5 py-0.2 rounded-full text-[10px] font-mono bg-white/10 text-zinc-300", "{b}" }
            }
        }
    }
}

#[component]
pub fn TabPanel(
    value: String,
    active: Signal<String>,
    #[props(default)] class: String,
    children: Element,
) -> Element {
    if *active.read() != value {
        return rsx! {};
    }

    rsx! {
        div {
            class: "pt-4 animate-fade-in {class}",
            {children}
        }
    }
}
