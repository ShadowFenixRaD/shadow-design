use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::components::{ChoicePicker, Playground, FeatureBanner, PageHeader, PropRow, ShowcaseTabs};

#[component]
pub fn SpoilerPage() -> Element {
    let selected_height = use_signal(|| 100u32);
    let mut controlled_expanded = use_signal(|| false);

    let props = vec![
        PropRow {
            name: "max_height",
            r#type: "u32",
            default: "120",
            description: "Altezza massima visibile in pixel quando il contenuto è compresso",
        },
        PropRow {
            name: "show_label",
            r#type: "String",
            default: "\"Mostra di più\"",
            description: "Testo visualizzato sul pulsante nello stato compresso",
        },
        PropRow {
            name: "hide_label",
            r#type: "String",
            default: "\"Mostra meno\"",
            description:
                "Testo visualizzato sul pulsante quando il contenuto è completamente espanso",
        },
        PropRow {
            name: "expanded",
            r#type: "Option<Signal<bool>>",
            default: "None",
            description:
                "Signal esterno per controllo reattivo bidirezionale dello stato di espansione",
        },
        PropRow {
            name: "default_expanded",
            r#type: "bool",
            default: "false",
            description: "Stato iniziale quando non controllato esternamente",
        },
        PropRow {
            name: "on_toggle",
            r#type: "Option<EventHandler<bool>>",
            default: "None",
            description: "Callback invocata a ogni espansione o contrazione del contenuto",
        },
        PropRow {
            name: "class",
            r#type: "String",
            default: "\"\"",
            description:
                "Classi CSS o modificatori BEM aggiuntivi applicati al contenitore principale",
        },
    ];

    let code_snippet = r#"let mut is_expanded = use_signal(|| false);

Spoiler {
    max_height: 120,
    show_label: "Espandi documentazione".to_string(),
    hide_label: "Riduci documentazione".to_string(),
    expanded: Some(is_expanded),

    div { class: "sc-stack",
        p { "Shadow Design integra un sistema ottico di contenimento con gradiente di dissolvenza progressivo." }
        p { "La topologia della rete mesh si articola su livelli distribuiti con instradamento Kademlia DHT..." }
        p { "Tutti i nodi scambiano heartbeat crittografati e aggiornano dinamicamente le tabelle di routing." }
        p { "La correzione d'errore automatica Reed-Solomon garantisce la persistenza del quorum decentralizzato." }
    }
}"#
    .to_string();

    rsx! {
        div { class: "sc-stack sc-stack--loose",
            PageHeader {
                title: "Spoiler".to_string(),
                description: "Contenitore espandibile a dissolvenza inferiore calcolata sui token di superficie scura, con pulsante d'azione fluido.".to_string(),
            }

            FeatureBanner { feature_name: "spoiler" }

            ShowcaseTabs {
                preview: rsx! {
                    Playground {
                        stage_start: true,
                        stage: rsx! {
                            Spoiler {
                                max_height: selected_height(),
                                show_label: "Mostra documentazione completa".to_string(),
                                hide_label: "Nascondi dettagli".to_string(),
                                expanded: Some(controlled_expanded),

                                div { class: "sc-stack",
                            p {
                                strong { "Protocollo di Gossip & Sincronizzazione Mesh" }
                            }
                            p {
                                "Ogni nodo della rete Shadowfenix mantiene un vettore di versioni logiche e diff crittografici basati su CRDT state-based. All'ingresso di un nuovo peer nel raggio di copertura Bluetooth LE o Wi-Fi Direct, viene avviata una sessione di handshaking TLS 1.3 con certificati self-signed generati su curve ellittiche Ed25519."
                            }
                            p {
                                "I pacchetti dati vengono frammentati in chunk deterministici da 64 KB, codificati con algoritmo erasure coding Reed-Solomon (8+4 frammenti di parità) per resistere alla perdita improvvisa di fino a quattro nodi contemporanei senza degradazione delle prestazioni di lettura."
                            }
                            p {
                                "La telemetria hardware locale registra frequenza di clock, temperatura dei core, latenza di bus e stato di carica della batteria di riserva, inviando heartbeat compressi zstd ogni 5000 millisecondi."
                            }
                            p {
                                "In caso di partizionamento della rete (split-brain), i sottogruppi continuano a operare in modalità degraded-local, accodando le transazioni su ring buffer SQLite cifrati con ChaCha20-Poly1305 pronti per la riconciliazione automatica al ripristino del link."
                            }

                                }
                            }
                        },
                        controls: rsx! {
                            ChoicePicker {
                                label: "Max height".to_string(),
                                options: vec![
                                    (60u32, "60px".to_string()),
                                    (100u32, "100px".to_string()),
                                    (160u32, "160px".to_string()),
                                ],
                                value: selected_height,
                            }
                            div { class: "sc-actions",
                                Button {
                                    size: ButtonSize::Sm,
                                    variant: ButtonVariant::Outline,
                                    onclick: move |_| controlled_expanded.toggle(),
                                    if controlled_expanded() { "Controlla: Riduci" } else { "Controlla: Espandi" }
                                }
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
