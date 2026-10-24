//! Story: Dropzone (feature `dropzone`). The playground render is stateless: Dropzone has no
//! value signal, it reports files through `on_drop`.
use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::story::schema::*;

fn render(k: &Knobs) -> Element {
    rsx! {
        Dropzone {
            title: k.text("title"),
            description: Some(k.text("description")).filter(|s| !s.is_empty()),
            accept: if k.bool("accept") { vec![".pdf".to_string(), ".png".to_string(), ".jpg".to_string()] } else { Vec::new() },
            max_size_bytes: k.bool("max_size").then_some(10 * 1024 * 1024),
            multiple: k.bool("multiple"),
            loading: k.bool("loading"),
            disabled: k.bool("disabled"),
        }
    }
}

#[component]
fn Uploads() -> Element {
    let mut files = use_signal(Vec::<FileMetadata>::new);
    rsx! {
        Stack { gap: SpacingToken::Md,
            Dropzone {
                title: "Trascina i log o clicca per sceglierli".to_string(),
                accept: vec![".log".to_string(), ".txt".to_string(), ".json".to_string()],
                multiple: true,
                on_drop: move |dropped: Vec<FileMetadata>| files.write().extend(dropped),
            }
            for file in files() {
                AttachmentCard {
                    title: file.name.clone(),
                    subtitle: Some(file.format_size()),
                    avatar_icon: Some(IconKind::Terminal.into()),
                }
            }
        }
    }
}
const UPLOADS_CODE: &str = r#"let mut files = use_signal(Vec::<FileMetadata>::new);

Stack { gap: SpacingToken::Md,
    Dropzone {
        title: "Trascina i log o clicca per sceglierli".to_string(),
        accept: vec![".log".to_string(), ".txt".to_string(), ".json".to_string()],
        multiple: true,
        on_drop: move |dropped: Vec<FileMetadata>| files.write().extend(dropped),
    }
    for file in files() {
        AttachmentCard {
            title: file.name.clone(),
            subtitle: Some(file.format_size()),
            avatar_icon: Some(IconKind::Terminal.into()),
        }
    }
}"#;

#[component]
fn States() -> Element {
    rsx! {
        SimpleGrid { cols: 1, sm: Some(2), spacing: GridSpacing::Md,
            Dropzone { title: "Analisi in corso".to_string(), loading: true }
            Dropzone {
                title: "Caricamenti sospesi".to_string(),
                description: Some("Quota di storage esaurita.".to_string()),
                disabled: true,
            }
        }
    }
}
const STATES_CODE: &str = r#"SimpleGrid { cols: 1, sm: Some(2), spacing: GridSpacing::Md,
    Dropzone { title: "Analisi in corso".to_string(), loading: true }
    Dropzone {
        title: "Caricamenti sospesi".to_string(),
        description: Some("Quota di storage esaurita.".to_string()),
        disabled: true,
    }
}"#;

#[component]
fn InContext() -> Element {
    let mut files = use_signal(|| {
        vec![FileMetadata {
            name: "traceroute-edge-07.txt".to_string(),
            size_bytes: 48 * 1024,
            mime_type: Some("text/plain".to_string()),
        }]
    });
    let subject = use_signal(|| "Latenza alta verso Francoforte".to_string());
    let mut toast = use_notification();
    rsx! {
        Card {
            title: Some("Apri un ticket".to_string()),
            subtitle: Some("Allega log e screenshot: il supporto risponde entro 2 ore".to_string()),
            footer: Some(rsx! {
                Group { gap: SpacingToken::Sm, justify: JustifyContent::End,
                    Button {
                        onclick: move |_| {
                            toast.success(format!("Ticket inviato con {} allegati", files().len()));
                        },
                        "Invia ticket"
                    }
                }
            }),
            Stack { gap: SpacingToken::Md,
                TextInput { value: subject, label: Some("Oggetto".to_string()) }
                Dropzone {
                    title: "Allega file".to_string(),
                    accept: vec![".txt".to_string(), ".log".to_string(), ".png".to_string()],
                    max_size_bytes: Some(10 * 1024 * 1024),
                    multiple: true,
                    on_drop: move |dropped: Vec<FileMetadata>| files.write().extend(dropped),
                }
                for (i, file) in files().into_iter().enumerate() {
                    AttachmentCard {
                        title: file.name.clone(),
                        subtitle: Some(file.format_size()),
                        avatar_icon: Some(IconKind::Paperclip.into()),
                        buttons: rsx! {
                            Button {
                                size: ButtonSize::Sm,
                                variant: ButtonVariant::Ghost,
                                onclick: move |_| {
                                    files.write().remove(i);
                                },
                                "Rimuovi"
                            }
                        },
                    }
                }
            }
        }
    }
}
const IN_CONTEXT_CODE: &str = r#"let mut files = use_signal(|| {
    vec![FileMetadata {
        name: "traceroute-edge-07.txt".to_string(),
        size_bytes: 48 * 1024,
        mime_type: Some("text/plain".to_string()),
    }]
});
let subject = use_signal(|| "Latenza alta verso Francoforte".to_string());
let mut toast = use_notification();

