//! Story: HoverCard (feature `hover_card`). Compound component (`HoverCard` +
//! `HoverCardTrigger` + `HoverCardDropdown`); the playground goes through `HoverCardDemo { knobs }`.
use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::story::schema::*;

const POSITIONS: [HoverCardPosition; 4] =
    [HoverCardPosition::Bottom, HoverCardPosition::Top, HoverCardPosition::Left, HoverCardPosition::Right];
const POSITION_OPTS: &[ChoiceOption] = &[
    opt("Bottom", "HoverCardPosition::Bottom"),
    opt("Top", "HoverCardPosition::Top"),
    opt("Left", "HoverCardPosition::Left"),
    opt("Right", "HoverCardPosition::Right"),
];

const CHILDREN_CODE: &str = r#"HoverCardTrigger {
    Button { variant: ButtonVariant::Ghost, size: ButtonSize::Sm, "@luca.moretti" }
}
HoverCardDropdown {
    Stack { gap: SpacingToken::Sm,
        Group { gap: SpacingToken::Sm,
            Avatar { name: Some("Luca Moretti".to_string()), status: Some(PresenceStatus::Online) }
            Stack { gap: SpacingToken::None, align: AlignItems::Start,
                Text { size: TextSize::Sm, "Luca Moretti" }
                Text { size: TextSize::Xs, dimmed: true, "Platform engineer · Torino" }
            }
        }
        Text { size: TextSize::Sm, dimmed: true, "Cura la pipeline di telemetria e i runner di CI del mesh." }
        Group { gap: SpacingToken::Xs,
            Badge { label: "Core team".to_string(), variant: BadgeVariant::Primary, pill: true }
            Badge { label: "Reperibile".to_string(), pill: true }
        }
    }
}"#;

#[component]
fn ProfileCard() -> Element {
    rsx! {
        Stack { gap: SpacingToken::Sm,
            Group { gap: SpacingToken::Sm,
                Avatar { name: Some("Luca Moretti".to_string()), status: Some(PresenceStatus::Online) }
                Stack { gap: SpacingToken::None, align: AlignItems::Start,
                    Text { size: TextSize::Sm, "Luca Moretti" }
                    Text { size: TextSize::Xs, dimmed: true, "Platform engineer · Torino" }
                }
            }
            Text { size: TextSize::Sm, dimmed: true, "Cura la pipeline di telemetria e i runner di CI del mesh." }
            Group { gap: SpacingToken::Xs,
                Badge { label: "Core team".to_string(), variant: BadgeVariant::Primary, pill: true }
                Badge { label: "Reperibile".to_string(), pill: true }
            }
        }
    }
}

#[component]
fn HoverCardDemo(knobs: Knobs) -> Element {
    let width = knobs.number("width") as u32;
    let open_delay = knobs.number("open_delay") as u32;
    let close_delay = knobs.number("close_delay") as u32;
    // HoverCard reads position, width and delays once when it mounts: remount on change.
    let remount = format!("{}-{width}-{open_delay}-{close_delay}", knobs.index("position"));
    rsx! {
        Group { gap: SpacingToken::Xs, justify: JustifyContent::Center,
            Text { size: TextSize::Sm, dimmed: true, "Merge della pipeline di telemetria di" }
            for key in [remount] {
                HoverCard {
                    key: "{key}",
                    position: knobs.pick("position", &POSITIONS),
                    open_delay,
                    close_delay,
                    width: (width > 0).then_some(width),
                    HoverCardTrigger {
                        Button { variant: ButtonVariant::Ghost, size: ButtonSize::Sm, "@luca.moretti" }
                    }
                    HoverCardDropdown { ProfileCard {} }
                }
            }
            Text { size: TextSize::Sm, dimmed: true, "· 2 ore fa" }
        }
    }
}

fn render(k: &Knobs) -> Element {
    rsx! {
        HoverCardDemo { knobs: k.clone() }
    }
}

#[component]
fn NodePreview() -> Element {
    rsx! {
        Group { gap: SpacingToken::Xs,
            Text { size: TextSize::Sm, dimmed: true, "Il traffico passa da" }
            HoverCard { position: HoverCardPosition::Top, width: Some(240),
                HoverCardTrigger {
                    Button { variant: ButtonVariant::Ghost, size: ButtonSize::Sm, "relay-03" }
                }
                HoverCardDropdown {
                    Stack { gap: SpacingToken::Sm,
                        Group { justify: JustifyContent::SpaceBetween,
                            Text { size: TextSize::Sm, "relay-03 · Parigi" }
                            Badge { label: "Degradato".to_string(), variant: BadgeVariant::Warning, pill: true, dot: true }
                        }
                        ProgressBar { value: 78.0, variant: ProgressVariant::Warning, size: ProgressSize::Sm }
                        Text { size: TextSize::Xs, dimmed: true, "Carico 78% · latenza 3.8 ms · 18 peer" }
                    }
                }
            }
        }
    }
}
const NODE_PREVIEW_CODE: &str = r#"Group { gap: SpacingToken::Xs,
    Text { size: TextSize::Sm, dimmed: true, "Il traffico passa da" }
    HoverCard { position: HoverCardPosition::Top, width: Some(240),
        HoverCardTrigger {
            Button { variant: ButtonVariant::Ghost, size: ButtonSize::Sm, "relay-03" }
        }
        HoverCardDropdown {
            Stack { gap: SpacingToken::Sm,
                Group { justify: JustifyContent::SpaceBetween,
                    Text { size: TextSize::Sm, "relay-03 · Parigi" }
                    Badge { label: "Degradato".to_string(), variant: BadgeVariant::Warning, pill: true, dot: true }
                }
                ProgressBar { value: 78.0, variant: ProgressVariant::Warning, size: ProgressSize::Sm }
                Text { size: TextSize::Xs, dimmed: true, "Carico 78% · latenza 3.8 ms · 18 peer" }
            }
        }
    }
}"#;

