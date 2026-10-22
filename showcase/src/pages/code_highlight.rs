use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::components::{FeatureBanner, PropRow, ShowcaseTabs};

#[component]
pub fn CodeHighlightPage() -> Element {
    let mut selected_lang = use_signal(|| "rust".to_string());

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
        div { class: "max-w-4xl space-y-8",
            div {
                Heading { order: HeadingOrder::H2, "CodeBlock & CodeHighlight" }
                Text { dimmed: true, size: TextSize::Sm, "Visualizzatore di codice sorgente con syntax highlighting multilinguaggio, numerazione righe, badge di file e pulsante rapido di copia." }
            }

            FeatureBanner { feature_name: "code_highlight" }

            ShowcaseTabs {
                preview: rsx! {
                    div { class: "p-8 rounded-xl bg-[#060608] border border-white/[0.08] flex flex-col gap-6 shadow-inner",
                        // Switch linguaggio
                        div { class: "flex items-center gap-3",
                            span { class: "text-xs font-mono text-zinc-400 font-semibold", "Linguaggio Demo:" }
                            for (lang, label) in &[("rust", "Rust / RSX"), ("javascript", "TypeScript"), ("python", "Python")] {
                                {
                                    let is_active = *selected_lang.read() == *lang;
                                    let l = lang.to_string();
                                    rsx! {
                                        button {
                                            key: "{lang}",
                                            r#type: "button",
                                            class: format!("px-3 py-1.5 text-xs font-mono rounded-lg border transition-all cursor-pointer font-semibold {}", if is_active { "bg-purple-950/80 text-purple-200 border-purple-500/50 shadow-[0_0_10px_rgba(168,85,247,0.3)]" } else { "bg-white/[0.04] text-zinc-400 border-white/[0.08]" }),
                                            onclick: move |_| selected_lang.set(l.clone()),
                                            "{label}"
                                        }
                                    }
                                }
                            }
                        }

                        // Blocco codice
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
                code: code_snippet,
                props: props,
            }
        }
    }
}
