//! Story: Facepile (feature `facepile`). The playground goes through `FacepileDemo { knobs }`
//! because the add button raises a toast.
use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::story::schema::*;

const SIZES: [AvatarSize; 4] = [AvatarSize::Xs, AvatarSize::Sm, AvatarSize::Md, AvatarSize::Lg];
const SIZE_OPTS: &[ChoiceOption] =
    &[opt("Xs", "AvatarSize::Xs"), opt("Sm", "AvatarSize::Sm"), opt("Md", "AvatarSize::Md"), opt("Lg", "AvatarSize::Lg")];

fn team() -> Vec<PersonaProps> {
    vec![
        PersonaProps::new("Giulia Ferri").with_role("Ops lead").with_status(PresenceStatus::Online),
        PersonaProps::new("Marco Bellini").with_role("Rust core").with_status(PresenceStatus::Busy),
        PersonaProps::new("Sara Conti").with_role("Design").with_status(PresenceStatus::Online),
        PersonaProps::new("Luca Moretti").with_role("Sicurezza").with_status(PresenceStatus::Offline),
        PersonaProps::new("Elena Russo").with_role("Rete"),
        PersonaProps::new("Paolo Gallo").with_role("QA"),
        PersonaProps::new("Anna Ricci").with_role("Supporto"),
    ]
}

const PRELUDE: &str = r#"let mut toast = use_notification();
let personas = vec![
    PersonaProps::new("Giulia Ferri").with_role("Ops lead").with_status(PresenceStatus::Online),
    PersonaProps::new("Marco Bellini").with_role("Rust core").with_status(PresenceStatus::Busy),
    PersonaProps::new("Sara Conti").with_role("Design").with_status(PresenceStatus::Online),
    PersonaProps::new("Luca Moretti").with_role("Sicurezza").with_status(PresenceStatus::Offline),
    PersonaProps::new("Elena Russo").with_role("Rete"),
    PersonaProps::new("Paolo Gallo").with_role("QA"),
    PersonaProps::new("Anna Ricci").with_role("Supporto"),
];"#;

#[component]
fn FacepileDemo(knobs: Knobs) -> Element {
    let mut toast = use_notification();
    let on_add = knobs.bool("add").then(|| EventHandler::new(move |_| {
        toast.info("Invita un collaboratore");
    }));
    rsx! {
        Facepile {
            personas: team(),
            max_displayable: knobs.number("max_displayable") as usize,
            size: knobs.pick("size", &SIZES),
            show_presence: knobs.bool("show_presence"),
            on_add_click: on_add,
        }
    }
}

fn render(k: &Knobs) -> Element {
    rsx! {
        FacepileDemo { knobs: k.clone() }
    }
}

#[component]
fn Clickable() -> Element {
    let mut toast = use_notification();
    rsx! {
        Facepile {
            personas: vec![
                PersonaProps::new("Giulia Ferri").with_role("Ops lead"),
                PersonaProps::new("Marco Bellini").with_role("Rust core"),
                PersonaProps::new("Sara Conti").with_role("Design"),
            ],
            on_persona_click: move |p: PersonaProps| {
                toast.info(format!("Profilo di {}", p.name));
            },
        }
    }
}
const CLICKABLE_CODE: &str = r#"let mut toast = use_notification();

Facepile {
    personas: vec![
        PersonaProps::new("Giulia Ferri").with_role("Ops lead"),
        PersonaProps::new("Marco Bellini").with_role("Rust core"),
        PersonaProps::new("Sara Conti").with_role("Design"),
    ],
    on_persona_click: move |p: PersonaProps| {
        toast.info(format!("Profilo di {}", p.name));
    },
}"#;

