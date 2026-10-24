use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::components::{FeatureBanner, PageHeader, PropRow, ShowcaseTabs};

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
        div { class: "sc-demo-cell",
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
        div { class: "sc-stack sc-stack--loose",
            PageHeader {
                title: "Carousel".to_string(),
                description: "Slider a carosello interattivo per schede informative con spaziatura protetta o gallerie immagini full-bleed edge-to-edge.".to_string(),
            }

            FeatureBanner { feature_name: "carousel" }

            ShowcaseTabs {
                preview: rsx! {
                    div { class: "sc-stack sc-stack--loose",
                        Card {
                            title: "1. Carosello Schede con Spaziatura Protetta (padded = true)".to_string(),
                            subtitle: "Spazio protetto per controlli e indicatori".to_string(),
                            Carousel {
                                slide_count: 3,
                                active_index: Some(active_slide),
                                padded: true,
                                r#loop: true,
                                with_controls: true,
                                with_indicators: true,

                                CarouselSlide {
                                    div { class: "sc-stack items-center text-center p-8",
                                        Badge { label: "Preset 1 / 3".to_string(), variant: BadgeVariant::Primary }
                                        h3 { class: "sc-section-title", "Estetica Pure SCSS Ultraviolet" }
                                        p { class: "sc-muted-text", "Nero profondo con accenti viola scuro, specular highlight e glow per un look d'impatto." }
                                    }
                                }
                                CarouselSlide {
                                    div { class: "sc-stack items-center text-center p-8",
                                        Badge { label: "Preset 2 / 3".to_string(), variant: BadgeVariant::Primary }
                                        h3 { class: "sc-section-title", "Modularità Cargo Zero-Bloat" }
                                        p { class: "sc-muted-text", "Ogni componente è racchiuso in una feature flag indipendente senza codice superfluo." }
                                    }
                                }
                                CarouselSlide {
                                    div { class: "sc-stack items-center text-center p-8",
                                        Badge { label: "Preset 3 / 3".to_string(), variant: BadgeVariant::Primary }
                                        h3 { class: "sc-section-title", "Dioxus 0.7 Signal Architecture" }
                                        p { class: "sc-muted-text", "Reattività istantanea a 60 FPS con transizioni cubic-bezier e zero overhead." }
                                    }
                                }
                            }
                        }

                        Card {
                            title: "2. Carosello Immagini Full-Bleed (padded = false, flush)".to_string(),
                            subtitle: "A tutto schermo / Edge-to-Edge".to_string(),
                            Carousel {
                                slide_count: 3,
                                active_index: Some(image_slide),
                                padded: false,
                                r#loop: true,
                                with_controls: true,
                                with_indicators: true,

                                CarouselSlide {
                                    div { class: "sc-stack items-center text-center p-8 h-64 justify-center",
                                        Badge { label: "Scena Ottica 01".to_string(), variant: BadgeVariant::Primary }
                                        h3 { class: "sc-section-title", "Nebulosa Ultraviolet & Dark Cyberpunk" }
                                        p { class: "sc-muted-text", "Rendering a tutto bordo senza margini per gallerie fotografiche e banner d'impatto." }
                                    }
                                }
                                CarouselSlide {
                                    div { class: "sc-stack items-center text-center p-8 h-64 justify-center",
                                        Badge { label: "Scena Ottica 02".to_string(), variant: BadgeVariant::Primary }
                                        h3 { class: "sc-section-title", "Matrice Spaziale Neon Cyan" }
                                        p { class: "sc-muted-text", "Controlli traslucidi con backdrop blur sovrapposti direttamente sull'immagine." }
                                    }
                                }
                                CarouselSlide {
                                    div { class: "sc-stack items-center text-center p-8 h-64 justify-center",
                                        Badge { label: "Scena Ottica 03".to_string(), variant: BadgeVariant::Primary }
                                        h3 { class: "sc-section-title", "Architettura Monolitica Emerald" }
                                        p { class: "sc-muted-text", "Transizione fluida dello slider a 60 FPS con indicatore di progresso espanso." }
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
