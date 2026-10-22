use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::components::{FeatureBanner, PropRow, ShowcaseTabs};

#[component]
pub fn LoadingOverlayPage() -> Element {
    let mut visible = use_signal(|| true);
    let mut blur_px = use_signal(|| 3u32);

    let props = vec![
        PropRow {
            name: "visible",
            r#type: "bool",
            default: "-",
            description: "Se true, attiva l'overlay con spinner e blur bloccando l'interazione sul contenitore",
        },
        PropRow {
            name: "blur_px",
            r#type: "u32",
            default: "3",
            description: "Raggio di sfocatura in pixel per l'effetto backdrop-filter",
        },
        PropRow {
            name: "loader_size",
            r#type: "u32",
            default: "32",
            description: "Dimensione dell'icona animata dello spinner in pixel",
        },
        PropRow {
            name: "description",
            r#type: "Option<String>",
            default: "None",
            description: "Testo opzionale visualizzato al di sotto dello spinner di caricamento",
        },
        PropRow {
            name: "children",
            r#type: "Element",
            default: "-",
            description: "Contenuto isolato o form da proteggere durante il caricamento",
        },
    ];

    let code_snippet = r#"LoadingOverlay {
    visible: is_loading(),
    blur_px: 3,
    description: Some("Elaborazione dati...".to_string()),
    div { class: "p-6",
        p { "Contenuto protetto" }
    }
}"#
    .to_string();

    rsx! {
        div { class: "max-w-4xl space-y-8",
            div {
                Heading { order: HeadingOrder::H2, "LoadingOverlay" }
                Text { dimmed: true, size: TextSize::Sm, "Strato semitrasparente in overlay con spinner circolare animato e sfocatura ottica per isolare e disabilitare sezioni durante operazioni asincrone." }
            }

            FeatureBanner { feature_name: "loading_overlay" }

            ShowcaseTabs {
                preview: rsx! {
                    div { class: "p-8 rounded-xl bg-[#060608] border border-white/[0.08] flex flex-col gap-6 shadow-inner",
                        div { class: "flex items-center gap-3 flex-wrap",
                            button {
                                r#type: "button",
                                class: "px-3 py-1.5 text-xs font-mono rounded-lg border border-purple-500/50 bg-purple-950/80 text-purple-200 transition-all cursor-pointer hover:bg-purple-900",
                                onclick: move |_| visible.toggle(),
                                if visible() { "Disattiva Overlay" } else { "Attiva Overlay" }
                            }
                            button {
                                r#type: "button",
                                class: "px-3 py-1.5 text-xs font-mono rounded-lg border border-white/[0.06] bg-white/[0.04] text-zinc-300 transition-all cursor-pointer",
                                onclick: move |_| {
                                    let next = if blur_px() >= 6 { 0 } else { blur_px() + 2 };
                                    blur_px.set(next);
                                },
                                "Blur: {blur_px()}px"
                            }
                        }

                        div { class: "max-w-md mx-auto w-full",
                            LoadingOverlay {
                                visible: visible(),
                                blur_px: blur_px(),
                                description: Some("Sincronizzazione archivio...".to_string()),
                                div { class: "p-6 rounded-xl border border-white/[0.06] bg-zinc-950/60 space-y-4",
                                    h4 { class: "text-sm font-semibold text-zinc-200", "Pannello Operativo" }
                                    p { class: "text-xs text-zinc-400 leading-relaxed", "Questo modulo include parametri e pulsanti la cui interazione viene inibita mentre l'overlay è visibile." }
                                    div { class: "flex gap-2 pt-2",
                                        Button {
                                            variant: ButtonVariant::Primary,
                                            size: ButtonSize::Sm,
                                            "Invia Task"
                                        }
                                        Button {
                                            variant: ButtonVariant::Outline,
                                            size: ButtonSize::Sm,
                                            "Annulla"
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
