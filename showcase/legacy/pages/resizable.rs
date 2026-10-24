use dioxus::prelude::*;
use shadow_design::prelude::*;
use shadow_design::resizable::ResizableDirection;

use crate::components::{ChoicePicker, Playground, FeatureBanner, PageHeader, PropRow, ShowcaseTabs};

#[component]
pub fn ResizablePage() -> Element {
    let current_direction = use_signal(|| ResizableDirection::Horizontal);
    let mut panel_1_size = use_signal(|| 35.0);
    let mut panel_2_size = use_signal(|| 65.0);

    let props = vec![
        PropRow {
            name: "ResizablePanelGroup",
            r#type: "Component",
            default: "-",
            description: "Contenitore flex che gestisce la direzione (Horizontal o Vertical)",
        },
        PropRow {
            name: "direction",
            r#type: "ResizableDirection",
            default: "Horizontal",
            description: "Orientamento dello split (Horizontal o Vertical)",
        },
        PropRow {
            name: "on_resize",
            r#type: "Option<EventHandler<f64>>",
            default: "None",
            description: "Callback invocato in tempo reale durante il trascinamento con la percentuale",
        },
        PropRow {
            name: "ResizablePanel",
            r#type: "Component",
            default: "-",
            description: "Pannello con default_size, min_size e max_size percentuali",
        },
        PropRow {
            name: "ResizableHandle",
            r#type: "Component",
            default: "-",
            description: "Maniglia divisoria trascinabile con icona grip opzionale",
        },
    ];

    let code_snippet = r#"ResizablePanelGroup {
    direction: ResizableDirection::Horizontal,
    on_resize: move |pct| println!("Panel 1 size: {pct:.1}%"),
    ResizablePanel { default_size: 35.0, min_size: 20.0, max_size: 70.0,
        div { "Sidebar di navigazione" }
    }
    ResizableHandle { with_handle: true }
    ResizablePanel { default_size: 65.0, min_size: 30.0,
        div { "Area di lavoro principale" }
    }
}"#
    .to_string();


    rsx! {
        div { class: "sc-stack sc-stack--loose",
            PageHeader {
                title: "Resizable (SplitPane)".to_string(),
                description: "Pannelli divisori ridimensionabili con maniglia di presa tattile per layout split orizzontali o verticali.".to_string(),
            }

            FeatureBanner { feature_name: "resizable" }

            ShowcaseTabs {
                preview: rsx! {
                    Playground {
                        stage_start: true,
                        stage: rsx! {
                            div { class: "sc-demo-frame w-full h-[320px] overflow-hidden",
                                ResizablePanelGroup {
                                    direction: current_direction(),
                                    on_resize: move |pct: f64| {
                                        panel_1_size.set(pct);
                                        panel_2_size.set(100.0 - pct);
                                    },
                                    ResizablePanel {
                                        default_size: 35.0,
                                        min_size: 20.0,
                                        max_size: 70.0,
                                        div { class: "h-full p-4 flex flex-col justify-between",
                                            div { class: "sc-stack",
                                                strong { class: "sc-accent-text", "Pannello 1 (Explorer)" }
                                                p { class: "sc-page-desc", "Albero file, moduli o parametri di configurazione." }
                                            }
                                            div {
                                                id: "sd-demo-panel-1-size",
                                                class: "sc-subtle-text sc-mono",
                                                "Size: {panel_1_size():.1}% (Min 20% / Max 70%)"
                                            }
                                        }
                                    }
                                    ResizableHandle { with_handle: true }
                                    ResizablePanel {
                                        default_size: 65.0,
                                        min_size: 30.0,
                                        div { class: "h-full p-4 flex flex-col justify-between",
                                            div { class: "sc-stack",
                                                strong { class: "sc-info-text", "Pannello 2 (Editor Principale)" }
                                                p { class: "sc-page-desc", "Visualizzazione contenuto, editor sorgente o canvas interattivo." }
                                            }
                                            div {
                                                id: "sd-demo-panel-2-size",
                                                class: "sc-subtle-text sc-mono",
                                                "Size: {panel_2_size():.1}%"
                                            }
                                        }
                                    }
                                }
                            }
                        },
                        controls: rsx! {
                            ChoicePicker {
                                label: "Direzione split".to_string(),
                                options: vec![
                                    (ResizableDirection::Horizontal, "Orizzontale".to_string()),
                                    (ResizableDirection::Vertical, "Verticale".to_string()),
                                ],
                                value: current_direction,
                                on_change: move |_| {
                                    panel_1_size.set(35.0);
                                    panel_2_size.set(65.0);
                                },
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
