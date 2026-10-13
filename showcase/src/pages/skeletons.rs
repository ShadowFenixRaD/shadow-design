use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::components::{FeatureBanner, PropRow, ShowcaseTabs};

#[component]
pub fn SkeletonsPage() -> Element {
    let mut selected_shape = use_signal(|| SkeletonShape::Text);
    let mut text_count = use_signal(|| 3usize);
    let mut is_animated = use_signal(|| true);

    let props = vec![
        PropRow {
            name: "shape",
            r#type: "SkeletonShape",
            default: "SkeletonShape::Text",
            description: "Forma geometrica del placeholder: Circle, Rectangle, Text",
        },
        PropRow {
            name: "count",
            r#type: "usize",
            default: "1",
            description: "Numero di righe di testo da simulare (l'ultima riga è resa leggermente più corta)",
        },
        PropRow {
            name: "animate",
            r#type: "bool",
            default: "true",
            description: "Attiva l'animazione di shimmer wave continua",
        },
        PropRow {
            name: "width",
            r#type: "Option<String>",
            default: "None",
            description: "Larghezza CSS esplicita (es. '120px', '80%')",
        },
        PropRow {
            name: "height",
            r#type: "Option<String>",
            default: "None",
            description: "Altezza CSS esplicita (es. '48px')",
        },
    ];

    let code_snippet = r#"// Placeholder Card profilo durante caricamento:
div { class: "flex items-center gap-4",
    Skeleton { shape: SkeletonShape::Circle }
    div { class: "space-y-2 flex-1",
        Skeleton { shape: SkeletonShape::Text, count: 2 }
    }
}"#
    .to_string();

    rsx! {
        div { class: "max-w-4xl space-y-8",
            div {
                Heading { order: HeadingOrder::H2, "Skeleton & Placeholder" }
                Text { dimmed: true, size: TextSize::Sm, "Placeholder di caricamento asincrono con animazione shimmer wave per profili, messaggi e schede prima della ricezione dei dati." }
            }

            FeatureBanner { feature_name: "skeleton" }

            ShowcaseTabs {
                preview: rsx! {
                    div { class: "grid grid-cols-1 lg:grid-cols-2 gap-8 items-center",
                        // Live Preview
                        div { class: "p-8 rounded-xl bg-[#060608] border border-white/[0.08] flex flex-col justify-center min-h-[180px] shadow-inner space-y-6",
                            // Controllo interattivo
                            div { class: "w-full",
                                Skeleton {
                                    shape: selected_shape(),
                                    count: text_count(),
                                    animate: is_animated(),
                                }
                            }

                            // Mock Card composita
                            div { class: "pt-4 border-t border-white/[0.06] space-y-2",
                                span { class: "text-[10px] text-zinc-500 font-mono uppercase block", "Esempio Card Caricamento:" }
                                div { class: "flex items-center gap-3 p-3 rounded-lg bg-white/[0.02] border border-white/[0.04]",
                                    Skeleton { shape: SkeletonShape::Circle, animate: is_animated() }
                                    div { class: "space-y-2 flex-1",
                                        Skeleton { shape: SkeletonShape::Text, count: 2, animate: is_animated() }
                                    }
                                }
                            }
                        }

                        // Configurator
                        div { class: "space-y-4 text-xs font-mono",
                            div {
                                span { class: "text-zinc-400 block mb-1.5 uppercase tracking-wider text-[10px]", "Forma (Shape):" }
                                div { class: "flex gap-2",
                                    for s in [SkeletonShape::Text, SkeletonShape::Circle, SkeletonShape::Rectangle] {
                                        button {
                                            r#type: "button",
                                            class: if selected_shape() == s {
                                                "px-2.5 py-1 rounded bg-purple-950/80 text-purple-200 border border-purple-500/40 font-bold cursor-pointer"
                                            } else {
                                                "px-2.5 py-1 rounded bg-white/[0.03] text-zinc-400 border border-white/[0.06] cursor-pointer"
                                            },
                                            onclick: move |_| selected_shape.set(s),
                                            match s {
                                                SkeletonShape::Text => "Text (Righe)",
                                                SkeletonShape::Circle => "Circle (Cerchio)",
                                                SkeletonShape::Rectangle => "Rectangle (Card)",
                                            }
                                        }
                                    }
                                }
                            }

                            if selected_shape() == SkeletonShape::Text {
                                div {
                                    span { class: "text-zinc-400 block mb-1.5 uppercase tracking-wider text-[10px]", "Numero Righe:" }
                                    div { class: "flex gap-2",
                                        for c in [1, 2, 3, 4, 5] {
                                            button {
                                                r#type: "button",
                                                class: if text_count() == c {
                                                    "px-2.5 py-1 rounded bg-purple-950/80 text-purple-200 border border-purple-500/40 font-bold cursor-pointer"
                                                } else {
                                                    "px-2.5 py-1 rounded bg-white/[0.03] text-zinc-400 border border-white/[0.06] cursor-pointer"
                                                },
                                                onclick: move |_| text_count.set(c),
                                                "{c}"
                                            }
                                        }
                                    }
                                }
                            }

                            div { class: "pt-2 border-t border-white/[0.06]",
                                label { class: "flex items-center gap-2 cursor-pointer text-zinc-300",
                                    input {
                                        r#type: "checkbox",
                                        checked: "{is_animated}",
                                        onchange: move |e| is_animated.set(e.checked()),
                                    }
                                    "Attiva Shimmer Wave Animato"
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