#[component]
fn Overflow() -> Element {
    rsx! {
        Stack { gap: SpacingToken::Md,
            Facepile {
                personas: vec![
                    PersonaProps::new("Giulia Ferri"),
                    PersonaProps::new("Marco Bellini"),
                    PersonaProps::new("Sara Conti"),
                    PersonaProps::new("Luca Moretti"),
                    PersonaProps::new("Elena Russo"),
                ],
                max_displayable: 2,
                size: AvatarSize::Sm,
            }
            Text { size: TextSize::Sm, dimmed: true, "Apri +3 per vedere chi manca." }
        }
    }
}
const OVERFLOW_CODE: &str = r#"Stack { gap: SpacingToken::Md,
    Facepile {
        personas: vec![
            PersonaProps::new("Giulia Ferri"),
            PersonaProps::new("Marco Bellini"),
            PersonaProps::new("Sara Conti"),
            PersonaProps::new("Luca Moretti"),
            PersonaProps::new("Elena Russo"),
        ],
        max_displayable: 2,
        size: AvatarSize::Sm,
    }
    Text { size: TextSize::Sm, dimmed: true, "Apri +3 per vedere chi manca." }
}"#;

#[component]
fn InContext() -> Element {
    let mut toast = use_notification();
    rsx! {
        Card {
            title: Some("Release v0.5.3".to_string()),
            subtitle: Some("In revisione · scadenza venerdì".to_string()),
            footer: Some(rsx! {
                Group { justify: JustifyContent::SpaceBetween, align: AlignItems::Center,
                    Facepile {
                        personas: vec![
                            PersonaProps::new("Giulia Ferri").with_role("Ops lead").with_status(PresenceStatus::Online),
                            PersonaProps::new("Marco Bellini").with_role("Rust core").with_status(PresenceStatus::Busy),
                            PersonaProps::new("Sara Conti").with_role("Design").with_status(PresenceStatus::Online),
                            PersonaProps::new("Luca Moretti").with_role("Sicurezza"),
                            PersonaProps::new("Elena Russo").with_role("Rete"),
                        ],
                        max_displayable: 3,
                        size: AvatarSize::Sm,
                        on_add_click: move |_| {
                            toast.info("Aggiungi un revisore");
                        },
                    }
                    Button {
                        onclick: move |_| {
                            toast.success("Revisione approvata");
                        },
                        "Approva"
                    }
                }
            }),
            Stack { gap: SpacingToken::Sm,
                Group { gap: SpacingToken::Sm,
                    Badge { label: "3 approvazioni su 5".to_string(), variant: BadgeVariant::Primary, pill: true }
                    Badge { label: "CI verde".to_string(), variant: BadgeVariant::Success, pill: true }
                }
                Text { size: TextSize::Sm, dimmed: true, "Relay più rapido, nuovo preset Cyan e correzioni al DatePicker." }
            }
        }
    }
}
const IN_CONTEXT_CODE: &str = r#"let mut toast = use_notification();

Card {
    title: Some("Release v0.5.3".to_string()),
    subtitle: Some("In revisione · scadenza venerdì".to_string()),
    footer: Some(rsx! {
        Group { justify: JustifyContent::SpaceBetween, align: AlignItems::Center,
            Facepile {
                personas: vec![
                    PersonaProps::new("Giulia Ferri").with_role("Ops lead").with_status(PresenceStatus::Online),
                    PersonaProps::new("Marco Bellini").with_role("Rust core").with_status(PresenceStatus::Busy),
                    PersonaProps::new("Sara Conti").with_role("Design").with_status(PresenceStatus::Online),
                    PersonaProps::new("Luca Moretti").with_role("Sicurezza"),
                    PersonaProps::new("Elena Russo").with_role("Rete"),
                ],
                max_displayable: 3,
                size: AvatarSize::Sm,
                on_add_click: move |_| {
                    toast.info("Aggiungi un revisore");
                },
            }
            Button {
                onclick: move |_| {
                    toast.success("Revisione approvata");
                },
                "Approva"
            }
        }
    }),
    Stack { gap: SpacingToken::Sm,
        Group { gap: SpacingToken::Sm,
            Badge { label: "3 approvazioni su 5".to_string(), variant: BadgeVariant::Primary, pill: true }
            Badge { label: "CI verde".to_string(), variant: BadgeVariant::Success, pill: true }
        }
        Text { size: TextSize::Sm, dimmed: true, "Relay più rapido, nuovo preset Cyan e correzioni al DatePicker." }
    }
}"#;

