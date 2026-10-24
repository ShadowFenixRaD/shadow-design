use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::components::{
    ChoicePicker, FeatureBanner, PageHeader, Playground, PropRow, ShowcaseTabs, ToggleControl,
};

#[component]
pub fn AccordionPage() -> Element {
    let variant = use_signal(|| AccordionVariant::Separated);
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
        div { class: "sc-stack sc-stack--loose",
            PageHeader {
                title: "Accordion & Collapsible".to_string(),
                description: "Pannelli espandibili a fisarmonica conformi a WAI-ARIA con animazioni ottiche fluide, rotazione del chevron e tre varianti di superficie.".to_string(),
            }

            FeatureBanner { feature_name: "accordion" }

            ShowcaseTabs {
                preview: rsx! {
                    Playground {
                        stage_start: true,
                        stage: rsx! {
                        
                        Accordion {
                            multiple: is_multiple(),
                            variant: variant(),
                            default_value: vec!["mesh-routing".to_string()],

                            AccordionItem { value: "mesh-routing".to_string(),
                                AccordionControl { icon: Some(IconKind::Radio), "Architettura di Rete & Mesh Routing" }
                                AccordionPanel {
                                    p { class: "sc-muted-text", "La rete Shadowfenix impiega un protocollo peer-to-peer a maglia con routing ad anello Kademlia distribuito e crittografia hop-by-hop." }
                                    p { class: "sc-subtle-text sc-mono", "Latenza tipica: < 12ms • Algoritmo: Kademlia DHT" }
                                }
                            }

                            AccordionItem { value: "security-keys".to_string(),
                                AccordionControl { icon: Some(IconKind::Eye), "Crittografia & Firme Ed25519" }
                                AccordionPanel {
                                    p { class: "sc-muted-text", "Ogni transazione e messaggio scambiato nel cluster viene firmato asimmetricamente tramite chiavi Curve25519 ad altissima velocità." }
                                }
                            }

                            AccordionItem { value: "storage-cluster".to_string(),
                                AccordionControl { icon: Some(IconKind::Box), "Storage Replicato Distribuito" }
                                AccordionPanel {
                                    p { class: "sc-muted-text", "Archiviazione a oggetti decentralizzata con sharding deterministico e correzione automatica degli errori Reed-Solomon." }
                                }
                            }

                            AccordionItem { value: "disabled-node".to_string(), disabled: true,
                                AccordionControl { icon: Some(IconKind::Terminal), "Cluster Telemetry (Nodo Offline)" }
                                AccordionPanel {
                                    p { class: "sc-muted-text", "Questa opzione non è disponibile quando il nodo è in modalità isolata." }
                                }
                            }
                        }

                        // Sezione Collapsible Standalone
                        div { class: "sc-stack",
                            div { class: "sc-title-row",
                                span { class: "sc-section-title", "Primitiva Standalone: Collapsible" }
                                Button {
                                    size: ButtonSize::Sm,
                                    variant: ButtonVariant::Outline,
                                    onclick: move |_| collapsible_open.toggle(),
                                    if collapsible_open() { "Collassa Pannello" } else { "Espandi Pannello" }
                                }
                            }

                            Collapsible { open: collapsible_open,
                                Card {
                                    p { class: "sc-section-title", "Contenuto isolato racchiuso in Collapsible" }
                                    p { class: "sc-muted-text", "Questa componente può essere usata singolarmente in form complessi, card espandibili o filtri avanzati senza dipendere da un Accordion genitore." }
                                }
                            }
                        }
                        },
                        controls: rsx! {
                            ChoicePicker {
                                label: "Variante".to_string(),
                                options: vec![
                                    (AccordionVariant::Separated, "Separated".to_string()),
                                    (AccordionVariant::Contained, "Contained".to_string()),
                                    (AccordionVariant::Default, "Default".to_string()),
                                ],
                                value: variant,
                            }
                            ToggleControl { label: "Espansione Multipla".to_string(), checked: is_multiple }
                        },
                    }
                },
                code: code_snippet,
                props: props,
            }
        }
    }
}
