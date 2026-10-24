//! Story: Timeline and TimelineItem (feature `timeline`). Stateless, but items read the shared
//! settings once from the context, so the playground remounts the timeline when a knob changes.
use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::story::schema::*;

const ALIGNS: [TimelineAlign; 2] = [TimelineAlign::Left, TimelineAlign::Right];
const ALIGN_OPTS: &[ChoiceOption] = &[opt("Left", "TimelineAlign::Left"), opt("Right", "TimelineAlign::Right")];
const LINES: [TimelineLineVariant; 2] = [TimelineLineVariant::Solid, TimelineLineVariant::Dashed];
const LINE_OPTS: &[ChoiceOption] = &[opt("Solid", "TimelineLineVariant::Solid"), opt("Dashed", "TimelineLineVariant::Dashed")];

const ITEMS_CODE: &str = r#"TimelineItem { title: Some("Build".to_string()), time: Some("09:02".to_string()), state: Some(TimelineBulletState::Completed),
    "wasm32 e desktop compilati in 3 min 12 s."
}
TimelineItem { title: Some("Test".to_string()), time: Some("09:06".to_string()), state: Some(TimelineBulletState::Completed),
    "214 test passati, nessun avviso di clippy."
}
TimelineItem { title: Some("Canary su Milano".to_string()), time: Some("09:10".to_string()), state: Some(TimelineBulletState::Active),
    "Il 5% del traffico passa su edge-22."
}
TimelineItem { title: Some("Rollout completo".to_string()), time: Some("in attesa".to_string()), state: Some(TimelineBulletState::Pending),
    "Parte quando il canary resta verde per un'ora."
}"#;

fn render(k: &Knobs) -> Element {
    let bullet_size = k.number("bullet_size") as u32;
    let line_width = k.number("line_width") as u32;
    // Items read bullet size, line width and line style once from the context: remount on change.
    let remount = format!("{}-{bullet_size}-{line_width}-{}", k.index("align"), k.index("line_variant"));
    rsx! {
        for key in [remount] {
            Timeline {
                key: "{key}",
                align: k.pick("align", &ALIGNS),
                bullet_size,
                line_width,
                line_variant: k.pick("line_variant", &LINES),
                TimelineItem { title: Some("Build".to_string()), time: Some("09:02".to_string()), state: Some(TimelineBulletState::Completed),
                    "wasm32 e desktop compilati in 3 min 12 s."
                }
                TimelineItem { title: Some("Test".to_string()), time: Some("09:06".to_string()), state: Some(TimelineBulletState::Completed),
                    "214 test passati, nessun avviso di clippy."
                }
                TimelineItem { title: Some("Canary su Milano".to_string()), time: Some("09:10".to_string()), state: Some(TimelineBulletState::Active),
                    "Il 5% del traffico passa su edge-22."
                }
                TimelineItem { title: Some("Rollout completo".to_string()), time: Some("in attesa".to_string()), state: Some(TimelineBulletState::Pending),
                    "Parte quando il canary resta verde per un'ora."
                }
            }
        }
    }
}

#[component]
fn States() -> Element {
    rsx! {
        Timeline {
            TimelineItem { title: Some("Default".to_string()), "Evento registrato." }
            TimelineItem { title: Some("Completed".to_string()), state: Some(TimelineBulletState::Completed), "Passo concluso." }
            TimelineItem { title: Some("Active".to_string()), state: Some(TimelineBulletState::Active), "Passo in corso." }
            TimelineItem { title: Some("Pending".to_string()), state: Some(TimelineBulletState::Pending), "In coda." }
            TimelineItem { title: Some("Error".to_string()), state: Some(TimelineBulletState::Error), "Passo fallito." }
        }
    }
}
const STATES_CODE: &str = r#"Timeline {
    TimelineItem { title: Some("Default".to_string()), "Evento registrato." }
    TimelineItem { title: Some("Completed".to_string()), state: Some(TimelineBulletState::Completed), "Passo concluso." }
    TimelineItem { title: Some("Active".to_string()), state: Some(TimelineBulletState::Active), "Passo in corso." }
    TimelineItem { title: Some("Pending".to_string()), state: Some(TimelineBulletState::Pending), "In coda." }
    TimelineItem { title: Some("Error".to_string()), state: Some(TimelineBulletState::Error), "Passo fallito." }
}"#;

