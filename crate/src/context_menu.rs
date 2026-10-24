use crate::icon::{Icon, IconKind};
use dioxus::prelude::*;

/// Shared context for [`ContextMenu`] and its subcomponents.
#[derive(Clone, Copy, PartialEq)]
pub struct ContextMenuContext {
    pub is_open: Signal<bool>,
    pub coords: Signal<(f64, f64)>,
}

/// Context menu container handling right-click activation and pointer-anchored placement.
#[component]
pub fn ContextMenu(
    /// Additional CSS classes.
    #[props(default)]
    class: String,
    /// Children: should contain [`ContextMenuTrigger`] and [`ContextMenuContent`].
    children: Element,
) -> Element {
    let is_open = use_signal(|| false);
    let coords = use_signal(|| (0.0, 0.0));

    use_context_provider(|| ContextMenuContext { is_open, coords });

    rsx! {
        div {
            class: "sd-context-menu-wrapper {class}",
            {children}
        }
    }
}

/// Trigger area that intercepts right-click events to open the [`ContextMenu`].
#[component]
pub fn ContextMenuTrigger(
    /// Additional CSS classes.
    #[props(default)]
    class: String,
    /// Trigger content or interactive surface.
    children: Element,
) -> Element {
    let mut ctx = use_context::<ContextMenuContext>();

    rsx! {
        div {
            class: "sd-context-menu-trigger-wrapper {class}",
            oncontextmenu: move |e| {
                e.prevent_default();
                let mouse_coords = e.client_coordinates();
                ctx.coords.set((mouse_coords.x, mouse_coords.y));
                ctx.is_open.set(true);
            },
            {children}
        }
    }
}

/// Menu panel displayed at pointer coordinates when [`ContextMenu`] is triggered.
#[component]
pub fn ContextMenuContent(
    /// Additional CSS classes.
    #[props(default)]
    class: String,
    /// Perimetric optical glow.
    #[props(default)]
    glow: Option<bool>,
    /// Menu items, separators, and actions.
    children: Element,
) -> Element {
    let mut ctx = use_context::<ContextMenuContext>();
    #[cfg(feature = "theme")]
    let theme_val = crate::theme::use_theme()();
    let is_open = *ctx.is_open.read();

    if !is_open {
        return rsx! {};
    }

    let (x, y) = *ctx.coords.read();

    #[cfg(feature = "theme")]
    let base_override = theme_val.resolve_component_base("context_menu").unwrap_or("");
    #[cfg(feature = "theme")]
    let glow_class = theme_val.resolve_elevation_class("context_menu", glow);
    #[cfg(not(feature = "theme"))]
    let base_override = "";
    #[cfg(not(feature = "theme"))]
    let glow_class = if glow == Some(true) { "sd-context-menu--glow" } else { "" };

    rsx! {
        // Transparent backdrop to close menu on click outside
        div {
            class: "sd-context-menu-backdrop",
            onclick: move |_| {
                ctx.is_open.set(false);
            },
            oncontextmenu: move |e| {
                e.prevent_default();
                ctx.is_open.set(false);
            },
        }
        div {
            role: "menu",
            tabindex: "-1",
            class: "sd-context-menu-content {glow_class} {base_override} {class}",
            style: "left: {x}px; top: {y}px;",
            onkeydown: move |e: KeyboardEvent| {
                if e.key() == Key::Escape {
                    ctx.is_open.set(false);
                }
            },
            {children}
        }
    }
}

/// Action item inside a [`ContextMenu`].
#[component]
pub fn ContextMenuItem(
    /// Primary label of the item.
    label: String,
    /// Optional leading icon.
    #[props(default)]
    icon: Option<IconKind>,
    /// Optional keyboard shortcut label (e.g. "Ctrl+C", "⌘D").
    #[props(default)]
    shortcut: Option<String>,
    /// Whether the action is disabled.
    #[props(default)]
    disabled: bool,
    /// Whether the action represents a destructive operation.
    #[props(default)]
    danger: bool,
    /// Optional click event handler.
    #[props(default)]
    onclick: Option<EventHandler<()>>,
    /// Additional CSS classes.
    #[props(default)]
    class: String,
) -> Element {
    let mut ctx = use_context::<ContextMenuContext>();

    let disabled_class = if disabled {
        "sd-context-menu-item--disabled"
    } else {
        ""
    };
    let danger_class = if danger {
        "sd-context-menu-item--danger"
    } else {
        ""
    };

    rsx! {
        button {
            r#type: "button",
            role: "menuitem",
            disabled: disabled,
            class: "sd-context-menu-item {disabled_class} {danger_class} {class}",
            onclick: move |_| {
                if !disabled {
                    if let Some(h) = onclick {
                        h.call(());
                    }
                    ctx.is_open.set(false);
                }
            },
            div { class: "sd-context-menu-item__main",
                if let Some(ic) = icon {
                    Icon { kind: ic, size: 14 }
                }
                span { "{label}" }
            }
            if let Some(sc) = shortcut {
                span { class: "sd-context-menu-shortcut", "{sc}" }
            }
        }
    }
}

/// Thin horizontal divider for separating item groups inside [`ContextMenu`].
#[component]
pub fn ContextMenuSeparator() -> Element {
    rsx! {
        div { class: "sd-context-menu-separator" }
    }
}

/// Checkable toggle item inside [`ContextMenu`].
#[component]
pub fn ContextMenuCheckboxItem(
    /// Current checked state.
    checked: bool,
    /// Primary label.
    label: String,
    /// Optional keyboard shortcut label.
    #[props(default)]
    shortcut: Option<String>,
    /// Optional toggle change callback.
    #[props(default)]
    onchange: Option<EventHandler<bool>>,
    /// Additional CSS classes.
    #[props(default)]
    class: String,
) -> Element {
    let mut ctx = use_context::<ContextMenuContext>();

    rsx! {
        button {
            r#type: "button",
            role: "menuitemcheckbox",
            "aria-checked": "{checked}",
            class: "sd-context-menu-item {class}",
            onclick: move |_| {
                if let Some(h) = onchange {
                    h.call(!checked);
                }
                ctx.is_open.set(false);
            },
            div { class: "sd-context-menu-item__main",
                span { class: "sd-context-menu-item__check",
                    if checked {
                        Icon { kind: IconKind::Check, size: 13 }
                    }
                }
                span { "{label}" }
            }
            if let Some(sc) = shortcut {
                span { class: "sd-context-menu-shortcut", "{sc}" }
            }
        }
    }
}
