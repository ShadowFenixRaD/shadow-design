use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::components::{FeatureBanner, PageHeader, PropRow, ShowcaseTabs};

#[component]
pub fn RichTextPage() -> Element {
    let editor_value = use_signal(|| {
        "# Titolo Documento\n\nQuesto è un paragrafo formattato con **Shadow Design**.\n\n- Voce elenco 1\n- Voce elenco 2\n\n> Citazione di rilievo\n\n```rust\nfn hello() {\n    println!(\"Hello World\");\n}\n```".to_string()
    });

    let props = vec![
        PropRow {
            name: "value",
            r#type: "Signal<String>",
            default: "-",
            description: "Segnale reattivo contenente il testo Markdown o HTML",
        },
        PropRow {
            name: "placeholder",
            r#type: "String",
            default: "\"Scrivi qui il contenuto...\"",
            description: "Testo segnaposto visualizzato quando l'editor è vuoto",
        },
        PropRow {
            name: "editable",
            r#type: "bool",
            default: "true",
            description: "Abilita o disabilita la modifica diretta del testo",
        },
        PropRow {
            name: "sticky_toolbar",
            r#type: "bool",
            default: "true",
            description: "Mantiene la barra degli strumenti ancorata in alto durante lo scroll",
        },
        PropRow {
            name: "mode",
            r#type: "Option<RichTextMode>",
            default: "Some(RichTextMode::Wysiwyg)",
            description: "Modalità iniziale di visualizzazione (Wysiwyg, Markdown, Split)",
        },
    ];

    let code_snippet = r##"let editor_value = use_signal(|| "# Titolo".to_string());

RichTextEditor {
    value: editor_value,
    placeholder: "Scrivi documentazione...".to_string(),
    sticky_toolbar: true,
}"##
    .to_string();

    rsx! {
        div { class: "sc-stack sc-stack--loose",
            PageHeader {
                title: "RichTextEditor".to_string(),
                description: "Editor di testo ricco con motore duale WYSIWYG + Markdown, toolbar modulare componibile e zero dipendenze JavaScript esterne.".to_string(),
            }

            FeatureBanner { feature_name: "rich_text" }

            ShowcaseTabs {
                preview: rsx! {
                    div { class: "w-full",
                        RichTextEditor {
                            value: editor_value,
                            placeholder: "Digita qualcosa o usa la toolbar...".to_string(),
                            min_height: Some("260px".to_string()),
                        }
                    }
                },
                code: code_snippet,
                props: props,
            }
        }
    }
}
