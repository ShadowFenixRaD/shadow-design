use crate::icon::{Icon, IconKind};
use dioxus::prelude::*;

/// Shared context providing base IDs for coordinating `Tab` and `TabPanel` associations.
#[derive(Clone, PartialEq)]
pub struct TabListCtx {
    pub base_id: String,
}

/// Container list for horizontal tab navigation adhering to the APG Tabs pattern.
#[component]
pub fn TabList(
    /// Optional unique identifier for the tab list.
    #[props(default)]
    id: Option<String>,
    /// Accessible label describing the purpose of the tab list.
    #[props(default)]
    aria_label: Option<String>,
    /// Additional CSS or SCSS classes.
    #[props(default)]
    class: String,
    /// Child [`Tab`] elements.
    children: Element,
) -> Element {
    let base_id = crate::a11y::use_unique_id("sd-tabs", id);
    use_context_provider(|| TabListCtx { base_id: base_id.clone() });

    #[cfg(feature = "theme")]
    let theme_val = crate::theme::use_theme()();
    #[cfg(feature = "theme")]
    let base_override = theme_val.resolve_component_base("tabs").unwrap_or("");
    #[cfg(feature = "theme")]
    let class_modifiers = theme_val.resolve_class_modifiers("tabs", &class);

    #[cfg(not(feature = "theme"))]
    let base_override = "";
    #[cfg(not(feature = "theme"))]
    let class_modifiers = String::new();

    rsx! {
        div {
            role: "tablist",
            id: "{base_id}",
            "aria-label": aria_label.as_deref(),
            "aria-orientation": "horizontal",
            class: "sd-tabs-list {base_override} {class_modifiers} {class}",
            {children}
        }
    }
}

/// Individual tab trigger inside a [`TabList`] supporting roving tabindex and keyboard navigation.
#[component]
pub fn Tab(
    /// Identifier matching the corresponding [`TabPanel`].
    value: String,
    /// Reactive signal holding the currently active tab value.
    active: Signal<String>,
    /// Optional leading icon.
    #[props(default)]
    icon: Option<IconKind>,
    /// Optional status or count badge.
    #[props(default)]
    badge: Option<String>,
    /// Disables user interaction with this tab.
    #[props(default)]
    disabled: bool,
    /// Additional CSS classes.
    #[props(default)]
    class: String,
    /// Tab label contents.
    children: Element,
) -> Element {
    let ctx = try_use_context::<TabListCtx>();
    let base_id = ctx.map(|c| c.base_id).unwrap_or_else(|| "sd-tabs".to_string());
    let val_sanitized = value.replace(' ', "-").to_lowercase();
    let tab_id = format!("{base_id}-tab-{val_sanitized}");
    let panel_id = format!("{base_id}-panel-{val_sanitized}");

    let is_active = *active.read() == value;
    let target = value.clone();
    let active_class = if is_active { "sd-tab--active" } else { "" };
    let disabled_class = if disabled { "sd-tab--disabled" } else { "" };

    #[cfg(feature = "theme")]
    let theme_val = crate::theme::use_theme()();
    #[cfg(feature = "theme")]
    let base_override = theme_val.resolve_component_base("tab").unwrap_or("");
    #[cfg(feature = "theme")]
    let class_modifiers = theme_val.resolve_class_modifiers("tab", &class);

    #[cfg(not(feature = "theme"))]
    let base_override = "";
    #[cfg(not(feature = "theme"))]
    let class_modifiers = String::new();

    let handle_keydown = move |e: KeyboardEvent| {
        let navigate_tab = |dir: &'static str| {
            let script = format!(
                r#"
                const tab = document.activeElement;
                if (tab && tab.getAttribute('role') === 'tab') {{
                    const tablist = tab.closest('[role="tablist"]');
                    if (tablist) {{
                        const tabs = Array.from(tablist.querySelectorAll('[role="tab"]:not([disabled])'));
                        const idx = tabs.indexOf(tab);
                        if (tabs.length > 0) {{
                            let target = null;
                            if ('{dir}' === 'next' && idx >= 0) target = tabs[(idx + 1) % tabs.length];
                            else if ('{dir}' === 'prev' && idx >= 0) target = tabs[(idx - 1 + tabs.length) % tabs.length];
                            else if ('{dir}' === 'home') target = tabs[0];
                            else if ('{dir}' === 'end') target = tabs[tabs.length - 1];
                            if (target) {{ target.focus(); target.click(); }}
                        }}
                    }}
                }}
            "#
            );
            let _ = document::eval(&script);
        };

        match e.key() {
            Key::ArrowRight => {
                e.prevent_default();
                navigate_tab("next");
            }
            Key::ArrowLeft => {
                e.prevent_default();
                navigate_tab("prev");
            }
            Key::Home => {
                e.prevent_default();
                navigate_tab("home");
            }
            Key::End => {
                e.prevent_default();
                navigate_tab("end");
            }
            _ => {}
        }
    };

    rsx! {
        button {
            r#type: "button",
            role: "tab",
            id: "{tab_id}",
            tabindex: if is_active && !disabled { "0" } else { "-1" },
            disabled: disabled,
            "aria-selected": "{is_active}",
            "aria-controls": "{panel_id}",
            "aria-disabled": if disabled { "true" } else { "false" },
            class: "sd-tab {base_override} {active_class} {disabled_class} {class_modifiers} {class}",
            onclick: move |_| {
                if !disabled {
                    *active.write() = target.clone();
                }
            },
            onkeydown: handle_keydown,
            if let Some(i) = icon {
                span {
                    class: "sd-tab__icon",
                    "aria-hidden": "true",
                    Icon { kind: i, size: 14 }
                }
            }
            {children}
            if let Some(b) = badge {
                span {
                    class: "sd-tab__badge",
                    span { class: "sd-visually-hidden", " (" }
                    "{b}"
                    span { class: "sd-visually-hidden", ")" }
                }
            }
        }
    }
}

