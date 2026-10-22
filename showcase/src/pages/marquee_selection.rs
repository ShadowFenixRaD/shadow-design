use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::components::{FeatureBanner, PropRow, ShowcaseTabs};

#[component]
pub fn MarqueeSelectionPage() -> Element {
    let mut selected_indices = use_signal(|| vec![0]);
    let mut last_coords = use_signal(|| "Nessuna selezione".to_string());

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
            description: "Callback scatenato all'aggiornamento o termine del rettangolo di selezione",
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
        println!("Selezione rettangolare: {:?}", rect);
    },
    div { class: "grid grid-cols-3 gap-4",
        // Elementi selezionabili...
    }
}"#
    .to_string();

    rsx! {
        div { class: "max-w-4xl space-y-8",
            div {
                Heading { order: HeadingOrder::H2, "MarqueeSelection" }
                Text { dimmed: true, size: TextSize::Sm, "Riquadro rettangolare a trascinamento elastico (Lasso / Rubber Band) per la selezione contestuale multipla di elementi o griglie." }
            }

            FeatureBanner { feature_name: "marquee_selection" }

            ShowcaseTabs {
                preview: rsx! {
                    div { class: "p-8 rounded-xl bg-[#060608] border border-white/[0.08] flex flex-col gap-6 shadow-inner",
                        div { class: "flex items-center justify-between text-xs font-mono text-zinc-400 pb-2 border-b border-white/[0.06]",
                            span { "Trascina con il mouse nell'area tratteggiata per creare il riquadro di selezione." }
                            span { class: "text-purple-300", "{last_coords()}" }
                        }

                        div { class: "p-4 rounded-xl border border-dashed border-white/[0.12] bg-white/[0.01]",
                            MarqueeSelection {
                                enabled: true,
                                on_change: move |rect: MarqueeRect| {
                                    last_coords.set(format!("x: {:.0}, y: {:.0}, w: {:.0}, h: {:.0}", rect.x, rect.y, rect.width, rect.height));
                                },
                                div { class: "grid grid-cols-2 sm:grid-cols-3 gap-3",
                                    for (i, item) in items.iter().enumerate() {
                                        {
                                            let is_sel = selected_indices().contains(&i);
                                            rsx! {
                                                div {
                                                    key: "{i}",
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
                                                        "p-4 rounded-lg border border-purple-500/60 bg-purple-950/40 cursor-pointer shadow-[0_0_12px_rgba(168,85,247,0.2)] transition-all select-none"
                                                    } else {
                                                        "p-4 rounded-lg border border-white/[0.06] bg-zinc-950/40 hover:border-white/[0.12] cursor-pointer transition-all select-none"
                                                    },
                                                    div { class: "flex items-center gap-2.5",
                                                        div { class: if is_sel { "w-2 h-2 rounded-full bg-purple-400" } else { "w-2 h-2 rounded-full bg-zinc-600" } }
                                                        span { class: "text-xs font-mono text-zinc-300 truncate", "{item}" }
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
