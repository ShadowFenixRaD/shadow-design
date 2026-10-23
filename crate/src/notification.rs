use crate::icon::{Icon, IconKind};
use crate::locale::use_locale;
use dioxus::prelude::*;

use crate::util::sleep_ms;

/// Severity variants for system toast notifications.
#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub enum NotificationVariant {
    #[default]
    Info,
    Success,
    Warning,
    Danger,
}

impl NotificationVariant {
    #[must_use]
    pub fn scss_class(&self) -> &'static str {
        match self {
            NotificationVariant::Info => "sd-toast--info",
            NotificationVariant::Success => "sd-toast--success",
            NotificationVariant::Warning => "sd-toast--warning",
            NotificationVariant::Danger => "sd-toast--danger",
        }
    }

    #[must_use]
    pub fn icon_color_class(&self) -> &'static str {
        match self {
            NotificationVariant::Info => "sd-toast__icon--info",
            NotificationVariant::Success => "sd-toast__icon--success",
            NotificationVariant::Warning => "sd-toast__icon--warning",
            NotificationVariant::Danger => "sd-toast__icon--danger",
        }
    }

    #[must_use]
    pub fn glow_class(&self) -> &'static str {
        match self {
            NotificationVariant::Info => "sd-toast--glow-info",
            NotificationVariant::Success => "sd-toast--glow-success",
            NotificationVariant::Warning => "sd-toast--glow-warning",
            NotificationVariant::Danger => "sd-toast--glow-danger",
        }
    }

    #[must_use]
    pub fn default_icon(&self) -> IconKind {
        match self {
            NotificationVariant::Info => IconKind::Info,
            NotificationVariant::Success => IconKind::Check,
            NotificationVariant::Warning => IconKind::AlertTriangle,
            NotificationVariant::Danger => IconKind::AlertCircle,
        }
    }

    #[must_use]
    pub fn prefix(&self) -> &'static str {
        match self {
            NotificationVariant::Info => "Info",
            NotificationVariant::Success => "Success",
            NotificationVariant::Warning => "Warning",
            NotificationVariant::Danger => "Error",
        }
    }
}

/// Represents an individual toast notification item in state.
#[derive(Clone, PartialEq, Debug)]
pub struct NotificationItem {
    pub id: usize,
    pub title: Option<String>,
    pub message: String,
    pub variant: NotificationVariant,
    pub duration: Option<std::time::Duration>,
}

/// Handle allowing components to dispatch and dismiss toast notifications.
#[derive(Clone, Copy, PartialEq)]
pub struct NotificationHandle {
    items: Signal<Vec<NotificationItem>>,
    next_id: Signal<usize>,
}

impl NotificationHandle {
    pub fn info(&mut self, message: impl Into<String>) -> usize {
        self.show_with_duration("Info", message, NotificationVariant::Info, Some(std::time::Duration::from_secs(6)))
    }

    pub fn success(&mut self, message: impl Into<String>) -> usize {
        self.show_with_duration("Success", message, NotificationVariant::Success, Some(std::time::Duration::from_secs(6)))
    }

    pub fn warning(&mut self, message: impl Into<String>) -> usize {
        self.show_with_duration("Warning", message, NotificationVariant::Warning, Some(std::time::Duration::from_secs(6)))
    }

    pub fn danger(&mut self, message: impl Into<String>) -> usize {
        self.show_with_duration("Error", message, NotificationVariant::Danger, None)
    }

    pub fn show(
        &mut self,
        title: impl Into<String>,
        message: impl Into<String>,
        variant: NotificationVariant,
    ) -> usize {
        let default_dur = if variant == NotificationVariant::Danger {
            None
        } else {
            Some(std::time::Duration::from_secs(6))
        };
        self.show_with_duration(title, message, variant, default_dur)
    }

    pub fn show_with_title(
        &mut self,
        title: impl Into<String>,
        message: impl Into<String>,
        variant: NotificationVariant,
    ) -> usize {
        self.show(title, message, variant)
    }

    pub fn show_with_duration(
        &mut self,
        title: impl Into<String>,
        message: impl Into<String>,
        variant: NotificationVariant,
        duration: Option<std::time::Duration>,
    ) -> usize {
        let id = *self.next_id.read();
        *self.next_id.write() += 1;

        let item = NotificationItem {
            id,
            title: Some(title.into()),
            message: message.into(),
            variant,
            duration,
        };

        self.items.write().push(item);
        id
    }

    pub fn dismiss(&mut self, id: usize) {
        self.items.write().retain(|item| item.id != id);
    }

    pub fn clear(&mut self) {
        self.items.write().clear();
    }
}

