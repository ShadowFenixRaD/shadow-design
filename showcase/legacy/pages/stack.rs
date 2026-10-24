use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::components::{
    ChoicePicker, DemoCell, FeatureBanner, PageHeader, Playground, PropRow, ShowcaseTabs,
};

#[component]
pub fn StackPage() -> Element {
    let selected_gap = use_signal(|| SpacingToken::Md);
    let selected_align = use_signal(|| AlignItems::Stretch);
    let selected_justify = use_signal(|| JustifyContent::Start);

    let props = vec![
        PropRow {
            name: "gap",
            r#type: "SpacingToken",
            default: "SpacingToken::Md",
            description: "Spaziatura verticale uniforme tra tutti i figli dello Stack (None, Xs, Sm, Md, Lg, Xl).",
        },
        PropRow {
            name: "align",
            r#type: "AlignItems",
            default: "AlignItems::Stretch",
            description: "Allineamento orizzontale trasversale dei figli (Start, Center, End, Stretch, Baseline).",
        },
        PropRow {
            name: "justify",
            r#type: "JustifyContent",
            default: "JustifyContent::Start",
            description: "Distribuzione verticale lungo l'asse principale (Start, Center, End, SpaceBetween, SpaceAround, SpaceEvenly).",
        },
        PropRow {
            name: "class",
            r#type: "String",
            default: "\"\"",
            description: "Classi CSS personalizzate applicate al contenitore Stack.",
        },
    ];

    let code_snippet = r#"Stack {
    gap: SpacingToken::Md,
    align: AlignItems::Stretch,
    Card {
        div { class: "p-3", "Primo blocco dati" }
    }
    Card {
        div { class: "p-3", "Secondo blocco dati" }
    }
    Button { variant: ButtonVariant::Primary, "Salva Configurazione" }
}"#
    .to_string();

    rsx! {
        div { class: "sc-stack sc-stack--loose",
            PageHeader {
                title: "Stack".to_string(),
                description: "Primitiva Flexbox verticale per impilare componenti con spaziatura tokenizzata, allineamento calibrato e zero margini esterni.".to_string(),
            }

            FeatureBanner { feature_name: "stack" }

            ShowcaseTabs {
                preview: rsx! {
                    Playground {
                        stage_start: true,
                        stage: rsx! {
                            div { class: "sc-demo-frame w-full",
                                Stack {
                                    gap: selected_gap(),
                                    align: selected_align(),
                                    justify: selected_justify(),
                                    class: "w-full",
                                    DemoCell { index: Some(1), label: Some("Modulo Alpha (Security Core)".to_string()), accent: true }
                                    DemoCell { index: Some(2), label: Some("Servizio Router Mesh P2P".to_string()) }
                                    DemoCell { index: Some(3), label: Some("Canale Telemetrico e Log".to_string()) }
                                }
                            }
                        },
                        controls: rsx! {
                            ChoicePicker {
                                label: "Spaziatura verticale (Gap)".to_string(),
                                options: vec![
                                    (SpacingToken::None, "None".to_string()),
                                    (SpacingToken::Xs, "Xs".to_string()),
                                    (SpacingToken::Sm, "Sm".to_string()),
                                    (SpacingToken::Md, "Md".to_string()),
                                    (SpacingToken::Lg, "Lg".to_string()),
                                    (SpacingToken::Xl, "Xl".to_string()),
                                ],
                                value: selected_gap,
                            }
                            ChoicePicker {
                                label: "Allineamento orizzontale (AlignItems)".to_string(),
                                options: vec![
                                    (AlignItems::Start, "Start".to_string()),
                                    (AlignItems::Center, "Center".to_string()),
                                    (AlignItems::End, "End".to_string()),
                                    (AlignItems::Stretch, "Stretch".to_string()),
                                ],
                                value: selected_align,
                            }
                            ChoicePicker {
                                label: "Giustificazione verticale (JustifyContent)".to_string(),
                                options: vec![
                                    (JustifyContent::Start, "Start".to_string()),
                                    (JustifyContent::Center, "Center".to_string()),
                                    (JustifyContent::End, "End".to_string()),
                                    (JustifyContent::SpaceBetween, "Between".to_string()),
                                ],
                                value: selected_justify,
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
