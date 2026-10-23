use crate::close_button::{CloseButton, CloseButtonSize, CloseButtonVariant};
use crate::locale::use_locale;
use dioxus::prelude::*;

#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub enum DrawerPosition {
    Left,
    #[default]
    Right,
}

#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub enum DrawerSize {
    Sm,
    #[default]
    Md,
    Lg,
}

impl DrawerSize {
    #[must_use]
    pub fn scss_class(&self) -> &'static str {
        match self {
            DrawerSize::Sm => "sd-drawer-panel--sm",
            DrawerSize::Md => "sd-drawer-panel--md",
            DrawerSize::Lg => "sd-drawer-panel--lg",
        }
    }
}

/// Slide-in drawer navigation and overlay panel adhering to the APG Dialog pattern.
#[component]
pub fn Drawer(
    /// Controls whether the drawer is currently open.
    is_open: Signal<bool>,
    /// Optional unique identifier for the drawer.
    #[props(default)]
    id: Option<String>,
    /// Header title for the drawer panel.
    #[props(default)]
    title: Option<String>,
    /// Accessible label when no visible title is provided.
    #[props(default)]
    aria_label: Option<String>,
    /// Screen edge from which the drawer slides in (Left or Right).
    #[props(default)]
    position: DrawerPosition,
    /// Width preset for the drawer panel (Sm, Md, or Lg).
    #[props(default)]
    size: DrawerSize,
    /// Whether pressing the Escape key dismisses the drawer. Defaults to true.
    #[props(default = true)]
    close_on_escape: bool,
    /// Whether clicking the backdrop dismisses the drawer. Defaults to true.
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
    /// Drawer body contents.
    children: Element,
) -> Element {
    #[cfg(feature = "theme")]
    let theme_val = crate::theme::use_theme()();

    let open = *is_open.read();
    let locale = use_locale();
    let drawer_id = crate::a11y::use_unique_id("sd-drawer", id);
    let title_id = format!("{drawer_id}-title");

    use_effect(move || {
        let is_currently_open = *is_open.read();
        if is_currently_open {
            let _ = document::eval("document.body.style.overflow = 'hidden'");
        } else {
            let _ = document::eval("document.body.style.overflow = ''");
        }
    });

    use_drop(move || {
        let _ = document::eval("document.body.style.overflow = ''");
    });

    if !open {
        return rsx! {};
    }

    #[cfg(feature = "theme")]
    let base_override = theme_val.resolve_component_base("drawer").unwrap_or("");
    #[cfg(feature = "theme")]
    let class_modifiers = theme_val.resolve_class_modifiers("drawer", &class);
    #[cfg(feature = "theme")]
    let glow_class = theme_val.resolve_elevation_class("drawer", glow);

    #[cfg(not(feature = "theme"))]
    let base_override = "";
    #[cfg(not(feature = "theme"))]
    let class_modifiers = String::new();
    #[cfg(not(feature = "theme"))]
    let glow_class = if glow.unwrap_or(false) {
        "sd-drawer-panel--glow".to_string()
    } else {
        String::new()
    };

    #[cfg(feature = "animation")]
    let anim_class = match position {
        DrawerPosition::Left => crate::animation::Animations::DRAWER_LEFT,
        DrawerPosition::Right => crate::animation::Animations::DRAWER_RIGHT,
    };
    #[cfg(not(feature = "animation"))]
    let anim_class = match position {
        DrawerPosition::Left => "animate-drawer-slide-left",
        DrawerPosition::Right => "animate-drawer-slide-right",
    };

    let pos_class = match position {
        DrawerPosition::Left => "sd-drawer-panel--left",
        DrawerPosition::Right => "sd-drawer-panel--right",
    };
    let size_class = size.scss_class();
    let close_btn_label = close_label.unwrap_or_else(|| locale.close.clone());

    rsx! {
        div {
            class: "sd-drawer-backdrop",
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
                id: "{drawer_id}",
                tabindex: "-1",
                "aria-modal": "true",
                "aria-labelledby": if title.is_some() { Some(title_id.as_str()) } else { None },
                "aria-label": if title.is_none() { aria_label.as_deref() } else { None },
                class: "sd-drawer-panel {base_override} {pos_class} {size_class} {glow_class} {anim_class} {class_modifiers} {class}",
                onmounted: move |m| {
                    let data = m.data();
                    spawn(async move {
                        let _ = data.set_focus(true).await;
                    });
                },
                onkeydown: {
                    let drawer_id = drawer_id.clone();
                    move |e: KeyboardEvent| {
                        if e.key() == Key::Tab {
                            let shift = e.modifiers().contains(Modifiers::SHIFT);
                            let did = drawer_id.clone();
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
                                    did,
                                    if shift { "true" } else { "false" }
                                );
                                let _ = document::eval(&script).await;
                            });
                        }
                    }
                },
                onclick: move |e| e.stop_propagation(),
                div {
                    class: "sd-drawer__header",
                    if let Some(t) = &title {
                        h3 {
                            id: "{title_id}",
                            class: "sd-drawer__title",
                            "{t}"
                        }
                    }
                    CloseButton {
                        size: CloseButtonSize::Md,
                        variant: CloseButtonVariant::Subtle,
                        aria_label: close_btn_label,
                        class: "sd-drawer__close",
                        onclick: move |_| *is_open.write() = false,
                    }
                }
                div {
                    class: "sd-drawer__body",
                    {children}
                }
            }
        }
    }
}
