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
            name: "padded",
            r#type: "bool",
            default: "false",
            description: "Applica una spaziatura interna protetta tra il contenuto e i bordi/controlli del carosello",
        },
        PropRow {
            name: "CarouselSlide",
            r#type: "Component",
            default: "-",
            description: "Contenitore della singola diapositiva",
        },
    ];

    let code_snippet = r#"// 1. Carosello con spaziatura protetta per schede e testo
Carousel {
    slide_count: 3,
    padded: true,
    with_controls: true,
    with_indicators: true,

    CarouselSlide {
        div { class: "p-8 rounded-xl bg-zinc-950 border border-purple-500/20 text-center",
            "Scheda con padding dai bordi"
        }
    }
}

// 2. Carosello Immagini Full-Bleed (senza bordi)
Carousel {
    slide_count: 3,
    padded: false,
    with_controls: true,
    with_indicators: true,

    CarouselSlide {
        img { src: "/assets/banner.jpg", class: "w-full h-64 object-cover" }
    }
}"#
    .to_string();

    let image_slide = use_signal(|| 0usize);

    rsx! {
        div { class: "max-w-4xl space-y-8",
            div {
                Heading { order: HeadingOrder::H2, "Carousel" }
                Text { dimmed: true, size: TextSize::Sm, "Slider a carosello interattivo per schede informative con spaziatura protetta o gallerie immagini full-bleed edge-to-edge." }
            }

            FeatureBanner { feature_name: "carousel" }

            ShowcaseTabs {
                preview: rsx! {
                    div { class: "p-8 rounded-xl bg-[#060608] border border-white/[0.08] flex flex-col gap-10 shadow-inner",
                        // 1. Carosello Contenuto & Schede con Padding Protetto
                        div { class: "space-y-3",
                            div { class: "flex items-center justify-between",
                                span { class: "text-xs font-mono text-zinc-300 font-semibold uppercase tracking-wider", "1. Carosello Schede con Spaziatura Protetta (padded = true)" }
                                span { class: "text-[11px] font-mono text-purple-400", "Spazio protetto per controlli e indicatori" }
                            }
                            Carousel {
                                slide_count: 3,
                                active_index: Some(active_slide),
                                padded: true,
                                r#loop: true,
                                with_controls: true,
                                with_indicators: true,
                                class: "border border-white/[0.06] bg-zinc-950/40",

                                CarouselSlide {
                                    div { class: "p-8 rounded-xl bg-gradient-to-br from-purple-950/50 via-zinc-950 to-black border border-purple-500/20 flex flex-col items-center justify-center text-center space-y-3",
                                        span { class: "px-2.5 py-1 rounded-full bg-purple-500/10 border border-purple-500/30 text-[10px] font-mono text-purple-300 font-bold", "Preset 1 / 3" }
                                        h3 { class: "text-base font-bold text-white", "Estetica Pure SCSS Ultraviolet" }
                                        p { class: "text-xs text-zinc-400 max-w-md", "Nero profondo con accenti viola scuro, specular highlight e glow per un look d'impatto." }
                                    }
                                }
                                CarouselSlide {
                                    div { class: "p-8 rounded-xl bg-gradient-to-br from-cyan-950/50 via-zinc-950 to-black border border-cyan-500/20 flex flex-col items-center justify-center text-center space-y-3",
                                        span { class: "px-2.5 py-1 rounded-full bg-cyan-500/10 border border-cyan-500/30 text-[10px] font-mono text-cyan-300 font-bold", "Preset 2 / 3" }
                                        h3 { class: "text-base font-bold text-white", "Modularità Cargo Zero-Bloat" }
                                        p { class: "text-xs text-zinc-400 max-w-md", "Ogni componente è racchiuso in una feature flag indipendente senza codice superfluo." }
                                    }
                                }
                                CarouselSlide {
                                    div { class: "p-8 rounded-xl bg-gradient-to-br from-emerald-950/50 via-zinc-950 to-black border border-emerald-500/20 flex flex-col items-center justify-center text-center space-y-3",
                                        span { class: "px-2.5 py-1 rounded-full bg-emerald-500/10 border border-emerald-500/30 text-[10px] font-mono text-emerald-300 font-bold", "Preset 3 / 3" }
                                        h3 { class: "text-base font-bold text-white", "Dioxus 0.7 Signal Architecture" }
                                        p { class: "text-xs text-zinc-400 max-w-md", "Reattività istantanea a 60 FPS con transizioni cubic-bezier e zero overhead." }
                                    }
                                }
                            }
                        }

                        // 2. Carosello Immagini Full-Bleed (Senza Spazio con i Bordi)
                        div { class: "space-y-3",
                            div { class: "flex items-center justify-between",
                                span { class: "text-xs font-mono text-zinc-300 font-semibold uppercase tracking-wider", "2. Carosello Immagini Full-Bleed (padded = false, flush)" }
                                span { class: "text-[11px] font-mono text-cyan-400", "A tutto schermo / Edge-to-Edge" }
                            }
                            Carousel {
                                slide_count: 3,
                                active_index: Some(image_slide),
                                padded: false,
                                r#loop: true,
                                with_controls: true,
                                with_indicators: true,
                                class: "border border-white/[0.1] rounded-xl overflow-hidden shadow-2xl",

                                CarouselSlide {
                                    div { class: "relative h-64 w-full bg-gradient-to-r from-purple-900 via-indigo-950 to-black flex items-end p-6",
                                        div { class: "absolute inset-0 bg-gradient-to-t from-black/90 via-black/40 to-transparent" }
                                        div { class: "relative z-10 space-y-1",
                                            span { class: "text-[10px] font-mono text-purple-400 font-bold uppercase tracking-wider", "Scena Ottica 01" }
                                            h4 { class: "text-sm font-bold text-white", "Nebulosa Ultraviolet & Dark Cyberpunk" }
                                            p { class: "text-xs text-zinc-300", "Rendering a tutto bordo senza margini per gallerie fotografiche e banner d'impatto." }
                                        }
                                    }
                                }
                                CarouselSlide {
                                    div { class: "relative h-64 w-full bg-gradient-to-r from-cyan-900 via-blue-950 to-black flex items-end p-6",
                                        div { class: "absolute inset-0 bg-gradient-to-t from-black/90 via-black/40 to-transparent" }
                                        div { class: "relative z-10 space-y-1",
                                            span { class: "text-[10px] font-mono text-cyan-400 font-bold uppercase tracking-wider", "Scena Ottica 02" }
                                            h4 { class: "text-sm font-bold text-white", "Matrice Spaziale Neon Cyan" }
                                            p { class: "text-xs text-zinc-300", "Controlli traslucidi con backdrop blur sovrapposti direttamente sull'immagine." }
                                        }
                                    }
                                }
                                CarouselSlide {
                                    div { class: "relative h-64 w-full bg-gradient-to-r from-emerald-900 via-teal-950 to-black flex items-end p-6",
                                        div { class: "absolute inset-0 bg-gradient-to-t from-black/90 via-black/40 to-transparent" }
                                        div { class: "relative z-10 space-y-1",
                                            span { class: "text-[10px] font-mono text-emerald-400 font-bold uppercase tracking-wider", "Scena Ottica 03" }
                                            h4 { class: "text-sm font-bold text-white", "Architettura Monolitica Emerald" }
                                            p { class: "text-xs text-zinc-300", "Transizione fluida dello slider a 60 FPS con indicatore di progresso espanso." }
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
