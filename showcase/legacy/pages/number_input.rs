use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::components::{ChoicePicker, ControlRow, Playground, ToggleControl, FeatureBanner, PageHeader, PropRow, ShowcaseTabs};

#[component]
pub fn NumberInputPage() -> Element {
    let quantity = use_signal(|| 1.0);
    let price = use_signal(|| 49.99);
    let percentage = use_signal(|| 75.0);

    let step_val = use_signal(|| 1.0);
    let precision_val = use_signal(|| 0usize);
    let is_disabled = use_signal(|| false);
    let has_error = use_signal(|| false);

    let props = vec![
        PropRow {
            name: "value",
            r#type: "Signal<f64>",
            default: "-",
            description: "Signal reattivo bidirezionale che memorizza il valore numerico corrente",
        },
        PropRow {
            name: "step",
            r#type: "f64",
            default: "1.0",
            description: "Passo di incremento e decremento tramite pulsanti stepper e frecce da tastiera",
        },
        PropRow {
            name: "min",
            r#type: "Option<f64>",
            default: "None",
            description: "Limite minimo consentito per il valore",
        },
        PropRow {
            name: "max",
            r#type: "Option<f64>",
            default: "None",
            description: "Limite massimo consentito per il valore",
        },
        PropRow {
            name: "precision",
            r#type: "usize",
            default: "0",
            description: "Numero di cifre decimali dopo la virgola (es. 2 per prezzi monetari)",
        },
        PropRow {
            name: "prefix",
            r#type: "Option<String>",
            default: "None",
            description: "Prefisso visivo visualizzato prima dell'input (es. '$', '€', '#')",
        },
        PropRow {
            name: "suffix",
            r#type: "Option<String>",
            default: "None",
            description: "Suffisso visivo visualizzato dopo l'input (es. '%', 'px', 'ms')",
        },
        PropRow {
            name: "disabled",
            r#type: "bool",
            default: "false",
            description: "Disabilita il campo di input e i pulsanti stepper",
        },
        PropRow {
            name: "error",
            r#type: "Option<String>",
            default: "None",
            description: "Messaggio di errore visualizzato sotto il campo con bordo cremisi",
        },
    ];

    let code_snippet = r#"let mut quantity = use_signal(|| 1.0);
let mut price = use_signal(|| 49.99);

NumberInput {
    label: "Quantità ordine".to_string(),
    value: quantity,
    step: 1.0,
    min: Some(1.0),
    max: Some(100.0),
}

NumberInput {
    label: "Prezzo unitario".to_string(),
    value: price,
    prefix: Some("€".to_string()),
    suffix: Some("EUR".to_string()),
    precision: 2,
    step: 0.5,
}"#
    .to_string();

    rsx! {
        div { class: "sc-stack sc-stack--loose",
            PageHeader {
                title: "NumberInput / SpinButton".to_string(),
                description: "Controllo numerico con stepper su/giù, precisione decimale, limiti min/max, supporto per prefisso/suffisso e navigazione da tastiera (ArrowUp / ArrowDown).".to_string(),
            }

            FeatureBanner { feature_name: "number_input" }

            ShowcaseTabs {
                preview: rsx! {
                    Playground {
                        stage_start: true,
                        stage: rsx! {
                            div { class: "sc-stack w-full",
                                NumberInput {
                                    label: "Quantità (Intero standard)".to_string(),
                                    value: quantity,
                                    step: step_val(),
                                    min: Some(0.0),
                                    max: Some(100.0),
                                    precision: precision_val(),
                                    disabled: is_disabled(),
                                    error: if has_error() { Some("Quantità non valida".to_string()) } else { None },
                                    help_text: Some(format!("Valore corrente: {}", quantity())),
                                }
                                NumberInput {
                                    label: "Prezzo con Valuta".to_string(),
                                    value: price,
                                    prefix: Some("€".to_string()),
                                    suffix: Some("EUR".to_string()),
                                    step: 0.50,
                                    precision: 2,
                                    min: Some(0.0),
                                    disabled: is_disabled(),
                                }
                                NumberInput {
                                    label: "Percentuale CPU".to_string(),
                                    value: percentage,
                                    suffix: Some("%".to_string()),
                                    step: 5.0,
                                    min: Some(0.0),
                                    max: Some(100.0),
                                    disabled: is_disabled(),
                                }
                            }
                        },
                        controls: rsx! {
                            ChoicePicker {
                                label: "Step incremento".to_string(),
                                options: vec![
                                    (1.0, "+/- 1".to_string()),
                                    (5.0, "+/- 5".to_string()),
                                    (10.0, "+/- 10".to_string()),
                                ],
                                value: step_val,
                            }
                            ChoicePicker {
                                label: "Precisione decimali".to_string(),
                                options: vec![
                                    (0usize, "0 dec".to_string()),
                                    (1usize, "1 dec".to_string()),
                                    (2usize, "2 dec".to_string()),
                                ],
                                value: precision_val,
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
