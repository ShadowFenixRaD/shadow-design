use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::components::{FeatureBanner, PropRow, ShowcaseTabs};

#[component]
pub fn AffixPage() -> Element {
    let mut selected_pos = use_signal(|| AffixPosition::BottomRight);

    let props = vec![
        PropRow {
            name: "position",
            r#type: "AffixPosition",
            default: "AffixPosition::BottomRight",
            description: "Angolo di ancoraggio fisso nella viewport (BottomRight, BottomLeft, TopRight, TopLeft)",
        },
        PropRow {
            name: "scroll_threshold",
            r#type: "Option<u32>",
            default: "None",
            description: "Soglia minima di scroll della pagina in pixel per attivare la visibilità dell'elemento",
        },
        PropRow {
            name: "z_index",
            r#type: "u32",
            default: "60",
            description: "Livello di elevazione z-index per sovrapporsi ai contenuti di pagina",
        },
        PropRow {
            name: "BackToTop",
            r#type: "Component",
            default: "-",
            description: "Pulsante autonomo con scroll fluido a inizio pagina (window.scrollTo)",
        },
    ];

    let code_snippet = r#"// Affix generico con soglia di scroll
Affix {
    position: AffixPosition::BottomRight,
    scroll_threshold: Some(250),
    Button {
        variant: ButtonVariant::Primary,
        left_icon: rsx! { Icon { kind: IconKind::Terminal, size: 14 } },
        "Terminale Rapido"
    }
}

// Oppure il componente dedicato BackToTop
BackToTop {
    scroll_threshold: 200,
    label: Some("Torna su".to_string()),
}"#
    .to_string();

    rsx! {
        div { class: "max-w-4xl space-y-8",
            div {
                Heading { order: HeadingOrder::H2, "Affix & BackToTop" }
                Text { dimmed: true, size: TextSize::Sm, "Contenitore fisso in overlay a coordinate assolute dello schermo con attivazione condizionale allo scroll e componente rapido BackToTop." }
            }

            FeatureBanner { feature_name: "affix" }

            ShowcaseTabs {
                preview: rsx! {
                    div { class: "p-12 rounded-xl bg-[#060608] border border-white/[0.08] flex flex-col items-center justify-center gap-6 text-center shadow-inner",
                        p { class: "text-xs text-zinc-400 max-w-md",
                            "Il componente Affix fissa qualsiasi elemento a una posizione della viewport. Il pulsante BackToTop rileva lo scroll verticale della pagina ed effettua uno scroll fluido verso l'alto."
                        }

                        div { class: "flex items-center gap-2 flex-wrap justify-center",
                            span { class: "text-xs text-zinc-400 mr-2 font-mono", "Angolo:" }
                            button {
                                r#type: "button",
                                class: format!("px-2.5 py-1 text-xs font-mono rounded-lg border transition-all cursor-pointer {}", if *selected_pos.read() == AffixPosition::BottomRight { "bg-purple-950/80 text-purple-200 border-purple-500/50" } else { "bg-white/[0.04] text-zinc-400 border-white/[0.06]" }),
                                onclick: move |_| selected_pos.set(AffixPosition::BottomRight),
                                "Bottom-Right"
                            }
                            button {
                                r#type: "button",
                                class: format!("px-2.5 py-1 text-xs font-mono rounded-lg border transition-all cursor-pointer {}", if *selected_pos.read() == AffixPosition::BottomLeft { "bg-purple-950/80 text-purple-200 border-purple-500/50" } else { "bg-white/[0.04] text-zinc-400 border-white/[0.06]" }),
                                onclick: move |_| selected_pos.set(AffixPosition::BottomLeft),
                                "Bottom-Left"
                            }
                        }

                        div { class: "p-6 rounded-xl bg-white/[0.02] border border-white/[0.06] flex items-center justify-center gap-4",
                            button {
                                r#type: "button",
                                class: "sd-back-to-top",
                                onclick: move |_| {
                                    let eval = document::eval("window.scrollTo({ top: 0, behavior: 'smooth' });");
                                    spawn(async move { let _ = eval.await; });
                                },
                                Icon { kind: IconKind::ChevronUp, size: 16 }
                                span { "Anteprima BackToTop" }
                            }
                        }
                    }
                },
                code: code_snippet,
                props: props,
            }

            // Componente BackToTop attivo sullo scroll reale della pagina
            BackToTop {
                scroll_threshold: 150,
                label: Some("Inizio pagina".to_string()),
            }
        }
    }
}