/// Content panel rendered when its `value` matches the active tab.
#[component]
pub fn TabPanel(
    /// Identifier matching the parent [`Tab`].
    value: String,
    /// Reactive signal holding the currently active tab value.
    active: Signal<String>,
    /// If true, inactive panels are kept mounted with `hidden: true` instead of unmounting.
    #[props(default)]
    keep_mounted: bool,
    /// Additional CSS classes.
    #[props(default)]
    class: String,
    /// Panel body contents.
    children: Element,
) -> Element {
    #[cfg(feature = "theme")]
    let theme_val = crate::theme::use_theme()();

    let is_active = *active.read() == value;

    if !is_active && !keep_mounted {
        return rsx! {};
    }

    let ctx = try_use_context::<TabListCtx>();
    let base_id = ctx.map(|c| c.base_id).unwrap_or_else(|| "sd-tabs".to_string());
    let val_sanitized = value.replace(' ', "-").to_lowercase();
    let tab_id = format!("{base_id}-tab-{val_sanitized}");
    let panel_id = format!("{base_id}-panel-{val_sanitized}");

    #[cfg(feature = "theme")]
    let base_override = theme_val.resolve_component_base("tab-panel").unwrap_or("");
    #[cfg(feature = "theme")]
    let class_modifiers = theme_val.resolve_class_modifiers("tab-panel", &class);

    #[cfg(not(feature = "theme"))]
    let base_override = "";
    #[cfg(not(feature = "theme"))]
    let class_modifiers = String::new();

    rsx! {
        div {
            role: "tabpanel",
            id: "{panel_id}",
            tabindex: "0",
            hidden: !is_active,
            "aria-labelledby": "{tab_id}",
            class: "sd-tab-panel {base_override} {class_modifiers} {class}",
            {children}
        }
    }
}
