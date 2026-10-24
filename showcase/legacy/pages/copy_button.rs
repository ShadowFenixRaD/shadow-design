use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::components::{FeatureBanner, PageHeader, PropRow, ShowcaseTabs, Stage};

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
        div { class: "sc-stack sc-stack--loose",
            PageHeader {
                title: "CopyButton".to_string(),
                description: "Pulsante di interazione con la clipboard di sistema, con feedback visivo temporizzato 'Copiato!' e transizione fluida all'icona di spunta.".to_string(),
            }

            FeatureBanner { feature_name: "copy_button" }

            ShowcaseTabs {
                preview: rsx! {
                    Stage {
                        Card { class: "w-full max-w-md",
                            div { class: "sc-title-row",
                                span { class: "sc-mono",
                                    span { class: "sc-accent-text", "$ " }
                                    "{sample_code}"
                                }
                                CopyButton {
                                    value: sample_code.clone(),
                                    label: Some("Copia".to_string()),
                                }
                            }
                        }

                        div { class: "sc-actions",
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
