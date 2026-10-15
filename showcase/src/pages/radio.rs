use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::components::{FeatureBanner, PropRow, ShowcaseTabs};

#[component]
pub fn RadioPage() -> Element {
    let plan = use_signal(|| "pro".to_string());
    let storage_type = use_signal(|| "nvme".to_string());
    let mut orientation = use_signal(|| RadioOrientation::Vertical);
    let mut is_disabled = use_signal(|| false);
    let mut has_error = use_signal(|| false);

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
        div { class: "max-w-4xl space-y-8",
            div {
                Heading { order: HeadingOrder::H2, "Radio, RadioGroup & RadioCard" }
                Text { dimmed: true, size: TextSize::Sm, "Selezione a scelta singola mutualmente esclusiva con supporto per layout orizzontale o verticale e variante ricca RadioCard con icone e micro-glow del tema attivo." }
            }

            FeatureBanner { feature_name: "radio" }

            ShowcaseTabs {
                preview: rsx! {
                    div { class: "grid grid-cols-1 lg:grid-cols-2 gap-8 items-start",
                        // Live Preview
                        div { class: "p-6 rounded-xl bg-[#060608] border border-white/[0.08] flex flex-col gap-6 shadow-inner",
                            // Classic RadioGroup
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

                            // Rich RadioCard Group
                            div { class: "pt-4 border-t border-white/[0.06] flex flex-col gap-3",
                                span { class: "text-xs font-semibold text-zinc-300 uppercase tracking-wider font-mono",
                                    "Architettura Storage (RadioCard)"
                                }
                                RadioGroup {
                                    value: storage_type,
                                    orientation: RadioOrientation::Vertical,
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

                        // Configurator
                        div { class: "space-y-4 text-xs font-mono",
                            div {
                                span { class: "text-zinc-400 block mb-1.5 uppercase tracking-wider text-[10px]", "Orientamento Gruppo:" }
                                div { class: "flex gap-2",
                                    button {
                                        r#type: "button",
                                        class: if orientation() == RadioOrientation::Vertical {
                                            "px-2.5 py-1 rounded bg-purple-950/80 text-purple-200 border border-purple-500/40 font-bold cursor-pointer"
                                        } else {
                                            "px-2.5 py-1 rounded bg-white/[0.03] text-zinc-400 border border-white/[0.06] cursor-pointer"
                                        },
                                        onclick: move |_| orientation.set(RadioOrientation::Vertical),
                                        "Verticale"
                                    }
                                    button {
                                        r#type: "button",
                                        class: if orientation() == RadioOrientation::Horizontal {
                                            "px-2.5 py-1 rounded bg-purple-950/80 text-purple-200 border border-purple-500/40 font-bold cursor-pointer"
                                        } else {
                                            "px-2.5 py-1 rounded bg-white/[0.03] text-zinc-400 border border-white/[0.06] cursor-pointer"
                                        },
                                        onclick: move |_| orientation.set(RadioOrientation::Horizontal),
                                        "Orizzontale"
                                    }
                                }
                            }

                            div { class: "pt-2 flex flex-col gap-2 border-t border-white/[0.06]",
                                label { class: "flex items-center gap-2 cursor-pointer text-zinc-300",
                                    input {
                                        r#type: "checkbox",
                                        checked: is_disabled(),
                                        oninput: move |e| is_disabled.set(e.value() == "true"),
                                    }
                                    "Disabilitato"
                                }
                                label { class: "flex items-center gap-2 cursor-pointer text-zinc-300",
                                    input {
                                        r#type: "checkbox",
                                        checked: has_error(),
                                        oninput: move |e| has_error.set(e.value() == "true"),
                                    }
                                    "Stato Errore"
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