#[component]
fn IconBullets() -> Element {
    rsx! {
        Timeline { bullet_size: 28,
            TimelineItem {
                title: Some("Chiave caricata".to_string()),
                time: Some("ieri".to_string()),
                bullet: Some(rsx! {
                    Icon { kind: IconKind::Key, size: 14 }
                }),
                "Sara Conti ha aggiunto una chiave Kyber-1024."
            }
            TimelineItem {
                title: Some("Firma verificata".to_string()),
                time: Some("ieri".to_string()),
                bullet: Some(rsx! {
                    Icon { kind: IconKind::ShieldCheck, size: 14 }
                }),
                "Tutti i 24 nodi accettano la nuova firma."
            }
            TimelineItem {
                title: Some("Chiave precedente revocata".to_string()),
                time: Some("oggi".to_string()),
                bullet: Some(rsx! {
                    Icon { kind: IconKind::Lock, size: 14 }
                }),
                "Le sessioni firmate con la vecchia chiave sono chiuse."
            }
        }
    }
}
const ICON_BULLETS_CODE: &str = r#"Timeline { bullet_size: 28,
    TimelineItem {
        title: Some("Chiave caricata".to_string()),
        time: Some("ieri".to_string()),
        bullet: Some(rsx! {
            Icon { kind: IconKind::Key, size: 14 }
        }),
        "Sara Conti ha aggiunto una chiave Kyber-1024."
    }
    TimelineItem {
        title: Some("Firma verificata".to_string()),
        time: Some("ieri".to_string()),
        bullet: Some(rsx! {
            Icon { kind: IconKind::ShieldCheck, size: 14 }
        }),
        "Tutti i 24 nodi accettano la nuova firma."
    }
    TimelineItem {
        title: Some("Chiave precedente revocata".to_string()),
        time: Some("oggi".to_string()),
        bullet: Some(rsx! {
            Icon { kind: IconKind::Lock, size: 14 }
        }),
        "Le sessioni firmate con la vecchia chiave sono chiuse."
    }
}"#;

#[component]
fn InContext() -> Element {
    rsx! {
        Card {
            title: Some("Incidente INC-2041".to_string()),
            subtitle: Some("relay-03 · Parigi · aperto da 42 minuti".to_string()),
            buttons: Some(rsx! {
                Badge { label: "In corso".to_string(), variant: BadgeVariant::Warning, pill: true, dot: true }
            }),
            Timeline {
                TimelineItem { title: Some("Allarme latenza".to_string()), time: Some("08:10".to_string()), state: Some(TimelineBulletState::Error),
                    "La p95 supera la soglia di 3 ms sulla tratta verso Parigi."
                }
                TimelineItem { title: Some("Presa in carico".to_string()), time: Some("08:14".to_string()), state: Some(TimelineBulletState::Completed),
                    Group { gap: SpacingToken::Xs,
                        Avatar { name: Some("Marco Bellini".to_string()), size: AvatarSize::Xs }
                        Text { size: TextSize::Sm, dimmed: true, "Marco Bellini è il responsabile." }
                    }
                }
                TimelineItem { title: Some("Traffico reindirizzato".to_string()), time: Some("08:31".to_string()), state: Some(TimelineBulletState::Active),
                    "edge-12 assorbe il 60% delle richieste; latenza scesa a 1.9 ms."
                }
                TimelineItem {
                    title: Some("Post-mortem".to_string()),
                    time: Some("entro domani".to_string()),
                    state: Some(TimelineBulletState::Pending),
                    line_variant: Some(TimelineLineVariant::Dashed),
                    "Bozza da completare con causa e azioni correttive."
                }
            }
        }
    }
}
const IN_CONTEXT_CODE: &str = r#"Card {
    title: Some("Incidente INC-2041".to_string()),
    subtitle: Some("relay-03 · Parigi · aperto da 42 minuti".to_string()),
    buttons: Some(rsx! {
        Badge { label: "In corso".to_string(), variant: BadgeVariant::Warning, pill: true, dot: true }
    }),
    Timeline {
        TimelineItem { title: Some("Allarme latenza".to_string()), time: Some("08:10".to_string()), state: Some(TimelineBulletState::Error),
            "La p95 supera la soglia di 3 ms sulla tratta verso Parigi."
        }
        TimelineItem { title: Some("Presa in carico".to_string()), time: Some("08:14".to_string()), state: Some(TimelineBulletState::Completed),
            Group { gap: SpacingToken::Xs,
                Avatar { name: Some("Marco Bellini".to_string()), size: AvatarSize::Xs }
                Text { size: TextSize::Sm, dimmed: true, "Marco Bellini è il responsabile." }
            }
        }
        TimelineItem { title: Some("Traffico reindirizzato".to_string()), time: Some("08:31".to_string()), state: Some(TimelineBulletState::Active),
            "edge-12 assorbe il 60% delle richieste; latenza scesa a 1.9 ms."
        }
        TimelineItem {
            title: Some("Post-mortem".to_string()),
            time: Some("entro domani".to_string()),
            state: Some(TimelineBulletState::Pending),
            line_variant: Some(TimelineLineVariant::Dashed),
            "Bozza da completare con causa e azioni correttive."
        }
    }
}"#;

