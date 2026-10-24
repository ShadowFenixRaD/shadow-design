use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::components::{ChoicePicker, ControlRow, Playground, ToggleControl, FeatureBanner, PageHeader, PropRow, ShowcaseTabs};

#[component]
pub fn RadioPage() -> Element {
    let plan = use_signal(|| "pro".to_string());
    let storage_type = use_signal(|| "nvme".to_string());
    let orientation = use_signal(|| RadioOrientation::Vertical);
    let is_disabled = use_signal(|| false);
    let has_error = use_signal(|| false);

    let props = vec![
        PropRow {
            name: "value",
            r#type: "Signal<String>",
            default: "-",
            description: "Signal reattivo bidirezionale legato al valore dell'opzione radio selezionata",
        },
        PropRow {
            name: "orientation",
            r#type: "RadioOrientation",
            default: "RadioOrientation::Vertical",
            description: "Disposizione degli elementi radio nel gruppo: Vertical o Horizontal",
        },
        PropRow {
            name: "name",
            r#type: "Option<String>",
            default: "None",
            description: "Nome univoco del gruppo per accessibilità nativa HTML dei campi radio",
        },
        PropRow {
            name: "label",
            r#type: "Option<String>",
            default: "None",
            description: "Etichetta descrittiva del gruppo o del singolo pulsante",
        },
        PropRow {
            name: "description",
            r#type: "Option<String>",
            default: "None",
            description: "Testo secondario di supporto per chiarire la funzione dell'opzione",
        },
        PropRow {
            name: "disabled",
            r#type: "bool",
            default: "false",
            description: "Disabilita l'interazione sul gruppo o sul singolo elemento radio",
        },
    ];

    let code_snippet = r#"let mut selected = use_signal(|| "pro".to_string());

// Standard RadioGroup
RadioGroup {
    value: selected,
    label: Some("Seleziona il piano".to_string()),
    orientation: RadioOrientation::Vertical,
    Radio {
        value: "free".to_string(),
        label: Some("Community Free".to_string()),
        description: Some("Per progetti open source".to_string()),
    }
    Radio {
        value: "pro".to_string(),
        label: Some("Pro Developer".to_string()),
        description: Some("Funzionalità avanzate illimitate".to_string()),
    }
}

// Rich RadioCard Group
RadioGroup {
    value: selected,
    orientation: RadioOrientation::Horizontal,
    RadioCard {
        value: "cloud".to_string(),
        title: Some("Cloud Managed".to_string()),
        description: Some("Serverless cluster distribuito".to_string()),
        icon: Some(IconKind::Server),
        badge: Some("Popular".to_string()),
    }
}"#
    .to_string();

    rsx! {
        div { class: "sc-stack sc-stack--loose",
            PageHeader {
                title: "Radio, RadioGroup & RadioCard".to_string(),
                description: "Selezione a scelta singola mutualmente esclusiva con supporto per layout orizzontale o verticale e variante ricca RadioCard con icone e micro-glow del tema attivo.".to_string(),
            }

            FeatureBanner { feature_name: "radio" }

            ShowcaseTabs {
                preview: rsx! {
                    Playground {
                        stage_start: true,
                        stage: rsx! {
                            div { class: "sc-stack sc-stack--loose w-full",
                                RadioGroup {
                                    value: plan,
                                    label: Some("Livello di Licenza".to_string()),
                                    orientation: orientation(),
                                    disabled: is_disabled(),
                                    error: if has_error() { Some("Seleziona un piano valido".to_string()) } else { None },
                                    help_text: Some(format!("Piano attivo: [{}]", plan())),

                                    Radio {
                                        value: "community".to_string(),
                                        label: Some("Community Edition".to_string()),
                                        description: Some("Accesso agli strumenti base e supporto forum".to_string()),
                                    }
                                    Radio {
                                        value: "pro".to_string(),
                                        label: Some("Pro Enterprise".to_string()),
                                        description: Some("Cluster ad alta disponibilità e crittografia PQ".to_string()),
                                    }
                                    Radio {
                                        value: "unlimited".to_string(),
                                        label: Some("Shadowfenix Core".to_string()),
                                        description: Some("Infrastruttura mesh distribuita completa".to_string()),
                                    }
                                }

                                div { class: "sc-stack",
                                    span { class: "sc-control-label", "Architettura storage (RadioCard)" }
                                    RadioGroup {
                                        value: storage_type,
                                        orientation: orientation(),
                                        disabled: is_disabled(),

                                        RadioCard {
                                            value: "nvme".to_string(),
                                            title: Some("Ultra NVMe Tier 1".to_string()),
                                            description: Some("1.2M IOPS, latenza sub-millisecondo".to_string()),
                                            icon: Some(IconKind::Server),
                                            badge: Some("Consigliato".to_string()),
                                        }
                                        RadioCard {
                                            value: "distributed".to_string(),
                                            title: Some("Mesh Distributed P2P".to_string()),
                                            description: Some("Sharding ridondato decentralizzato".to_string()),
                                            icon: Some(IconKind::Layers),
                                            badge: Some("Zero-Knowledge".to_string()),
                                        }
                                    }
                                }
                            }
                        },
                        controls: rsx! {
                            ChoicePicker {
                                label: "Orientamento gruppo".to_string(),
                                options: vec![
                                    (RadioOrientation::Vertical, "Verticale".to_string()),
                                    (RadioOrientation::Horizontal, "Orizzontale".to_string()),
                                ],
                                value: orientation,
                            }
                            hr { class: "sc-control-divider" }
                            ControlRow {
                                ToggleControl { label: "Disabilitato".to_string(), checked: is_disabled }
                                ToggleControl { label: "Stato errore".to_string(), checked: has_error }
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
