//! Story: AttachmentCard and ContactCard (features `attachment_card`, `contact_card`).
//! The playground goes through `AttachmentDemo { knobs }` because the actions raise toasts.
use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::story::schema::*;

const ICONS: [Option<IconKind>; 4] = [None, Some(IconKind::Lock), Some(IconKind::Terminal), Some(IconKind::Server)];
const ICON_OPTS: &[ChoiceOption] =
    &[opt("Paperclip", "Paperclip"), opt("Lock", "Lock"), opt("Terminal", "Terminal"), opt("Server", "Server")];
const PROGRESS: [Option<f32>; 4] = [None, Some(24.0), Some(64.0), Some(100.0)];
const PROGRESS_OPTS: &[ChoiceOption] =
    &[opt("Nessuno", "None"), opt("24%", "Some(24.0)"), opt("64%", "Some(64.0)"), opt("100%", "Some(100.0)")];

const PRELUDE: &str = "let mut toast = use_notification();";

#[component]
fn AttachmentDemo(knobs: Knobs) -> Element {
    let mut toast = use_notification();
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
    rsx! {
        AttachmentCard {
            title: knobs.text("title"),
            subtitle: Some(knobs.text("subtitle")).filter(|s| !s.is_empty()),
            description: Some(knobs.text("description")).filter(|s| !s.is_empty()),
            avatar_icon: knobs.pick("avatar_icon", &ICONS).map(Into::into),
            progress: knobs.pick("progress", &PROGRESS),
            on_download: download,
            on_share: share,
        }
    }
}

fn render(k: &Knobs) -> Element {
    rsx! {
        AttachmentDemo { knobs: k.clone() }
    }
}

#[component]
fn Transfers() -> Element {
    rsx! {
        Stack { gap: SpacingToken::Sm,
            AttachmentCard {
                title: "snapshot-ledger.enc".to_string(),
                subtitle: Some("14,8 MB · in caricamento".to_string()),
                avatar_icon: Some(IconKind::Upload.into()),
                progress: Some(42.0),
            }
            AttachmentCard {
                title: "chiavi-kyber.pem".to_string(),
                subtitle: Some("4 KB · cifrato".to_string()),
                avatar_icon: Some(IconKind::Lock.into()),
                progress: Some(100.0),
            }
        }
    }
}
const TRANSFERS_CODE: &str = r#"Stack { gap: SpacingToken::Sm,
    AttachmentCard {
        title: "snapshot-ledger.enc".to_string(),
        subtitle: Some("14,8 MB · in caricamento".to_string()),
        avatar_icon: Some(IconKind::Upload.into()),
        progress: Some(42.0),
    }
    AttachmentCard {
        title: "chiavi-kyber.pem".to_string(),
        subtitle: Some("4 KB · cifrato".to_string()),
        avatar_icon: Some(IconKind::Lock.into()),
        progress: Some(100.0),
    }
}"#;

#[component]
fn Contact() -> Element {
    let mut toast = use_notification();
    rsx! {
        ContactCard {
            name: "Giulia Ferri".to_string(),
            subtitle: Some("Ops lead · Milano".to_string()),
            description: Some("Reperibile per incidenti P1 fino alle 20:00.".to_string()),
            avatar: rsx! {
                Avatar { name: Some("Giulia Ferri".to_string()), status: Some(PresenceStatus::Online) }
            },
            on_click: move |_| {
                toast.info("Apro la conversazione con Giulia");
            },
        }
    }
}
const CONTACT_CODE: &str = r#"let mut toast = use_notification();

ContactCard {
    name: "Giulia Ferri".to_string(),
    subtitle: Some("Ops lead · Milano".to_string()),
    description: Some("Reperibile per incidenti P1 fino alle 20:00.".to_string()),
    avatar: rsx! {
        Avatar { name: Some("Giulia Ferri".to_string()), status: Some(PresenceStatus::Online) }
    },
    on_click: move |_| {
        toast.info("Apro la conversazione con Giulia");
    },
}"#;

