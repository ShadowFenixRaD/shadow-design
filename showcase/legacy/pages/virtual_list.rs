use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::components::{Counter, Stage, FeatureBanner, PageHeader, PropRow, ShowcaseTabs};

#[component]
pub fn VirtualListPage() -> Element {
    let item_count = 10_000usize;

    let props = vec![
        PropRow {
            name: "item_count",
            r#type: "usize",
            default: "-",
            description: "Numero totale di righe (supporta agevolmente decine di migliaia di nodi)",
        },
        PropRow {
            name: "item_height",
            r#type: "u32",
            default: "48",
            description: "Altezza uniforme in pixel di ogni singola riga per il calcolo geometrico",
        },
        PropRow {
            name: "height",
            r#type: "u32",
            default: "360",
            description: "Altezza in pixel del viewport visibile scorribile",
        },
        PropRow {
            name: "overscan",
            r#type: "usize",
            default: "4",
            description: "Numero di righe invisibili bufferizzate sopra e sotto per evitare flickers",
        },
        PropRow {
            name: "render_item",
            r#type: "Callback<usize, Element>",
            default: "-",
            description: "Funzione generatrice dell'RSX dato l'indice assoluto dell'elemento",
        },
    ];

    let code_snippet = r#"VirtualList {
    item_count: 10_000,
    item_height: 48,
    height: 360,
    overscan: 5,
    render_item: move |idx| rsx! {
        div { class: "flex items-center justify-between px-4 w-full h-full text-xs font-mono",
            span { "Record ID: #{idx + 1}" }
            span { class: "sc-accent-text", "0x{idx * 1337:08X}" }
        }
    },
}"#
    .to_string();

    rsx! {
        div { class: "sc-stack sc-stack--loose",
            PageHeader {
                title: "VirtualList (VirtualScroll)".to_string(),
                description: "Renderer virtualizzato ad alte prestazioni a 60 FPS per insiemi massivi di dati. Nel DOM sono istanziati unicamente i nodi correnti visibili.".to_string(),
            }

            FeatureBanner { feature_name: "virtual_list" }

            ShowcaseTabs {
                preview: rsx! {
                    Stage { start: true, wide: true,
                        div { class: "sc-stack w-full",
                            div { class: "sc-title-row",
                                Counter { label: "Elementi totali virtualizzati".to_string(), value: "10,000".to_string() }
                                Badge { label: "60 FPS smooth scroll".to_string(), variant: BadgeVariant::Success, dot: true }
                            }

                            VirtualList {
                                item_count: item_count,
                                item_height: 46,
                                height: 340,
                                overscan: 4,
                                render_item: move |idx| rsx! {
                                    div { class: "flex items-center justify-between px-4 w-full h-full sc-mono",
                                        div { class: "flex items-center gap-3",
                                            span { class: "w-16 sc-subtle-text", "#{idx + 1:05}" }
                                            span { "Nodo Mesh Datacenter #{idx + 1}" }
                                        }
                                        div { class: "flex items-center gap-4",
                                            span { class: "sc-subtle-text", "0x{(idx * 7919) % 0xFFFFFF:06X}" }
                                            span { class: if idx % 2 == 0 { "sc-success-text" } else { "sc-info-text" },
                                                if idx % 2 == 0 { "ONLINE" } else { "SYNCED" }
                                            }
                                        }
                                    }
                                },
                            }
                        }
                    }
                },
                code: code_snippet,
                props: props,
            }
        }
    }
}
