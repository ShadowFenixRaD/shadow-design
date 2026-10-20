use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::components::{FeatureBanner, PropRow, ShowcaseTabs};

#[component]
pub fn TimelinesPage() -> Element {
    let mut align = use_signal(|| TimelineAlign::Left);
    let mut line_variant = use_signal(|| TimelineLineVariant::Solid);
    let mut bullet_size = use_signal(|| 24u32);

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
        bullet: rsx! { Icon { kind: IconKind::Radio, class: "w-3.5 h-3.5 text-purple-400 animate-pulse" } },
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
        div { class: "max-w-4xl space-y-8",
            div {
                Heading { order: HeadingOrder::H2, "Timeline & Cronologia Eventi" }
                Text { dimmed: true, size: TextSize::Sm, "Tracciamento cronologico di stati ed eventi con connettori continui o tratteggiati, stati di avanzamento e icone contestuali." }
            }

            FeatureBanner { feature_name: "timeline" }

            ShowcaseTabs {
                preview: rsx! {
                    div { class: "grid grid-cols-1 lg:grid-cols-2 gap-8 items-start",
                        // Live Preview
                        div { class: "p-8 rounded-xl bg-[#060608] border border-white/[0.08] min-h-[380px] shadow-inner flex flex-col justify-center",
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
                                    bullet: rsx! { Icon { kind: IconKind::AlertTriangle, class: "w-3.5 h-3.5 text-rose-400" } },
                                    "Rilevata incoerenza di firma sul relay #07: isolamento immediato."
                                }
                            }
                        }

                        // Controls
                        div { class: "p-6 rounded-xl bg-zinc-950/60 border border-white/[0.08] space-y-5 text-sm",
                            div { class: "font-semibold text-zinc-200 border-b border-white/[0.06] pb-2", "Configurazione Live" }

                            // Align
                            div { class: "space-y-1.5",
                                label { class: "text-xs font-mono text-zinc-400", "Allineamento" }
                                div { class: "flex gap-2",
                                    button {
                                        class: format!("px-3 py-1 rounded text-xs border transition-colors {}",
                                            if align() == TimelineAlign::Left { "bg-purple-950/60 border-purple-500/50 text-purple-200 font-semibold" } else { "bg-zinc-900 border-zinc-800 text-zinc-400" }),
                                        onclick: move |_| align.set(TimelineAlign::Left),
                                        "Sinistra (Standard)"
                                    }
                                    button {
                                        class: format!("px-3 py-1 rounded text-xs border transition-colors {}",
                                            if align() == TimelineAlign::Right { "bg-purple-950/60 border-purple-500/50 text-purple-200 font-semibold" } else { "bg-zinc-900 border-zinc-800 text-zinc-400" }),
                                        onclick: move |_| align.set(TimelineAlign::Right),
                                        "Destra"
                                    }
                                }
                            }

                            // Line Variant
                            div { class: "space-y-1.5",
                                label { class: "text-xs font-mono text-zinc-400", "Stile Linea" }
                                div { class: "flex gap-2",
                                    button {
                                        class: format!("px-3 py-1 rounded text-xs border transition-colors {}",
                                            if line_variant() == TimelineLineVariant::Solid { "bg-purple-950/60 border-purple-500/50 text-purple-200 font-semibold" } else { "bg-zinc-900 border-zinc-800 text-zinc-400" }),
                                        onclick: move |_| line_variant.set(TimelineLineVariant::Solid),
                                        "Continua (Solid)"
                                    }
                                    button {
                                        class: format!("px-3 py-1 rounded text-xs border transition-colors {}",
                                            if line_variant() == TimelineLineVariant::Dashed { "bg-purple-950/60 border-purple-500/50 text-purple-200 font-semibold" } else { "bg-zinc-900 border-zinc-800 text-zinc-400" }),
                                        onclick: move |_| line_variant.set(TimelineLineVariant::Dashed),
                                        "Tratteggiata (Dashed)"
                                    }
                                }
                            }

                            // Bullet Size
                            div { class: "space-y-1.5",
                                label { class: "text-xs font-mono text-zinc-400", "Dimensione Bullet: {bullet_size}px" }
                                div { class: "flex gap-2",
                                    for sz in [20u32, 24u32, 28u32, 32u32] {
                                        button {
                                            class: format!("px-3 py-1 rounded text-xs border transition-colors {}",
                                                if bullet_size() == sz { "bg-purple-950/60 border-purple-500/50 text-purple-200 font-semibold" } else { "bg-zinc-900 border-zinc-800 text-zinc-400" }),
                                            onclick: move |_| bullet_size.set(sz),
                                            "{sz}px"
                                        }
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