#[component]
fn InContext() -> Element {
    let mut toast = use_notification();
    rsx! {
        Card {
            title: Some("Incidente #SF-104 · latenza edge-eu".to_string()),
            subtitle: Some("Conversazione con il supporto".to_string()),
            Stack { gap: SpacingToken::Sm,
                MessageBubble { sender_name: Some("Giulia Ferri".to_string()), timestamp: Some("11:02".to_string()),
                    "Ecco il traceroute e il contatto del provider."
                }
                MessageBubble { timestamp: Some("11:02".to_string()),
                    AttachmentCard {
                        title: "traceroute-edge-07.txt".to_string(),
                        subtitle: Some("48 KB · testo".to_string()),
                        avatar_icon: Some(IconKind::Terminal.into()),
                        on_download: move |_| {
                            toast.info("Download avviato");
                        },
                    }
                }
                MessageBubble { timestamp: Some("11:03".to_string()),
                    ContactCard {
                        name: "NOC Francoforte".to_string(),
                        subtitle: Some("Provider · 24/7".to_string()),
                        avatar: rsx! {
                            Avatar { icon: Some(IconKind::Server.into()), alt: Some("NOC Francoforte".to_string()) }
                        },
                    }
                }
                MessageBubble { direction: BubbleDirection::Outgoing, timestamp: Some("11:05".to_string()), status: Some(DeliveryStatus::Delivered),
                    "Ricevuto, apro una segnalazione."
                }
            }
        }
    }
}
const IN_CONTEXT_CODE: &str = r#"let mut toast = use_notification();

Card {
    title: Some("Incidente #SF-104 · latenza edge-eu".to_string()),
    subtitle: Some("Conversazione con il supporto".to_string()),
    Stack { gap: SpacingToken::Sm,
        MessageBubble { sender_name: Some("Giulia Ferri".to_string()), timestamp: Some("11:02".to_string()),
            "Ecco il traceroute e il contatto del provider."
        }
        MessageBubble { timestamp: Some("11:02".to_string()),
            AttachmentCard {
                title: "traceroute-edge-07.txt".to_string(),
                subtitle: Some("48 KB · testo".to_string()),
                avatar_icon: Some(IconKind::Terminal.into()),
                on_download: move |_| {
                    toast.info("Download avviato");
                },
            }
        }
        MessageBubble { timestamp: Some("11:03".to_string()),
            ContactCard {
                name: "NOC Francoforte".to_string(),
                subtitle: Some("Provider · 24/7".to_string()),
                avatar: rsx! {
                    Avatar { icon: Some(IconKind::Server.into()), alt: Some("NOC Francoforte".to_string()) }
                },
            }
        }
        MessageBubble { direction: BubbleDirection::Outgoing, timestamp: Some("11:05".to_string()), status: Some(DeliveryStatus::Delivered),
            "Ricevuto, apro una segnalazione."
        }
    }
}"#;

const ATTACHMENT_PROPS: &[PropDoc] = &[
    prop("title", "String", "-", "Nome del file o titolo."),
    prop("subtitle / description", "Option<String>", "None", "Metadati (dimensione, tipo) e testo sotto."),
    prop("avatar_icon", "Option<SvgIcon>", "None", "Icona nel cerchio; Paperclip se assente."),
    prop("avatar_src / avatar", "Option<String> / Option<Element>", "None", "Miniatura immagine o elemento libero al posto dell'icona."),
    prop("image / image_alt", "Option<String>", "None", "Anteprima grande in testa alla scheda."),
    prop("progress", "Option<f32>", "None", "Barra di trasferimento da 0 a 100."),
    prop("on_download / on_share", "Option<EventHandler<MouseEvent>>", "None", "Pulsanti Scarica e Condividi con testi da Locale."),
    prop("buttons", "Option<Element>", "None", "Azioni personalizzate accanto a quelle standard."),
    prop("on_click", "Option<EventHandler<MouseEvent>>", "None", "Rende la scheda attivabile."),
    prop("children / class", "Element / String", "-", "Contenuto extra e classi aggiuntive."),
];
const CONTACT_PROPS: &[PropDoc] = &[
    prop("name", "String", "-", "Nome del contatto."),
    prop("avatar", "Option<Element>", "None", "Avatar; di default le iniziali del nome."),
    prop("subtitle / description", "Option<String>", "None", "Ruolo e nota."),
    prop("on_click", "Option<EventHandler<MouseEvent>>", "None", "Rende la scheda attivabile."),
    prop("children / class", "Element / String", "-", "Contenuto extra e classi aggiuntive."),
];

