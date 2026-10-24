//! Story: DocumentCard (feature `document_card`). The playground goes through `DocumentDemo { knobs }`
//! because the quick actions raise toasts.
use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::story::schema::*;

const TYPES: [&str; 5] = ["PDF", "DOCX", "XLSX", "RS", "ZIP"];
const TYPE_OPTS: &[ChoiceOption] = &[
    opt("PDF", "\"PDF\".to_string()"),
    opt("DOCX", "\"DOCX\".to_string()"),
    opt("XLSX", "\"XLSX\".to_string()"),
    opt("RS", "\"RS\".to_string()"),
    opt("ZIP", "\"ZIP\".to_string()"),
];

const PRELUDE: &str = r#"let mut toast = use_notification();
let author = PersonaProps::new("Giulia Ferri").with_status(PresenceStatus::Online);"#;

#[component]
fn DocumentDemo(knobs: Knobs) -> Element {
    let mut toast = use_notification();
    let author = PersonaProps::new("Giulia Ferri").with_status(PresenceStatus::Online);
    let download = knobs.bool("download").then(|| {
        EventHandler::new(move |_| {
            toast.info("Download avviato");
        })
    });
    let share = knobs.bool("share").then(|| {
        EventHandler::new(move |_| {
            toast.info("Link copiato");
        })
    });
    let delete = knobs.bool("delete").then(|| {
        EventHandler::new(move |_| {
            toast.warning("Documento spostato nel cestino");
        })
    });
    rsx! {
        DocumentCard {
            file_name: knobs.text("file_name"),
            file_type: knobs.pick("file_type", &TYPES).to_string(),
            file_size: Some(knobs.text("file_size")).filter(|s| !s.is_empty()),
            author,
            modified_at: "Oggi, 11:20".to_string(),
            views: knobs.bool("views").then_some(254),
            on_download: download,
            on_share: share,
            on_delete: delete,
        }
    }
}

fn render(k: &Knobs) -> Element {
    rsx! {
        DocumentDemo { knobs: k.clone() }
    }
}

#[component]
fn Types() -> Element {
    let author = PersonaProps::new("Marco Bellini");
    rsx! {
        SimpleGrid { cols: 2, spacing: GridSpacing::Sm,
            DocumentCard { file_name: "budget-2027.xlsx".to_string(), file_type: "XLSX".to_string(), author: author.clone(), modified_at: "Ieri".to_string() }
            DocumentCard { file_name: "relay.rs".to_string(), file_type: "RS".to_string(), author: author.clone(), modified_at: "2 giorni fa".to_string() }
            DocumentCard { file_name: "contratto-noc.docx".to_string(), file_type: "DOCX".to_string(), author: author.clone(), modified_at: "Lunedì".to_string() }
            DocumentCard { file_name: "backup-edge.zip".to_string(), file_type: "ZIP".to_string(), author, modified_at: "Settembre".to_string() }
        }
    }
}
const TYPES_CODE: &str = r#"let author = PersonaProps::new("Marco Bellini");

SimpleGrid { cols: 2, spacing: GridSpacing::Sm,
    DocumentCard { file_name: "budget-2027.xlsx".to_string(), file_type: "XLSX".to_string(), author: author.clone(), modified_at: "Ieri".to_string() }
    DocumentCard { file_name: "relay.rs".to_string(), file_type: "RS".to_string(), author: author.clone(), modified_at: "2 giorni fa".to_string() }
    DocumentCard { file_name: "contratto-noc.docx".to_string(), file_type: "DOCX".to_string(), author: author.clone(), modified_at: "Lunedì".to_string() }
    DocumentCard { file_name: "backup-edge.zip".to_string(), file_type: "ZIP".to_string(), author, modified_at: "Settembre".to_string() }
}"#;

