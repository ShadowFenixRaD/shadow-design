use crate::icon::{Icon, IconKind};
use dioxus::prelude::*;

#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub enum CheckboxSize {
    Sm,
    #[default]
    Md,
    Lg,
}

impl CheckboxSize {
    pub fn box_classes(&self) -> &'static str {
        match self {
            CheckboxSize::Sm => "w-4 h-4 rounded-[3px]",
            CheckboxSize::Md => "w-5 h-5 rounded-md",
            CheckboxSize::Lg => "w-6 h-6 rounded-lg",
        }
    }

    pub fn icon_size(&self) -> u32 {
        match self {
            CheckboxSize::Sm => 11,
            CheckboxSize::Md => 13,
            CheckboxSize::Lg => 16,
        }
    }
}

#[component]
pub fn Checkbox(
    checked: Signal<bool>,
    #[props(default)] label: Option<String>,
    #[props(default)] description: Option<String>,
    #[props(default)] disabled: bool,
    #[props(default)] error: Option<String>,
    #[props(default)] indeterminate: bool,
    #[props(default)] size: CheckboxSize,
    #[props(default)] glow: Option<bool>,
    #[props(default)] class: String,
) -> Element {
    let is_checked = *checked.read();

    #[cfg(feature = "theme")]
    let theme = crate::theme::use_theme();
    #[cfg(feature = "theme")]
    let active_color = match theme().primary_color {
        crate::theme::PrimaryColor::Violet => "bg-purple-600 border-purple-400 text-white",
        crate::theme::PrimaryColor::Obsidian => "bg-zinc-100 border-white text-black",
        crate::theme::PrimaryColor::Crimson => "bg-rose-600 border-rose-400 text-white",
        crate::theme::PrimaryColor::Cyan => "bg-cyan-500 border-cyan-300 text-black",
        crate::theme::PrimaryColor::Emerald => "bg-emerald-600 border-emerald-400 text-white",
    };
    #[cfg(feature = "theme")]
    let glow_class = match glow {
        Some(true) => theme().glow_checkbox.clone(),
        Some(false) => String::new(),
        None => {
            if theme().glow_mode != crate::theme::GlowMode::Off && (is_checked || indeterminate) {
                theme().glow_checkbox.clone()
            } else {
                String::new()
            }
        }
    };

    #[cfg(not(feature = "theme"))]
    let active_color = "bg-purple-600 border-purple-400 text-white";
    #[cfg(not(feature = "theme"))]
    let glow_class = if is_checked || indeterminate {
        "shadow-[0_0_10px_rgba(147,51,234,0.35)]"
    } else {
        ""
    };

    let state_class = if is_checked || indeterminate {
        format!(
            "{} shadow-[inset_0_1px_0_0_rgba(255,255,255,0.2)] {}",
            active_color, glow_class
        )
    } else if error.is_some() {
        "bg-[#060608] border-rose-500/70 hover:border-rose-400 text-transparent".to_string()
    } else {
        "bg-[#060608] border-white/20 hover:border-white/40 text-transparent".to_string()
    };

    let box_size = size.box_classes();
    let is_disabled = disabled;
    let cursor_class = if is_disabled {
        "opacity-50 cursor-not-allowed pointer-events-none"
    } else {
        "cursor-pointer"
    };

    rsx! {
        div {
            class: "inline-flex items-start gap-2.5 select-none {cursor_class} {class}",
            onclick: move |_| {
                if !is_disabled {
                    let next = !*checked.read();
                    *checked.write() = next;
                }
            },
            // Custom tactile checkbox box
            div {
                class: "relative flex items-center justify-center shrink-0 border transition-all duration-150 ease-out active:scale-95 mt-0.5 {box_size} {state_class}",
                if indeterminate {
                    span { class: "block w-2.5 h-0.5 bg-current rounded-full" }
                } else if is_checked {
                    Icon { kind: IconKind::Check, size: size.icon_size() }
                }
            }
            // Label & description
            if label.is_some() || description.is_some() || error.is_some() {
                div { class: "flex flex-col gap-0.5",
                    if let Some(lbl) = label {
                        span { class: "text-xs font-medium text-zinc-200 leading-tight", "{lbl}" }
                    }
                    if let Some(desc) = description {
                        span { class: "text-[11px] text-zinc-500 leading-normal", "{desc}" }
                    }
                    if let Some(err) = error {
                        span { class: "text-[11px] text-rose-400 font-medium", "{err}" }
                    }
                }
            }
        }
    }
}
