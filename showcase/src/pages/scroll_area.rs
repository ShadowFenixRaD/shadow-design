use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::components::{ChoicePicker, ControlRow, DemoCell, Playground, ToggleControl, FeatureBanner, PageHeader, PropRow, ShowcaseTabs};

#[component]
pub fn ScrollAreaPage() -> Element {
    let scroll_type = use_signal(|| ScrollType::Hover);
    let offset_scrollbars = use_signal(|| true);
    let fade_edges = use_signal(|| true);
    let demo_mode = use_signal(|| 0); // 0: Vertical, 1: Horizontal, 2: Both

    let props = vec![
        PropRow {
            name: "scroll_type",
            r#type: "ScrollType",
            default: "ScrollType::Hover",
            description: "Comportamento di visualizzazione scrollbar: Hover, Always, Auto o Never",
        },
        PropRow {
            name: "offset_scrollbars",
            r#type: "bool",
            default: "false",
            description: "Aggiunge padding automatico per evitare che la scrollbar si sovrapponga visivamente al testo",
        },
        PropRow {
            name: "fade_edges",
            r#type: "bool",
            default: "false",
            description: "Attiva una maschera gradiente ottica (fade edges) alle estremità per evidenziare contenuto non visibile",
        },
        PropRow {
            name: "scroll_x",
            r#type: "bool",
            default: "false",
            description: "Abilita lo scorrimento lungo l'asse orizzontale",
        },
        PropRow {
            name: "scroll_y",
            r#type: "bool",
            default: "true",
            description: "Abilita lo scorrimento lungo l'asse verticale",
        },
        PropRow {
            name: "height / max_height",
            r#type: "Option<String>",
            default: "None",
            description: "Vincolo di altezza o altezza massima per l'area di scorrimento (es. '280px')",
        },
        PropRow {
            name: "width / max_width",
            r#type: "Option<String>",
            default: "None",
            description: "Vincolo di larghezza o larghezza massima per l'area di scorrimento",
        },
        PropRow {
            name: "viewport_class",
            r#type: "String",
            default: "String::new()",
            description: "Classi CSS addizionali applicate al viewport interno di scorrimento",
        },
    ];

    let code_snippet = r#"ScrollArea {
    scroll_type: ScrollType::Hover,
    offset_scrollbars: true,
    fade_edges: true,
    height: Some("280px".to_string()),
    div { class: "space-y-2 p-2",
        // Contenuto scrollabile con micro-scrollbar
    }
}"#
    .to_string();

    rsx! {
        div { class: "sc-stack sc-stack--loose",
            PageHeader {
                title: "ScrollArea".to_string(),
                description: "Contenitore a scorrimento con micro-scrollbar stilizzate coordinate al tema Shadow, dissolvenza bordi (fade edges) e supporto a scomparsa su inattività.".to_string(),
            }

            FeatureBanner { feature_name: "scroll_area" }

            ShowcaseTabs {
                preview: rsx! {
                    Playground {
                        stage_start: true,
                        stage: rsx! {
                            div { class: "sc-stack w-full",
                                span { class: "sc-control-label", "Shadow scroll container, 6px micro-track, ScrollType: {scroll_type():?}" }

                                if demo_mode() == 0 {
                                    ScrollArea {
                                        key: "demo-v-{demo_mode}",
                                        scroll_type: scroll_type(),
                                        offset_scrollbars: offset_scrollbars(),
                                        fade_edges: fade_edges(),
                                        scroll_x: false,
                                        scroll_y: true,
                                        height: Some("260px".to_string()),
                                        div { class: "sc-stack p-2",
                                            for i in 1..=20 {
                                                DemoCell {
                                                    key: "{i}",
                                                    index: Some(i),
                                                    label: Some(format!("Evento di sicurezza registrato nel blocco #{}", 1000 + i)),
                                                    short: true,
                                                }
                                            }
                                        }
                                    }
                                } else if demo_mode() == 1 {
                                    ScrollArea {
                                        key: "demo-h-{demo_mode}",
                                        scroll_type: scroll_type(),
                                        offset_scrollbars: offset_scrollbars(),
                                        fade_edges: fade_edges(),
                                        scroll_x: true,
                                        scroll_y: false,
                                        width: Some("100%".to_string()),
                                        div {
                                            class: "flex gap-4 p-2",
                                            style: "width: max-content;",
                                            for i in 1..=12 {
                                                DemoCell {
                                                    key: "{i}",
                                                    index: Some(i),
                                                    label: Some(format!("Nodo di routing, {} MB/s", i * 120)),
                                                    class: "w-64 shrink-0".to_string(),
                                                }
                                            }
                                        }
                                    }
                                } else {
                                    ScrollArea {
                                        key: "demo-b-{demo_mode}",
                                        scroll_type: scroll_type(),
                                        offset_scrollbars: offset_scrollbars(),
                                        fade_edges: fade_edges(),
                                        scroll_x: true,
                                        scroll_y: true,
                                        height: Some("320px".to_string()),
                                        width: Some("100%".to_string()),
                                        div {
                                            style: "width: max-content; padding: 8px;",
                                            div {
                                                style: "display: grid; grid-template-columns: repeat(8, 160px); gap: 12px;",
                                                for row in 1..=6 {
                                                    for col in 1..=8 {
                                                        DemoCell {
                                                            key: "cell-{row}-{col}",
                                                            label: Some(format!("NODE-[{row:02},{col:02}]")),
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        },
                        controls: rsx! {
                            ChoicePicker {
                                label: "Modalità scrollbar".to_string(),
                                options: vec![
                                    (ScrollType::Hover, "Hover".to_string()),
                                    (ScrollType::Always, "Always".to_string()),
                                    (ScrollType::Auto, "Auto".to_string()),
                                    (ScrollType::Never, "Never".to_string()),
                                ],
                                value: scroll_type,
                            }
                            ChoicePicker {
                                label: "Demo orientamento".to_string(),
                                options: vec![
                                    (0, "Verticale".to_string()),
                                    (1, "Orizzontale".to_string()),
                                    (2, "Bidirezionale".to_string()),
                                ],
                                value: demo_mode,
                            }
                            hr { class: "sc-control-divider" }
                            ControlRow {
                                ToggleControl { label: "Offset scrollbars".to_string(), checked: offset_scrollbars }
                                ToggleControl { label: "Fade edges".to_string(), checked: fade_edges }
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
