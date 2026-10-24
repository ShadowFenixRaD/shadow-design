use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::components::{ChoicePicker, ControlRow, Counter, Playground, ToggleControl, FeatureBanner, PageHeader, PropRow, ShowcaseTabs};

#[component]
pub fn RatingPage() -> Element {
    let score = use_signal(|| 4.0);
    let half_score = use_signal(|| 3.5);
    let flame_score = use_signal(|| 5.0);

    let star_count = use_signal(|| 5usize);
    let fractions_val = use_signal(|| 1usize);
    let is_readonly = use_signal(|| false);
    let is_disabled = use_signal(|| false);

    let props = vec![
        PropRow {
            name: "value",
            r#type: "Signal<f64>",
            default: "-",
            description: "Signal reattivo bidirezionale collegato al punteggio di valutazione corrente",
        },
        PropRow {
            name: "count",
            r#type: "usize",
            default: "5",
            description: "Numero totale di stelle o icone visualizzate nella scala",
        },
        PropRow {
            name: "fractions",
            r#type: "usize",
            default: "1",
            description: "Frazioni consentite: 1 per stelle intere (1, 2, 3), 2 per mezze stelle (0.5, 1.0, 1.5)",
        },
        PropRow {
            name: "readonly",
            r#type: "bool",
            default: "false",
            description: "Impedisce le modifiche dell'utente visualizzando la valutazione in sola lettura",
        },
        PropRow {
            name: "disabled",
            r#type: "bool",
            default: "false",
            description: "Disabilita le interazioni opacizzando il componente",
        },
        PropRow {
            name: "icon",
            r#type: "Option<IconKind>",
            default: "None",
            description: "Icona vettoriale personalizzata da utilizzare al posto della classica stella a 5 punte",
        },
        PropRow {
            name: "size",
            r#type: "u32",
            default: "22",
            description: "Dimensione in pixel di ciascuna stella o icona",
        },
    ];

    let code_snippet = r#"let mut score = use_signal(|| 4.0);
let mut half_score = use_signal(|| 3.5);

// Stelle Intere Standard
Rating {
    value: score,
    count: 5,
    fractions: 1,
}

// Mezze Stelle (Precisione 0.5)
Rating {
    value: half_score,
    count: 5,
    fractions: 2,
}

// Icona Personalizzata (Flame)
Rating {
    value: score,
    icon: Some(IconKind::Flame),
    size: 24,
}"#
    .to_string();

    rsx! {
        div { class: "sc-stack sc-stack--loose",
            PageHeader {
                title: "Rating".to_string(),
                description: "Componente di valutazione a stelle o icone vettoriali con supporto per mezze valutazioni (precisione 0.5), anteprima hover dinamica e navigazione da tastiera (ArrowLeft / ArrowRight).".to_string(),
            }

            FeatureBanner { feature_name: "rating" }

            ShowcaseTabs {
                preview: rsx! {
                    Playground {
                        stage_start: true,
                        stage: rsx! {
                            div { class: "sc-stack w-full",
                                div { class: "sc-stack",
                                    Counter { label: "Valutazione standard".to_string(), value: score().to_string() }
                                    Rating {
                                        value: score,
                                        count: star_count(),
                                        fractions: fractions_val(),
                                        readonly: is_readonly(),
                                        disabled: is_disabled(),
                                    }
                                }
                                hr { class: "sc-control-divider" }
                                div { class: "sc-stack",
                                    Counter { label: "Mezze stelle (frazioni 0.5)".to_string(), value: half_score().to_string() }
                                    Rating {
                                        value: half_score,
                                        count: 5,
                                        fractions: 2,
                                        readonly: is_readonly(),
                                        disabled: is_disabled(),
                                    }
                                }
                                hr { class: "sc-control-divider" }
                                div { class: "sc-stack",
                                    Counter { label: "Icone custom (Flame)".to_string(), value: flame_score().to_string() }
                                    Rating {
                                        value: flame_score,
                                        count: 5,
                                        icon: Some(IconKind::Flame),
                                        size: 24,
                                        readonly: is_readonly(),
                                        disabled: is_disabled(),
                                    }
                                }
                            }
                        },
                        controls: rsx! {
                            ChoicePicker {
                                label: "Numero stelle".to_string(),
                                options: vec![
                                    (5usize, "5 stelle".to_string()),
                                    (7usize, "7 stelle".to_string()),
                                    (10usize, "10 stelle".to_string()),
                                ],
                                value: star_count,
                            }
                            ChoicePicker {
                                label: "Frazioni".to_string(),
                                options: vec![
                                    (1usize, "Intere (1.0)".to_string()),
                                    (2usize, "Mezze (0.5)".to_string()),
                                ],
                                value: fractions_val,
                            }
                            hr { class: "sc-control-divider" }
                            ControlRow {
                                ToggleControl { label: "Sola lettura (readonly)".to_string(), checked: is_readonly }
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
