use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::components::{
    ChoicePicker, ControlGroup, FeatureBanner, PageHeader, Playground, PropRow, ShowcaseTabs, ToggleControl,
};

#[component]
pub fn ChatPage() -> Element {
    let locale = shadow_design::locale::use_locale();
    let direction = use_signal(|| BubbleDirection::Outgoing);
    let status = use_signal(|| DeliveryStatus::Delivered);
    let with_pq_badge = use_signal(|| true);
    let sender_name = use_signal(|| "TheMind Peer #104".to_string());  // sd-lint-allow: runtime data value, not styling
    let message_text = use_signal(|| "Tunnel P2P cifrato stabilito con Kyber-1024 + Dilithium. Latenza stimata 18ms.".to_string());

    let props = vec![
        PropRow {
            name: "direction",
            r#type: "BubbleDirection",
            default: "BubbleDirection::Incoming",
            description: "Direzione del messaggio: Incoming (ricevuto a sinistra) o Outgoing (inviato ad accento tema)",
        },
        PropRow {
            name: "timestamp",
            r#type: "Option<String>",
            default: "None",
            description: "Orario del messaggio visualizzato nell'angolo inferiore (es. '14:28')",
        },
        PropRow {
            name: "status",
            r#type: "Option<DeliveryStatus>",
            default: "None",
            description: "Micro-icona di recapito: Sending (orologio), Sent (spunta), Delivered (doppia spunta), Failed",
        },
        PropRow {
            name: "badge",
            r#type: "Option<Element>",
            default: "None",
            description: "Badge opzionale di crittografia (es. Post-Quantum PQ o icona lucchetto)",
        },
        PropRow {
            name: "sender_name",
            r#type: "Option<String>",
            default: "None",
            description: "Intestazione del mittente (utile in chat di gruppo o identificativo nodo)",
        },
    ];

    let code_snippet = r#"MessageBubble {
    direction: BubbleDirection::Outgoing,
    timestamp: "14:28".to_string(),
    status: DeliveryStatus::Delivered,
    badge: rsx! { span { "PQ" } },
    "Tunnel P2P cifrato stabilito con Kyber-1024."
}"#
    .to_string();

    rsx! {
        div { class: "sc-stack sc-stack--loose",
            PageHeader {
                title: "Chat & MessageBubble".to_string(),
                description: "Bolla messaggio asimmetrica per conversazioni e canali cifrati con indicatori di recapito, timestamp e badge crittografici.".to_string(),
            }

            FeatureBanner { feature_name: "chat" }

            ShowcaseTabs {
                preview: rsx! {
                    Playground {
                        stage_start: true,
                        stage: rsx! {
                            MessageBubble {
                                direction: BubbleDirection::Incoming,
                                timestamp: Some("14:27".to_string()),
                                sender_name: Some("Relay Node #02".to_string()),
                                "Richiesta handshake cifrato ricevuta."
                            }

                            MessageBubble {
                                direction: direction(),
                                timestamp: Some("14:28".to_string()),
                                status: Some(status()),
                                sender_name: if direction() == BubbleDirection::Incoming { Some(sender_name()) } else { None },
                                badge: if with_pq_badge() {
                                    Some(rsx! {
                                        span { class: "flex items-center gap-1",
                                            Icon { kind: IconKind::ShieldCheck, size: 10 }
                                            "PQ"
                                        }
                                    })
                                } else {
                                    None
                                },
                                "{message_text}"
                            }
                        },
                        controls: rsx! {
                            ChoicePicker {
                                label: "Direzione Bolla".to_string(),
                                options: vec![
                                    (BubbleDirection::Incoming, "Incoming (Ricevuto)".to_string()),
                                    (BubbleDirection::Outgoing, "Outgoing (Inviato)".to_string()),
                                ],
                                value: direction,
                            }
                            ChoicePicker {
                                label: "Stato di Recapito (DeliveryStatus)".to_string(),
                                options: [DeliveryStatus::Sending, DeliveryStatus::Sent, DeliveryStatus::Delivered, DeliveryStatus::Failed]
                                    .iter()
                                    .map(|s| (*s, s.label(&locale).to_string()))
                                    .collect::<Vec<_>>(),
                                value: status,
                            }
                            hr { class: "sc-control-divider" }
                            ToggleControl { label: "Mostra Badge Cifratura Post-Quantum (PQ)".to_string(), checked: with_pq_badge }
                            ControlGroup { label: "Testo del Messaggio".to_string(),
                                TextInput { value: message_text }
                            }
                        },
                    }
                },
                code: code_snippet,
                props: props,
            }
        }
    }
}
