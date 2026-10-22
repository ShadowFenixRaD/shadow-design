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
    rsx! {
        div { class: "sd-speed-dial-action-item",
            if let Some(l) = label {
                span { class: "sd-speed-dial-label", "{l}" }
            }
            button {
                r#type: "button",
                class: "sd-speed-dial-btn {class}",
                onclick: move |_| {
                    if let Some(h) = onclick {
                        h.call(());
                    }
                },
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
    /// Whether the speed dial is fixed to the viewport corner. Defaults to false.
    #[props(default = false)]
    fixed: bool,
    /// Additional CSS classes.
    #[props(default)]
    class: String,
    /// Secondary action items (e.g. [`SpeedDialAction`]).
    children: Element,
) -> Element {
    let mut internal_open = use_signal(|| false);
    let is_open = if let Some(sig) = opened {
        *sig.read()
    } else {
        *internal_open.read()
    };

    let dir_class = direction.class_name();
    let fixed_class = if fixed { "sd-speed-dial--fixed" } else { "" };
    let open_class = if is_open { "sd-fab--open" } else { "" };

    let toggle = move |_| {
        if let Some(mut sig) = opened {
            let next = !*sig.read();
            sig.set(next);
        } else {
            internal_open.toggle();
        }
    };

    rsx! {
        div { class: "sd-speed-dial {dir_class} {fixed_class} {class}",
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
