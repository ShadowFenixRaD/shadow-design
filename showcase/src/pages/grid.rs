use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::components::{
    ChoicePicker, DemoCell, FeatureBanner, PageHeader, Playground, PropRow, ShowcaseTabs,
};

#[component]
pub fn GridPage() -> Element {
    let selected_gutter = use_signal(|| GridSpacing::Md);
    let selected_align = use_signal(|| GridAlign::Stretch);
    let selected_justify = use_signal(|| GridJustify::Start);
    let simple_cols = use_signal(|| 3u8);

    let props = vec![
        PropRow {
            name: "gutter",
            r#type: "GridSpacing",
            default: "GridSpacing::Md",
            description: "Spaziatura orizzontale e verticale tra le celle (None, Xs, Sm, Md, Lg, Xl).",
        },
        PropRow {
            name: "row_gutter",
            r#type: "Option<GridSpacing>",
            default: "None",
            description: "Gutter verticale indipendente per separare le righe con passo diverso.",
        },
        PropRow {
            name: "align",
            r#type: "GridAlign",
            default: "GridAlign::Stretch",
            description: "Allineamento verticale delle celle (Start, Center, End, Stretch, Baseline).",
        },
        PropRow {
            name: "justify",
            r#type: "GridJustify",
            default: "GridJustify::Start",
            description: "Giustificazione orizzontale (Start, Center, End, SpaceBetween, SpaceAround).",
        },
        PropRow {
            name: "span (GridCol)",
            r#type: "u8",
            default: "12",
            description: "Numero di colonne occupate nella griglia a 12 colonne (1..=12).",
        },
        PropRow {
            name: "offset (GridCol)",
            r#type: "u8",
            default: "0",
            description: "Numero di colonne di offset iniziale per spaziatura o rientro a sinistra (0..=11).",
        },
        PropRow {
            name: "sm, md, lg, xl (GridCol)",
            r#type: "Option<u8>",
            default: "None",
            description: "Span responsive per i rispettivi breakpoint WAI-ARIA (640px, 768px, 1024px, 1280px).",
        },
        PropRow {
            name: "cols (SimpleGrid)",
            r#type: "u8",
            default: "1",
            description: "Numero di colonne a larghezza equivalente della SimpleGrid (1..=12).",
        },
        PropRow {
            name: "spacing (SimpleGrid)",
            r#type: "GridSpacing",
            default: "GridSpacing::Md",
            description: "Spaziatura uniforme orizzontale e verticale tra le celle di SimpleGrid.",
        },
    ];

    let code_snippet = r#"// --- Grid a 12 Colonne Responsive ---
Grid {
    gutter: GridSpacing::Md,
    align: GridAlign::Stretch,
    GridCol { span: 12, md: 8,
        DemoCell { label: "Colonna Principale (Span 8 su Desktop)".to_string(), accent: true }
    }
    GridCol { span: 12, md: 4,
        DemoCell { label: "Sidebar Laterale (Span 4 su Desktop)".to_string() }
    }
}

// --- SimpleGrid a Colonne Equivalenti ---
SimpleGrid {
    cols: 3,
    spacing: GridSpacing::Md,
    sm: 1, md: 2, lg: 3,
    DemoCell { label: "Cella A".to_string() }
    DemoCell { label: "Cella B".to_string() }
    DemoCell { label: "Cella C".to_string() }
}"#
    .to_string();

    rsx! {
        div { class: "sc-stack sc-stack--loose",
            PageHeader {
                title: "Grid & SimpleGrid".to_string(),
                description: "Sistemi strutturali responsive a 12 colonne (Grid / GridCol) e a colonne equivalenti auto-bilanciate (SimpleGrid) con gutter tokenizzato e breakpoint standard.".to_string(),
            }

            FeatureBanner { feature_name: "grid" }

            ShowcaseTabs {
                preview: rsx! {
                    Playground {
                        stage_start: true,
                        stage: rsx! {
                            div { class: "sc-stack w-full",
                                div { class: "sc-title-row",
                                    span { class: "sc-section-title", "1. Grid a 12 Colonne con Span e Offset" }
                                    span { class: "sc-subtle-text sc-mono", "Gutter: {selected_gutter():?}" }
                                }

                                Grid {
                                    gutter: selected_gutter(),
                                    align: selected_align(),
                                    justify: selected_justify(),
                                    GridCol { span: 12, md: 8,
                                        DemoCell { label: Some("Span: 12 (Mobile) / 8 (Desktop)".into()), accent: true }
                                    }
                                    GridCol { span: 12, md: 4,
                                        DemoCell { label: Some("Span: 12 / 4".into()) }
                                    }
                                    GridCol { span: 6, md: 4,
                                        DemoCell { label: Some("Colonna 1 (Span 4)".into()), short: true }
                                    }
                                    GridCol { span: 6, md: 4,
                                        DemoCell { label: Some("Colonna 2 (Span 4)".into()), short: true }
                                    }
                                    GridCol { span: 12, md: 4,
                                        DemoCell { label: Some("Colonna 3 (Span 4)".into()), short: true }
                                    }
                                    GridCol { span: 12, md: 6, offset: 3,
                                        DemoCell { label: Some("Span 6 con Offset 3 (Centrato su 12 col)".into()), short: true }
                                    }
                                }

                                hr { class: "sc-control-divider" }
                                div { class: "sc-title-row",
                                    span { class: "sc-section-title", "2. SimpleGrid a Colonne Equivalenti" }
                                    span { class: "sc-subtle-text sc-mono", "Cols attive: {simple_cols()}" }
                                }

                                SimpleGrid {
                                    cols: simple_cols(),
                                    spacing: selected_gutter(),
                                    sm: 1,
                                    md: simple_cols(),
                                    for i in 1..=6 {
                                        DemoCell { key: "{i}", index: Some(i), label: Some(format!("Modulo #{i}")) }
                                    }
                                }
                            }
                        },
                        controls: rsx! {
                            ChoicePicker {
                                label: "Gutter Spaziatura".to_string(),
                                options: [GridSpacing::None, GridSpacing::Xs, GridSpacing::Sm, GridSpacing::Md, GridSpacing::Lg, GridSpacing::Xl]
                                    .iter()
                                    .map(|s| (*s, format!("{s:?}")))
                                    .collect::<Vec<_>>(),
                                value: selected_gutter,
                            }
                            ChoicePicker {
                                label: "Allineamento Verticale".to_string(),
                                options: [GridAlign::Start, GridAlign::Center, GridAlign::End, GridAlign::Stretch]
                                    .iter()
                                    .map(|a| (*a, format!("{a:?}")))
                                    .collect::<Vec<_>>(),
                                value: selected_align,
                            }
                            ChoicePicker {
                                label: "Giustificazione Orizzontale".to_string(),
                                options: [GridJustify::Start, GridJustify::Center, GridJustify::End, GridJustify::SpaceBetween, GridJustify::SpaceAround]
                                    .iter()
                                    .map(|j| (*j, format!("{j:?}")))
                                    .collect::<Vec<_>>(),
                                value: selected_justify,
                            }
                            ChoicePicker {
                                label: "Colonne SimpleGrid".to_string(),
                                options: [1u8, 2, 3, 4, 6]
                                    .iter()
                                    .map(|c| (*c, format!("{c} Colonne")))
                                    .collect::<Vec<_>>(),
                                value: simple_cols,
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
