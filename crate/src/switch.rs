use dioxus::prelude::*;

#[component]
pub fn Switch(
    checked: Signal<bool>,
    #[props(default)] label: Option<String>,
    #[props(default)] description: Option<String>,
    #[props(default)] disabled: bool,
    #[props(default)] glow: Option<bool>,
    #[props(default)] class: String,
) -> Element {
    let is_checked = *checked.read();

    #[cfg(feature = "theme")]
    let theme = crate::theme::use_theme();
    #[cfg(feature = "theme")]
    let has_glow = theme().is_component_glow_enabled("switch", glow);
    #[cfg(not(feature = "theme"))]
    let has_glow = glow.unwrap_or(false);

    #[cfg(feature = "theme")]
    let active_track = match theme().primary_color {
        crate::theme::PrimaryColor::Violet => {
            if has_glow {
                "bg-purple-600 border-purple-400/50 shadow-[0_0_14px_rgba(147,51,234,0.45)]"
            } else {
                "bg-purple-600 border-purple-500/50 shadow-none"
            }
        }
        crate::theme::PrimaryColor::Obsidian => {
            if has_glow {
                "bg-zinc-200 border-white shadow-[0_0_12px_rgba(255,255,255,0.3)]"
            } else {
                "bg-zinc-200 border-white/80 shadow-none"
            }
        }
        crate::theme::PrimaryColor::Crimson => {
            if has_glow {
                "bg-rose-600 border-rose-400/50 shadow-[0_0_14px_rgba(244,63,94,0.45)]"
            } else {
                "bg-rose-600 border-rose-500/50 shadow-none"
            }
        }
        crate::theme::PrimaryColor::Cyan => {
            if has_glow {
                "bg-cyan-600 border-cyan-400/50 shadow-[0_0_14px_rgba(6,182,212,0.45)]"
            } else {
                "bg-cyan-600 border-cyan-500/50 shadow-none"
            }
        }
        crate::theme::PrimaryColor::Emerald => {
            if has_glow {
                "bg-emerald-600 border-emerald-400/50 shadow-[0_0_14px_rgba(16,185,129,0.45)]"
            } else {
                "bg-emerald-600 border-emerald-500/50 shadow-none"
            }
        }
    };

    #[cfg(not(feature = "theme"))]
    let active_track = if has_glow {
        "bg-purple-600 border-purple-400/50 shadow-[0_0_14px_rgba(147,51,234,0.45)]"
    } else {
        "bg-purple-600 border-purple-500/50 shadow-none"
    };

    let track_bg = if is_checked {
        active_track
    } else {
        "bg-[#14141c] border-white/10"
    };

    let thumb_pos = if is_checked {
        "translate-x-5"
    } else {
        "translate-x-0.5"
    };
    let thumb_color = if is_checked && cfg!(feature = "theme") {
        "bg-white shadow-sm"
    } else {
        "bg-zinc-300 shadow-sm"
    };

    let cursor_class = if disabled {
        "opacity-50 cursor-not-allowed pointer-events-none"
    } else {
        "cursor-pointer"
    };

    rsx! {
        div {
            class: "flex items-start gap-3 select-none {cursor_class} {class}",
            onclick: move |_| {
                if !disabled {
                    let next = !*checked.read();
                    *checked.write() = next;
                }
            },
            // Switch Track Pill
            div {
                class: "relative inline-flex h-6 w-11 shrink-0 items-center rounded-full border transition-colors duration-200 ease-in-out {track_bg}",
                // Switch Thumb Circle
                span {
                    class: "pointer-events-none inline-block h-4 w-4 transform rounded-full transition-transform duration-200 ease-in-out {thumb_color} {thumb_pos}",
                }
            }
            // Label & Description
            if label.is_some() || description.is_some() {
                div { class: "flex flex-col gap-0.5",
                    if let Some(lbl) = label {
                        span { class: "text-xs font-medium text-zinc-200 leading-tight", "{lbl}" }
                    }
                    if let Some(desc) = description {
                        span { class: "text-[11px] text-zinc-500 leading-normal", "{desc}" }
                    }
                }
            }
        }
    }
}
