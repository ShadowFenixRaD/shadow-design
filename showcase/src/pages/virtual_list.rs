use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::components::{FeatureBanner, PropRow, ShowcaseTabs};

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
            span { class: "text-purple-400", "0x{idx * 1337:08X}" }
        }
    },
}"#
    .to_string();

    rsx! {
        div { class: "max-w-4xl space-y-8",
            div {
                Heading { order: HeadingOrder::H2, "VirtualList (VirtualScroll)" }
                Text { dimmed: true, size: TextSize::Sm, "Renderer virtualizzato ad alte prestazioni a 60 FPS per insiemi massivi di dati. Nel DOM sono istanziati unicamente i nodi correnti visibili." }
            }

            FeatureBanner { feature_name: "virtual_list" }

            ShowcaseTabs {
                preview: rsx! {
                    div { class: "p-8 rounded-xl bg-[#060608] border border-white/[0.08] flex flex-col gap-4 shadow-inner",
                        div { class: "flex items-center justify-between text-xs font-mono text-zinc-400",
                            span { "Elementi totali virtualizzati: " span { class: "text-purple-300 font-bold", "10,000" } }
                            span { class: "text-emerald-400 font-semibold", "● 60 FPS Smooth Scroll" }
                        }

                        VirtualList {
                            item_count: item_count,
                            item_height: 46,
                            height: 340,
                            overscan: 4,
                            render_item: move |idx| rsx! {
                                div { class: "flex items-center justify-between px-4 w-full h-full text-xs font-mono",
                                    div { class: "flex items-center gap-3",
                                        span { class: "w-16 text-zinc-600 font-bold", "#{idx + 1:05}" }
                                        span { class: "text-zinc-200", "Nodo Mesh Datacenter #{idx + 1}" }
                                    }
                                    div { class: "flex items-center gap-4",
                                        span { class: "text-zinc-500 text-[11px]", "0x{(idx * 7919) % 0xFFFFFF:06X}" }
                                        span { class: if idx % 2 == 0 { "text-emerald-400" } else { "text-cyan-400" },
                                            if idx % 2 == 0 { "ONLINE" } else { "SYNCED" }
                                        }
                                    }
                                }
                            },
                        }
                    }
                },
                code: code_snippet,
                props: props,
            }
        }
    }
}
