use dioxus::prelude::*;

#[component]
pub fn Switch(
    checked: Signal<bool>,
    #[props(default)] label: Option<String>,
    #[props(default)] description: Option<String>,
    #[props(default)] disabled: bool,
    #[props(default)] glow: Option<bool>,
    #[props(default)] id: Option<String>,
    #[props(default)] class: String,
    #[props(default)] on_change: Option<EventHandler<bool>>,
) -> Element {
    let is_checked = *checked.read();
    let switch_id = crate::a11y::use_unique_id("sd-switch", id);
    let label_id = if label.is_some() {
        Some(format!("{}-label", switch_id))
    } else {
        None
    };
    let desc_id = if description.is_some() {
        Some(format!("{}-desc", switch_id))
    } else {
        None
    };

    #[cfg(feature = "theme")]
    let theme = crate::theme::use_theme();
    #[cfg(feature = "theme")]
    let theme_val = theme.read();
    #[cfg(feature = "theme")]
    let track_checked_modifier = "sd-switch__track--checked".to_string();
    #[cfg(feature = "theme")]
    let glow_modifier = if is_checked {
        theme_val.resolve_elevation_class("switch", glow)
    } else {
        String::new()
    };
    #[cfg(feature = "theme")]
    let base_override = theme_val.resolve_component_base("switch").unwrap_or("");
    #[cfg(feature = "theme")]
    let class_modifiers = theme_val.resolve_class_modifiers("switch", &class);

    #[cfg(not(feature = "theme"))]
    let base_override = "";
    #[cfg(not(feature = "theme"))]
    let class_modifiers = String::new();
    #[cfg(not(feature = "theme"))]
    let track_checked_modifier = "sd-switch__track--checked".to_string();
    #[cfg(not(feature = "theme"))]
    let glow_modifier = if is_checked && glow.unwrap_or(false) {
        "sd-switch__track--glow".to_string()
    } else {
        String::new()
    };

    let track_active_class = if is_checked {
        format!("{track_checked_modifier} {glow_modifier}")
    } else {
        String::new()
    };

    let thumb_active_class = if is_checked {
        "sd-switch__thumb--checked"
    } else {
        ""
    };

    let disabled_class = if disabled {
        "sd-switch--disabled"
    } else {
        ""
    };

    let do_toggle = move || {
        if !disabled {
            let next = !*checked.read();
            let mut checked = checked;
            *checked.write() = next;
            if let Some(handler) = on_change.as_ref() {
                handler.call(next);
            }
        }
    };

    rsx! {
        div {
            class: "sd-switch {base_override} {disabled_class} {class_modifiers} {class}",
            button {
                r#type: "button",
                role: "switch",
                id: "{switch_id}",
                class: "sd-switch__track {track_active_class}",
                disabled: disabled,
                "aria-checked": if is_checked { "true" } else { "false" },
                "aria-labelledby": label_id.as_deref(),
                "aria-describedby": desc_id.as_deref(),
                onclick: move |_| do_toggle(),
                span {
                    class: "sd-switch__thumb {thumb_active_class}",
                    "aria-hidden": "true",
                }
            }
            if label.is_some() || description.is_some() {
                div {
                    class: "sd-switch__label-group",
                    onclick: move |_| do_toggle(),
                    if let Some(lbl) = label {
                        span {
                            id: label_id.as_deref(),
                            class: "sd-switch__label",
                            "{lbl}"
                        }
                    }
                    if let Some(desc) = description {
                        span {
                            id: desc_id.as_deref(),
                            class: "sd-switch__description",
                            "{desc}"
                        }
                    }
                }
            }
        }
    }
}