#[component]
fn Preview() -> Element {
    let mut toast = use_notification();
    rsx! {
        DocumentCard {
            file_name: "mappa-rete-eu.png".to_string(),
            file_type: "PNG".to_string(),
            file_size: Some("820 KB".to_string()),
            preview_url: Some("https://picsum.photos/seed/shadowmesh/640/360".to_string()),
            author: PersonaProps::new("Sara Conti").with_status(PresenceStatus::Busy),
            modified_at: "Oggi, 09:12".to_string(),
            views: Some(31),
            on_click: move |_| {
                toast.info("Apro l'anteprima");
            },
        }
    }
}
const PREVIEW_CODE: &str = r#"let mut toast = use_notification();

DocumentCard {
    file_name: "mappa-rete-eu.png".to_string(),
    file_type: "PNG".to_string(),
    file_size: Some("820 KB".to_string()),
    preview_url: Some("https://picsum.photos/seed/shadowmesh/640/360".to_string()),
    author: PersonaProps::new("Sara Conti").with_status(PresenceStatus::Busy),
    modified_at: "Oggi, 09:12".to_string(),
    views: Some(31),
    on_click: move |_| {
        toast.info("Apro l'anteprima");
    },
}"#;

#[component]
fn InContext() -> Element {
    let mut docs = use_signal(|| {
        vec![
            ("runbook-incidenti.pdf", "PDF", "1,2 MB", "Giulia Ferri", "Oggi"),
            ("topologia-mesh.docx", "DOCX", "380 KB", "Marco Bellini", "Ieri"),
            ("capacita-q4.xlsx", "XLSX", "96 KB", "Sara Conti", "Martedì"),
        ]
    });
    let mut toast = use_notification();
    rsx! {
        Stack { gap: SpacingToken::Md,
            Group { justify: JustifyContent::SpaceBetween, align: AlignItems::Center,
                Heading { order: HeadingOrder::H4, "Documenti del progetto" }
                Badge { label: format!("{} file", docs().len()), pill: true }
            }
            SimpleGrid { cols: 1, sm: Some(2), lg: Some(3), spacing: GridSpacing::Md,
                for (i, (name, kind, size, owner, when)) in docs().into_iter().enumerate() {
                    DocumentCard {
                        file_name: name.to_string(),
                        file_type: kind.to_string(),
                        file_size: Some(size.to_string()),
                        author: PersonaProps::new(owner),
                        modified_at: when.to_string(),
                        on_download: move |_| {
                            toast.info("Download avviato");
                        },
                        on_delete: move |_| {
                            docs.write().remove(i);
                            toast.warning("Documento spostato nel cestino");
                        },
                    }
                }
            }
        }
    }
}
const IN_CONTEXT_CODE: &str = r#"let mut docs = use_signal(|| {
    vec![
        ("runbook-incidenti.pdf", "PDF", "1,2 MB", "Giulia Ferri", "Oggi"),
        ("topologia-mesh.docx", "DOCX", "380 KB", "Marco Bellini", "Ieri"),
        ("capacita-q4.xlsx", "XLSX", "96 KB", "Sara Conti", "Martedì"),
    ]
});
let mut toast = use_notification();

Stack { gap: SpacingToken::Md,
    Group { justify: JustifyContent::SpaceBetween, align: AlignItems::Center,
        Heading { order: HeadingOrder::H4, "Documenti del progetto" }
        Badge { label: format!("{} file", docs().len()), pill: true }
    }
    SimpleGrid { cols: 1, sm: Some(2), lg: Some(3), spacing: GridSpacing::Md,
        for (i, (name, kind, size, owner, when)) in docs().into_iter().enumerate() {
            DocumentCard {
                file_name: name.to_string(),
                file_type: kind.to_string(),
                file_size: Some(size.to_string()),
                author: PersonaProps::new(owner),
                modified_at: when.to_string(),
                on_download: move |_| {
                    toast.info("Download avviato");
                },
                on_delete: move |_| {
                    docs.write().remove(i);
                    toast.warning("Documento spostato nel cestino");
                },
            }
        }
    }
}"#;