pub fn story() -> Story {
    Story {
        slug: "attachment-cards",
        name: "AttachmentCard",
        category: Category::Contenuti,
        features: &["attachment_card", "contact_card"],
        icon: IconKind::Paperclip,
        tagline: "Schede compatte per file allegati e contatti, con avanzamento e azioni.",
        keywords: &["attachment", "allegato", "file", "contatto", "contact card", "download"],
        status: Status::Stable,
        playground: Some(Playground {
            component: "AttachmentCard",
            controls: vec![
                Control::text("title", "Titolo", "report-latenza-q3.pdf").emit(Emit::Template("title: \"{v}\".to_string()")),
                Control::text("subtitle", "Sottotitolo", "2,4 MB · PDF").emit(Emit::Template("subtitle: Some(\"{v}\".to_string())")),
                Control::text("description", "Descrizione", "")
                    .emit(Emit::Template("description: Some(\"{v}\".to_string())")),
                Control::choice("avatar_icon", "Icona", ICON_OPTS, 0).emit(Emit::Template("avatar_icon: Some(IconKind::{v}.into())")),
                Control::choice("progress", "Avanzamento", PROGRESS_OPTS, 0).emit(Emit::Template("progress: {v}")),
                Control::bool("download", "Scarica", true)
                    .emit(Emit::Template("on_download: move |_| { toast.info(\"Download avviato\"); }")),
                Control::bool("share", "Condividi", false).emit(Emit::Template("on_share: move |_| { toast.info(\"Link copiato\"); }")),
            ],
            render,
            prelude_code: Some(PRELUDE),
            fixed_code: &[],
            children_code: None,
            stage: Stage::Form,
        }),
        examples: vec![
            Example {
                id: "transfers",
                title: "Trasferimenti",
                description: "progress mostra l'avanzamento del caricamento.",
                code: TRANSFERS_CODE,
                render: || rsx! { Transfers {} },
                span: Span::Half,
            },
            Example {
                id: "contact",
                title: "ContactCard",
                description: "Scheda contatto attivabile con avatar e presenza.",
                code: CONTACT_CODE,
                render: || rsx! { Contact {} },
                span: Span::Half,
            },
            Example {
                id: "context",
                title: "In contesto",
                description: "Allegati e contatti dentro una conversazione di supporto.",
                code: IN_CONTEXT_CODE,
                render: || rsx! { InContext {} },
                span: Span::Full,
            },
        ],
        props: vec![
            PropGroup { component: "AttachmentCard", rows: ATTACHMENT_PROPS },
            PropGroup { component: "ContactCard", rows: CONTACT_PROPS },
        ],
        recipe: Some(Recipe {
            recipe: "Container (§3.1), interattivo con on_click",
            depth: "Flat-lit",
            tokens: &["--sd-surface-1", "--sd-border", "--sd-specular", "--sd-elev-1", "--sd-radius-lg", "--sd-radius-full", "--sd-accent"],
            notes: "È una Card della libreria: statica a riposo, bordo hover solo se cliccabile; l'avanzamento è un value fill accento.",
        }),
        a11y: &[
            "Con on_click la scheda ha role=button, tabindex=0 e si attiva da tastiera (Card).",
            "Scarica e Condividi sono Button con testo da Locale (download, share).",
            "La barra di avanzamento è un ProgressBar della libreria.",
            "L'icona nel cerchio è decorativa: il nome del file è il titolo della scheda.",
        ],
        related: &["document-card", "dropzone", "chat", "media-card"],
    }
}
