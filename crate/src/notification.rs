use crate::icon::{Icon, IconKind};
use dioxus::prelude::*;

#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub enum NotificationVariant {
    #[default]
    Info,
    Success,
    Warning,
    Danger,
}

impl NotificationVariant {
    pub fn scss_class(&self) -> &'static str {
        match self {
            NotificationVariant::Info => "sd-toast--info",
            NotificationVariant::Success => "sd-toast--success",
            NotificationVariant::Warning => "sd-toast--warning",
            NotificationVariant::Danger => "sd-toast--danger",
        }
    }

    pub fn glow_class(&self) -> &'static str {
        match self {
            NotificationVariant::Info => "sd-toast--glow-info",
            NotificationVariant::Success => "sd-toast--glow-success",
            NotificationVariant::Warning => "sd-toast--glow-warning",
            NotificationVariant::Danger => "sd-toast--glow-danger",
        }
    }

    pub fn icon_kind(&self) -> IconKind {
        match self {
            NotificationVariant::Info => IconKind::Info,
            NotificationVariant::Success => IconKind::CheckCircle,
            NotificationVariant::Warning => IconKind::AlertTriangle,
            NotificationVariant::Danger => IconKind::XCircle,
        }
    }

    pub fn icon_color_class(&self) -> &'static str {
        match self {
            NotificationVariant::Info => "sd-toast__icon--info",
            NotificationVariant::Success => "sd-toast__icon--success",
            NotificationVariant::Warning => "sd-toast__icon--warning",
            NotificationVariant::Danger => "sd-toast__icon--danger",
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct NotificationItem {
    pub id: usize,
    pub title: Option<String>,
    pub message: String,
    pub variant: NotificationVariant,
}

#[derive(Clone, Copy, PartialEq)]
pub struct NotificationHandle {
    items: Signal<Vec<NotificationItem>>,
    next_id: Signal<usize>,
}

impl NotificationHandle {
    pub fn info(&mut self, message: impl Into<String>) -> usize {
        self.show(message, NotificationVariant::Info)
    }

    pub fn success(&mut self, message: impl Into<String>) -> usize {
        self.show(message, NotificationVariant::Success)
    }

    pub fn warning(&mut self, message: impl Into<String>) -> usize {
        self.show(message, NotificationVariant::Warning)
    }

    pub fn danger(&mut self, message: impl Into<String>) -> usize {
        self.show(message, NotificationVariant::Danger)
    }

    pub fn show(&mut self, message: impl Into<String>, variant: NotificationVariant) -> usize {
        let id = *self.next_id.read();
        *self.next_id.write() += 1;

        let item = NotificationItem {
            id,
            title: None,
            message: message.into(),
            variant,
        };

        self.items.write().push(item);
        id
    }

    pub fn show_with_title(
        &mut self,
        title: impl Into<String>,
        message: impl Into<String>,
        variant: NotificationVariant,
    ) -> usize {
        let id = *self.next_id.read();
        *self.next_id.write() += 1;

        let item = NotificationItem {
            id,
            title: Some(title.into()),
            message: message.into(),
            variant,
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

#[component]
pub fn NotificationProvider(children: Element) -> Element {
    let items = use_signal(Vec::<NotificationItem>::new);
    let next_id = use_signal(|| 1usize);
    let handle = NotificationHandle { items, next_id };
    use_context_provider(|| handle);

    rsx! {
        {children}
        NotificationContainer { items }
    }
}

pub fn use_notification() -> NotificationHandle {
    match try_use_context::<NotificationHandle>() {
        Some(handle) => handle,
        None => {
            let items = use_signal(Vec::<NotificationItem>::new);
            let next_id = use_signal(|| 1usize);
            let handle = NotificationHandle { items, next_id };
            use_context_provider(|| handle)
        }
    }
}

#[component]
fn NotificationContainer(mut items: Signal<Vec<NotificationItem>>) -> Element {
    rsx! {
        div {
            class: "sd-notification-container",
            for item in items.read().iter().cloned() {
                NotificationToast {
                    key: "{item.id}",
                    item: item.clone(),
                    on_dismiss: move |_| {
                        items.write().retain(|i| i.id != item.id);
                    }
                }
            }
        }
    }
}

#[component]
fn NotificationToast(item: NotificationItem, on_dismiss: EventHandler<MouseEvent>) -> Element {
    let variant_class = item.variant.scss_class();
    let icon_color_class = item.variant.icon_color_class();
    let icon_kind = item.variant.icon_kind();

    #[cfg(feature = "theme")]
    let theme = crate::theme::use_theme();
    #[cfg(feature = "theme")]
    let has_glow = theme().is_component_glow_enabled("notification", None);
    #[cfg(not(feature = "theme"))]
    let has_glow = true;

    let glow_class = if has_glow {
        item.variant.glow_class()
    } else {
        ""
    };

    #[cfg(feature = "animation")]
    let anim_class = crate::animation::Animations::TOAST_ENTER;
    #[cfg(not(feature = "animation"))]
    let anim_class = "animate-toast-enter";

    rsx! {
        div {
            class: "sd-toast {variant_class} {glow_class} {anim_class}",
            span { class: "sd-toast__icon {icon_color_class}",
                Icon { kind: icon_kind, size: 16 }
            }
            div { class: "sd-toast__content",
                if let Some(ref title) = item.title {
                    h5 { class: "sd-toast__title", "{title}" }
                }
                p { class: "sd-toast__body", "{item.message}" }
            }
            button {
                r#type: "button",
                class: "sd-toast__close",
                onclick: move |e| on_dismiss.call(e),
                Icon { kind: IconKind::X, size: 13 }
            }
        }
    }
}
