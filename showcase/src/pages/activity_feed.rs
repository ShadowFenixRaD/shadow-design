use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::components::{FeatureBanner, PageHeader, PropRow, ShowcaseTabs, Stage};

#[component]
pub fn ActivityFeedPage() -> Element {
    let mut feed_events = use_signal(|| vec![
        ActivityEvent::new(
            "1",
            PersonaProps::new("Christian").with_status(PresenceStatus::Online),
            "ha rilasciato un nuovo commit nel branch",
            "10 minuti fa",
        )
        .with_target("feat/charts-and-nav")
        .with_icon(IconKind::Code)
        .with_description("Aggiunta suite completa di grafici vettoriali SVG ad alte prestazioni e MegaMenu.")
        .with_comments(vec![
            ActivityComment::new(PersonaProps::new("Elena"), "I grafici SVG e il glow violet sono fantastici!", "8 minuti fa"),
        ]),
        ActivityEvent::new(
            "2",
            PersonaProps::new("Marco").with_status(PresenceStatus::DirectP2P),
            "ha trasferito 4 componenti verso la colonna",
            "35 minuti fa",
        )
        .with_target("Completati")
        .with_icon(IconKind::CheckCircle)
        .with_description("Verificati con successo tutti i test unitari del workspace."),
        ActivityEvent::new(
            "3",
            PersonaProps::new("ShadowfenixRaD").with_status(PresenceStatus::Online),
            "ha configurato la connessione SSH isolata",
            "2 ore fa",
        )
        .with_icon(IconKind::ShieldCheck)
        .with_target("source.shadowfenix.net"),
    ]);

    let props = vec![
        PropRow {
            name: "items",
            r#type: "Vec<ActivityEvent>",
            default: "-",
            description: "Elenco cronologico di eventi collaborativi con avatar, icone e commenti",
        },
        PropRow {
            name: "compact",
            r#type: "bool",
            default: "false",
            description: "Modalità ad alta densità con avatar più piccoli e spaziatura ridotta",
        },
        PropRow {
            name: "allow_replies",
            r#type: "bool",
            default: "true",
            description: "Abilita il pulsante e l'area di inserimento per commenti inline",
        },
        PropRow {
            name: "on_reply",
            r#type: "Option<EventHandler<(String, String)>>",
            default: "None",
            description: "Callback scatenata quando l'utente invia una risposta (event_id, text)",
        },
    ];

    let code_snippet = r#"ActivityFeed {
    items: events,
    allow_replies: true,
    on_reply: move |(id, text)| {
        println!("Nuovo commento all'evento {id}: {text}");
    },
}"#
    .to_string();

    rsx! {
        div { class: "sc-stack sc-stack--loose",
            PageHeader {
                title: "ActivityFeed / ActivityItem".to_string(),
                description: "Feed e timeline di attività collaborative con avatar autore, badge icona evento, connettori continui e supporto per risposte e commenti inline.".to_string(),
            }

            FeatureBanner { feature_name: "activity_feed" }

            ShowcaseTabs {
                preview: rsx! {
                    Stage { start: true,
                        ActivityFeed {
                            items: feed_events(),
                            allow_replies: true,
                            on_reply: move |(event_id, reply_text): (String, String)| {
                                let mut events = feed_events();
                                if let Some(ev) = events.iter_mut().find(|e| e.id == event_id) {
                                    ev.comments.push(ActivityComment::new(
                                        PersonaProps::new("Tu").with_status(PresenceStatus::Online),
                                        reply_text,
                                        "Adesso",
                                    ));
                                }
                                feed_events.set(events);
                            },
                        }
                    }
                },
                code: code_snippet,
                props: props,
            }
        }
    }
}