#[component]
fn Delays() -> Element {
    rsx! {
        Group { gap: SpacingToken::Sm, justify: JustifyContent::Center,
            HoverCard { open_delay: 0, close_delay: 100,
                HoverCardTrigger {
                    Button { variant: ButtonVariant::Secondary, size: ButtonSize::Sm, "Immediato" }
                }
                HoverCardDropdown {
                    Text { size: TextSize::Sm, "Si apre subito: utile nelle liste dense." }
                }
            }
            HoverCard { open_delay: 600, close_delay: 400,
                HoverCardTrigger {
                    Button { variant: ButtonVariant::Secondary, size: ButtonSize::Sm, "Ritardato" }
                }
                HoverCardDropdown {
                    Text { size: TextSize::Sm, "Attende 600 ms: evita aperture al semplice passaggio." }
                }
            }
        }
    }
}
const DELAYS_CODE: &str = r#"Group { gap: SpacingToken::Sm, justify: JustifyContent::Center,
    HoverCard { open_delay: 0, close_delay: 100,
        HoverCardTrigger {
            Button { variant: ButtonVariant::Secondary, size: ButtonSize::Sm, "Immediato" }
        }
        HoverCardDropdown {
            Text { size: TextSize::Sm, "Si apre subito: utile nelle liste dense." }
        }
    }
    HoverCard { open_delay: 600, close_delay: 400,
        HoverCardTrigger {
            Button { variant: ButtonVariant::Secondary, size: ButtonSize::Sm, "Ritardato" }
        }
        HoverCardDropdown {
            Text { size: TextSize::Sm, "Attende 600 ms: evita aperture al semplice passaggio." }
        }
    }
}"#;

