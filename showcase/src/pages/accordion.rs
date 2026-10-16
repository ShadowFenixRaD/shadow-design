use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::components::{FeatureBanner, PropRow, ShowcaseTabs};

#[component]
pub fn AccordionPage() -> Element {
    let mut variant = use_signal(|| AccordionVariant::Separated);
    let is_multiple = use_signal(|| false);
    let mut collapsible_open = use_signal(|| true);

    let props = vec![
        PropRow {
            name: "multiple",
            r#type: "bool",
            default: "false",
            description: "Consente l'espansione contemporanea di più pannelli contemporaneamente",
        },
        PropRow {
            name: "variant",
            r#type: "AccordionVariant",
            default: "Default",
            description: "Stile visivo: Default (linee divisorie), Contained (pannello unificato), Separated (card staccate)",
        },
        PropRow {
            name: "value",
            r#type: "Option<Signal<Vec<String>>>",
            default: "None",
            description: "Signal esterno per controllo bidirezionale e reattivo degli elementi aperti",
        },
        PropRow {
            name: "default_value",
            r#type: "Option<Vec<String>>",
            default: "None",
            description: "Valori degli elementi aperti inizialmente in modalità non controllata",
        },
        PropRow {
            name: "AccordionItem.value",
            r#type: "String",
            default: "-",
            description: "Identificatore univoco del pannello per l'associazione con stato ed eventi",
        },
        PropRow {
            name: "AccordionItem.disabled",
            r#type: "bool",
            default: "false",
            description: "Disabilita l'interazione sul pannello mantenendone la visibilità",
        },
        PropRow {
            name: "AccordionControl.icon",
            r#type: "Option<IconKind>",
            default: "None",
            description: "Icona SVG nativa visualizzata prima del testo dell'intestazione",
        },
        PropRow {
            name: "Collapsible.open",
            r#type: "Signal<bool>",
            default: "-",
            description: "Signal per collassare ed espandere fluidamente qualsiasi elemento arbitrario",
        },
    ];

    let code_snippet = r#"// Accordion con supporto a espansione multipla e stili separati
Accordion {
    multiple: true,
    variant: AccordionVariant::Separated,
    default_value: vec!["mesh-routing".into()],

    AccordionItem { value: "mesh-routing".into(),
        AccordionControl { icon: Some(IconKind::Radio), "Routing Mesh & P2P" }
        AccordionPanel {
            p { "Gestione topologia peer-to-peer crittografata con scambio chiavi Noise." }
        }
    }

    AccordionItem { value: "security".into(),
        AccordionControl { icon: Some(IconKind::Eye), "Sicurezza Crittografica" }
        AccordionPanel {
            p { "Algoritmi Ed25519 e ChaCha20-Poly1305 a prova di manomissione." }
        }
    }

    AccordionItem { value: "backup".into(), disabled: true,
        AccordionControl { icon: Some(IconKind::Terminal), "Snapshot Cluster (Disabilitato)" }
        AccordionPanel {
            p { "Modulo non attivo sul nodo corrente." }
        }
    }
}

// Primitiva indipendente Collapsible
Collapsible { open: is_open,
    p { "Contenuto collassabile indipendente fluido..." }
}"#
    .to_string();

    rsx! {
        div { class: "max-w-4xl space-y-8",
            div {
                Heading { order: HeadingOrder::H2, "Accordion & Collapsible" }
                Text { dimmed: true, size: TextSize::Sm, "Pannelli espandibili a fisarmonica conformi a WAI-ARIA con animazioni ottiche fluide, rotazione del chevron e tre varianti di superficie." }
            }

            FeatureBanner { feature_name: "accordion" }

            ShowcaseTabs {
                preview: rsx! {
                    div { class: "p-6 rounded-xl bg-[#09090d] border border-white/[0.08] shadow-xl space-y-6",
                        // Toolbar interattiva per playground
                        div { class: "flex flex-wrap items-center justify-between gap-4 pb-4 border-b border-white/[0.06]",
                            div { class: "flex items-center gap-3",
                                span { class: "text-xs font-mono text-zinc-400 uppercase", "Variante:" }
                                div { class: "flex items-center gap-1.5",
                                    button {
                                        r#type: "button",
                                        class: format!(
                                            "px-2.5 py-1 text-xs rounded-md border transition-all {}",
                                            if variant() == AccordionVariant::Separated { "bg-purple-950/80 text-purple-200 border-purple-500/50 shadow-sm" } else { "bg-white/[0.03] text-zinc-400 border-white/[0.06] hover:text-white" }
                                        ),
                                        onclick: move |_| variant.set(AccordionVariant::Separated),
                                        "Separated"
                                    }
                                    button {
                                        r#type: "button",
                                        class: format!(
                                            "px-2.5 py-1 text-xs rounded-md border transition-all {}",
                                            if variant() == AccordionVariant::Contained { "bg-purple-950/80 text-purple-200 border-purple-500/50 shadow-sm" } else { "bg-white/[0.03] text-zinc-400 border-white/[0.06] hover:text-white" }
                                        ),
                                        onclick: move |_| variant.set(AccordionVariant::Contained),
                                        "Contained"
                                    }
                                    button {
                                        r#type: "button",
                                        class: format!(
                                            "px-2.5 py-1 text-xs rounded-md border transition-all {}",
                                            if variant() == AccordionVariant::Default { "bg-purple-950/80 text-purple-200 border-purple-500/50 shadow-sm" } else { "bg-white/[0.03] text-zinc-400 border-white/[0.06] hover:text-white" }
                                        ),
                                        onclick: move |_| variant.set(AccordionVariant::Default),
                                        "Default"
                                    }
                                }
                            }

                            div { class: "flex items-center gap-2",
                                Switch {
                                    checked: is_multiple,
                                    label: "Espansione Multipla".to_string(),
                                }
                            }
                        }

                        // Accordion dimostrativo
                        Accordion {
                            multiple: is_multiple(),
                            variant: variant(),
                            default_value: vec!["mesh-routing".to_string()],

                            AccordionItem { value: "mesh-routing".to_string(),
                                AccordionControl { icon: Some(IconKind::Radio), "Architettura di Rete & Mesh Routing" }
                                AccordionPanel {
                                    p { class: "text-zinc-300", "La rete Shadowfenix impiega un protocollo peer-to-peer a maglia con routing ad anello Kademlia distribuito e crittografia hop-by-hop." }
                                    p { class: "text-xs text-zinc-500 mt-2 font-mono", "Latenza tipica: < 12ms • Algoritmo: Kademlia DHT" }
                                }
                            }

                            AccordionItem { value: "security-keys".to_string(),
                                AccordionControl { icon: Some(IconKind::Eye), "Crittografia & Firme Ed25519" }
                                AccordionPanel {
                                    p { class: "text-zinc-300", "Ogni transazione e messaggio scambiato nel cluster viene firmato asimmetricamente tramite chiavi Curve25519 ad altissima velocità." }
                                }
                            }

                            AccordionItem { value: "storage-cluster".to_string(),
                                AccordionControl { icon: Some(IconKind::Box), "Storage Replicato Distribuito" }
                                AccordionPanel {
                                    p { class: "text-zinc-300", "Archiviazione a oggetti decentralizzata con sharding deterministico e correzione automatica degli errori Reed-Solomon." }
                                }
                            }

                            AccordionItem { value: "disabled-node".to_string(), disabled: true,
                                AccordionControl { icon: Some(IconKind::Terminal), "Cluster Telemetry (Nodo Offline)" }
                                AccordionPanel {
                                    p { class: "text-zinc-400", "Questa opzione non è disponibile quando il nodo è in modalità isolata." }
                                }
                            }
                        }

                        // Sezione Collapsible Standalone
                        div { class: "pt-6 border-t border-white/[0.06] space-y-3",
                            div { class: "flex items-center justify-between",
                                span { class: "text-sm font-semibold text-white", "Primitiva Standalone: Collapsible" }
                                Button {
                                    size: ButtonSize::Sm,
                                    variant: ButtonVariant::Outline,
                                    onclick: move |_| collapsible_open.toggle(),
                                    if collapsible_open() { "Collassa Pannello" } else { "Espandi Pannello" }
                                }
                            }

                            Collapsible { open: collapsible_open,
                                div { class: "p-4 rounded-lg bg-white/[0.02] border border-white/[0.08] text-sm text-zinc-400 space-y-2",
                                    p { class: "text-zinc-200 font-medium", "Contenuto isolato racchiuso in Collapsible" }
                                    p { "Questa componente può essere usata singolarmente in form complessi, card espandibili o filtri avanzati senza dipendere da un Accordion genitore." }
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