const PROPS: &[PropDoc] = &[
    prop("file_name", "String", "-", "Nome del file, troncato con title completo."),
    prop("file_type", "String", "-", "Estensione: decide il colore del badge (PDF, DOCX, XLSX, RS, ZIP…)."),
    prop("file_size", "Option<String>", "None", "Dimensione già formattata."),
    prop("preview_url", "Option<String>", "None", "Miniatura; senza, un'icona con il badge del tipo."),
    prop("author", "PersonaProps", "-", "Autore con avatar e presenza."),
    prop("views", "Option<usize>", "None", "Contatore di visualizzazioni."),
    prop("modified_at", "String", "-", "Data di modifica già formattata."),
    prop("on_download / on_share / on_delete", "Option<EventHandler<MouseEvent>>", "None", "Azioni rapide in alto a destra."),
    prop("on_click", "Option<EventHandler<MouseEvent>>", "None", "Rende l'intera scheda attivabile."),
    prop("class", "String", "\"\"", "Classi aggiuntive."),
];

pub fn story() -> Story {
    Story {
        slug: "document-card",
        name: "DocumentCard",
        category: Category::Contenuti,
        features: &["document_card"],
        icon: IconKind::Paperclip,
        tagline: "Scheda di un documento con anteprima, tipo, autore e azioni rapide.",
        keywords: &["document card", "documento", "file", "anteprima", "allegato", "drive"],
        status: Status::Stable,
        playground: Some(Playground {
            component: "DocumentCard",
            controls: vec![
                Control::text("file_name", "Nome file", "runbook-incidenti.pdf")
                    .emit(Emit::Template("file_name: \"{v}\".to_string()")),
                Control::choice("file_type", "Tipo", TYPE_OPTS, 0).emit(Emit::Always),
                Control::text("file_size", "Dimensione", "1,2 MB").emit(Emit::Template("file_size: Some(\"{v}\".to_string())")),
                Control::bool("views", "Visualizzazioni", true).emit(Emit::Template("views: Some(254)")),
                Control::bool("download", "Scarica", true)
                    .emit(Emit::Template("on_download: move |_| { toast.info(\"Download avviato\"); }")),
                Control::bool("share", "Condividi", true).emit(Emit::Template("on_share: move |_| { toast.info(\"Link copiato\"); }")),
                Control::bool("delete", "Elimina", false)
                    .emit(Emit::Template("on_delete: move |_| { toast.warning(\"Documento spostato nel cestino\"); }")),
            ],
            render,
            prelude_code: Some(PRELUDE),
            fixed_code: &["author,", "modified_at: \"Oggi, 11:20\".to_string(),"],
            children_code: None,
            stage: Stage::Form,
        }),
        examples: vec![
            Example {
                id: "types",
                title: "Tipi di file",
                description: "Il badge cambia tono in base all'estensione.",
                code: TYPES_CODE,
                render: || rsx! { Types {} },
                span: Span::Half,
            },
            Example {
                id: "preview",
                title: "Anteprima e clic",
                description: "Miniatura reale e scheda attivabile.",
                code: PREVIEW_CODE,
                render: || rsx! { Preview {} },
                span: Span::Half,
            },
            Example {
                id: "context",
                title: "In contesto",
                description: "La cartella di un progetto: scarica o elimina un documento.",
                code: IN_CONTEXT_CODE,
                render: || rsx! { InContext {} },
                span: Span::Full,
            },
        ],
        props: vec![PropGroup { component: "DocumentCard", rows: PROPS }],
        recipe: Some(Recipe {
            recipe: "Container (§3.1) + semantic badge (§3.10)",
            depth: "Flat-lit",
            tokens: &["--sd-surface-1", "--sd-border", "--sd-radius-lg", "--sd-tint-1", "--sd-danger-soft", "--sd-success-soft", "--sd-info-soft"],
            notes: "Il tipo di file è l'unico colore: un badge semantico soft (PDF danger, XLSX success, DOCX info, codice accento).",
        }),
        a11y: &[
            "Le azioni rapide sono <button> con title da Locale (download, share, delete).",
            "Con on_click la scheda ha role=button e tabindex=0 (Card); i clic sulle azioni non la attivano.",
            "La miniatura ha alt uguale al nome del file.",
            "Il nome completo è disponibile in title anche quando è troncato.",
        ],
        related: &["attachment-cards", "media-card", "dropzone", "avatars"],
    }
}