const TIMELINE_PROPS: &[PropDoc] = &[
    prop("align", "TimelineAlign", "Left", "Linea e bullet a sinistra o a destra del contenuto."),
    prop("bullet_size", "u32", "24", "Diametro dei bullet in px."),
    prop("line_width", "u32", "2", "Spessore della linea in px."),
    prop("line_variant", "TimelineLineVariant", "Solid", "Solid o Dashed per tutte le voci."),
    prop("active", "usize", "0", "Indice condiviso nel contesto; lo stato visibile di ogni voce si imposta con state."),
    prop("id / class", "Option<String> / String", "None / \"\"", "Id e classi aggiuntive."),
    prop("children", "Element", "-", "Le TimelineItem in ordine cronologico."),
];
const ITEM_PROPS: &[PropDoc] = &[
    prop("title", "Option<String>", "None", "Titolo dell'evento."),
    prop("time", "Option<String>", "None", "Orario o data, in mono a destra del titolo."),
    prop("state", "Option<TimelineBulletState>", "Default", "Default, Active, Completed, Pending, Error."),
    prop("bullet", "Option<Element>", "None", "Contenuto del bullet, di solito un'icona."),
    prop("line_variant", "Option<TimelineLineVariant>", "None", "Sovrascrive lo stile della linea per questa voce."),
    prop("id / class", "Option<String> / String", "None / \"\"", "Id e classi aggiuntive."),
    prop("children", "Element", "-", "Descrizione dell'evento."),
];

pub fn story() -> Story {
    Story {
        slug: "timelines",
        name: "Timeline",
        category: Category::Dati,
        features: &["timeline"],
        icon: IconKind::Clock,
        tagline: "Sequenza di eventi nel tempo, con lo stato di ogni passo e la linea che li collega.",
        keywords: &["timeline", "cronologia", "eventi", "storico", "history", "attività"],
        status: Status::Stable,
        playground: Some(Playground {
            component: "Timeline",
            controls: vec![
                Control::choice("align", "Allineamento", ALIGN_OPTS, 0),
                Control::choice("line_variant", "Linea", LINE_OPTS, 0),
                Control::number("bullet_size", "Bullet (px)", 24.0, 16.0, 40.0, 4.0, 0),
                Control::number("line_width", "Spessore linea (px)", 2.0, 1.0, 4.0, 1.0, 0),
            ],
            render,
            prelude_code: None,
            fixed_code: &[],
            children_code: Some(ITEMS_CODE),
            stage: Stage::Start,
        }),
        examples: vec![
            Example {
                id: "states",
                title: "Stati",
                description: "I cinque stati del bullet.",
                code: STATES_CODE,
                render: || rsx! { States {} },
                span: Span::Half,
            },
            Example {
                id: "icon-bullets",
                title: "Bullet con icona",
                description: "Un'icona racconta il tipo di evento.",
                code: ICON_BULLETS_CODE,
                render: || rsx! { IconBullets {} },
                span: Span::Half,
            },
            Example {
                id: "context",
                title: "In contesto",
                description: "La cronologia di un incidente, con il passo futuro tratteggiato.",
                code: IN_CONTEXT_CODE,
                render: || rsx! { InContext {} },
                span: Span::Full,
            },
        ],
        props: vec![
            PropGroup { component: "Timeline", rows: TIMELINE_PROPS },
            PropGroup { component: "TimelineItem", rows: ITEM_PROPS },
        ],
        recipe: Some(Recipe {
            recipe: "Selected (language.md §3.7) per il passo attivo + semantic marks (§3.10)",
            depth: "Nessuna superficie: testo e marcatori sul contenitore",
            tokens: &["--sd-accent", "--sd-accent-soft", "--sd-glow-sm", "--sd-danger-soft", "--sd-well", "--sd-border-subtle", "--sd-font-mono"],
            notes: "Solo il bullet Active usa l'accento con glow; gli altri stati sono segni semantici piccoli.",
        }),
        a11y: &[
            "Contenitore con role=list e voci con role=listitem: viene letta come un elenco ordinato nel DOM.",
            "Bullet e linea sono aria-hidden: lo stato di un passo va scritto anche nel testo (titolo o descrizione).",
            "L'orario è testo normale accanto al titolo, letto subito dopo di esso.",
        ],
        related: &["lists", "activity-feed", "stepper", "progress"],
    }
}