const PROPS: &[PropDoc] = &[
    prop("personas", "Vec<PersonaProps>", "-", "Persone: nome, immagine, presenza, ruolo."),
    prop("max_displayable", "usize", "4", "Avatar visibili prima del pulsante +N."),
    prop("size", "AvatarSize", "Md", "Dimensione degli avatar."),
    prop("show_presence", "bool", "true", "Mostra il punto di presenza."),
    prop("on_add_click", "Option<EventHandler<()>>", "None", "Mostra il pulsante + per aggiungere una persona."),
    prop("on_persona_click", "Option<EventHandler<PersonaProps>>", "None", "Rende ogni avatar attivabile, anche nel menu +N."),
    prop("class", "String", "\"\"", "Classi aggiuntive."),
];

pub fn story() -> Story {
    Story {
        slug: "facepile",
        name: "Facepile",
        category: Category::Contenuti,
        features: &["facepile"],
        icon: IconKind::Share2,
        tagline: "Chi partecipa, in poco spazio: avatar sovrapposti, contatore +N e invito.",
        keywords: &["facepile", "avatar", "collaboratori", "team", "partecipanti", "persone"],
        status: Status::Stable,
        playground: Some(Playground {
            component: "Facepile",
            controls: vec![
                Control::number("max_displayable", "Visibili", 4.0, 1.0, 7.0, 1.0, 0),
                Control::choice("size", "Dimensione", SIZE_OPTS, 2),
                Control::bool("show_presence", "Presenza", true),
                Control::bool("add", "Pulsante aggiungi", true)
                    .emit(Emit::Template("on_add_click: move |_| { toast.info(\"Invita un collaboratore\"); }")),
            ],
            render,
            prelude_code: Some(PRELUDE),
            fixed_code: &["personas,"],
            children_code: None,
            stage: Stage::Center,
        }),
        examples: vec![
            Example {
                id: "clickable",
                title: "Persone cliccabili",
                description: "on_persona_click apre il profilo; il title mostra il ruolo.",
                code: CLICKABLE_CODE,
                render: || rsx! { Clickable {} },
                span: Span::Half,
            },
            Example {
                id: "overflow",
                title: "Menu +N",
                description: "Oltre max_displayable le persone finiscono in un menu.",
                code: OVERFLOW_CODE,
                render: || rsx! { Overflow {} },
                span: Span::Half,
            },
            Example {
                id: "context",
                title: "In contesto",
                description: "I revisori di una release, con invito e approvazione.",
                code: IN_CONTEXT_CODE,
                render: || rsx! { InContext {} },
                span: Span::Full,
            },
        ],
        props: vec![PropGroup { component: "Facepile", rows: PROPS }],
        recipe: Some(Recipe {
            recipe: "Pill shape (§2) + floating panel per il menu +N (§3.2)",
            depth: "Flat-lit (avatar) · Raised (menu)",
            tokens: &["--sd-radius-full", "--sd-surface-2", "--sd-border-strong", "--sd-elev-2", "--sd-item-h", "--sd-focus-ring"],
            notes: "Gli avatar si sovrappongono con un anello del colore della superficie; il menu +N è un pannello flottante con voci sd-item.",
        }),
        a11y: &[
            "Con on_persona_click ogni avatar ha role=button, tabindex=0 e si attiva con Invio o Spazio.",
            "Il pulsante +N è un <button> con aria-haspopup e aria-expanded; il title viene da Locale.",
            "Ogni avatar ha un title con nome e ruolo.",
            "Le voci del menu +N sono <button> nativi.",
        ],
        related: &["avatars", "activity-feed", "document-card", "chat"],
    }
}
