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
    let theme_val = crate::theme::use_theme()();
    #[cfg(feature = "theme")]
    let track_checked_modifier = match &theme_val.primary_color {
        crate::theme::PrimaryColor::Violet => "sd-switch__track--checked".to_string(),
        crate::theme::PrimaryColor::Obsidian => "sd-switch__track--checked-obsidian".to_string(),
        crate::theme::PrimaryColor::Crimson => "sd-switch__track--checked-crimson".to_string(),
        crate::theme::PrimaryColor::Cyan => "sd-switch__track--checked-cyan".to_string(),
        crate::theme::PrimaryColor::Emerald => "sd-switch__track--checked-emerald".to_string(),
        crate::theme::PrimaryColor::Custom { .. } => {
            format!("sd-switch__track--checked-{}", theme_val.primary_color.css_prefix())
        }
    };
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

    let disabled_class = if disabled { "sd-switch--disabled" } else { "" };

    rsx! {
        div {
            class: "sd-switch {base_override} {disabled_class} {class_modifiers} {class}",
            onclick: move |_| {
                if !disabled {
                    let next = !*checked.read();
                    *checked.write() = next;
                }
            },
            div {
                class: "sd-switch__track {track_active_class}",
                span {
                    class: "sd-switch__thumb {thumb_active_class}",
                }
            }
            if label.is_some() || description.is_some() {
                div { class: "sd-switch__label-group",
                    if let Some(lbl) = label {
                        span { class: "sd-switch__label", "{lbl}" }
                    }
                    if let Some(desc) = description {
                        span { class: "sd-switch__description", "{desc}" }
                    }
                }
            }
        }
    }
}