#[component]
fn InContext() -> Element {
    let commits = [
        ("a91f3c2", "Pipeline di telemetria", "Luca Moretti", "Platform engineer"),
        ("7c0d8e1", "Rotazione chiavi Kyber", "Sara Conti", "Security engineer"),
        ("e44b9a0", "Canary edge-22 su Milano", "Giulia Ferri", "SRE lead"),
    ];
    rsx! {
        Card { title: Some("Commit recenti".to_string()), subtitle: Some("main · mesh-config".to_string()),
            Stack { gap: SpacingToken::Sm,
                for (sha, message, author, role) in commits {
                    Group { key: "{sha}", gap: SpacingToken::Sm, justify: JustifyContent::SpaceBetween,
                        Group { gap: SpacingToken::Sm,
                            Code { "{sha}" }
                            Text { size: TextSize::Sm, "{message}" }
                        }
                        HoverCard { position: HoverCardPosition::Left, width: Some(240),
                            HoverCardTrigger {
                                Button { variant: ButtonVariant::Ghost, size: ButtonSize::Xs,
                                    Avatar { name: Some(author.to_string()), size: AvatarSize::Xs }
                                    "{author}"
                                }
                            }
                            HoverCardDropdown {
                                Group { gap: SpacingToken::Sm,
                                    Avatar { name: Some(author.to_string()), size: AvatarSize::Lg }
                                    Stack { gap: SpacingToken::None, align: AlignItems::Start,
                                        Text { size: TextSize::Sm, "{author}" }
                                        Text { size: TextSize::Xs, dimmed: true, "{role}" }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
const IN_CONTEXT_CODE: &str = r#"let commits = [
    ("a91f3c2", "Pipeline di telemetria", "Luca Moretti", "Platform engineer"),
    ("7c0d8e1", "Rotazione chiavi Kyber", "Sara Conti", "Security engineer"),
    ("e44b9a0", "Canary edge-22 su Milano", "Giulia Ferri", "SRE lead"),
];

Card { title: Some("Commit recenti".to_string()), subtitle: Some("main · mesh-config".to_string()),
    Stack { gap: SpacingToken::Sm,
        for (sha, message, author, role) in commits {
            Group { gap: SpacingToken::Sm, justify: JustifyContent::SpaceBetween,
                Group { gap: SpacingToken::Sm,
                    Code { "{sha}" }
                    Text { size: TextSize::Sm, "{message}" }
                }
                HoverCard { position: HoverCardPosition::Left, width: Some(240),
                    HoverCardTrigger {
                        Button { variant: ButtonVariant::Ghost, size: ButtonSize::Xs,
                            Avatar { name: Some(author.to_string()), size: AvatarSize::Xs }
                            "{author}"
                        }
                    }
                    HoverCardDropdown {
                        Group { gap: SpacingToken::Sm,
                            Avatar { name: Some(author.to_string()), size: AvatarSize::Lg }
                            Stack { gap: SpacingToken::None, align: AlignItems::Start,
                                Text { size: TextSize::Sm, "{author}" }
                                Text { size: TextSize::Xs, dimmed: true, "{role}" }
                            }
                        }
                    }
                }
            }
        }
    }
}"#;

const CARD_PROPS: &[PropDoc] = &[
    prop("open_delay", "u32", "150", "Millisecondi di permanenza sul trigger prima dell'apertura."),
    prop("close_delay", "u32", "200", "Millisecondi prima della chiusura dopo l'uscita da trigger e pannello."),
    prop("width", "Option<u32>", "None", "Larghezza fissa del pannello in px; None segue il contenuto."),
    prop("position", "HoverCardPosition", "Bottom", "Bottom, Top, Left, Right."),
    prop("class", "String", "\"\"", "Classi aggiuntive sul contenitore."),
    prop("children", "Element", "-", "Un HoverCardTrigger e un HoverCardDropdown."),
];
const TRIGGER_PROPS: &[PropDoc] = &[
    prop("children", "Element", "-", "Elemento che apre l'anteprima al passaggio del puntatore."),
    prop("class", "String", "\"\"", "Classi aggiuntive sul wrapper."),
];
const DROPDOWN_PROPS: &[PropDoc] = &[
    prop("children", "Element", "-", "Contenuto dell'anteprima; resta aperto mentre il puntatore è sopra."),
    prop("glow", "Option<bool>", "None", "Glow opzionale sul pannello."),
    prop("class", "String", "\"\"", "Classi aggiuntive sul pannello."),
];

pub fn story() -> Story {
    Story {
        slug: "hover-card",
        name: "HoverCard",
        category: Category::Overlay,
        features: &["hover_card"],
        icon: IconKind::Eye,
        tagline: "Anteprima ricca di una persona o di una risorsa che appare al passaggio del puntatore.",
        keywords: &["hover card", "anteprima", "preview", "profilo", "hover", "card"],
        status: Status::Stable,
        playground: Some(Playground {
            component: "HoverCard",
            controls: vec![
                Control::choice("position", "Posizione", POSITION_OPTS, 0),
                Control::number("open_delay", "Ritardo apertura (ms)", 150.0, 0.0, 1000.0, 50.0, 0),
                Control::number("close_delay", "Ritardo chiusura (ms)", 200.0, 0.0, 1000.0, 50.0, 0),
                Control::number("width", "Larghezza (px)", 0.0, 0.0, 420.0, 20.0, 0)
                    .emit(Emit::Template("width: Some({v})"))
                    .hint("0 = segue il contenuto"),
            ],
            render,
            prelude_code: None,
            fixed_code: &[],
            children_code: Some(CHILDREN_CODE),
            stage: Stage::Center,
        }),
        examples: vec![
            Example {
                id: "node-preview",
                title: "Anteprima di una risorsa",
                description: "Stato e carico di un nodo senza aprire la sua pagina.",
                code: NODE_PREVIEW_CODE,
                render: || rsx! { NodePreview {} },
                span: Span::Half,
            },
            Example {
                id: "delays",
                title: "Ritardi",
                description: "open_delay 0 contro 600 ms.",
                code: DELAYS_CODE,
                render: || rsx! { Delays {} },
                span: Span::Half,
            },
            Example {
                id: "context",
                title: "In contesto",
                description: "Gli autori dei commit mostrano il profilo al passaggio del puntatore.",
                code: IN_CONTEXT_CODE,
                render: || rsx! { InContext {} },
                span: Span::Full,
            },
        ],
        props: vec![
            PropGroup { component: "HoverCard", rows: CARD_PROPS },
            PropGroup { component: "HoverCardTrigger", rows: TRIGGER_PROPS },
            PropGroup { component: "HoverCardDropdown", rows: DROPDOWN_PROPS },
        ],
        recipe: Some(Recipe {
            recipe: "Floating panel (language.md §3.2)",
            depth: "Raised",
            tokens: &["--sd-surface-2", "--sd-border-strong", "--sd-elev-2", "--sd-specular", "--sd-radius-lg", "--sd-space-4"],
            notes: "Stesso pannello del Popover, aperto dal puntatore invece che dal clic; nessun backdrop.",
        }),
        a11y: &[
            "Il pannello ha role=dialog e resta aperto finché il puntatore è sul trigger o sul pannello.",
            "Si apre solo con il puntatore: le informazioni indispensabili devono essere raggiungibili anche altrove (pagina o Popover).",
            "Il trigger resta un elemento normale della pagina (qui un Button), quindi raggiungibile con Tab.",
            "close_delay lascia il tempo di spostare il puntatore dentro il pannello senza chiuderlo.",
        ],
        related: &["popover", "tooltip", "avatars", "facepile"],
    }
}
