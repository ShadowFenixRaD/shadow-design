use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::components::{
    FeatureBanner, OptionPicker, PageHeader, Playground, PropRow, ShowcaseTabs,
};

#[component]
pub fn CodeHighlightPage() -> Element {
    let selected_lang = use_signal(|| "rust".to_string());

    let rust_sample = r#"// Modulo di Routing Dioxus 0.7
use dioxus::prelude::*;
use shadow_design::prelude::*;

#[component]
pub fn AppRouter() -> Element {
    let theme = use_theme();
    rsx! {
        div { class: "app-shell",
            Navbar { title: "Shadow Design Studio" }
            main { class: "content-container",
                Outlet::<Route> {}
            }
        }
    }
}"#
    .to_string();

    let js_sample = r#"// Client API WebSocket
import { createClient } from "@shadow/mesh";

export const connectVault = async (nodeId, token) => {
    const client = createClient({ endpoint: "wss://node1.shadowfenix.net" });
    const session = await client.authenticate(token);
    console.log("Connected to node:", nodeId);
    return session;
};"#
    .to_string();

    let py_sample = r#"# Script di calcolo entropia
import hashlib
import json

def generate_vault_key(secret: str, salt: str) -> str:
    combined = f"{secret}:{salt}".encode("utf-8")
    return hashlib.sha256(combined).hexdigest()

if __name__ == "__main__":
    key = generate_vault_key("master_token", "salt_9988")
    print(f"Generated Key: {key}")"#
    .to_string();

    let props = vec![
        PropRow {
            name: "code",
            r#type: "String",
            default: "-",
            description: "Sorgente di testo da visualizzare con formattazione e syntax highlighting",
        },
        PropRow {
            name: "language",
            r#type: "String",
            default: "\"rust\"",
            description: "Linguaggio sorgente per il tokenizer (\"rust\", \"javascript\", \"python\", \"json\", \"bash\")",
        },
        PropRow {
            name: "filename",
            r#type: "Option<String>",
            default: "None",
            description: "Nome file visualizzato nel badge della barra superiore (es. \"main.rs\")",
        },
        PropRow {
            name: "with_line_numbers",
            r#type: "bool",
            default: "true",
            description: "Abilita la colonna con la numerazione progressiva delle righe",
        },
        PropRow {
            name: "with_copy_button",
            r#type: "bool",
            default: "true",
            description: "Mostra il pulsante di copia negli appunti con feedback di avvenuta copia",
        },
        PropRow {
            name: "highlight_lines",
            r#type: "Vec<usize>",
            default: "vec![]",
            description: "Numeri di riga (1-indexed) da evidenziare con accento viola fluorescente",
        },
    ];

    let code_snippet = r##"CodeBlock {
    code: r#"fn main() { println!("Hello from Shadow Design!"); }"#.to_string(),
    language: "rust".to_string(),
    filename: Some("main.rs".to_string()),
    with_line_numbers: true,
    highlight_lines: vec![1],
}"##
    .to_string();

    let cur_code = match selected_lang.read().as_str() {
        "javascript" => js_sample,
        "python" => py_sample,
        _ => rust_sample,
    };

    let cur_filename = match selected_lang.read().as_str() {
        "javascript" => "client.ts",
        "python" => "entropy.py",
        _ => "router.rs",
    };

    rsx! {
        div { class: "sc-stack sc-stack--loose",
            PageHeader {
                title: "CodeBlock & CodeHighlight".to_string(),
                description: "Visualizzatore di codice sorgente con syntax highlighting multilinguaggio, numerazione righe, badge di file e pulsante rapido di copia.".to_string(),
            }

            FeatureBanner { feature_name: "code_highlight" }

            ShowcaseTabs {
                preview: rsx! {
                    Playground {
                        stage_start: true,
                        stage: rsx! {
                            div { class: "w-full",
                                CodeBlock {
                                    code: cur_code,
                                    language: selected_lang.read().clone(),
                                    filename: Some(cur_filename.to_string()),
                                    with_line_numbers: true,
                                    with_copy_button: true,
                                    highlight_lines: vec![2, 8],
                                }
                            }
                        },
                        controls: rsx! {
                            OptionPicker {
                                label: "Linguaggio Demo".to_string(),
                                options: vec![
                                    ("rust".to_string(), "Rust / RSX".to_string()),
                                    ("javascript".to_string(), "TypeScript".to_string()),
                                    ("python".to_string(), "Python".to_string()),
                                ],
                                value: selected_lang,
                            }
                        },
                    }
                },
                code: code_snippet,
                props: props,
            }
        }
    }
}