/// Provides notification state and mounts the global container.
#[component]
pub fn NotificationProvider(
    #[props(default)] aria_label: Option<String>,
    children: Element,
) -> Element {
    let items = use_signal(Vec::<NotificationItem>::new);
    let next_id = use_signal(|| 1usize);
    let handle = NotificationHandle { items, next_id };
    use_context_provider(|| handle);

    rsx! {
        {children}
        NotificationContainer { items, aria_label }
    }
}

/// Hook to access the notification system.
///
/// # Panics
/// Panics if called without an ancestor [`NotificationProvider`].
/// For non-panicking access, use [`try_use_notification`].
#[must_use]
pub fn use_notification() -> NotificationHandle {
    try_use_context::<NotificationHandle>()
        .expect("use_notification must be used within a NotificationProvider")
}

/// Non-panicking version of [`use_notification`].
#[must_use]
pub fn try_use_notification() -> Option<NotificationHandle> {
    try_use_context::<NotificationHandle>()
}

#[component]
fn NotificationContainer(
    mut items: Signal<Vec<NotificationItem>>,
    #[props(default)] aria_label: Option<String>,
) -> Element {
    let current_items = items.read().clone();
    let polite_items: Vec<NotificationItem> = current_items
        .iter()
        .filter(|i| i.variant != NotificationVariant::Danger)
        .cloned()
        .collect();
    let assertive_items: Vec<NotificationItem> = current_items
        .iter()
        .filter(|i| i.variant == NotificationVariant::Danger)
        .cloned()
        .collect();

    rsx! {
        div {
            class: "sd-notification-container",
            role: "region",
            "aria-label": aria_label.as_deref().unwrap_or("Notifications"),
            div {
                class: "sd-notification-stack",
                role: "status",
                "aria-live": "polite",
                for item in polite_items {
                    NotificationToast {
                        key: "{item.id}",
                        item: item.clone(),
                        on_dismiss: move |_| {
                            items.write().retain(|i| i.id != item.id);
                        },
                    }
                }
            }
            div {
                class: "sd-notification-stack",
                role: "alert",
                "aria-live": "assertive",
                for item in assertive_items {
                    NotificationToast {
                        key: "{item.id}",
                        item: item.clone(),
                        on_dismiss: move |_| {
                            items.write().retain(|i| i.id != item.id);
                        },
                    }
                }
            }
        }
    }
}

#[component]
fn NotificationToast(item: NotificationItem, on_dismiss: EventHandler<()>) -> Element {
    let locale = use_locale();
    let mut is_paused = use_signal(|| false);

    let variant_class = item.variant.scss_class();
    let icon_color_class = item.variant.icon_color_class();
    let default_icon = item.variant.default_icon();
    let prefix = item.variant.prefix();

    #[cfg(feature = "theme")]
    let theme_val = crate::theme::use_theme()();
    #[cfg(feature = "theme")]
    let glow_class = if theme_val.is_component_glow_enabled("notification", None) {
        item.variant.glow_class()
    } else {
        ""
    };

    #[cfg(not(feature = "theme"))]
    let glow_class = item.variant.glow_class();

    #[cfg(feature = "animation")]
    let anim_class = crate::animation::Animations::TOAST_ENTER;
    #[cfg(not(feature = "animation"))]
    let anim_class = "animate-toast-enter";

    let mut timer_started = use_signal(|| false);

    use_effect(move || {
        if !timer_started() {
            timer_started.set(true);
            if let Some(dur) = item.duration {
                let on_dismiss_fn = on_dismiss;
                spawn(async move {
                    let total_ms = dur.as_millis() as u64;
                    let mut elapsed_ms = 0u64;
                    while elapsed_ms < total_ms {
                        sleep_ms(100).await;
                        if !is_paused() {
                            elapsed_ms += 100;
                        }
                    }
                    on_dismiss_fn.call(());
                });
            }
        }
    });

    rsx! {
        div {
            class: "sd-toast {variant_class} {glow_class} {anim_class}",
            onmouseenter: move |_| is_paused.set(true),
            onmouseleave: move |_| is_paused.set(false),
            onfocusin: move |_| is_paused.set(true),
            onfocusout: move |_| is_paused.set(false),
            span {
                class: "sd-toast__icon {icon_color_class}",
                "aria-hidden": "true",
                Icon { kind: default_icon, size: 16 }
            }
            div { class: "sd-toast__content",
                span { class: "sd-visually-hidden", "{prefix}: " }
                if let Some(t) = &item.title {
                    span { class: "sd-toast__title", "{t}" }
                }
                span { class: "sd-toast__body", "{item.message}" }
            }
            button {
                r#type: "button",
                class: "sd-toast__close",
                "aria-label": "{locale.close}",
                title: "{locale.close}",
                onclick: move |_| on_dismiss.call(()),
                Icon { kind: IconKind::X, size: 14 }
            }
        }
    }
}
