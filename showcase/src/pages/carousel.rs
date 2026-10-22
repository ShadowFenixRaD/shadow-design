use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::components::{FeatureBanner, PropRow, ShowcaseTabs};

#[component]
pub fn CarouselPage() -> Element {
    let active_slide = use_signal(|| 0usize);

    let props = vec![
        PropRow {
            name: "slide_count",
            r#type: "usize",
            default: "-",
            description: "Numero complessivo di diapositive nel carosello",
        },
        PropRow {
            name: "loop",
            r#type: "bool",
            default: "true",
            description: "Ciclo infinito automatico tra ultima e prima slide",
        },
        PropRow {
            name: "with_controls",
            r#type: "bool",
            default: "true",
            description: "Mostra pulsanti freccia avanti/indietro fluttuanti",
        },
        PropRow {
            name: "with_indicators",
            r#type: "bool",
            default: "true",
            description: "Mostra gli indicatori di posizione a punto (dots) in basso",
        },
        PropRow {
            name: "CarouselSlide",
            r#type: "Component",
            default: "-",
            description: "Contenitore della singola diapositiva",
        },
    ];

    let code_snippet = r#"Carousel {
    slide_count: 3,
    r#loop: true,
    with_controls: true,
    with_indicators: true,

    CarouselSlide {
        div { class: "p-12 text-center", "Slide 1: Benvenuto" }
    }
    CarouselSlide {
        div { class: "p-12 text-center", "Slide 2: Architettura" }
    }
    CarouselSlide {
        div { class: "p-12 text-center", "Slide 3: Prestazioni" }
    }
}"#
    .to_string();

    rsx! {
        div { class: "max-w-4xl space-y-8",
            div {
                Heading { order: HeadingOrder::H2, "Carousel" }
                Text { dimmed: true, size: TextSize::Sm, "Slider a carosello interattivo con transizioni fluide, controlli laterali e indicatori di avanzamento a punto." }
            }

            FeatureBanner { feature_name: "carousel" }

            ShowcaseTabs {
                preview: rsx! {
                    div { class: "p-8 rounded-xl bg-[#060608] border border-white/[0.08] flex flex-col gap-6 shadow-inner",
                        Carousel {
                            slide_count: 3,
                            active_index: Some(active_slide),
                            r#loop: true,
                            with_controls: true,
                            with_indicators: true,

                            CarouselSlide {
                                div { class: "p-14 rounded-xl bg-gradient-to-br from-purple-950/40 via-zinc-950 to-black border border-purple-500/20 flex flex-col items-center justify-center text-center space-y-3",
                                    span { class: "px-2.5 py-1 rounded-full bg-purple-500/10 border border-purple-500/30 text-[10px] font-mono text-purple-300", "Diapositiva 1 di 3" }
                                    h3 { class: "text-lg font-bold text-white", "Estetica Pure SCSS Ultraviolet" }
                                    p { class: "text-xs text-zinc-400 max-w-md", "Nero assoluto con accenti viola e specular highlights per un'interfaccia ultra-moderna." }
                                }
                            }
                            CarouselSlide {
                                div { class: "p-14 rounded-xl bg-gradient-to-br from-cyan-950/40 via-zinc-950 to-black border border-cyan-500/20 flex flex-col items-center justify-center text-center space-y-3",
                                    span { class: "px-2.5 py-1 rounded-full bg-cyan-500/10 border border-cyan-500/30 text-[10px] font-mono text-cyan-300", "Diapositiva 2 di 3" }
                                    h3 { class: "text-lg font-bold text-white", "Modularità Cargo Zero-Bloat" }
                                    p { class: "text-xs text-zinc-400 max-w-md", "Ogni componente è racchiuso in una feature flag indipendente senza dipendenze pesanti." }
                                }
                            }
                            CarouselSlide {
                                div { class: "p-14 rounded-xl bg-gradient-to-br from-emerald-950/40 via-zinc-950 to-black border border-emerald-500/20 flex flex-col items-center justify-center text-center space-y-3",
                                    span { class: "px-2.5 py-1 rounded-full bg-emerald-500/10 border border-emerald-500/30 text-[10px] font-mono text-emerald-300", "Diapositiva 3 di 3" }
                                    h3 { class: "text-lg font-bold text-white", "Dioxus 0.7 Signal Architecture" }
                                    p { class: "text-xs text-zinc-400 max-w-md", "Reattività istantanea a 60 FPS senza overhead o memory leaks per Web e Desktop." }
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
