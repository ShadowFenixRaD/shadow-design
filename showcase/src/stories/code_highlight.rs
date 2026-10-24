//! Story: CodeHighlight (feature `code_highlight`). `CodeHighlight` wraps `CodeBlock` with the
//! same signature. The playground sample lives in one macro so the live block and the printed
//! `let sample = r#"…"#;` line cannot drift.
use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::story::schema::*;

macro_rules! rust_sample {
    () => {
        r#"use dioxus::prelude::*;
use shadow_design::prelude::*;

#[component]
pub fn NodeStatus(name: String, latency_ms: u32) -> Element {
    let slow = latency_ms > 120;
    rsx! {
        Card { title: Some(name),
            Badge { "{latency_ms} ms" }
            if slow {
                Alert { "Nodo lento: verifica il relay" }
            }
        }
    }
}"#
    };
}

const SAMPLE: &str = rust_sample!();
const PRELUDE: &str = concat!("let sample = r#\"", rust_sample!(), "\"#;");

const HIGHLIGHTS: [&[usize]; 3] = [&[], &[6], &[10, 11, 12]];
const HIGHLIGHT_OPTS: &[ChoiceOption] = &[
    opt("Nessuna", "vec![]"),
    opt("Riga 6", "vec![6]"),
    opt("Righe 10–12", "vec![10, 11, 12]"),
];

fn render(k: &Knobs) -> Element {
    let filename = k.text("filename");
    rsx! {
        CodeHighlight {
            code: SAMPLE.to_string(),
            filename: (!filename.is_empty()).then_some(filename),
            highlight_lines: k.pick("highlight_lines", &HIGHLIGHTS).to_vec(),
            with_line_numbers: k.bool("with_line_numbers"),
            with_copy_button: k.bool("with_copy_button"),
        }
    }
}

#[component]
fn TypeScript() -> Element {
    let sample = r#"import { createClient } from "@shadowfenix/mesh";

export async function connect(nodeId: string, token: string) {
  const client = createClient({ endpoint: "wss://edge-07.mesh.local" });
  const session = await client.authenticate(token);
  console.log("Nodo collegato:", nodeId);
  return session;
}"#;
    rsx! {
        CodeHighlight {
            code: sample.to_string(),
            language: "ts",
            filename: Some("client.ts".to_string()),
            highlight_lines: vec![5],
        }
    }
}
const TYPESCRIPT_CODE: &str = r##"let sample = r#"import { createClient } from "@shadowfenix/mesh";

export async function connect(nodeId: string, token: string) {
  const client = createClient({ endpoint: "wss://edge-07.mesh.local" });
  const session = await client.authenticate(token);
  console.log("Nodo collegato:", nodeId);
  return session;
}"#;

CodeHighlight {
    code: sample.to_string(),
    language: "ts",
    filename: Some("client.ts".to_string()),
    highlight_lines: vec![5],
}"##;

#[component]
fn Python() -> Element {
    let sample = r#"# Rotazione della chiave di sessione
import hashlib

def session_key(secret: str, salt: str) -> str:
    combined = f"{secret}:{salt}".encode("utf-8")
    return hashlib.sha256(combined).hexdigest()

if __name__ == "__main__":
    print(session_key("edge-07", "turno-notte"))"#;
    rsx! {
        CodeHighlight {
            code: sample.to_string(),
            language: "python",
            filename: Some("rotate_key.py".to_string()),
        }
    }
}
const PYTHON_CODE: &str = r##"let sample = r#"# Rotazione della chiave di sessione
import hashlib

def session_key(secret: str, salt: str) -> str:
    combined = f"{secret}:{salt}".encode("utf-8")
    return hashlib.sha256(combined).hexdigest()

if __name__ == "__main__":
    print(session_key("edge-07", "turno-notte"))"#;

CodeHighlight {
    code: sample.to_string(),
    language: "python",
    filename: Some("rotate_key.py".to_string()),
}"##;

