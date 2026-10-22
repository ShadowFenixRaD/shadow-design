use crate::icon::{Icon, IconKind};
use dioxus::prelude::*;

/// Direction in which the secondary actions fan out.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum SpeedDialDirection {
    #[default]
    Up,
    Down,
    Left,
    Right,
}

impl SpeedDialDirection {
    #[must_use]
    pub fn class_name(&self) -> &'static str {
        match self {
            Self::Up => "sd-speed-dial--up",
            Self::Down => "sd-speed-dial--down",
            Self::Left => "sd-speed-dial--left",
            Self::Right => "sd-speed-dial--right",
        }
    }
}

/// Standalone Floating Action Button (FAB).
#[component]
pub fn Fab(
    /// Icon displayed inside the button.
    icon: IconKind,
    /// Accessible title or tooltip label.
    #[props(default)]
    title: Option<String>,
    /// Optional click handler.
    #[props(default)]
    onclick: Option<EventHandler<()>>,
    /// Additional CSS classes.
    #[props(default)]
    class: String,
) -> Element {
    rsx! {
        button {
            r#type: "button",
            class: "sd-fab {class}",
            title: title,
            onclick: move |_| {
                if let Some(h) = onclick {
                    h.call(());
                }
            },
            Icon { kind: icon, size: 22 }
        }
    }
}

/// Context passed to children of [`SpeedDial`].
#[derive(Clone, Copy, PartialEq)]
pub struct SpeedDialContext {
    pub is_open: Signal<bool>,
    pub close_on_action: bool,
}

/// Secondary action button inside a [`SpeedDial`].
#[component]
pub fn SpeedDialAction(
    /// Icon displayed inside the button.
    icon: IconKind,
    /// Optional text label shown next to the button.
    #[props(default)]
    label: Option<String>,
    /// Click callback.
    #[props(default)]
    onclick: Option<EventHandler<()>>,
    /// Additional CSS classes.
    #[props(default)]
    class: String,
) -> Element {
    let ctx = try_use_context::<SpeedDialContext>();

    let handle_click = move |_| {
        if let Some(h) = onclick {
            h.call(());
        }
        if let Some(mut c) = ctx {
            if c.close_on_action {
                c.is_open.set(false);
            }
        }
    };

    rsx! {
        div { class: "sd-speed-dial-action-item",
            if let Some(l) = label {
                span {
                    class: "sd-speed-dial-label",
                    onclick: handle_click,
                    "{l}"
                }
            }
            button {
                r#type: "button",
                class: "sd-speed-dial-btn {class}",
                onclick: handle_click,
                Icon { kind: icon, size: 18 }
            }
        }
    }
}

/// Expandable Floating Action Button suite revealing secondary contextual actions.
#[component]
pub fn SpeedDial(
    /// Primary icon on the main trigger button. Defaults to [`IconKind::Plus`].
    #[props(default = IconKind::Plus)]
    icon: IconKind,
    /// Optional icon shown when open. Defaults to rotating the primary icon.
    #[props(default)]
    active_icon: Option<IconKind>,
    /// Fan-out direction for secondary actions. Defaults to Up.
    #[props(default)]
    direction: SpeedDialDirection,
    /// Optional external signal controlling open/closed state.
    #[props(default)]
    opened: Option<Signal<bool>>,
    /// Whether clicking an action closes the speed dial. Defaults to true.
    #[props(default = true)]
    close_on_action: bool,
    /// Whether clicking outside the speed dial closes it. Defaults to true.
    #[props(default = true)]
    close_on_click_outside: bool,
    /// Whether the speed dial is fixed to the viewport corner. Defaults to false.
    #[props(default = false)]
    fixed: bool,
    /// Additional CSS classes.
    #[props(default)]
    class: String,
    /// Secondary action items (e.g. [`SpeedDialAction`]).
    children: Element,
) -> Element {
    let internal_open = use_signal(|| false);
    let mut effective_sig = opened.unwrap_or(internal_open);
    let is_open = *effective_sig.read();

    use_context_provider(|| SpeedDialContext {
        is_open: effective_sig,
        close_on_action,
    });

    let dir_class = direction.class_name();
    let fixed_class = if fixed { "sd-speed-dial--fixed" } else { "" };
    let open_container_class = if is_open { "sd-speed-dial--open" } else { "" };
    let open_class = if is_open {
        if active_icon.is_some() {
            "sd-fab--open-custom"
        } else {
            "sd-fab--open"
        }
    } else {
        ""
    };

    let toggle = move |_| {
        let next = !*effective_sig.read();
        effective_sig.set(next);
    };

    rsx! {
        div { class: "sd-speed-dial {dir_class} {fixed_class} {open_container_class} {class}",
            if is_open && close_on_click_outside {
                div {
                    class: "sd-speed-dial-backdrop",
                    onclick: move |_| effective_sig.set(false),
                }
            }

            button {
                r#type: "button",
                class: "sd-fab {open_class}",
                onclick: toggle,
                "aria-expanded": "{is_open}",
                if is_open && active_icon.is_some() {
                    Icon { kind: active_icon.unwrap(), size: 22 }
                } else {
                    Icon { kind: icon, size: 22 }
                }
            }

            if is_open {
                div { class: "sd-speed-dial-actions",
                    {children}
                }
            }
        }
    }
}
