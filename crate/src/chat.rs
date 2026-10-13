use dioxus::prelude::*;

#[cfg(feature = "icon")]
use crate::icon::{Icon, IconKind};

/// Direzione della bolla di messaggio (inviato o ricevuto).
#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub enum BubbleDirection {
    #[default]
    Incoming,
    Outgoing,
}

impl BubbleDirection {
    #[must_use]
    pub fn scss_class(&self) -> &'static str {
        match self {
            BubbleDirection::Incoming => "sd-chat-bubble--incoming",
            BubbleDirection::Outgoing => "sd-chat-bubble--outgoing",
        }
    }
}

/// Stato di consegna del messaggio con micro-icone di tracciamento.
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum DeliveryStatus {
    /// In fase di trasmissione o invio in corso (orologio).
    Sending,
    /// Trasmesso con successo al server o peer (spunta singola).
    Sent,
    /// Consegnato e ricevuto dal destinatario (doppia spunta).
    Delivered,
    /// Errore di invio o timeout di trasmissione (punto esclamativo).
    Failed,
}

impl DeliveryStatus {
    #[must_use]
    pub fn scss_class(&self) -> &'static str {
        match self {
            DeliveryStatus::Sending => "sd-chat-bubble__status--sending",
            DeliveryStatus::Sent => "sd-chat-bubble__status--sent",
            DeliveryStatus::Delivered => "sd-chat-bubble__status--delivered",
            DeliveryStatus::Failed => "sd-chat-bubble__status--failed",
        }
    }

    #[must_use]
    pub fn label(&self) -> &'static str {
        match self {
            DeliveryStatus::Sending => "In invio...",
            DeliveryStatus::Sent => "Inviato",
            DeliveryStatus::Delivered => "Consegnato",
            DeliveryStatus::Failed => "Invio non riuscito",
        }
    }
}

/// Componente MessageBubble per conversazioni, messaggi istantanei e feed P2P.
#[component]
pub fn MessageBubble(
    #[props(default)] direction: BubbleDirection,
    #[props(default)] timestamp: Option<String>,
    #[props(default)] status: Option<DeliveryStatus>,
    #[props(default)] badge: Option<Element>,
    #[props(default)] sender_name: Option<String>,
    #[props(default)] class: String,
    children: Element,
) -> Element {
    let dir_class = direction.scss_class();

    #[cfg(feature = "theme")]
    let theme_val = crate::theme::use_theme()();
    #[cfg(feature = "theme")]
    let base_override = theme_val.resolve_component_base("chat_bubble").unwrap_or("");
    #[cfg(feature = "theme")]
    let class_modifiers = theme_val.resolve_class_modifiers("chat_bubble", &class);
    #[cfg(feature = "theme")]
    let radius = theme_val.radius_scale.sm();

    #[cfg(not(feature = "theme"))]
    let base_override = "";
    #[cfg(not(feature = "theme"))]
    let class_modifiers = String::new();
    #[cfg(not(feature = "theme"))]
    let radius = "";

    rsx! {
        div {
            class: "sd-chat-bubble-container {dir_class}",
            div {
                class: "sd-chat-bubble {dir_class} {radius} {base_override} {class_modifiers} {class}",

                if let Some(sender) = sender_name {
                    div {
                        class: "sd-chat-bubble__sender",
                        "{sender}"
                    }
                }

                div {
                    class: "sd-chat-bubble__content",
                    {children}
                }

                if timestamp.is_some() || badge.is_some() || status.is_some() {
                    div {
                        class: "sd-chat-bubble__meta",

                        if let Some(custom_badge) = badge {
                            span {
                                class: "sd-chat-bubble__badge",
                                {custom_badge}
                            }
                        }

                        if let Some(time) = timestamp {
                            span {
                                class: "sd-chat-bubble__time",
                                "{time}"
                            }
                        }

                        if let Some(delivery) = status {
                            span {
                                class: "sd-chat-bubble__status {delivery.scss_class()}",
                                title: "{delivery.label()}",
                                match delivery {
                                    DeliveryStatus::Sending => rsx! {
                                        Icon { kind: IconKind::Clock, size: 12 }
                                    },
                                    DeliveryStatus::Sent => rsx! {
                                        Icon { kind: IconKind::Check, size: 12 }
                                    },
                                    DeliveryStatus::Delivered => rsx! {
                                        Icon { kind: IconKind::CheckCheck, size: 12 }
                                    },
                                    DeliveryStatus::Failed => rsx! {
                                        Icon { kind: IconKind::AlertCircle, size: 12 }
                                    },
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
