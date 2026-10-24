//! Story: ActivityFeed and ActivityItem (feature `activity_feed`). The playground render is
//! stateless; the "In contesto" example stores replies in a signal.
use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::story::schema::*;

fn events() -> Vec<ActivityEvent> {
    let giulia = PersonaProps::new("Giulia Ferri").with_status(PresenceStatus::Online);
    let marco = PersonaProps::new("Marco Bellini");
    vec![
        ActivityEvent::new("e1", giulia.clone(), "ha distribuito", "2 min fa")
            .with_icon(IconKind::Upload)
            .with_target("v0.5.3 su edge-eu")
            .with_description("12 nodi aggiornati, nessun errore.")
            .with_unread(true),
        ActivityEvent::new("e2", marco.clone(), "ha commentato", "1 ora fa")
            .with_icon(IconKind::MessageSquare)
            .with_target("Latenza Francoforte")
            .with_comments(vec![ActivityComment::new(giulia, "Il relay di Milano copre il picco.", "45 min fa")]),
        ActivityEvent::new("e3", marco, "ha ruotato le chiavi", "ieri").with_icon(IconKind::Key).with_target("cluster edge-asia"),
    ]
}

const PRELUDE: &str = r#"let giulia = PersonaProps::new("Giulia Ferri").with_status(PresenceStatus::Online);
let marco = PersonaProps::new("Marco Bellini");
let items = vec![
    ActivityEvent::new("e1", giulia.clone(), "ha distribuito", "2 min fa")
        .with_icon(IconKind::Upload)
        .with_target("v0.5.3 su edge-eu")
        .with_description("12 nodi aggiornati, nessun errore.")
        .with_unread(true),
    ActivityEvent::new("e2", marco.clone(), "ha commentato", "1 ora fa")
        .with_icon(IconKind::MessageSquare)
        .with_target("Latenza Francoforte")
        .with_comments(vec![ActivityComment::new(giulia, "Il relay di Milano copre il picco.", "45 min fa")]),
    ActivityEvent::new("e3", marco, "ha ruotato le chiavi", "ieri").with_icon(IconKind::Key).with_target("cluster edge-asia"),
];"#;

fn render(k: &Knobs) -> Element {
    rsx! {
        ActivityFeed { items: events(), compact: k.bool("compact"), allow_replies: k.bool("allow_replies") }
    }
}

#[component]
fn Compact() -> Element {
    rsx! {
        ActivityFeed {
            items: vec![
                ActivityEvent::new("c1", PersonaProps::new("Sara Conti"), "ha aperto", "5 min fa")
                    .with_icon(IconKind::AlertTriangle)
                    .with_target("incidente #42"),
                ActivityEvent::new("c2", PersonaProps::new("Luca Moretti"), "ha chiuso", "20 min fa")
                    .with_icon(IconKind::CheckCircle)
                    .with_target("incidente #41"),
            ],
            compact: true,
            allow_replies: false,
        }
    }
}
const COMPACT_CODE: &str = r#"ActivityFeed {
    items: vec![
        ActivityEvent::new("c1", PersonaProps::new("Sara Conti"), "ha aperto", "5 min fa")
            .with_icon(IconKind::AlertTriangle)
            .with_target("incidente #42"),
        ActivityEvent::new("c2", PersonaProps::new("Luca Moretti"), "ha chiuso", "20 min fa")
            .with_icon(IconKind::CheckCircle)
            .with_target("incidente #41"),
    ],
    compact: true,
    allow_replies: false,
}"#;

#[component]
fn Empty() -> Element {
    rsx! {
        ActivityFeed { items: Vec::new() }
    }
}
const EMPTY_CODE: &str = "ActivityFeed { items: Vec::new() }";

#[component]
fn InContext() -> Element {
    let me = PersonaProps::new("Tu").with_status(PresenceStatus::Online);
    // events(): gli stessi eventi del playground.
    let mut items = use_signal(events);
    let mut toast = use_notification();
    rsx! {
        Card {
            title: Some("Attività del cluster".to_string()),
            subtitle: Some("edge-eu · ultime 24 ore".to_string()),
            ActivityFeed {
                items: items(),
                on_reply: move |(id, text): (String, String)| {
                    if let Some(ev) = items.write().iter_mut().find(|e| e.id == id) {
                        ev.comments.push(ActivityComment::new(me.clone(), text, "ora"));
                    }
                    toast.success("Risposta pubblicata");
                },
            }
        }
    }
}
const IN_CONTEXT_CODE: &str = r#"let me = PersonaProps::new("Tu").with_status(PresenceStatus::Online);
// events(): gli stessi eventi del playground.
let mut items = use_signal(events);
let mut toast = use_notification();

