//! Accessibility and shared semantic helpers for shadow-design.

use std::sync::atomic::{AtomicUsize, Ordering};
use dioxus::prelude::*;

static NEXT_ID: AtomicUsize = AtomicUsize::new(1);

/// Generates a stable unique ID for a component instance, or uses `user_id` if provided.
#[must_use]
pub fn use_unique_id(prefix: &str, user_id: Option<String>) -> String {
    use_hook(move || {
        if let Some(id) = user_id {
            id
        } else {
            format!("sd-{prefix}-{}", NEXT_ID.fetch_add(1, Ordering::Relaxed))
        }
    })
}

/// Shared size scale for form controls (RS-25, DS-15).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ControlSize {
    Sm,
    #[default]
    Md,
    Lg,
}

impl ControlSize {
    #[must_use]
    pub fn class_modifier(&self) -> &'static str {
        match self {
            Self::Sm => "--sm",
            Self::Md => "--md",
            Self::Lg => "--lg",
        }
    }
}

/// Helper component to render headings with accessible ARIA levels (A11Y-21).
#[component]
pub fn AriaHeading(
    level: Option<u8>,
    #[props(default)] id: Option<String>,
    #[props(default)] class: String,
    children: Element,
) -> Element {
    let id_attr = id.as_deref();
    if let Some(raw_lvl) = level {
        let lvl = raw_lvl.clamp(1, 6);
        rsx! {
            div {
                id: id_attr,
                role: "heading",
                aria_level: "{lvl}",
                class: "{class}",
                {children}
            }
        }
    } else {
        rsx! {
            div {
                id: id_attr,
                class: "{class}",
                {children}
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_control_size_modifiers() {
        assert_eq!(ControlSize::Sm.class_modifier(), "--sm");
        assert_eq!(ControlSize::Md.class_modifier(), "--md");
        assert_eq!(ControlSize::Lg.class_modifier(), "--lg");
    }
}

/// Manages body scroll locking with reference counting to support multiple simultaneous overlays.
pub fn use_body_scroll_lock(is_open: Signal<bool>) {
    let mut was_locked = use_signal(|| false);

    use_effect(move || {
        let open = *is_open.read();
        let locked = *was_locked.read();

        if open && !locked {
            was_locked.set(true);
            let _ = document::eval(
                r#"
                const current = parseInt(document.body.dataset.sdScrollLocks || "0", 10);
                document.body.dataset.sdScrollLocks = (current + 1).toString();
                document.body.style.overflow = "hidden";
                "#
            );
        } else if !open && locked {
            was_locked.set(false);
            let _ = document::eval(
                r#"
                const current = parseInt(document.body.dataset.sdScrollLocks || "0", 10);
                const next = Math.max(0, current - 1);
                document.body.dataset.sdScrollLocks = next.toString();
                if (next === 0) {
                    document.body.style.overflow = "";
                }
                "#
            );
        }
    });

    use_drop(move || {
        if *was_locked.peek() {
            let _ = document::eval(
                r#"
                const current = parseInt(document.body.dataset.sdScrollLocks || "0", 10);
                const next = Math.max(0, current - 1);
                document.body.dataset.sdScrollLocks = next.toString();
                if (next === 0) {
                    document.body.style.overflow = "";
                }
                "#
            );
        }
    });
}
