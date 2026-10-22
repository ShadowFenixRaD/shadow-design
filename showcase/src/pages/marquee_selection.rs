use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::components::{FeatureBanner, PropRow, ShowcaseTabs};

#[component]
pub fn MarqueeSelectionPage() -> Element {
    let mut selected_indices = use_signal(|| vec![0, 1]);
    let mut last_coords = use_signal(|| "In attesa di trascinamento".to_string());

    let props = vec![
        PropRow {
            name: "enabled",
            r#type: "bool",
            default: "true",
            description: "Se true, abilita il trascinamento del riquadro elastico di selezione",
        },
        PropRow {
            name: "on_change",
            r#type: "Option<EventHandler<MarqueeRect>>",
            default: "None",
            description: "Callback scatenato al termine del trascinamento con le coordinate x, y, width, height",
        },
        PropRow {
            name: "on_select_indices",
            r#type: "Option<EventHandler<Vec<usize>>>",
            default: "None",
            description: "Restituisce automaticamente gli indici dei figli con attributo data-marquee-item intersecati dal lasso",
        },
        PropRow {
            name: "children",
            r#type: "Element",
            default: "-",
            description: "Elementi e card contenute all'interno dell'area di selezione",
        },
    ];

    let items = vec![
        "mesh_router.rs",
        "peer_protocol.proto",
        "crypto_vault.bin",
        "shadow_config.json",
        "pipeline.yaml",
        "kernel_patch.diff",
    ];

    let code_snippet = r#"MarqueeSelection {
    enabled: true,
    on_change: move |rect: MarqueeRect| {
        println!("Rettangolo calcolato: {:?}", rect);
    },
    on_select_indices: move |indices: Vec<usize>| {
        selected_indices.set(indices);
    },
    div { class: "grid grid-cols-3 gap-3",
        for (i, item) in items.iter().enumerate() {
            div {
                "data-marquee-item": "{i}",
                // Stile selezionato / non selezionato...
            }
        }
    }
}"#
    .to_string();

    let sel_count = selected_indices().len();

    rsx! {
        div { class: "max-w-4xl space-y-8",
            div {
                Heading { order: HeadingOrder::H2, "MarqueeSelection" }
                Text { dimmed: true, size: TextSize::Sm, "Riquadro rettangolare a trascinamento elastico (Lasso / Rubber Band) con collision detection integrata per selezionare blocchi di file, card o nodi." }
            }

            FeatureBanner { feature_name: "marquee_selection" }

            ShowcaseTabs {
                preview: rsx! {
                    div { class: "p-8 rounded-xl bg-[#060608] border border-white/[0.08] flex flex-col gap-6 shadow-inner",
                        div { class: "flex items-center justify-between text-xs font-mono text-zinc-400 pb-2 border-b border-white/[0.06] flex-wrap gap-2",
                            div { class: "flex items-center gap-2",
                                span { class: "text-zinc-300 font-semibold", "{sel_count} elementi selezionati" }
                                if sel_count > 0 {
                                    button {
                                        r#type: "button",
                                        class: "text-[11px] text-purple-400 hover:text-purple-300 underline cursor-pointer",
                                        onclick: move |_| selected_indices.set(vec![]),
                                        "Deseleziona tutto"
                                    }
                                }
                            }
                            span { class: "text-purple-300", "{last_coords()}" }
                        }

                        div { class: "p-4 rounded-xl border border-dashed border-white/[0.12] bg-white/[0.01]",
                            MarqueeSelection {
                                enabled: true,
                                on_change: move |rect: MarqueeRect| {
                                    last_coords.set(format!("x: {:.0}, y: {:.0}, w: {:.0}, h: {:.0}", rect.x, rect.y, rect.width, rect.height));
                                },
                                on_select_indices: move |indices: Vec<usize>| {
                                    selected_indices.set(indices);
                                },
                                div { class: "grid grid-cols-2 sm:grid-cols-3 gap-3",
                                    for (i, item) in items.iter().enumerate() {
                                        {
                                            let is_sel = selected_indices().contains(&i);
                                            rsx! {
                                                div {
                                                    key: "{i}",
                                                    "data-marquee-item": "{i}",
                                                    onclick: move |_| {
                                                        let mut current = selected_indices();
                                                        if current.contains(&i) {
                                                            current.retain(|&x| x != i);
                                                        } else {
                                                            current.push(i);
                                                        }
                                                        selected_indices.set(current);
                                                    },
                                                    class: if is_sel {
                                                        "p-4 rounded-lg border border-purple-500/80 bg-purple-950/60 cursor-pointer shadow-[0_0_16px_rgba(168,85,247,0.3)] transition-all select-none scale-[1.01]"
                                                    } else {
                                                        "p-4 rounded-lg border border-white/[0.06] bg-zinc-950/40 hover:border-white/[0.12] cursor-pointer transition-all select-none"
                                                    },
                                                    div { class: "flex items-center gap-2.5 pointer-events-none",
                                                        div { class: if is_sel { "w-2.5 h-2.5 rounded-full bg-purple-400 shadow-[0_0_8px_rgba(168,85,247,0.8)]" } else { "w-2.5 h-2.5 rounded-full bg-zinc-600" } }
                                                        span { class: if is_sel { "text-xs font-mono text-purple-200 font-semibold truncate" } else { "text-xs font-mono text-zinc-300 truncate" }, "{item}" }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
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
