use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::components::{FeatureBanner, PropRow, ShowcaseTabs};

#[component]
pub fn CopyButtonPage() -> Element {
    let sample_code = "cargo add shadow-design".to_string();

    let props = vec![
        PropRow {
            name: "value",
            r#type: "String",
            default: "-",
            description: "Testo da copiare negli appunti di sistema",
        },
        PropRow {
            name: "timeout_ms",
            r#type: "u32",
            default: "2000",
            description: "Durata in millisecondi dello stato di conferma 'Copiato!'",
        },
        PropRow {
            name: "label",
            r#type: "Option<String>",
            default: "None",
            description: "Etichetta testuale opzionale affiancata all'icona di copia",
        },
        PropRow {
            name: "copied_label",
            r#type: "String",
            default: "\"Copiato!\"",
            description: "Etichetta di conferma visualizzata dopo la copia avvenuta",
        },
        PropRow {
            name: "icon_only",
            r#type: "bool",
            default: "false",
            description: "Mostra esclusivamente l'icona compatta senza etichetta",
        },
    ];

    let code_snippet = r#"CopyButton {
    value: "cargo add shadow-design".to_string(),
    label: Some("Copia comando".to_string()),
    timeout_ms: 2000,
}"#
    .to_string();

    rsx! {
        div { class: "max-w-4xl space-y-8",
            div {
                Heading { order: HeadingOrder::H2, "CopyButton" }
                Text { dimmed: true, size: TextSize::Sm, "Pulsante di interazione con la clipboard di sistema, con feedback visivo temporizzato 'Copiato!' e transizione fluida all'icona di spunta." }
            }

            FeatureBanner { feature_name: "copy_button" }

            ShowcaseTabs {
                preview: rsx! {
                    div { class: "p-12 py-16 rounded-xl bg-[#060608] border border-white/[0.08] flex flex-col items-center justify-center gap-8 shadow-inner min-h-[320px]",
                        div { class: "p-4 rounded-xl bg-white/[0.03] border border-white/[0.08] flex items-center justify-between gap-4 max-w-md w-full shadow-lg",
                            div { class: "flex items-center gap-2 text-xs font-mono text-zinc-300",
                                span { class: "text-purple-400 select-none", "$" }
                                span { "{sample_code}" }
                            }
                            CopyButton {
                                value: sample_code.clone(),
                                label: Some("Copia".to_string()),
                            }
                        }

                        div { class: "flex items-center gap-4 flex-wrap justify-center",
                            CopyButton {
                                value: "https://shadowfenix.net".to_string(),
                                label: Some("Copia Link".to_string()),
                            }
                            CopyButton {
                                value: "0x7F9B1E0DC42A".to_string(),
                                icon_only: true,
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
