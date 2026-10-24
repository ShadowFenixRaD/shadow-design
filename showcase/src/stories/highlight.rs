//! Story: Highlight (feature `highlight`). Stateless: the playground renders it directly.
use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::story::schema::*;

const TEXT: &str = "La rotazione della chiave di edge-07 è pianificata: la nuova Chiave entra in vigore alle 03:00.";

/// Option code is the raw CSS value printed into `highlight_color`; "" means the default (accent).
const COLORS: [&str; 4] = ["", "var(--sd-warning-soft)", "var(--sd-success-soft)", "var(--sd-info-soft)"];
const COLOR_OPTS: &[ChoiceOption] = &[
    opt("Accento", ""),
    opt("Warning", "var(--sd-warning-soft)"),
    opt("Success", "var(--sd-success-soft)"),
    opt("Info", "var(--sd-info-soft)"),
];

fn render(k: &Knobs) -> Element {
    let color = k.pick("highlight_color", &COLORS);
    rsx! {
        Text {
            Highlight {
                text: TEXT.to_string(),
                query: k.text("query"),
                case_sensitive: k.bool("case_sensitive"),
                highlight_color: (!color.is_empty()).then(|| color.to_string()),
            }
        }
    }
}

#[component]
fn CaseSensitivity() -> Element {
    rsx! {
        Stack { gap: SpacingToken::Sm,
            Text { size: TextSize::Sm,
                Highlight { text: "Rust, rust-analyzer e RUSTFLAGS".to_string(), query: "rust".to_string() }
            }
            Text { size: TextSize::Sm,
                Highlight { text: "Rust, rust-analyzer e RUSTFLAGS".to_string(), query: "rust".to_string(), case_sensitive: true }
            }
        }
    }
}
const CASE_CODE: &str = r#"Stack { gap: SpacingToken::Sm,
    Text { size: TextSize::Sm,
        Highlight { text: "Rust, rust-analyzer e RUSTFLAGS".to_string(), query: "rust".to_string() }
    }
    Text { size: TextSize::Sm,
        Highlight { text: "Rust, rust-analyzer e RUSTFLAGS".to_string(), query: "rust".to_string(), case_sensitive: true }
    }
}"#;

#[component]
fn Semantic() -> Element {
    rsx! {
        Stack { gap: SpacingToken::Sm,
            Text { size: TextSize::Sm,
                Highlight { text: "Build completata con 2 avvisi.".to_string(), query: "avvisi".to_string(), highlight_color: Some("var(--sd-warning-soft)".to_string()) }
            }
            Text { size: TextSize::Sm,
                Highlight { text: "214 test passati, 0 falliti.".to_string(), query: "passati".to_string(), highlight_color: Some("var(--sd-success-soft)".to_string()) }
            }
        }
    }
}
const SEMANTIC_CODE: &str = r#"Stack { gap: SpacingToken::Sm,
    Text { size: TextSize::Sm,
        Highlight { text: "Build completata con 2 avvisi.".to_string(), query: "avvisi".to_string(), highlight_color: Some("var(--sd-warning-soft)".to_string()) }
    }
    Text { size: TextSize::Sm,
        Highlight { text: "214 test passati, 0 falliti.".to_string(), query: "passati".to_string(), highlight_color: Some("var(--sd-success-soft)".to_string()) }
    }
}"#;

const COMMANDS: [&str; 6] = [
    "Riavvia il nodo edge-07",
    "Ruota le chiavi del cluster",
    "Esporta il registro eventi",
    "Apri le impostazioni di rete",
    "Invita un operatore",
    "Riavvia tutti i relay",
];

#[component]
fn InContext() -> Element {
    let query = use_signal(|| "riavvia".to_string());
    let q = query().to_lowercase();
    let hits: Vec<&str> = COMMANDS.iter().copied().filter(|c| c.to_lowercase().contains(&q)).collect();
    rsx! {
        Stack { gap: SpacingToken::Md,
            TextInput {
                value: query,
                label: Some("Cerca un comando".to_string()),
                left_icon: Some(IconKind::Search.into()),
                clearable: true,
            }
            List { spacing: ListSpacing::Sm,
                for cmd in hits {
                    ListItem {
                        Highlight { text: cmd.to_string(), query: query() }
                    }
                }
            }
        }
    }
}
const IN_CONTEXT_CODE: &str = r#"let query = use_signal(|| "riavvia".to_string());
let q = query().to_lowercase();
let hits: Vec<&str> = COMMANDS.iter().copied().filter(|c| c.to_lowercase().contains(&q)).collect();

