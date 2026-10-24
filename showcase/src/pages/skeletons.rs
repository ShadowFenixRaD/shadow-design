use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::components::{ChoicePicker, Playground, ToggleControl, FeatureBanner, PageHeader, PropRow, ShowcaseTabs};

#[component]
pub fn SkeletonsPage() -> Element {
    let selected_shape = use_signal(|| SkeletonShape::Text);
    let text_count = use_signal(|| 3usize);
    let is_animated = use_signal(|| true);

    let props = vec![
        PropRow {
            name: "shape",
            r#type: "SkeletonShape",
            default: "SkeletonShape::Text",
            description: "Forma geometrica del placeholder: Circle, Rectangle, Text",
        },
        PropRow {
            name: "count",
            r#type: "usize",
            default: "1",
            description: "Numero di righe di testo da simulare (l'ultima riga è resa leggermente più corta)",
        },
        PropRow {
            name: "animate",
            r#type: "bool",
            default: "true",
            description: "Attiva l'animazione di shimmer wave continua",
        },
        PropRow {
            name: "width",
            r#type: "Option<String>",
            default: "None",
            description: "Larghezza CSS esplicita (es. '120px', '80%')",
        },
        PropRow {
            name: "height",
            r#type: "Option<String>",
            default: "None",
            description: "Altezza CSS esplicita (es. '48px')",
        },
    ];

    let code_snippet = r#"// Placeholder Card profilo durante caricamento:
div { class: "flex items-center gap-4",
    Skeleton { shape: SkeletonShape::Circle }
    div { class: "space-y-2 flex-1",
        Skeleton { shape: SkeletonShape::Text, count: 2 }
    }
}"#
    .to_string();

    rsx! {
        div { class: "sc-stack sc-stack--loose",
            PageHeader {
                title: "Skeleton & Placeholder".to_string(),
                description: "Placeholder di caricamento asincrono con animazione shimmer wave per profili, messaggi e schede prima della ricezione dei dati.".to_string(),
            }

            FeatureBanner { feature_name: "skeleton" }

            ShowcaseTabs {
                preview: rsx! {
                    Playground {
                        stage_start: true,
                        stage: rsx! {
                            div { class: "sc-stack w-full",
                                Skeleton {
                                    shape: selected_shape(),
                                    count: text_count(),
                                    animate: is_animated(),
                                }
                                hr { class: "sc-control-divider" }
                                span { class: "sc-control-label", "Esempio card caricamento" }
                                div { class: "flex items-center gap-3",
                                    Skeleton { shape: SkeletonShape::Circle, animate: is_animated() }
                                    div { class: "flex-1",
                                        Skeleton { shape: SkeletonShape::Text, count: 2, animate: is_animated() }
                                    }
                                }
                            }
                        },
                        controls: rsx! {
                            ChoicePicker {
                                label: "Forma (Shape)".to_string(),
                                options: vec![
                                    (SkeletonShape::Text, "Text (Righe)".to_string()),
                                    (SkeletonShape::Circle, "Circle (Cerchio)".to_string()),
                                    (SkeletonShape::Rectangle, "Rectangle (Card)".to_string()),
                                ],
                                value: selected_shape,
                            }
                            if selected_shape() == SkeletonShape::Text {
                                ChoicePicker {
                                    label: "Numero righe".to_string(),
                                    options: vec![
                                        (1usize, "1".to_string()),
                                        (2usize, "2".to_string()),
                                        (3usize, "3".to_string()),
                                        (4usize, "4".to_string()),
                                        (5usize, "5".to_string()),
                                    ],
                                    value: text_count,
                                }
                            }
                            hr { class: "sc-control-divider" }
                            ToggleControl { label: "Shimmer wave animato".to_string(), checked: is_animated }
                        },
                    }
                },
                code: code_snippet,
                props: props,
            }
        }
    }
}
