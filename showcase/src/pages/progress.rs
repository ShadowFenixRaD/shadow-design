use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::components::{ChoicePicker, ControlGroup, ControlRow, Playground, ToggleControl, FeatureBanner, PageHeader, PropRow, ShowcaseTabs};

#[component]
pub fn ProgressPage() -> Element {
    let progress_val = use_signal(|| 68.0f64);
    let selected_size = use_signal(|| ProgressSize::Md);
    let selected_variant = use_signal(|| ProgressVariant::Primary);
    let is_indeterminate = use_signal(|| false);
    let show_label = use_signal(|| true);

    let props = vec![
        PropRow {
            name: "value",
            r#type: "f32",
            default: "0.0",
            description: "Percentuale di avanzamento da 0.0 a 100.0",
        },
        PropRow {
            name: "size",
            r#type: "ProgressSize",
            default: "ProgressSize::Md",
            description: "Spessore della barra: Sm (4px), Md (8px), Lg (12px)",
        },
        PropRow {
            name: "variant",
            r#type: "ProgressVariant",
            default: "ProgressVariant::Primary",
            description: "Variante cromatica: Primary (neon accent), Success, Warning, Danger",
        },
        PropRow {
            name: "indeterminate",
            r#type: "bool",
            default: "false",
            description: "Attiva l'animazione shimmer continua per caricamenti infiniti o scansioni beacon",
        },
        PropRow {
            name: "show_label",
            r#type: "bool",
            default: "false",
            description: "Mostra automaticamente l'etichetta percentuale (es. '68%')",
        },
    ];

    let code_snippet = r#"ProgressBar {
    value: 68.0,
    size: ProgressSize::Md,
    variant: ProgressVariant::Primary,
    show_label: true,
}"#
    .to_string();

    rsx! {
        div { class: "sc-stack sc-stack--loose",
            PageHeader {
                title: "Progress & Barra di Avanzamento".to_string(),
                description: "Barra di progressione fluida per upload chunked, trasferimento file P2P, benchmarking latenza e scansioni radio.".to_string(),
            }

            FeatureBanner { feature_name: "progress" }

            ShowcaseTabs {
                preview: rsx! {
                    Playground {
                        stage_start: true,
                        stage: rsx! {
                            div { class: "w-full",
                                ProgressBar {
                                    value: progress_val() as f32,
                                    size: selected_size(),
                                    variant: selected_variant(),
                                    indeterminate: is_indeterminate(),
                                    show_label: show_label(),
                                }
                            }
                        },
                        controls: rsx! {
                            ChoicePicker {
                                label: "Spessore barra (Size)".to_string(),
                                options: vec![
                                    (ProgressSize::Sm, "SM (4px)".to_string()),
                                    (ProgressSize::Md, "MD (8px)".to_string()),
                                    (ProgressSize::Lg, "LG (12px)".to_string()),
                                ],
                                value: selected_size,
                            }
                            ChoicePicker {
                                label: "Variante cromatica".to_string(),
                                options: vec![
                                    (ProgressVariant::Primary, "Primary".to_string()),
                                    (ProgressVariant::Success, "Success".to_string()),
                                    (ProgressVariant::Warning, "Warning".to_string()),
                                    (ProgressVariant::Danger, "Danger".to_string()),
                                ],
                                value: selected_variant,
                            }
                            ControlGroup { label: format!("Valore: {:.0}%", progress_val()),
                                Slider { value: progress_val, min: 0.0, max: 100.0, step: 1.0 }
                            }
                            hr { class: "sc-control-divider" }
                            ControlRow {
                                ToggleControl { label: "Indeterminato (scan shimmer)".to_string(), checked: is_indeterminate }
                                ToggleControl { label: "Mostra etichetta %".to_string(), checked: show_label }
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
