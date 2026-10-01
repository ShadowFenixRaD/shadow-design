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
    pub fn border_and_accent(&self) -> (&'static str, &'static str, IconKind) {
        match self {
            NotificationVariant::Info => (
                "border-purple-500/30 border-l-4 border-l-purple-500",
                "text-purple-400",
                IconKind::Info,
            ),
            NotificationVariant::Success => (
                "border-emerald-500/30 border-l-4 border-l-emerald-500",
                "text-emerald-400",
                IconKind::CheckCircle,
            ),
            NotificationVariant::Warning => (
                "border-amber-500/30 border-l-4 border-l-amber-500",
                "text-amber-400",
                IconKind::AlertTriangle,
            ),
            NotificationVariant::Danger => (
                "border-rose-500/30 border-l-4 border-l-rose-500",
                "text-rose-400",
                IconKind::XCircle,
            ),
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
        self.items.write().push(NotificationItem {
            id,
            title: None,
            message: message.into(),
            variant,
        });
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
        self.items.write().push(NotificationItem {
            id,
            title: Some(title.into()),
            message: message.into(),
            variant,
        });
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
            class: "fixed bottom-5 right-5 z-[90] flex flex-col gap-2.5 max-w-sm w-full pointer-events-none px-4 sm:px-0",
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
    let (border_classes, icon_color, icon_kind) = item.variant.border_and_accent();

    #[cfg(feature = "theme")]
    let theme = crate::theme::use_theme();
    #[cfg(feature = "theme")]
    let has_glow = theme().is_component_glow_enabled("notification", None);
    #[cfg(not(feature = "theme"))]
    let has_glow = true;

    let glow_class = if has_glow {
        match item.variant {
            NotificationVariant::Info => "shadow-[0_0_24px_rgba(168,85,247,0.25)]",
            NotificationVariant::Success => "shadow-[0_0_24px_rgba(16,185,129,0.25)]",
            NotificationVariant::Warning => "shadow-[0_0_24px_rgba(245,158,11,0.25)]",
            NotificationVariant::Danger => "shadow-[0_0_24px_rgba(244,63,94,0.28)]",
        }
    } else {
        "shadow-2xl shadow-black/90"
    };

    #[cfg(feature = "animation")]
    let anim_class = crate::animation::Animations::TOAST_ENTER;
    #[cfg(not(feature = "animation"))]
    let anim_class = "animate-in fade-in slide-in-from-bottom-2 duration-200";

    rsx! {
        div {
            class: "pointer-events-auto p-4 rounded-xl bg-[#0e0e14] border border-white/10 {border_classes} {glow_class} {anim_class} flex items-start gap-3 transition-all specular-top",
            span { class: "shrink-0 mt-0.5 {icon_color}",
                Icon { kind: icon_kind, size: 16 }
            }
            div { class: "flex-1 min-w-0 space-y-0.5",
                if let Some(ref title) = item.title {
                    h5 { class: "text-xs font-semibold text-white tracking-tight", "{title}" }
                }
                p { class: "text-xs text-zinc-300 leading-relaxed", "{item.message}" }
            }
            button {
                r#type: "button",
                class: "shrink-0 p-1 text-zinc-500 hover:text-white rounded-lg hover:bg-white/[0.06] transition-colors cursor-pointer -mr-1 -mt-1",
                onclick: move |e| on_dismiss.call(e),
                Icon { kind: IconKind::X, size: 13 }
            }
        }
    }
}
