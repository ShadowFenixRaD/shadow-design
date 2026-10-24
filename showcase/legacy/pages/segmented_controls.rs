use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::components::{ChoicePicker, ControlRow, Counter, Playground, ToggleControl, FeatureBanner, PageHeader, PropRow, ShowcaseTabs};

#[component]
pub fn SegmentedControlsPage() -> Element {
    let storage_val = use_signal(|| "sqlite".to_string());
    let crypto_val = use_signal(|| "pq".to_string());
    let selected_size = use_signal(|| SegmentedControlSize::Md);
    let is_full_width = use_signal(|| false);
    let is_disabled = use_signal(|| false);

    let props = vec![
        PropRow {
            name: "options",
            r#type: "Vec<SegmentOption>",
            default: "-",
            description: "Elenco delle opzioni selezionabili con chiave, etichetta e icona opzionale",
        },
        PropRow {
            name: "value",
            r#type: "Signal<String>",
            default: "-",
            description: "Signal reattivo bidirezionale collegato al valore dell'opzione selezionata",
        },
        PropRow {
            name: "size",
            r#type: "SegmentedControlSize",
            default: "SegmentedControlSize::Md",
            description: "Dimensione del controllo (Sm, Md, Lg)",
        },
        PropRow {
            name: "full_width",
            r#type: "bool",
            default: "false",
            description: "Espande il controllo per occupare l'intera larghezza disponibile del contenitore",
        },
        PropRow {
            name: "disabled",
            r#type: "bool",
            default: "false",
            description: "Disabilita tutte le interazioni sul selettore",
        },
    ];

    let code_snippet = r#"let mut storage_mode = use_signal(|| "sqlite".to_string());

SegmentedControl {
    value: storage_mode,
    options: vec![
        SegmentOption::new("ram", "RAM").with_icon(IconKind::Terminal.into()),
        SegmentOption::new("sqlite", "SQLite").with_icon(IconKind::Server.into()),
        SegmentOption::new("encrypted", "Cifrato").with_icon(IconKind::Lock.into()),
    ],
}"#
    .to_string();

    let storage_options = vec![
        SegmentOption::new("ram", "RAM Volatile").with_icon(IconKind::Terminal.into()),
        SegmentOption::new("sqlite", "SQLite").with_icon(IconKind::Server.into()),
        SegmentOption::new("encrypted", "Cifrato PQ").with_icon(IconKind::Lock.into()),
    ];

    let crypto_options = vec![
        SegmentOption::new("auto", "Auto").with_icon(IconKind::Sparkles.into()),
        SegmentOption::new("pq", "Post-Quantum").with_icon(IconKind::ShieldCheck.into()),
        SegmentOption::new("classic", "Classica").with_icon(IconKind::Key.into()),
        SegmentOption::new("compat", "Compat").with_icon(IconKind::Radio.into()),
    ];

    rsx! {
        div { class: "sc-stack sc-stack--loose",
            PageHeader {
                title: "SegmentedControl".to_string(),
                description: "Selettore orizzontale a segmenti tattile ad alta precisione con animazione scorrevole e micro-glow del tema attivo.".to_string(),
            }

            FeatureBanner { feature_name: "segmented_control" }

            ShowcaseTabs {
                preview: rsx! {
                    Playground {
                        stage: rsx! {
                            div { class: "sc-stack w-full items-center",
                                Counter { label: "Modalità storage".to_string(), value: storage_val() }
                                SegmentedControl {
                                    options: storage_options,
                                    value: storage_val,
                                    size: selected_size(),
                                    full_width: is_full_width(),
                                    disabled: is_disabled(),
                                }
                                hr { class: "sc-control-divider w-full" }
                                Counter { label: "Suite crittografica".to_string(), value: crypto_val() }
                                SegmentedControl {
                                    options: crypto_options,
                                    value: crypto_val,
                                    size: selected_size(),
                                    full_width: is_full_width(),
                                    disabled: is_disabled(),
                                }
                            }
                        },
                        controls: rsx! {
                            ChoicePicker {
                                label: "Dimensione SegmentedControl".to_string(),
                                options: vec![
                                    (SegmentedControlSize::Sm, "SM".to_string()),
                                    (SegmentedControlSize::Md, "MD".to_string()),
                                    (SegmentedControlSize::Lg, "LG".to_string()),
                                ],
                                value: selected_size,
                            }
                            hr { class: "sc-control-divider" }
                            ControlRow {
                                ToggleControl { label: "Larghezza piena".to_string(), checked: is_full_width }
                                ToggleControl { label: "Disabilitato".to_string(), checked: is_disabled }
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
