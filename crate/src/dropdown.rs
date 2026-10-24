use crate::icon::{Icon, IconKind};
use dioxus::prelude::*;

#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub enum DropdownPlacement {
    #[default]
    BottomRight,
    BottomLeft,
    TopRight,
    TopLeft,
}

impl DropdownPlacement {
    #[must_use]
    pub fn scss_class(&self) -> &'static str {
        match self {
            DropdownPlacement::BottomRight => "sd-dropdown-menu--bottom-right",
            DropdownPlacement::BottomLeft => "sd-dropdown-menu--bottom-left",
            DropdownPlacement::TopRight => "sd-dropdown-menu--top-right",
            DropdownPlacement::TopLeft => "sd-dropdown-menu--top-left",
        }
    }
}

/// Context provided to child [`DropdownItem`] components to allow automatic menu closing.
#[derive(Clone, Copy, PartialEq)]
pub struct DropdownCtx {
    pub is_open: Signal<bool>,
    pub close_on_click: bool,
}

/// Dropdown menu container adhering to the APG Menu Button pattern.
#[component]
pub fn Dropdown(
    /// Reactive signal controlling open/closed state of the dropdown.
    is_open: Signal<bool>,
    /// Trigger element (e.g. Button) that activates the dropdown menu.
    trigger: Element,
    /// Optional unique identifier for the dropdown.
    #[props(default)]
    id: Option<String>,
    /// Menu placement anchor relative to the trigger.
    #[props(default)]
    placement: DropdownPlacement,
    /// Whether clicking a DropdownItem closes the menu automatically. Defaults to true.
    #[props(default = true)]
    close_on_click: bool,
    /// Perimetric optical glow coordinated with the active theme.
    #[props(default)]
    glow: Option<bool>,
    /// Additional CSS or SCSS classes.
    #[props(default)]
    class: String,
    /// Menu items and dividers.
    children: Element,
) -> Element {
    let mut trigger_mount = use_signal(|| None::<std::rc::Rc<MountedData>>);
    let open = *is_open.read();

    use_context_provider(|| DropdownCtx { is_open, close_on_click });

    #[cfg(feature = "theme")]
    let theme_val = crate::theme::use_theme()();
    #[cfg(feature = "theme")]
    let base_override = theme_val.resolve_component_base("dropdown").unwrap_or("");
    #[cfg(feature = "theme")]
    let class_modifiers = theme_val.resolve_class_modifiers("dropdown", &class);
    #[cfg(feature = "theme")]
    let glow_class = theme_val.resolve_elevation_class("dropdown", glow);

    #[cfg(not(feature = "theme"))]
    let base_override = "";
    #[cfg(not(feature = "theme"))]
    let class_modifiers = String::new();
    #[cfg(not(feature = "theme"))]
    let glow_class = if glow == Some(true) {
        "sd-dropdown-menu--glow".to_string()
    } else {
        String::new()
    };

    let dropdown_id = crate::a11y::use_unique_id("sd-dropdown", id); // sd-lint-allow: id prefix, not a class
    let trigger_id = format!("{dropdown_id}-trigger");
    let menu_id = format!("{dropdown_id}-menu");

    let open_container_class = if open { "sd-dropdown-container--open" } else { "" };
    let placement_class = placement.scss_class();

    #[cfg(feature = "animation")]
    let anim_class = crate::animation::Animations::DROPDOWN_ENTER;
    #[cfg(not(feature = "animation"))]
    let anim_class = "animate-dropdown-enter";

    let focus_trigger = move || {
        if let Some(m) = trigger_mount() {
            spawn(async move {
                let _ = m.set_focus(true).await;
            });
        }
    };

    let handle_menu_keydown = move |e: KeyboardEvent| {
        match e.key() {
            Key::Escape => {
                e.prevent_default();
                *is_open.write() = false;
                focus_trigger();
            }
            Key::Tab => {
                *is_open.write() = false;
            }
            _ => {}
        }
    };

    rsx! {
        div {
            id: "{dropdown_id}",
            class: "sd-dropdown-container {open_container_class} {class_modifiers} {class}",
            onfocusout: move |_| {
                let cid = dropdown_id.clone();
                spawn(async move {
                    crate::util::sleep_ms(50).await;
                    let script = format!(
                        r#"
                        const el = document.getElementById("{}");
                        if (el && !el.contains(document.activeElement)) {{
                            return true;
                        }}
                        return false;
                        "#,
                        cid
                    );
                    if let Ok(res) = document::eval(&script).await {
                        if res.as_bool().unwrap_or(false) {
                            *is_open.write() = false;
                        }
                    }
                });
            },
            div {
                id: "{trigger_id}",
                onmounted: move |m| trigger_mount.set(Some(m.data())),
                onclick: move |_| {
                    let next = !*is_open.read();
                    *is_open.write() = next;
                },
                {trigger}
            }
            if open {
                div {
                    role: "menu",
                    id: "{menu_id}",
                    tabindex: "-1",
                    "aria-labelledby": "{trigger_id}",
                    class: "sd-dropdown-menu {base_override} {placement_class} {glow_class} {anim_class}",
                    onclick: move |e| e.stop_propagation(),
                    onkeydown: handle_menu_keydown,
                    {children}
                }
            }
        }
    }
}

/// Action item rendered inside a [`Dropdown`] menu.
#[component]
pub fn DropdownItem(
    /// Optional leading icon for the menu item.
    #[props(default)]
    icon: Option<IconKind>,
    /// Renders the item in danger/destructive styling.
    #[props(default)]
    danger: bool,
    /// Disables user interaction with this item.
    #[props(default)]
    disabled: bool,
    /// Additional CSS classes.
    #[props(default)]
    class: String,
    /// Click event handler.
    onclick: Option<EventHandler<MouseEvent>>,
    /// Item label contents.
    children: Element,
) -> Element {
    let ctx = try_use_context::<DropdownCtx>();
    let danger_class = if danger { "sd-dropdown-item--danger" } else { "" };
    let disabled_class = if disabled { "sd-dropdown-item--disabled" } else { "" };

    #[cfg(feature = "theme")]
    let theme_val = crate::theme::use_theme()();
    #[cfg(feature = "theme")]
    let base_override = theme_val.resolve_component_base("dropdown-item").unwrap_or("");
    #[cfg(feature = "theme")]
    let class_modifiers = theme_val.resolve_class_modifiers("dropdown-item", &class);

    #[cfg(not(feature = "theme"))]
    let base_override = "";
    #[cfg(not(feature = "theme"))]
    let class_modifiers = String::new();

    rsx! {
        button {
            r#type: "button",
            role: "menuitem",
            tabindex: "-1",
            disabled: disabled,
            "aria-disabled": if disabled { "true" } else { "false" },
            class: "sd-dropdown-item {base_override} {danger_class} {disabled_class} {class_modifiers} {class}",
            onmousedown: move |e| e.prevent_default(),
            onclick: move |e| {
                if !disabled {
                    if let Some(h) = onclick.as_ref() {
                        h.call(e);
                    }
                    if let Some(mut c) = ctx {
                        if c.close_on_click {
                            *c.is_open.write() = false;
                        }
                    }
                }
            },
            if let Some(i) = icon {
                span {
                    class: "sd-dropdown-item__icon",
                    "aria-hidden": "true",
                    Icon { kind: i, size: 15 }
                }
            }
            span { class: "sd-dropdown-item__text", {children} }
        }
    }
}

/// Visual horizontal separator between dropdown menu items.
#[component]
pub fn DropdownDivider() -> Element {
    rsx! {
        div {
            role: "separator",
            class: "sd-dropdown-divider"
        }
    }
}