Stack { gap: SpacingToken::Md,
    TextInput {
        value: query,
        label: Some("Cerca un comando".to_string()),
        left_icon: Some(IconKind::Search.into()),
        clearable: true,
    }
    List { spacing: ListSpacing::Sm,
        for cmd in hits {
            ListItem {
                Highlight { text: cmd.to_string(), query: query() }
            }
        }
    }
}"#;

const PROPS: &[PropDoc] = &[
    prop("text", "String", "-", "Il testo completo da mostrare."),
    prop("query", "String", "-", "La sottostringa da evidenziare in ogni occorrenza; vuota non evidenzia nulla."),
    prop("case_sensitive", "bool", "false", "Distingue maiuscole e minuscole."),
    prop("highlight_color", "Option<String>", "None", "Sfondo CSS del segno, come var(--sd-warning-soft); senza, --sd-accent-soft."),
    prop("highlight_class", "String", "\"\"", "Classi aggiuntive su ogni <mark>."),
    prop("class", "String", "\"\"", "Classi aggiuntive sullo span contenitore."),
];

pub fn story() -> Story {
    Story {
        slug: "highlight",
        name: "Highlight",
        category: Category::Utility,
        features: &["highlight"],
        icon: IconKind::Search,
        tagline: "Evidenzia dentro un testo le occorrenze di ciò che l'utente ha cercato.",
        keywords: &["highlight", "evidenzia", "ricerca", "mark", "search", "filtro"],
        status: Status::Stable,
        playground: Some(Playground {
            component: "Highlight",
            controls: vec![
                Control::text("query", "Ricerca", "chiave").emit(Emit::Always),
                Control::choice("highlight_color", "Colore", COLOR_OPTS, 0)
                    .emit(Emit::Template("highlight_color: Some(\"{v}\".to_string())")),
                Control::bool("case_sensitive", "Maiuscole esatte", false),
            ],
            render,
            prelude_code: None,
            fixed_code: &["text: \"La rotazione della chiave di edge-07 è pianificata: la nuova Chiave entra in vigore alle 03:00.\".to_string(),"],
            children_code: None,
            stage: Stage::Form,
        }),
        examples: vec![
            Example {
                id: "case",
                title: "Maiuscole e minuscole",
                description: "Stessa ricerca: sopra ignora le maiuscole, sotto le rispetta.",
                code: CASE_CODE,
                render: || rsx! { CaseSensitivity {} },
                span: Span::Half,
            },
            Example {
                id: "semantic",
                title: "Tinte semantiche",
                description: "highlight_color accetta un token: soft, mai pieno.",
                code: SEMANTIC_CODE,
                render: || rsx! { Semantic {} },
                span: Span::Half,
            },
            Example {
                id: "context",
                title: "In contesto",
                description: "Un elenco di comandi filtrato mentre scrivi, con la ricerca evidenziata.",
                code: IN_CONTEXT_CODE,
                render: || rsx! { InContext {} },
                span: Span::Full,
            },
        ],
        props: vec![PropGroup { component: "Highlight", rows: PROPS }],
        recipe: Some(Recipe {
            recipe: "Semantic surface in miniatura (language.md §3.10) su testo (§3.12)",
            depth: "Nessuna superficie: segno dentro il testo",
            tokens: &["--sd-accent-soft", "--sd-accent-strong", "--sd-radius-xs", "--sd-space-1", "--sd-warning-soft"],
            notes: "Fondo soft e testo accent-strong a peso 500; il segno va a capo con il testo (box-decoration-break: clone).",
        }),
        a11y: &[
            "Le occorrenze sono elementi <mark> nativi dentro il testo: l'ordine di lettura non cambia.",
            "Molti screen reader non annunciano <mark>: non affidare a esso un'informazione che non sia anche nel testo.",
            "Il testo evidenziato resta in --sd-accent-strong su fondo soft, con contrasto da testo normale.",
        ],
        related: &["code-highlight", "command-palette", "inputs", "lists"],
    }
}
