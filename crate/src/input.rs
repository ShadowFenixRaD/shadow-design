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
    let theme_val = crate::theme::use_theme()();
    #[cfg(feature = "theme")]
    let radius_class = theme_val.radius_scale.md();
    #[cfg(feature = "theme")]
    let base_override = theme_val.resolve_component_base("input").unwrap_or("");
    #[cfg(feature = "theme")]
    let class_modifiers = theme_val.resolve_class_modifiers("input", &class);
    #[cfg(feature = "theme")]
    let glow_class = theme_val.resolve_elevation_class("input", glow);

    #[cfg(not(feature = "theme"))]
    let base_override = "";
    #[cfg(not(feature = "theme"))]
    let class_modifiers = String::new();
    #[cfg(not(feature = "theme"))]
    let radius_class = "";
    #[cfg(not(feature = "theme"))]
    let glow_class = if glow.unwrap_or(false) {
        "sd-input--glow".to_string()
    } else {
        String::new()
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

    let icon_class = match (has_left, has_right) {
        (true, true) => "sd-input--has-both-icons",
        (true, false) => "sd-input--has-left-icon",
        (false, true) => "sd-input--has-right-icon",
        (false, false) => "",
    };

    let error_class = if error.is_some() {
        "sd-input--error"
    } else {
        ""
    };

    rsx! {
        div {
            class: "sd-input-wrapper {class_modifiers} {class}",
            if let Some(lbl) = label {
                label { class: "sd-input-label", "{lbl}" }
            }
            div {
                class: "sd-input-container",
                if let Some(licon) = left_icon {
                    if let Some(handler) = on_left_icon_click {
                        button {
                            r#type: "button",
                            class: "sd-input-icon sd-input-icon--left sd-input-icon--interactive",
                            onclick: move |e| {
                                e.stop_propagation();
                                handler.call(e);
                            },
                            RenderSvgIcon { icon: licon, size: 16 }
                        }
                    } else {
                        div {
                            class: "sd-input-icon sd-input-icon--left",
                            style: "{left_pointer_style}",
                            RenderSvgIcon { icon: licon, size: 16 }
                        }
                    }
                }
                input {
                    r#type: "{itype}",
                    value: "{value}",
                    placeholder: placeholder.as_deref().unwrap_or_default(),
                    disabled,
                    class: "sd-input {base_override} {icon_class} {error_class} {radius_class} {glow_class}",
                    oninput: move |e| {
                        *value.write() = e.value();
                    },
                }
                if has_clear {
                    button {
                        r#type: "button",
                        class: "sd-input-icon sd-input-icon--right sd-input-icon--interactive",
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
                            onclick: move |e| {
                                e.stop_propagation();
                                handler.call(e);
                            },
                            RenderSvgIcon { icon: ricon, size: 16 }
                        }
                    } else {
                        div {
                            class: "sd-input-icon sd-input-icon--right",
                            style: "{right_pointer_style}",
                            RenderSvgIcon { icon: ricon, size: 16 }
                        }
                    }
                }
            }
            if let Some(err) = error {
                span { class: "sd-input-error", "{err}" }
            } else if let Some(help) = help_text {
                span { class: "sd-input-help", "{help}" }
            }
        }
    }
}
