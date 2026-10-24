use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::components::{ChoicePicker, Playground, FeatureBanner, PageHeader, PropRow, ShowcaseTabs};

#[component]
pub fn TimelinesPage() -> Element {
    let align = use_signal(|| TimelineAlign::Left);
    let line_variant = use_signal(|| TimelineLineVariant::Solid);
    let bullet_size = use_signal(|| 24u32);

    let props = vec![
        PropRow {
            name: "align",
            r#type: "TimelineAlign",
            default: "TimelineAlign::Left",
            description: "Allineamento della linea e del testo: Left o Right",
        },
        PropRow {
            name: "bullet_size",
            r#type: "u32",
            default: "24",
            description: "Diametro in pixel del bullet circolare",
        },
        PropRow {
            name: "line_variant",
            r#type: "TimelineLineVariant",
            default: "TimelineLineVariant::Solid",
            description: "Stile della linea di connessione: Solid o Dashed",
        },
        PropRow {
            name: "state",
            r#type: "Option<TimelineBulletState>",
            default: "None (Default)",
            description: "Stato del singolo item: Default, Active, Completed, Pending, Error",
        },
        PropRow {
            name: "bullet",
            r#type: "Option<Element>",
            default: "None",
            description: "Icona personalizzata renderizzata al centro del bullet",
        },
        PropRow {
            name: "title",
            r#type: "Option<String>",
            default: "None",
            description: "Titolo o intestazione principale dell'evento",
        },
        PropRow {
            name: "time",
            r#type: "Option<String>",
            default: "None",
            description: "Data, orario o badge temporale posizionato a destra",
        },
    ];

    let code_snippet = r#"Timeline {
    align: TimelineAlign::Left,
    line_variant: TimelineLineVariant::Solid,
    TimelineItem {
        title: Some("Inizializzazione Nodo Mesh".to_string()),
        time: Some("14:32:01".to_string()),
        state: Some(TimelineBulletState::Completed),
        bullet: rsx! { Icon { kind: IconKind::CheckCircle, class: "w-3.5 h-3.5" } },
        "Handshake TLS 1.3 completato con il peer gateway."
    }
    TimelineItem {
        title: Some("Sincronizzazione Snapshot".to_string()),
        time: Some("In corso".to_string()),
        state: Some(TimelineBulletState::Active),
        bullet: rsx! { Icon { kind: IconKind::Radio, class: "w-3.5 h-3.5 sc-accent-text animate-pulse" } },
        "Verifica crittografica dei blocchi in memoria volatile."
    }
    TimelineItem {
        title: Some("Attivazione Canale Segreto".to_string()),
        time: Some("In attesa".to_string()),
        state: Some(TimelineBulletState::Pending),
        "In attesa del raggiungimento del quorum decentralizzato."
    }
}"#
    .to_string();

    rsx! {
        div { class: "sc-stack sc-stack--loose",
            PageHeader {
                title: "Timeline & Cronologia Eventi".to_string(),
                description: "Tracciamento cronologico di stati ed eventi con connettori continui o tratteggiati, stati di avanzamento e icone contestuali.".to_string(),
            }

            FeatureBanner { feature_name: "timeline" }

            ShowcaseTabs {
                preview: rsx! {
                    Playground {
                        stage_start: true,
                        stage: rsx! {
                            Timeline {
                                align: align(),
                                line_variant: line_variant(),
                                bullet_size: bullet_size(),
                                TimelineItem {
                                    title: Some("Nodo Master Connesso".to_string()),
                                    time: Some("10:45:00".to_string()),
                                    state: Some(TimelineBulletState::Completed),
                                    bullet: rsx! { Icon { kind: IconKind::CheckCircle, class: "w-3.5 h-3.5" } },
                                    "Inizializzazione cluster crittografico e generazione chiavi di sessione."
                                }
                                TimelineItem {
                                    title: Some("Handshake Peer P2P".to_string()),
                                    time: Some("10:45:12".to_string()),
                                    state: Some(TimelineBulletState::Completed),
                                    bullet: rsx! { Icon { kind: IconKind::Lock, class: "w-3.5 h-3.5" } },
                                    "Autenticazione mutua con 8 nodi vicini tramite ED25519."
                                }
                                TimelineItem {
                                    title: Some("Sincronizzazione Ledger".to_string()),
                                    time: Some("In corso".to_string()),
                                    state: Some(TimelineBulletState::Active),
                                    bullet: rsx! { Icon { kind: IconKind::Radio, class: "w-3.5 h-3.5 animate-pulse" } },
                                    "Download dei blocchi mancanti dal quorum di rete decentralizzato."
                                }
                                TimelineItem {
                                    title: Some("Verifica Integrità Snapshot".to_string()),
                                    time: Some("In coda".to_string()),
                                    state: Some(TimelineBulletState::Pending),
                                    "Calcolo checksum SHA-256 e verifica di consistenza."
                                }
                                TimelineItem {
                                    title: Some("Canale Anonimo Compromesso".to_string()),
                                    time: Some("Segnalato".to_string()),
                                    state: Some(TimelineBulletState::Error),
                                    bullet: rsx! { Icon { kind: IconKind::AlertTriangle, class: "w-3.5 h-3.5 sc-danger-text" } },
                                    "Rilevata incoerenza di firma sul relay #07: isolamento immediato."
                                }
                            }
                        },
                        controls: rsx! {
                            ChoicePicker {
                                label: "Allineamento".to_string(),
                                options: vec![
                                    (TimelineAlign::Left, "Sinistra (Standard)".to_string()),
                                    (TimelineAlign::Right, "Destra".to_string()),
                                ],
                                value: align,
                            }
                            ChoicePicker {
                                label: "Stile linea".to_string(),
                                options: vec![
                                    (TimelineLineVariant::Solid, "Continua (Solid)".to_string()),
                                    (TimelineLineVariant::Dashed, "Tratteggiata (Dashed)".to_string()),
                                ],
                                value: line_variant,
                            }
                            ChoicePicker {
                                label: format!("Dimensione bullet: {}px", bullet_size()),
                                options: vec![
                                    (20u32, "20px".to_string()),
                                    (24u32, "24px".to_string()),
                                    (28u32, "28px".to_string()),
                                    (32u32, "32px".to_string()),
                                ],
                                value: bullet_size,
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
