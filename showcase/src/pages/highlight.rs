use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::components::{FeatureBanner, PropRow, ShowcaseTabs};

#[component]
pub fn HighlightPage() -> Element {
    let query = use_signal(|| "shadow".to_string());
    let mut case_sensitive = use_signal(|| false);
    let sample_text = "Shadow Design è una suite di componenti modulari per Dioxus 0.7. L'architettura Shadow garantisce micro-interazioni tattili, zero-bloat e stile cyberpunk.";

    let props = vec![
        PropRow {
            name: "text",
            r#type: "String",
            default: "-",
            description: "Testo di input sorgente in cui individuare i frammenti",
        },
        PropRow {
            name: "query",
            r#type: "String",
            default: "-",
            description: "Termine o token da evidenziare all'interno della stringa",
        },
        PropRow {
            name: "case_sensitive",
            r#type: "bool",
            default: "false",
            description: "Se true, distingue tra caratteri maiuscoli e minuscoli nel matching",
        },
        PropRow {
            name: "highlight_color",
            r#type: "Option<String>",
            default: "None",
            description: "Colore personalizzato per lo sfondo del marker <mark>",
        },
        PropRow {
            name: "highlight_class",
            r#type: "String",
            default: "\"\"",
            description: "Classi CSS applicate all'elemento <mark>",
        },
    ];

    let code_snippet = r#"Highlight {
    text: "Shadow Design per Dioxus 0.7".to_string(),
    query: "Shadow".to_string(),
    case_sensitive: false,
}"#
    .to_string();

    rsx! {
        div { class: "max-w-4xl space-y-8",
            div {
                Heading { order: HeadingOrder::H2, "Highlight" }
                Text { dimmed: true, size: TextSize::Sm, "Evidenzia visivamente occorrenze di testo o query di ricerca tramite tag <mark> integrati con supporto case-insensitive e stili ad alto contrasto." }
            }

            FeatureBanner { feature_name: "highlight" }

            ShowcaseTabs {
                preview: rsx! {
                    div { class: "p-8 rounded-xl bg-[#060608] border border-white/[0.08] flex flex-col gap-6 shadow-inner",
                        div { class: "flex items-center gap-4 flex-wrap",
                            div { class: "flex-1 min-w-[200px]",
                                TextInput {
                                    value: query,
                                    placeholder: "Cerca nel testo...",
                                }
                            }
                            button {
                                r#type: "button",
                                class: format!("px-3 py-1.5 text-xs font-mono rounded-lg border transition-all cursor-pointer {}", if case_sensitive() { "bg-purple-950/80 text-purple-200 border-purple-500/50" } else { "bg-white/[0.04] text-zinc-400 border-white/[0.06]" }),
                                onclick: move |_| case_sensitive.toggle(),
                                "Case Sensitive"
                            }
                        }

                        div { class: "p-5 rounded-lg bg-white/[0.02] border border-white/[0.06]",
                            p { class: "text-sm text-zinc-300 leading-relaxed font-sans",
                                Highlight {
                                    text: sample_text.to_string(),
                                    query: query(),
                                    case_sensitive: case_sensitive(),
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
