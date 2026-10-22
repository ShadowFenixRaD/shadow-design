use dioxus::prelude::*;

/// Keyboard shortcut keytip hint displayed near commands or interactive controls.
#[component]
pub fn Keytip(
    /// Key sequence to display (e.g. "Alt+K", "S", "1").
    key_sequence: String,
    /// Explicit control over keytip visibility. If None, remains visible.
    #[props(default)]
    visible: Option<bool>,
    /// Whether the keytip is disabled. Defaults to false.
    #[props(default = false)]
    disabled: bool,
    /// Additional CSS classes.
    #[props(default)]
    class: String,
    /// Optional element wrapped by the keytip.
    #[props(default)]
    children: Element,
) -> Element {
    let show = visible.unwrap_or(true) && !disabled;

    rsx! {
        div { class: "sd-keytip-wrapper {class}",
            {children}
            if show {
                span { class: "sd-keytip-badge",
                    "{key_sequence}"
                }
            }
        }
    }
}
