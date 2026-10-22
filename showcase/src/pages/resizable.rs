use dioxus::prelude::*;
use shadow_design::prelude::*;
use shadow_design::resizable::ResizableDirection;

use crate::components::{FeatureBanner, PropRow, ShowcaseTabs};

#[component]
pub fn ResizablePage() -> Element {
    let mut current_direction = use_signal(|| ResizableDirection::Horizontal);

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
    ResizablePanel { default_size: 30.0, min_size: 15.0,
        div { "Sidebar di navigazione" }
    }
    ResizableHandle { with_handle: true }
    ResizablePanel { default_size: 70.0,
        div { "Area di lavoro principale" }
    }
}"#
    .to_string();

    let is_horiz = *current_direction.read() == ResizableDirection::Horizontal;

    rsx! {
        div { class: "max-w-4xl space-y-8",
            div {
                Heading { order: HeadingOrder::H2, "Resizable (SplitPane)" }
                Text { dimmed: true, size: TextSize::Sm, "Pannelli divisori ridimensionabili con maniglia di presa tattile per layout split orizzontali o verticali." }
            }

            FeatureBanner { feature_name: "resizable" }

            ShowcaseTabs {
                preview: rsx! {
                    div { class: "p-8 rounded-xl bg-[#060608] border border-white/[0.08] flex flex-col gap-6 shadow-inner",
                        div { class: "flex items-center gap-3",
                            span { class: "text-xs font-mono text-zinc-400 font-semibold", "Direzione Split:" }
                            button {
                                r#type: "button",
                                class: format!("px-3 py-1.5 text-xs font-mono rounded-lg border transition-all cursor-pointer font-semibold {}", if is_horiz { "bg-purple-950/80 text-purple-200 border-purple-500/50 shadow-[0_0_10px_rgba(168,85,247,0.3)]" } else { "bg-white/[0.04] text-zinc-400 border-white/[0.08]" }),
                                onclick: move |_| current_direction.set(ResizableDirection::Horizontal),
                                "Orizzontale"
                            }
                            button {
                                r#type: "button",
                                class: format!("px-3 py-1.5 text-xs font-mono rounded-lg border transition-all cursor-pointer font-semibold {}", if !is_horiz { "bg-purple-950/80 text-purple-200 border-purple-500/50 shadow-[0_0_10px_rgba(168,85,247,0.3)]" } else { "bg-white/[0.04] text-zinc-400 border-white/[0.08]" }),
                                onclick: move |_| current_direction.set(ResizableDirection::Vertical),
                                "Verticale"
                            }
                        }

                        div { class: "h-[320px] rounded-xl border border-white/[0.1] bg-black/60 overflow-hidden",
                            ResizablePanelGroup {
                                direction: *current_direction.read(),
                                ResizablePanel {
                                    default_size: 35.0,
                                    min_size: 20.0,
                                    max_size: 70.0,
                                    div { class: "h-full p-4 flex flex-col justify-between bg-zinc-950/50 border-r border-white/[0.04]",
                                        div { class: "space-y-2",
                                            span { class: "text-xs font-mono text-purple-300 font-bold", "Pannello 1 (Explorer)" }
                                            p { class: "text-[11px] text-zinc-400", "Albero file, moduli o parametri di configurazione." }
                                        }
                                        div { class: "text-[10px] font-mono text-zinc-600", "Size: 35% (Min 20% / Max 70%)" }
                                    }
                                }
                                ResizableHandle { with_handle: true }
                                ResizablePanel {
                                    default_size: 65.0,
                                    min_size: 30.0,
                                    div { class: "h-full p-4 flex flex-col justify-between bg-black",
                                        div { class: "space-y-2",
                                            span { class: "text-xs font-mono text-cyan-300 font-bold", "Pannello 2 (Editor Principale)" }
                                            p { class: "text-[11px] text-zinc-400", "Visualizzazione contenuto, editor sorgente o canvas interattivo." }
                                        }
                                        div { class: "text-[10px] font-mono text-zinc-600", "Size: 65%" }
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