Card {
    title: Some("Apri un ticket".to_string()),
    subtitle: Some("Allega log e screenshot: il supporto risponde entro 2 ore".to_string()),
    footer: Some(rsx! {
        Group { gap: SpacingToken::Sm, justify: JustifyContent::End,
            Button {
                onclick: move |_| {
                    toast.success(format!("Ticket inviato con {} allegati", files().len()));
                },
                "Invia ticket"
            }
        }
    }),
    Stack { gap: SpacingToken::Md,
        TextInput { value: subject, label: Some("Oggetto".to_string()) }
        Dropzone {
            title: "Allega file".to_string(),
            accept: vec![".txt".to_string(), ".log".to_string(), ".png".to_string()],
            max_size_bytes: Some(10 * 1024 * 1024),
            multiple: true,
            on_drop: move |dropped: Vec<FileMetadata>| files.write().extend(dropped),
        }
        for (i, file) in files().into_iter().enumerate() {
            AttachmentCard {
                title: file.name.clone(),
                subtitle: Some(file.format_size()),
                avatar_icon: Some(IconKind::Paperclip.into()),
                buttons: rsx! {
                    Button {
                        size: ButtonSize::Sm,
                        variant: ButtonVariant::Ghost,
                        onclick: move |_| {
                            files.write().remove(i);
                        },
                        "Rimuovi"
                    }
                },
            }
        }
    }
}"#;

const PROPS: &[PropDoc] = &[
    prop("title", "String", "\"\"", "Testo principale; da Locale (dropzone_title) se vuoto."),
    prop("description", "Option<String>", "None", "Riga secondaria; senza, mostra i formati accettati e il limite."),
    prop("accept", "Vec<String>", "[]", "Estensioni o tipi MIME (es. \".pdf\", \"image/*\")."),
    prop("max_size_bytes", "Option<u64>", "None", "Dimensione massima mostrata accanto ai formati."),
    prop("multiple", "bool", "false", "Più file in una volta."),
    prop("loading", "bool", "false", "Spinner e aria-busy; i nuovi file sono ignorati."),
    prop("disabled", "bool", "false", "Area inattiva."),
    prop("on_drop", "Option<EventHandler<Vec<FileMetadata>>>", "None", "File rilasciati o scelti: nome, dimensione, tipo MIME."),
    prop("class", "String", "\"\"", "Classi aggiuntive."),
];
const FILE_PROPS: &[PropDoc] = &[
    prop("name", "String", "-", "Nome del file."),
    prop("size_bytes", "u64", "-", "Dimensione in byte."),
    prop("mime_type", "Option<String>", "None", "Tipo MIME dichiarato dal browser."),
    prop(".format_size()", "fn -> String", "-", "Dimensione leggibile: B, KB, MB o GB."),
];

pub fn story() -> Story {
    Story {
        slug: "dropzone",
        name: "Dropzone",
        category: Category::Form,
        features: &["dropzone"],
        icon: IconKind::Upload,
        tagline: "Area per trascinare o scegliere file, con formati ammessi e limite di dimensione.",
        keywords: &["dropzone", "upload", "file", "trascina", "allegati", "drag and drop"],
        status: Status::Stable,
        playground: Some(Playground {
            component: "Dropzone",
            controls: vec![
                Control::text("title", "Titolo", "Trascina qui i file del report")
                    .emit(Emit::Template("title: \"{v}\".to_string()")),
                Control::text("description", "Descrizione", "")
                    .emit(Emit::Template("description: Some(\"{v}\".to_string())")),
                Control::bool("accept", "Solo PDF e immagini", true)
                    .emit(Emit::Template("accept: vec![\".pdf\".to_string(), \".png\".to_string(), \".jpg\".to_string()]")),
                Control::bool("max_size", "Limite 10 MB", true)
                    .emit(Emit::Template("max_size_bytes: Some(10 * 1024 * 1024)")),
                Control::bool("multiple", "Più file", true).emit(Emit::Template("multiple: true")),
                Control::bool("loading", "Loading", false),
                Control::bool("disabled", "Disabilitato", false),
            ],
            render,
            prelude_code: None,
            fixed_code: &[],
            children_code: None,
            stage: Stage::Form,
        }),
        examples: vec![
            Example {
                id: "uploads",
                title: "Elenco dei file",
                description: "on_drop aggiunge ogni file a una lista di AttachmentCard.",
                code: UPLOADS_CODE,
                render: || rsx! { Uploads {} },
                span: Span::Half,
            },
            Example {
                id: "states",
                title: "Stati",
                description: "Caricamento in corso e area disabilitata.",
                code: STATES_CODE,
                render: || rsx! { States {} },
                span: Span::Half,
            },
            Example {
                id: "context",
                title: "In contesto",
                description: "Un ticket di supporto con allegati rimovibili.",
                code: IN_CONTEXT_CODE,
                render: || rsx! { InContext {} },
                span: Span::Full,
            },
        ],
        props: vec![
            PropGroup { component: "Dropzone", rows: PROPS },
            PropGroup { component: "FileMetadata", rows: FILE_PROPS },
        ],
        recipe: Some(Recipe {
            recipe: "Recessed field (§3.6), bordo tratteggiato",
            depth: "Well",
            tokens: &["--sd-well", "--sd-border-strong", "--sd-accent", "--sd-accent-soft", "--sd-danger-border", "--sd-radius-lg", "--sd-text-subtle"],
            notes: "Durante il trascinamento il bordo passa all'accento; un file rifiutato usa i token danger.",
        }),
        a11y: &[
            "Dentro c'è un input file nativo con aria-label uguale al titolo: si raggiunge con Tab e si apre con Invio o Spazio.",
            "accept filtra la finestra di scelta del sistema operativo.",
            "loading imposta aria-busy; disabled imposta aria-disabled e disabilita l'input.",
            "Le icone di stato sono decorative.",
        ],
        related: &["attachment-cards", "document-card", "form", "progress"],
    }
}