Card {
    title: Some("Attività del cluster".to_string()),
    subtitle: Some("edge-eu · ultime 24 ore".to_string()),
    ActivityFeed {
        items: items(),
        on_reply: move |(id, text): (String, String)| {
            if let Some(ev) = items.write().iter_mut().find(|e| e.id == id) {
                ev.comments.push(ActivityComment::new(me.clone(), text, "ora"));
            }
            toast.success("Risposta pubblicata");
        },
    }
}"#;

const FEED_PROPS: &[PropDoc] = &[
    prop("items", "Vec<ActivityEvent>", "-", "Eventi in ordine di visualizzazione; vuoto mostra il testo di Locale."),
    prop("compact", "bool", "false", "Avatar Sm e spaziatura ridotta."),
    prop("allow_replies", "bool", "true", "Pulsante Rispondi con campo di risposta inline."),
    prop("on_reply", "Option<EventHandler<(String, String)>>", "None", "Id dell'evento e testo della risposta."),
    prop("class", "String", "\"\"", "Classi aggiuntive."),
];
const EVENT_PROPS: &[PropDoc] = &[
    prop("ActivityEvent::new(id, author, action, timestamp)", "fn", "-", "Evento con autore PersonaProps."),
    prop(".with_icon(IconKind)", "fn", "Sparkles", "Simbolo sull'avatar."),
    prop(".with_target(..) / .with_description(..)", "fn", "-", "Oggetto dell'azione e riga di dettaglio."),
    prop(".with_comments(Vec<ActivityComment>)", "fn", "[]", "Commenti annidati: ActivityComment::new(author, text, timestamp)."),
    prop(".with_unread(bool)", "fn", "false", "Evidenzia l'evento come non letto."),
];
const ITEM_PROPS: &[PropDoc] = &[
    prop("event", "ActivityEvent", "-", "Singolo evento, per feed costruiti a mano."),
    prop("is_last", "bool", "false", "Nasconde il connettore verticale."),
    prop("compact / allow_replies / on_reply", "-", "-", "Come ActivityFeed."),
];

pub fn story() -> Story {
    Story {
        slug: "activity-feed",
        name: "ActivityFeed",
        category: Category::Contenuti,
        features: &["activity_feed"],
        icon: IconKind::Clock,
        tagline: "Cronologia di chi ha fatto cosa, con commenti annidati e risposte inline.",
        keywords: &["activity feed", "attività", "cronologia", "eventi", "commenti", "timeline"],
        status: Status::Stable,
        playground: Some(Playground {
            component: "ActivityFeed",
            controls: vec![Control::bool("compact", "Compatto", false), Control::bool("allow_replies", "Risposte", true)],
            render,
            prelude_code: Some(PRELUDE),
            fixed_code: &["items,"],
            children_code: None,
            stage: Stage::Start,
        }),
        examples: vec![
            Example {
                id: "compact",
                title: "Compatto, senza risposte",
                description: "Per una colonna laterale o un pannello stretto.",
                code: COMPACT_CODE,
                render: || rsx! { Compact {} },
                span: Span::Half,
            },
            Example {
                id: "empty",
                title: "Feed vuoto",
                description: "Il testo dello stato vuoto arriva da Locale.",
                code: EMPTY_CODE,
                render: || rsx! { Empty {} },
                span: Span::Half,
            },
            Example {
                id: "context",
                title: "In contesto",
                description: "Il feed di un cluster: rispondi a un evento e il commento compare sotto.",
                code: IN_CONTEXT_CODE,
                render: || rsx! { InContext {} },
                span: Span::Full,
            },
        ],
        props: vec![
            PropGroup { component: "ActivityFeed", rows: FEED_PROPS },
            PropGroup { component: "ActivityEvent", rows: EVENT_PROPS },
            PropGroup { component: "ActivityItem", rows: ITEM_PROPS },
        ],
        recipe: Some(Recipe {
            recipe: "Container (§3.1) · text levels (§3.12)",
            depth: "Void o Flat-lit (dentro una Card)",
            tokens: &["--sd-border-subtle", "--sd-text", "--sd-text-muted", "--sd-text-subtle", "--sd-accent-soft", "--sd-font-mono"],
            notes: "Il connettore è una linea --sd-border-subtle; solo gli eventi non letti prendono la tinta accento.",
        }),
        a11y: &[
            "Rispondi, Annulla e Invia sono Button della libreria con testi da Locale.",
            "Il campo di risposta è una Textarea con segnaposto da Locale (activity_reply_placeholder).",
            "Autore, azione, oggetto e ora sono testo letto in ordine; l'icona dell'evento è decorativa.",
        ],
        related: &["timelines", "avatars", "chat", "facepile"],
    }
}
