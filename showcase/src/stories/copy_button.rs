//! Story: CopyButton (feature `copy_button`). Stateless for the caller: the copied state lives
//! inside the component, so the playground renders it directly.
use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::story::schema::*;

fn render(k: &Knobs) -> Element {
    let label = k.text("label");
    rsx! {
        CopyButton {
            value: k.text("value"),
            label: (!label.is_empty()).then_some(label),
            copied_label: k.text("copied_label"),
            icon_only: k.bool("icon_only"),
            timeout_ms: k.number("timeout_ms") as u32,
        }
    }
}

#[component]
fn Inline() -> Element {
    rsx! {
        Group { gap: SpacingToken::Sm, wrap: false,
            Code { "cargo add shadow-design" }
            CopyButton { value: "cargo add shadow-design".to_string(), icon_only: true, aria_label: "Copia il comando".to_string() }
        }
    }
}
const INLINE_CODE: &str = r#"Group { gap: SpacingToken::Sm, wrap: false,
    Code { "cargo add shadow-design" }
    CopyButton { value: "cargo add shadow-design".to_string(), icon_only: true, aria_label: "Copia il comando".to_string() }
}"#;

#[component]
fn Labels() -> Element {
    rsx! {
        Group { gap: SpacingToken::Sm,
            CopyButton { value: "https://mesh.shadowfenix.net/invito/7k2p".to_string(), label: Some("Copia link".to_string()) }
            CopyButton {
                value: "edge-07.eu.mesh".to_string(),
                label: Some("Copia host".to_string()),
                copied_label: "Host copiato".to_string(),
                timeout_ms: 4000,
            }
        }
    }
}
const LABELS_CODE: &str = r#"Group { gap: SpacingToken::Sm,
    CopyButton { value: "https://mesh.shadowfenix.net/invito/7k2p".to_string(), label: Some("Copia link".to_string()) }
    CopyButton {
        value: "edge-07.eu.mesh".to_string(),
        label: Some("Copia host".to_string()),
        copied_label: "Host copiato".to_string(),
        timeout_ms: 4000,
    }
}"#;

/// API keys shown in the context example: name, masked key, value copied.
const KEYS: [(&str, &str, &str); 3] = [
    ("Produzione", "sk_live_••••••••4f2a", "sk_live_demo_key_produzione"),
    ("Staging", "sk_test_••••••••91cc", "sk_test_demo_key_staging"),
    ("CI", "sk_test_••••••••07de", "sk_test_demo_key_ci"),
];

#[component]
fn InContext() -> Element {
    rsx! {
        Card {
            title: Some("Chiavi API".to_string()),
            subtitle: Some("Copia una chiave nel gestore dei segreti: non la mostriamo più per intero.".to_string()),
            Stack { gap: SpacingToken::Sm,
                for (name, masked, value) in KEYS {
                    Group { gap: SpacingToken::Md, justify: JustifyContent::SpaceBetween, wrap: false,
                        Stack { gap: SpacingToken::Xs,
                            span { "{name}" }
                            span { class: "sc-mono sc-subtle-text", "{masked}" }
                        }
                        CopyButton { value: value.to_string(), label: Some("Copia".to_string()), aria_label: format!("Copia la chiave {name}") }
                    }
                }
            }
        }
    }
}
const IN_CONTEXT_CODE: &str = r#"Card {
    title: Some("Chiavi API".to_string()),
    subtitle: Some("Copia una chiave nel gestore dei segreti: non la mostriamo più per intero.".to_string()),
    Stack { gap: SpacingToken::Sm,
        for (name, masked, value) in KEYS {
            Group { gap: SpacingToken::Md, justify: JustifyContent::SpaceBetween, wrap: false,
                Stack { gap: SpacingToken::Xs,
                    span { "{name}" }
                    span { "{masked}" }
                }
                CopyButton { value: value.to_string(), label: Some("Copia".to_string()), aria_label: format!("Copia la chiave {name}") }
            }
        }
    }
}"#;

const PROPS: &[PropDoc] = &[
    prop("value", "String", "-", "Il testo copiato negli appunti."),
    prop("label", "Option<String>", "None", "Testo accanto all'icona; senza, resta solo l'icona."),
    prop("copied_label", "String", "\"\"", "Conferma dopo la copia; vuota usa \"Copied!\" da Locale."),
    prop("icon_only", "bool", "false", "Pulsante quadrato con sola icona, anche se label è presente."),
    prop("aria_label", "String", "\"\"", "Nome accessibile a riposo; vuoto usa \"Copy to clipboard\" da Locale."),
    prop("timeout_ms", "u32", "2000", "Quanto resta visibile la conferma, in millisecondi."),
    prop("class", "String", "\"\"", "Classi aggiuntive sul <button>."),
];

pub fn story() -> Story {
    Story {
        slug: "copy-button",
        name: "CopyButton",
        category: Category::Utility,
        features: &["copy_button"],
        icon: IconKind::Copy,
        tagline: "Copia un valore negli appunti e conferma con una spunta per qualche secondo.",
        keywords: &["copy", "copia", "appunti", "clipboard", "comando", "chiave api"],
        status: Status::Stable,
        playground: Some(Playground {
            component: "CopyButton",
            controls: vec![
                Control::text("value", "Valore", "cargo add shadow-design").emit(Emit::Always),
                Control::text("label", "Etichetta", "Copia").emit(Emit::Template("label: Some(\"{v}\".to_string())")),
                Control::text("copied_label", "Conferma", "").hint("Vuota: \"Copied!\" da Locale."),
                Control::number("timeout_ms", "Durata conferma (ms)", 2000.0, 1000.0, 6000.0, 500.0, 0),
                Control::bool("icon_only", "Solo icona", false),
            ],
            render,
            prelude_code: None,
            fixed_code: &[],
            children_code: None,
            stage: Stage::Center,
        }),
        examples: vec![
            Example {
                id: "inline",
                title: "Accanto al valore",
                description: "Solo icona, con un nome accessibile che dice cosa copia.",
                code: INLINE_CODE,
                render: || rsx! { Inline {} },
                span: Span::Half,
            },
            Example {
                id: "labels",
                title: "Etichette e conferma",
                description: "Testo personalizzato e conferma più lunga.",
                code: LABELS_CODE,
                render: || rsx! { Labels {} },
                span: Span::Half,
            },
            Example {
                id: "context",
                title: "In contesto",
                description: "Chiavi API mascherate: si copiano senza mostrarle.",
                code: IN_CONTEXT_CODE,
                render: || rsx! { InContext {} },
                span: Span::Full,
            },
        ],
        props: vec![PropGroup { component: "CopyButton", rows: PROPS }],
        recipe: Some(Recipe {
            recipe: "Raised control, variante secondary → success (language.md §3.5)",
            depth: "Lit control",
            tokens: &["--sd-control-sm", "--sd-surface-3", "--sd-border-strong", "--sd-success-soft", "--sd-success-strong", "--sd-text-xs"],
            notes: "A riposo è un secondary piccolo; dopo la copia passa per qualche secondo alla tinta soft success, mai piena.",
        }),
        a11y: &[
            "Elemento <button type=button> nativo, attivabile con Invio e Spazio.",
            "aria-label e title passano a copied_label dopo la copia, così lo screen reader può rileggere la conferma.",
            "Con icon_only il nome accessibile è aria_label (o \"Copy to clipboard\" da Locale): rendilo specifico.",
            "La copia usa navigator.clipboard, con un ripiego per i browser senza Clipboard API.",
        ],
        related: &["code-highlight", "buttons", "close-button", "reactive-hooks"],
    }
}
