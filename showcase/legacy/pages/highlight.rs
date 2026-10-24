use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::components::{
    ControlGroup, FeatureBanner, PageHeader, Playground, PropRow, ShowcaseTabs, ToggleControl,
};

#[component]
pub fn HighlightPage() -> Element {
    let query = use_signal(|| "shadow".to_string());
    let case_sensitive = use_signal(|| false);
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
        div { class: "sc-stack sc-stack--loose",
            PageHeader {
                title: "Highlight".to_string(),
                description: "Evidenzia visivamente occorrenze di testo o query di ricerca tramite tag <mark> integrati con supporto case-insensitive e stili ad alto contrasto.".to_string(),
            }

            FeatureBanner { feature_name: "highlight" }

            ShowcaseTabs {
                preview: rsx! {
                    Playground {
                        stage_start: true,
                        stage: rsx! {
                            p { class: "sc-muted-text",
                                Highlight {
                                    text: sample_text.to_string(),
                                    query: query(),
                                    case_sensitive: case_sensitive(),
                                }
                            }
                        },
                        controls: rsx! {
                            ControlGroup { label: "Ricerca".to_string(),
                                TextInput {
                                    value: query,
                                    placeholder: "Cerca nel testo...",
                                }
                            }
                            ToggleControl { label: "Case Sensitive".to_string(), checked: case_sensitive }
                        },
                    }
                },
                code: code_snippet,
                props: props,
            }
        }
    }
}