#[component]
fn Minimal() -> Element {
    let sample = r#"{
  "node": "edge-07",
  "region": "eu-south",
  "relay": true,
  "latency_budget_ms": 120
}"#;
    rsx! {
        CodeHighlight {
            code: sample.to_string(),
            language: "json",
            with_line_numbers: false,
            with_copy_button: false,
        }
    }
}
const MINIMAL_CODE: &str = r##"let sample = r#"{
  "node": "edge-07",
  "region": "eu-south",
  "relay": true,
  "latency_budget_ms": 120
}"#;

CodeHighlight {
    code: sample.to_string(),
    language: "json",
    with_line_numbers: false,
    with_copy_button: false,
}"##;

#[component]
fn InContext() -> Element {
    let manifest = r#"[dependencies]
dioxus = "0.7"
shadow-design = { version = "0.5", features = ["code_highlight"] }"#;
    let commands = r#"# Avvia il nodo e registra la chiave pubblica
mesh-agent init --region eu-south
mesh-agent up --name edge-07"#;
    let mut toast = use_notification();
    rsx! {
        Card {
            title: Some("Collega il primo nodo".to_string()),
            subtitle: Some("Due passaggi · circa due minuti".to_string()),
            footer: Some(rsx! {
                Group { gap: SpacingToken::Sm, justify: JustifyContent::End,
                    Button { variant: ButtonVariant::Ghost, "Documentazione" }
                    Button {
                        variant: ButtonVariant::Primary,
                        onclick: move |_| {
                            toast.success("edge-07 è in linea");
                        },
                        "Verifica connessione"
                    }
                }
            }),
            Stack { gap: SpacingToken::Md,
                p { class: "sc-eyebrow", "1 · Aggiungi la dipendenza" }
                CodeHighlight {
                    code: manifest.to_string(),
                    language: "toml",
                    filename: Some("Cargo.toml".to_string()),
                    highlight_lines: vec![3],
                }
                p { class: "sc-eyebrow", "2 · Avvia l'agente" }
                CodeHighlight {
                    code: commands.to_string(),
                    language: "bash",
                    filename: Some("terminale".to_string()),
                    with_line_numbers: false,
                }
            }
        }
    }
}
// The live render adds the showcase-only `class: "sc-eyebrow"` on the step labels; sample code
// never carries sc-* classes (REVAMP §5 item 6).
const IN_CONTEXT_CODE: &str = r##"let manifest = r#"[dependencies]
dioxus = "0.7"
shadow-design = { version = "0.5", features = ["code_highlight"] }"#;
let commands = r#"# Avvia il nodo e registra la chiave pubblica
mesh-agent init --region eu-south
mesh-agent up --name edge-07"#;
let mut toast = use_notification();

Card {
    title: Some("Collega il primo nodo".to_string()),
    subtitle: Some("Due passaggi · circa due minuti".to_string()),
    footer: Some(rsx! {
        Group { gap: SpacingToken::Sm, justify: JustifyContent::End,
            Button { variant: ButtonVariant::Ghost, "Documentazione" }
            Button {
                variant: ButtonVariant::Primary,
                onclick: move |_| {
                    toast.success("edge-07 è in linea");
                },
                "Verifica connessione"
            }
        }
    }),
    Stack { gap: SpacingToken::Md,
        p { "1 · Aggiungi la dipendenza" }
        CodeHighlight {
            code: manifest.to_string(),
            language: "toml",
            filename: Some("Cargo.toml".to_string()),
            highlight_lines: vec![3],
        }
        p { "2 · Avvia l'agente" }
        CodeHighlight {
            code: commands.to_string(),
            language: "bash",
            filename: Some("terminale".to_string()),
            with_line_numbers: false,
        }
    }
}"##;

