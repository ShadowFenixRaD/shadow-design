//! Story: Center (feature `center`). Trivial component: no playground (BATCH_GUIDE §4.4).
use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::story::schema::*;
use crate::story::sections::DemoCell;

#[component]
fn Block() -> Element {
    rsx! {
        Center { class: "sc-demo-frame h-48",
            DemoCell { label: Some("Al centro".to_string()), accent: true }
        }
    }
}
const BLOCK_CODE: &str = r#"Center { class: "h-48",
    DemoCell { label: Some("Al centro".to_string()), accent: true }
}"#;

#[component]
fn Inline() -> Element {
    rsx! {
        div { class: "sc-muted-text",
            "Sincronizzazione "
            Center { inline: true,
                Icon { kind: IconKind::CheckCircle, size: 14 }
            }
            " completata su 12 nodi."
        }
    }
}
const INLINE_CODE: &str = r#"div {
    "Sincronizzazione "
    Center { inline: true,
        Icon { kind: IconKind::CheckCircle, size: 14 }
    }
    " completata su 12 nodi."
}"#;

#[component]
fn InContext() -> Element {
    let mut toast = use_notification();
    rsx! {
        Card { title: Some("Nodi edge".to_string()), subtitle: Some("Regione eu-central".to_string()),
            Center { class: "h-56",
                Stack { gap: SpacingToken::Sm, align: AlignItems::Center, class: "text-center",
                    Icon { kind: IconKind::Server, size: 28 }
                    Text { "Nessun nodo collegato" }
                    Text { size: TextSize::Sm, dimmed: true, "Aggiungi il primo nodo per iniziare a raccogliere metriche." }
                    Button {
                        variant: ButtonVariant::Primary,
                        size: ButtonSize::Sm,
                        left_icon: Some(rsx! {
                            Icon { kind: IconKind::Plus, size: 14 }
                        }),
                        onclick: move |_| {
                            toast.info("Procedura di registrazione avviata");
                        },
                        "Aggiungi nodo"
                    }
                }
            }
        }
    }
}
const IN_CONTEXT_CODE: &str = r#"let mut toast = use_notification();

Card { title: Some("Nodi edge".to_string()), subtitle: Some("Regione eu-central".to_string()),
    Center { class: "h-56",
        Stack { gap: SpacingToken::Sm, align: AlignItems::Center, class: "text-center",
            Icon { kind: IconKind::Server, size: 28 }
            Text { "Nessun nodo collegato" }
            Text { size: TextSize::Sm, dimmed: true, "Aggiungi il primo nodo per iniziare a raccogliere metriche." }
            Button {
                variant: ButtonVariant::Primary,
                size: ButtonSize::Sm,
                left_icon: Some(rsx! {
                    Icon { kind: IconKind::Plus, size: 14 }
                }),
                onclick: move |_| {
                    toast.info("Procedura di registrazione avviata");
                },
                "Aggiungi nodo"
            }
        }
    }
}"#;

const PROPS: &[PropDoc] = &[
    prop("inline", "bool", "false", "inline-flex al posto di flex: si allinea al testo che lo circonda."),
    prop("class", "String", "\"\"", "Classi aggiuntive; dagli un'altezza per centrare in verticale."),
    prop("children", "Element", "-", "Contenuto da centrare."),
];

pub fn story() -> Story {
    Story {
        slug: "center",
        name: "Center",
        category: Category::Layout,
        features: &["center"],
        icon: IconKind::Box,
        tagline: "Centra il contenuto in orizzontale e in verticale, a blocco o in linea col testo.",
        keywords: &["center", "centra", "centrato", "allineamento", "empty state", "flex"],
        status: Status::Stable,
        playground: None,
        examples: vec![
            Example {
                id: "block",
                title: "A blocco",
                description: "Centrato in entrambe le direzioni in un'area alta 12rem.",
                code: BLOCK_CODE,
                render: || rsx! { Block {} },
                span: Span::Half,
            },
            Example {
                id: "inline",
                title: "In linea",
                description: "Un'icona allineata al testo con inline.",
                code: INLINE_CODE,
                render: || rsx! { Inline {} },
                span: Span::Half,
            },
            Example {
                id: "context",
                title: "In contesto",
                description: "Stato vuoto di un pannello con una sola azione.",
                code: IN_CONTEXT_CODE,
                render: || rsx! { InContext {} },
                span: Span::Full,
            },
        ],
        props: vec![PropGroup { component: "Center", rows: PROPS }],
        recipe: Some(Recipe {
            recipe: "Primitiva di layout (nessuna superficie, language.md §2 Void)",
            depth: "Void: dispone, non disegna",
            tokens: &["--sd-space-2", "--sd-space-4", "--sd-text-sm", "--sd-text-muted"],
            notes: "Nessuno sfondo né bordo: la superficie la porta il contenitore che lo ospita.",
        }),
        a11y: &[
            "Contenitore div senza ruolo: il contenuto resta nell'ordine del sorgente.",
            "Centrare non cambia l'ordine del focus né la struttura annunciata.",
            "Negli stati vuoti affianca all'icona un testo: l'icona da sola non viene annunciata.",
        ],
        related: &["stack", "group", "aspect-ratio", "cards"],
    }
}
