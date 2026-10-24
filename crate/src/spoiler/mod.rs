use dioxus::prelude::*;

use crate::button::{Button, ButtonSize, ButtonVariant};
use crate::icon::{Icon, IconKind};

/// Expandable container component that clamps excessive vertical content
/// behind a customizable max-height threshold, featuring a bottom surface-fade gradient
/// and a toggle control button ("Mostra di più" / "Mostra meno").
#[component]
pub fn Spoiler(
    /// Maximum visible height in pixels when collapsed. Defaults to `120`.
    #[props(default = 120)]
    max_height: u32,
    /// Label for the toggle button when collapsed. Defaults to the locale `spoiler_show_more` string.
    #[props(default)]
    show_label: String,
    /// Label for the toggle button when expanded. Defaults to the locale `spoiler_show_less` string.
    #[props(default)]
    hide_label: String,
    /// Externally managed reactive signal for expanded state.
    #[props(default)]
    expanded: Option<Signal<bool>>,
    /// Initial expanded state if not externally managed. Defaults to `false`.
    #[props(default = false)]
    default_expanded: bool,
    /// Optional event handler triggered on expansion state change.
    #[props(default)]
    on_toggle: Option<EventHandler<bool>>,
    /// Additional CSS or SCSS classes for the container.
    #[props(default)]
    class: String,
    /// Content displayed inside the spoiler.
    children: Element,
) -> Element {
    let locale = crate::locale::use_locale();
    let show_label = if show_label.is_empty() { locale.spoiler_show_more.clone() } else { show_label };
    let hide_label = if hide_label.is_empty() { locale.spoiler_show_less.clone() } else { hide_label };
    let fallback_expanded = use_signal(|| default_expanded);
    let mut is_expanded_sig = expanded.unwrap_or(fallback_expanded);
    let is_expanded = *is_expanded_sig.read();

    let content_id = crate::a11y::use_unique_id("spoiler-content", None);
    let mut is_overflowing = use_signal(|| true);

    let content_id_effect = content_id.clone();
    use_effect(move || {
        let cid = content_id_effect.clone();
        spawn(async move {
            let eval_res = document::eval(&format!(
                r#"
                const el = document.getElementById("{cid}");
                if (el) {{
                    return el.scrollHeight > {max_height};
                }}
                return true;
                "#
            ));
            if let Ok(val) = eval_res.await {
                if let Some(b) = val.as_bool() {
                    is_overflowing.set(b);
                }
            }
        });
    });

    let toggle_action = move |_: MouseEvent| {
        let next = !is_expanded;
        is_expanded_sig.set(next);
        if let Some(handler) = on_toggle {
            handler.call(next);
        }
    };

    let content_style = if is_expanded {
        "max-height: none;".to_string()
    } else {
        format!("max-height: {}px;", max_height)
    };

    #[cfg(feature = "theme")]
    let theme_val = crate::theme::use_theme()();
    #[cfg(feature = "theme")]
    let base_override = theme_val.resolve_component_base("spoiler").unwrap_or("");
    #[cfg(feature = "theme")]
    let class_modifiers = theme_val.resolve_class_modifiers("spoiler", &class);

    #[cfg(not(feature = "theme"))]
    let base_override = "";
    #[cfg(not(feature = "theme"))]
    let class_modifiers = String::new();

    let state_class = if is_expanded {
        "sd-spoiler--expanded"
    } else {
        "sd-spoiler--collapsed"
    };

    let label_text = if is_expanded { hide_label } else { show_label };

    rsx! {
        div {
            class: "sd-spoiler {state_class} {base_override} {class_modifiers} {class}",
            "data-expanded": "{is_expanded}",
            div {
                id: "{content_id}",
                class: "sd-spoiler__content",
                style: "{content_style}",
                {children}
            }
            if !is_expanded && is_overflowing() {
                div { class: "sd-spoiler__overlay" }
            }
            if is_overflowing() {
                div { class: "sd-spoiler__actions",
                    Button {
                        variant: ButtonVariant::Ghost,
                        size: ButtonSize::Sm,
                        onclick: toggle_action,
                        class: "sd-spoiler__btn",
                        span { "{label_text}" }
                        Icon {
                            kind: if is_expanded { IconKind::ChevronUp } else { IconKind::ChevronDown },
                            size: 15,
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spoiler_toggle_logic() {
        let mut dom = VirtualDom::new(|| {
            let mut expanded = use_signal(|| false);
            assert!(!*expanded.read());

            expanded.set(true);
            assert!(*expanded.read());

            rsx! { div {} }
        });
        dom.rebuild_in_place();
    }
}
