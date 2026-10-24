use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::components::{
    ChoicePicker, DemoCell, FeatureBanner, PageHeader, Playground, PropRow, ShowcaseTabs, ToggleControl,
};

#[component]
pub fn GroupPage() -> Element {
    let selected_gap = use_signal(|| SpacingToken::Md);
    let selected_align = use_signal(|| AlignItems::Center);
    let selected_justify = use_signal(|| JustifyContent::Start);
    let wrap = use_signal(|| true);
    let grow = use_signal(|| false);

    let props = vec![
        PropRow {
            name: "gap",
            r#type: "SpacingToken",
            default: "SpacingToken::Md",
            description: "Spaziatura orizzontale e verticale tra gli elementi (None, Xs, Sm, Md, Lg, Xl).",
        },
        PropRow {
            name: "align",
            r#type: "AlignItems",
            default: "AlignItems::Center",
            description: "Allineamento trasversale (cross-axis: Start, Center, End, Stretch, Baseline).",
        },
        PropRow {
            name: "justify",
            r#type: "JustifyContent",
            default: "JustifyContent::Start",
            description: "Giustificazione lungo l'asse orizzontale (Start, Center, End, SpaceBetween, SpaceAround, SpaceEvenly).",
        },
        PropRow {
            name: "wrap",
            r#type: "bool",
            default: "true",
            description: "Abilita o disabilita il ritorno a capo automatico (flex-wrap) quando lo spazio si esaurisce.",
        },
        PropRow {
            name: "grow",
            r#type: "bool",
            default: "false",
            description: "Se abilitato, forza tutti gli elementi figli ad espandersi equamente per riempire lo spazio residuo.",
        },
        PropRow {
            name: "class",
            r#type: "String",
            default: "\"\"",
            description: "Classi CSS aggiuntive applicate al contenitore Group.",
        },
    ];

    let code_snippet = r#"Group {
    gap: SpacingToken::Md,
    align: AlignItems::Center,
    justify: JustifyContent::SpaceBetween,
    wrap: true,
    grow: false,
    Button { variant: ButtonVariant::Primary, "Azione Primaria" }
    Button { variant: ButtonVariant::Outline, "Azione Secondaria" }
    Badge { label: "Stato Attivo".to_string(), variant: BadgeVariant::Success }
}"#
    .to_string();

    rsx! {
        div { class: "sc-stack sc-stack--loose",
            PageHeader {
                title: "Group".to_string(),
                description: "Primitiva Flexbox orizzontale ad alta ergonomia. Gestisce allineamento, giustificazione, spaziatura tokenizzata uniforme e ritorno a capo automatico senza codice CSS boilerplate.".to_string(),
            }

            FeatureBanner { feature_name: "group" }

            ShowcaseTabs {
                preview: rsx! {
                    Playground {
                        stage_start: true,
                        stage: rsx! {
                            Group {
                                gap: selected_gap(),
                                align: selected_align(),
                                justify: selected_justify(),
                                wrap: wrap(),
                                grow: grow(),
                                class: "w-full",
                                Button { variant: ButtonVariant::Primary, size: ButtonSize::Sm, "Primario" }
                                Button { variant: ButtonVariant::Outline, size: ButtonSize::Sm, "Secondario" }
                                Button { variant: ButtonVariant::Ghost, size: ButtonSize::Sm, "Ghost" }
                                Badge { label: "Mesh Online".to_string(), variant: BadgeVariant::Success, dot: true }
                                Badge { label: "Kyber-1024".to_string(), variant: BadgeVariant::Primary }
                                DemoCell { label: Some("Custom Box".into()), short: true }
                            }
                        },
                        controls: rsx! {
                            ChoicePicker {
                                label: "Spaziatura (Gap)".to_string(),
                                options: [SpacingToken::None, SpacingToken::Xs, SpacingToken::Sm, SpacingToken::Md, SpacingToken::Lg, SpacingToken::Xl]
                                    .iter()
                                    .map(|s| (*s, format!("{s:?}")))
                                    .collect::<Vec<_>>(),
                                value: selected_gap,
                            }
                            ChoicePicker {
                                label: "Allineamento (AlignItems)".to_string(),
                                options: [AlignItems::Start, AlignItems::Center, AlignItems::End, AlignItems::Stretch, AlignItems::Baseline]
                                    .iter()
                                    .map(|a| (*a, format!("{a:?}")))
                                    .collect::<Vec<_>>(),
                                value: selected_align,
                            }
                            ChoicePicker {
                                label: "Giustificazione (JustifyContent)".to_string(),
                                options: [JustifyContent::Start, JustifyContent::Center, JustifyContent::End, JustifyContent::SpaceBetween, JustifyContent::SpaceAround, JustifyContent::SpaceEvenly]
                                    .iter()
                                    .map(|j| (*j, format!("{j:?}")))
                                    .collect::<Vec<_>>(),
                                value: selected_justify,
                            }
                            hr { class: "sc-control-divider" }
                            ToggleControl { label: "Wrap (Ritorno a capo automatico)".to_string(), checked: wrap }
                            ToggleControl { label: "Grow (Espansione uniforme elementi)".to_string(), checked: grow }
                        },
                    }
                },
                code: code_snippet,
                props: props,
            }
        }
    }
}
