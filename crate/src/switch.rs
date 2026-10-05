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
    let (track_checked_modifier, glow_modifier) = match theme().primary_color {
        crate::theme::PrimaryColor::Violet => (
            "sd-switch__track--checked",
            if has_glow { "sd-switch__track--glow" } else { "" },
        ),
        crate::theme::PrimaryColor::Obsidian => (
            "sd-switch__track--checked-obsidian",
            if has_glow { "sd-switch__track--glow-obsidian" } else { "" },
        ),
        crate::theme::PrimaryColor::Crimson => (
            "sd-switch__track--checked-crimson",
            if has_glow { "sd-switch__track--glow-crimson" } else { "" },
        ),
        crate::theme::PrimaryColor::Cyan => (
            "sd-switch__track--checked-cyan",
            if has_glow { "sd-switch__track--glow-cyan" } else { "" },
        ),
        crate::theme::PrimaryColor::Emerald => (
            "sd-switch__track--checked-emerald",
            if has_glow { "sd-switch__track--glow-emerald" } else { "" },
        ),
    };

    #[cfg(not(feature = "theme"))]
    let (track_checked_modifier, glow_modifier) = (
        "sd-switch__track--checked",
        if has_glow { "sd-switch__track--glow" } else { "" },
    );

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
            class: "sd-switch {disabled_class} {class}",
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