const PROPS: &[PropDoc] = &[
    prop("code", "String", "-", "Sorgente da mostrare; ogni riga è tokenizzata a parte."),
    prop(
        "language",
        "String",
        "\"rust\"",
        "Parole chiave per rust/rs, js/ts e py; negli altri linguaggi si colorano stringhe, numeri e commenti.",
    ),
    prop("filename", "Option<String>", "None", "Testo del badge in alto; senza, il linguaggio in maiuscolo."),
    prop("with_line_numbers", "bool", "true", "Colonna dei numeri di riga, esclusa dalla selezione."),
    prop("with_copy_button", "bool", "true", "CopyButton nella barra, con le etichette copy e copied di Locale."),
    prop("highlight_lines", "Vec<usize>", "vec![]", "Righe da evidenziare (da 1) con tinta e filetto accento."),
    prop("class", "String", "\"\"", "Classi aggiuntive sul contenitore."),
];
const PARTS_PROPS: &[PropDoc] = &[
    prop("CodeBlock { .. }", "component", "-", "Stessa firma di CodeHighlight, che lo avvolge: usa l'uno o l'altro."),
    prop("tokenize_line(line, lang)", "fn(&str, &str) -> Vec<SyntaxToken>", "-", "Tokenizer di una riga, senza dipendenze."),
    prop(
        "SyntaxTokenKind",
        "enum",
        "-",
        "Keyword, Function, Type, String, Comment, Number, Operator, Plain; class_name() dà la classe sd-token-*.",
    ),
];

pub fn story() -> Story {
    Story {
        slug: "code-highlight",
        name: "CodeHighlight",
        category: Category::Contenuti,
        features: &["code_highlight"],
        icon: IconKind::Code,
        tagline: "Blocco di codice con evidenziazione sintattica, numeri di riga, righe in risalto e copia.",
        keywords: &["code", "codice", "codeblock", "syntax highlighting", "evidenziazione sintattica", "snippet", "copia"],
        status: Status::Stable,
        playground: Some(Playground {
            component: "CodeHighlight",
            controls: vec![
                Control::text("filename", "Nome file", "node_status.rs")
                    .emit(Emit::Template("filename: Some(\"{v}\".to_string())"))
                    .hint("Vuoto: il badge mostra il linguaggio."),
                Control::choice("highlight_lines", "Righe evidenziate", HIGHLIGHT_OPTS, 2).emit(Emit::Always),
                Control::bool("with_line_numbers", "Numeri di riga", true),
                Control::bool("with_copy_button", "Pulsante copia", true),
            ],
            render,
            prelude_code: Some(PRELUDE),
            fixed_code: &["code: sample.to_string(),"],
            children_code: None,
            stage: Stage::Start,
        }),
        examples: vec![
            Example {
                id: "typescript",
                title: "TypeScript",
                description: "Parole chiave, stringhe e una riga in risalto.",
                code: TYPESCRIPT_CODE,
                render: || rsx! { TypeScript {} },
                span: Span::Half,
            },
            Example {
                id: "python",
                title: "Python",
                description: "Commenti, f-string e funzioni.",
                code: PYTHON_CODE,
                render: || rsx! { Python {} },
                span: Span::Half,
            },
            Example {
                id: "minimal",
                title: "Essenziale",
                description: "Senza numeri di riga né copia, per frammenti brevi.",
                code: MINIMAL_CODE,
                render: || rsx! { Minimal {} },
                span: Span::Half,
            },
            Example {
                id: "context",
                title: "In contesto",
                description: "Una guida di installazione in due passaggi con la riga da aggiungere in risalto.",
                code: IN_CONTEXT_CODE,
                render: || rsx! { InContext {} },
                span: Span::Full,
            },
        ],
        props: vec![
            PropGroup { component: "CodeHighlight", rows: PROPS },
            PropGroup { component: "Parti", rows: PARTS_PROPS },
        ],
        recipe: Some(Recipe {
            recipe: "Container (language.md §3.1)",
            depth: "Flat-lit",
            tokens: &[
                "--sd-surface-1",
                "--sd-border",
                "--sd-tint-1",
                "--sd-accent-soft",
                "--sd-accent",
                "--sd-font-mono",
                "--sd-code-keyword",
                "--sd-code-string",
            ],
            notes: "Le righe sono statiche, senza hover; la riga evidenziata usa la tinta accento con un filetto a sinistra, mai un riempimento pieno.",
        }),
        a11y: &[
            "Il codice è in <pre><code>: gli screen reader lo leggono come testo preformattato.",
            "I numeri di riga hanno user-select: none e restano fuori dal testo selezionato.",
            "Il pulsante copia ha aria-label e title da Locale e cambia etichetta dopo la copia.",
            "L'evidenziazione delle righe è solo visiva: se conta, spiegala nel testo accanto.",
        ],
        related: &["copy-button", "rich-text", "highlight", "document-card"],
    }
}
