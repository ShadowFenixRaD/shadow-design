use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::components::{FeatureBanner, PropRow, ShowcaseTabs};

#[component]
pub fn ChatPage() -> Element {
    let locale = shadow_design::locale::use_locale();
    let mut direction = use_signal(|| BubbleDirection::Outgoing);
    let mut status = use_signal(|| DeliveryStatus::Delivered);
    let with_pq_badge = use_signal(|| true);
    let sender_name = use_signal(|| "TheMind Peer #104".to_string());
    let mut message_text = use_signal(|| "Tunnel P2P cifrato stabilito con Kyber-1024 + Dilithium. Latenza stimata 18ms.".to_string());

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
        div { class: "max-w-4xl space-y-8",
            div {
                Heading { order: HeadingOrder::H2, "Chat & MessageBubble" }
                Text { dimmed: true, size: TextSize::Sm, "Bolla messaggio asimmetrica per conversazioni e canali cifrati con indicatori di recapito, timestamp e badge crittografici." }
            }

            FeatureBanner { feature_name: "chat" }

            ShowcaseTabs {
                preview: rsx! {
                    div { class: "grid grid-cols-1 lg:grid-cols-2 gap-8 items-center",
                        // Live Preview (Chat Canvas)
                        div { class: "p-6 rounded-xl bg-[#060608] border border-white/[0.08] flex flex-col justify-center min-h-[200px] shadow-inner space-y-3",
                            // Incoming example for context
                            MessageBubble {
                                direction: BubbleDirection::Incoming,
                                timestamp: Some("14:27".to_string()),
                                sender_name: Some("Relay Node #02".to_string()),
                                "Richiesta handshake cifrato ricevuta."
                            }

                            // Interactive bubble
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
                        }

                        // Configurator
                        div { class: "space-y-4 text-xs font-mono",
                            div {
                                span { class: "text-zinc-400 block mb-1.5 uppercase tracking-wider text-[10px]", "Direzione Bolla:" }
                                div { class: "flex gap-2",
                                    button {
                                        r#type: "button",
                                        class: if direction() == BubbleDirection::Incoming {
                                            "px-3 py-1.5 rounded bg-purple-950/80 text-purple-200 border border-purple-500/40 font-bold cursor-pointer"
                                        } else {
                                            "px-3 py-1.5 rounded bg-white/[0.03] text-zinc-400 border border-white/[0.06] cursor-pointer"
                                        },
                                        onclick: move |_| direction.set(BubbleDirection::Incoming),
                                        "Incoming (Ricevuto)"
                                    }
                                    button {
                                        r#type: "button",
                                        class: if direction() == BubbleDirection::Outgoing {
                                            "px-3 py-1.5 rounded bg-purple-950/80 text-purple-200 border border-purple-500/40 font-bold cursor-pointer"
                                        } else {
                                            "px-3 py-1.5 rounded bg-white/[0.03] text-zinc-400 border border-white/[0.06] cursor-pointer"
                                        },
                                        onclick: move |_| direction.set(BubbleDirection::Outgoing),
                                        "Outgoing (Inviato)"
                                    }
                                }
                            }

                            div {
                                span { class: "text-zinc-400 block mb-1.5 uppercase tracking-wider text-[10px]", "Stato di Recapito (DeliveryStatus):" }
                                div { class: "flex flex-wrap gap-1.5",
                                    for s in [DeliveryStatus::Sending, DeliveryStatus::Sent, DeliveryStatus::Delivered, DeliveryStatus::Failed] {
                                        button {
                                            r#type: "button",
                                            class: if status() == s {
                                                "px-2.5 py-1 rounded bg-purple-950/80 text-purple-200 border border-purple-500/40 font-bold cursor-pointer"
                                            } else {
                                                "px-2.5 py-1 rounded bg-white/[0.03] text-zinc-400 border border-white/[0.06] cursor-pointer"
                                            },
                                            onclick: move |_| status.set(s),
                                            "{s.label(&locale)}"
                                        }
                                    }
                                }
                            }

                            div { class: "pt-2 border-t border-white/[0.06] space-y-2",
                                Checkbox {
                                    checked: with_pq_badge,
                                    label: Some("Mostra Badge Cifratura Post-Quantum (PQ)".to_string()),
                                }
                                div { class: "space-y-1",
                                    span { class: "text-zinc-400 block text-[10px]", "Testo del Messaggio:" }
                                    input {
                                        r#type: "text",
                                        class: "w-full px-2.5 py-1.5 rounded bg-black/60 border border-white/[0.1] text-zinc-200 focus:outline-none focus:border-purple-500",
                                        value: "{message_text}",
                                        oninput: move |e| message_text.set(e.value()),
                                    }
                                }
                            }
                        }
                    }
                },
                code: code_snippet,
                props: props,
            }
        }
    }
}
