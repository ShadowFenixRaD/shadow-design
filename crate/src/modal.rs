use crate::close_button::{CloseButton, CloseButtonSize, CloseButtonVariant};
use crate::locale::use_locale;
use dioxus::prelude::*;

/// Preset width size for the modal dialog.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum ModalSize {
    Sm,
    #[default]
    Md,
    Lg,
    Xl,
    Xxl,
}

impl ModalSize {
    #[must_use]
    pub fn class_name(&self) -> &'static str {
        match self {
            Self::Sm => "sd-modal-dialog--sm",
            Self::Md => "sd-modal-dialog--md",
            Self::Lg => "sd-modal-dialog--lg",
            Self::Xl => "sd-modal-dialog--xl",
            Self::Xxl => "sd-modal-dialog--2xl",
        }
    }
}

/// Modal dialog component that renders in an overlay with APG Dialog pattern accessibility.
#[component]
pub fn Modal(
    /// Controls whether the modal dialog is currently visible.
    is_open: Signal<bool>,
    /// Optional unique identifier for the modal.
    #[props(default)]
    id: Option<String>,
    /// Header title for the modal dialog.
    #[props(default)]
    title: Option<String>,
    /// Accessible label when no visible title is provided.
    #[props(default)]
    aria_label: Option<String>,
    /// Preset width size for the dialog (Sm, Md, Lg, Xl, Xxl). Defaults to Md.
    #[props(default)]
    size: Option<ModalSize>,
    /// Whether pressing the Escape key dismisses the modal. Defaults to true.
    #[props(default = true)]
    close_on_escape: bool,
    /// Whether clicking the backdrop dismisses the modal. Defaults to true.
    #[props(default = true)]
    close_on_backdrop: bool,
    /// Accessible label for the close button.
    #[props(default)]
    close_label: Option<String>,
    /// Perimetric optical glow coordinated with the active theme.
    #[props(default)]
    glow: Option<bool>,
    /// Additional CSS or SCSS classes.
    #[props(default)]
    class: String,
    /// Dialog contents.
    children: Element,
) -> Element {
    #[cfg(feature = "theme")]
    let theme_val = crate::theme::use_theme()();

    let open = *is_open.read();
    let locale = use_locale();
    let modal_id = crate::a11y::use_unique_id("sd-modal", id);
    let title_id = format!("{modal_id}-title");

    crate::a11y::use_body_scroll_lock(is_open);

    if !open {
        return rsx! {};
    }

    #[cfg(feature = "theme")]
    let base_override = theme_val.resolve_component_base("modal").unwrap_or("");
    #[cfg(feature = "theme")]
    let class_modifiers = theme_val.resolve_class_modifiers("modal", &class);
    #[cfg(feature = "theme")]
    let glow_class = theme_val.resolve_elevation_class("modal", glow);

    #[cfg(not(feature = "theme"))]
    let base_override = "";
    #[cfg(not(feature = "theme"))]
    let class_modifiers = String::new();
    #[cfg(not(feature = "theme"))]
    let glow_class = if glow == Some(true) {
        "sd-modal-dialog--glow".to_string()
    } else {
        String::new()
    };

    #[cfg(feature = "animation")]
    let anim_class = crate::animation::Animations::MODAL_ENTER;
    #[cfg(not(feature = "animation"))]
    let anim_class = "animate-modal-enter";

    let close_btn_label = close_label.unwrap_or_else(|| locale.close.clone());

    rsx! {
        div {
            class: "sd-modal-backdrop",
            tabindex: "-1",
            onclick: move |_| {
                if close_on_backdrop {
                    *is_open.write() = false;
                }
            },
            onkeydown: move |e: KeyboardEvent| {
                if close_on_escape && e.key() == Key::Escape {
                    e.stop_propagation();
                    *is_open.write() = false;
                }
            },
            div {
                role: "dialog",
                id: "{modal_id}",
                tabindex: "-1",
                "aria-modal": "true",
                "aria-labelledby": if title.is_some() { Some(title_id.as_str()) } else { None },
                "aria-label": if title.is_none() { aria_label.as_deref() } else { None },
                class: "sd-modal-dialog {size.unwrap_or_default().class_name()} {base_override} {glow_class} {anim_class} {class_modifiers} {class}",
                onmounted: move |m| {
                    let data = m.data();
                    spawn(async move {
                        let _ = data.set_focus(true).await;
                    });
                },
                onkeydown: {
                    let modal_id = modal_id.clone();
                    move |e: KeyboardEvent| {
                        if e.key() == Key::Tab {
                            let shift = e.modifiers().contains(Modifiers::SHIFT);
                            let mid = modal_id.clone();
                            e.prevent_default();
                            spawn(async move {
                                let script = format!(
                                    r#"
                                    const dialog = document.getElementById("{}");
                                    if (!dialog) return;
                                    const focusable = dialog.querySelectorAll('button:not([disabled]), [href], input:not([disabled]), select:not([disabled]), textarea:not([disabled]), [tabindex]:not([tabindex="-1"])');
                                    if (focusable.length === 0) return;
                                    const first = focusable[0];
                                    const last = focusable[focusable.length - 1];
                                    if ({}) {{
                                        if (document.activeElement === first) {{
                                            last.focus();
                                        }} else {{
                                            const idx = Array.prototype.indexOf.call(focusable, document.activeElement);
                                            if (idx > 0) focusable[idx - 1].focus();
                                            else last.focus();
                                        }}
                                    }} else {{
                                        if (document.activeElement === last) {{
                                            first.focus();
                                        }} else {{
                                            const idx = Array.prototype.indexOf.call(focusable, document.activeElement);
                                            if (idx >= 0 && idx < focusable.length - 1) focusable[idx + 1].focus();
                                            else first.focus();
                                        }}
                                    }}
                                    "#,
                                    mid,
                                    if shift { "true" } else { "false" }
                                );
                                let _ = document::eval(&script).await;
                            });
                        }
                    }
                },
                onclick: move |e| e.stop_propagation(),
                div {
                    class: "sd-modal-dialog__header",
                    if let Some(t) = &title {
                        h3 {
                            id: "{title_id}",
                            class: "sd-modal-dialog__title",
                            "{t}"
                        }
                    }
                    CloseButton {
                        size: CloseButtonSize::Md,
                        variant: CloseButtonVariant::Subtle,
                        aria_label: close_btn_label,
                        class: "sd-modal-dialog__close",
                        onclick: move |_| *is_open.write() = false,
                    }
                }
                div { class: "sd-modal-dialog__body", {children} }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_modal_size_class_names() {
        assert_eq!(ModalSize::Sm.class_name(), "sd-modal-dialog--sm");
        assert_eq!(ModalSize::Md.class_name(), "sd-modal-dialog--md");
        assert_eq!(ModalSize::Lg.class_name(), "sd-modal-dialog--lg");
        assert_eq!(ModalSize::Xl.class_name(), "sd-modal-dialog--xl");
        assert_eq!(ModalSize::Xxl.class_name(), "sd-modal-dialog--2xl");
        assert_eq!(ModalSize::default(), ModalSize::Md);
    }
}
