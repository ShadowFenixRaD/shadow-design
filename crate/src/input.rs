use crate::icon::{Icon, IconKind, RenderSvgIcon, SvgIcon};
use dioxus::prelude::*;

#[component]
pub fn TextInput(
    value: Signal<String>,
    #[props(default)] label: Option<String>,
    #[props(default)] placeholder: Option<String>,
    #[props(default)] help_text: Option<String>,
    #[props(default)] error: Option<String>,
    #[props(default)] disabled: bool,
    #[props(default)] clearable: bool,
    #[props(default)] glow: Option<bool>,
    #[props(default)] left_icon: Option<SvgIcon>,
    #[props(default)] right_icon: Option<SvgIcon>,
    #[props(default)] on_left_icon_click: Option<EventHandler<MouseEvent>>,
    #[props(default)] on_right_icon_click: Option<EventHandler<MouseEvent>>,
    #[props(default)] input_type: Option<String>,
    #[props(default)] class: String,
) -> Element {
    let itype = input_type.unwrap_or_else(|| "text".to_string());

    #[cfg(feature = "theme")]
    let theme = crate::theme::use_theme();
    #[cfg(feature = "theme")]
    let border_color = if error.is_some() {
        theme().input_error_border.clone()
    } else {
        theme().input_normal_border.clone()
    };
    #[cfg(feature = "theme")]
    let radius_class = theme().radius_scale.md();
    #[cfg(feature = "theme")]
    let glow_class = match glow {
        Some(true) => format!("focus:{}", theme().glow_input),
        Some(false) => String::new(),
        None => {
            if theme().glow_mode != crate::theme::GlowMode::Off {
                format!("focus:{}", theme().glow_input)
            } else {
                String::new()
            }
        }
    };

    #[cfg(not(feature = "theme"))]
    let border_color = if error.is_some() {
        "border-rose-500/70 focus:border-rose-400 focus:ring-rose-500/20".to_string()
    } else {
        "border-white/[0.08] focus:border-purple-500/60 focus:ring-purple-600/20 hover:border-white/15".to_string()
    };
    #[cfg(not(feature = "theme"))]
    let radius_class = "rounded-lg";
    #[cfg(not(feature = "theme"))]
    let glow_class = if glow == Some(true) {
        "focus:shadow-[0_0_15px_rgba(147,51,234,0.3)]"
    } else {
        ""
    };

    let has_clear = clearable && !value().is_empty() && !disabled;
    let has_left = left_icon.is_some();
    let has_right = right_icon.is_some() || has_clear;
    let is_left_custom = matches!(&left_icon, Some(SvgIcon::Custom(_)));
    let is_right_custom = matches!(&right_icon, Some(SvgIcon::Custom(_)));
    let left_pointer_style = if is_left_custom {
        "pointer-events: auto;"
    } else {
        "pointer-events: none;"
    };
    let right_pointer_style = if is_right_custom {
        "pointer-events: auto;"
    } else {
        "pointer-events: none;"
    };

    // Calcolo automatico della spaziatura per garantire che testo e icone non collidano mai
    let (icon_class, style_padding) = match (has_left, has_right) {
        (true, true) => (
            "sd-input--has-both-icons pl-11 pr-11",
            "padding-left: 42px !important; padding-right: 42px !important;",
        ),
        (true, false) => (
            "sd-input--has-left-icon pl-11 pr-3.5",
            "padding-left: 42px !important; padding-right: 14px !important;",
        ),
        (false, true) => (
            "sd-input--has-right-icon pl-3.5 pr-11",
            "padding-left: 14px !important; padding-right: 42px !important;",
        ),
        (false, false) => (
            "pl-3.5 pr-3.5",
            "padding-left: 14px !important; padding-right: 14px !important;",
        ),
    };

    rsx! {
        div {
            class: "sd-input-wrapper flex flex-col gap-1.5 {class}",
            if let Some(lbl) = label {
                label { class: "sd-input-label text-xs font-medium text-zinc-300 select-none", "{lbl}" }
            }
            // Il contenitore ha position: relative blindato per vincolare l'ancoraggio delle icone
            div {
                class: "sd-input-container relative flex items-center w-full",
                style: "position: relative; width: 100%; display: flex; align-items: center;",
                // Icona Sinistra posizionata e centrata verticalmente
                if let Some(licon) = left_icon {
                    if let Some(handler) = on_left_icon_click {
                        button {
                            r#type: "button",
                            class: "sd-input-icon sd-input-icon--left sd-input-icon--interactive",
                            style: "position: absolute; left: 14px; top: 50%; transform: translateY(-50%); display: inline-flex; align-items: center; justify-content: center; width: 16px; height: 16px; z-index: 10; cursor: pointer; pointer-events: auto; background: transparent; border: none; padding: 0; margin: 0; outline: none; color: #a1a1aa;",
                            onclick: move |e| {
                                e.stop_propagation();
                                handler.call(e);
                            },
                            RenderSvgIcon { icon: licon, size: 16 }
                        }
                    } else {
                        div {
                            class: "sd-input-icon sd-input-icon--left",
                            style: "position: absolute; left: 14px; top: 50%; transform: translateY(-50%); display: inline-flex; align-items: center; justify-content: center; width: 16px; height: 16px; {left_pointer_style} z-index: 10; color: #a1a1aa;",
                            RenderSvgIcon { icon: licon, size: 16 }
                        }
                    }
                }
                // Campo di testo reattivo
                input {
                    r#type: "{itype}",
                    value: "{value}",
                    placeholder: placeholder.as_deref().unwrap_or_default(),
                    disabled,
                    class: "sd-input w-full h-[42px] bg-[#060608] text-zinc-100 border focus:outline-none focus:ring-2 transition-all placeholder:text-zinc-600 disabled:opacity-50 disabled:cursor-not-allowed text-xs sm:text-sm shadow-[inset_0_1px_2px_rgba(0,0,0,0.5)] {icon_class} {radius_class} {border_color} {glow_class}",
                    style: "{style_padding}",
                    oninput: move |e| {
                        *value.write() = e.value();
                    },
                }
                // Icona Destra o Pulsante Clear con azione di svuotamento
                if has_clear {
                    button {
                        r#type: "button",
                        class: "sd-input-icon sd-input-icon--right sd-input-icon--interactive",
                        style: "position: absolute; right: 14px; top: 50%; transform: translateY(-50%); display: inline-flex; align-items: center; justify-content: center; width: 16px; height: 16px; z-index: 10; cursor: pointer; pointer-events: auto; background: transparent; border: none; padding: 0; margin: 0; outline: none; color: #a1a1aa;",
                        title: "Cancella testo",
                        onclick: move |e| {
                            e.stop_propagation();
                            *value.write() = String::new();
                        },
                        Icon { kind: IconKind::X, size: 14 }
                    }
                } else if let Some(ricon) = right_icon {
                    if let Some(handler) = on_right_icon_click {
                        button {
                            r#type: "button",
                            class: "sd-input-icon sd-input-icon--right sd-input-icon--interactive",
                            style: "position: absolute; right: 14px; top: 50%; transform: translateY(-50%); display: inline-flex; align-items: center; justify-content: center; width: 16px; height: 16px; z-index: 10; cursor: pointer; pointer-events: auto; background: transparent; border: none; padding: 0; margin: 0; outline: none; color: #a1a1aa;",
                            onclick: move |e| {
                                e.stop_propagation();
                                handler.call(e);
                            },
                            RenderSvgIcon { icon: ricon, size: 16 }
                        }
                    } else {
                        div {
                            class: "sd-input-icon sd-input-icon--right",
                            style: "position: absolute; right: 14px; top: 50%; transform: translateY(-50%); display: inline-flex; align-items: center; justify-content: center; width: 16px; height: 16px; {right_pointer_style} z-index: 10; color: #a1a1aa;",
                            RenderSvgIcon { icon: ricon, size: 16 }
                        }
                    }
                }
            }
            if let Some(err) = error {
                span { class: "text-[11px] text-rose-400 mt-0.5 font-medium", "{err}" }
            } else if let Some(help) = help_text {
                span { class: "text-[11px] text-zinc-500 mt-0.5", "{help}" }
            }
        }
    }
}
