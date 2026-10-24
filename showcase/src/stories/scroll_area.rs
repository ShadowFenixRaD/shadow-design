//! Story: ScrollArea (feature `scroll_area`).
use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::story::schema::*;

const TYPES: [ScrollType; 4] = [ScrollType::Hover, ScrollType::Always, ScrollType::Auto, ScrollType::Never];
const TYPE_OPTS: &[ChoiceOption] = &[
    opt("Hover", "ScrollType::Hover"),
    opt("Always", "ScrollType::Always"),
    opt("Auto", "ScrollType::Auto"),
    opt("Never", "ScrollType::Never"),
];

const CHILDREN: &str = r#"List { spacing: ListSpacing::Sm,
    for i in 1..=24 {
        ListItem { "08:{i:02} · handshake con edge-{i:02} completato" }
    }
}"#;

fn render(k: &Knobs) -> Element {
    let height = Some(k.text("height")).filter(|h| !h.is_empty());
    rsx! {
        ScrollArea {
            scroll_type: k.pick("scroll_type", &TYPES),
            height,
            fade_edges: k.bool("fade_edges"),
            offset_scrollbars: k.bool("offset_scrollbars"),
            List { spacing: ListSpacing::Sm,
                for i in 1..=24 {
                    ListItem { key: "{i}", "08:{i:02} · handshake con edge-{i:02} completato" }
                }
            }
        }
    }
}

#[component]
fn Horizontal() -> Element {
    rsx! {
        ScrollArea { scroll_x: true, scroll_y: false, fade_edges: true,
            Group { wrap: false, gap: SpacingToken::Sm,
                for region in ["eu-central", "eu-west", "eu-south", "us-east", "us-west", "ap-south", "ap-east", "sa-east"] {
                    Badge { key: "{region}", label: region.to_string(), pill: true, dot: true, variant: BadgeVariant::Success }
                }
            }
        }
    }
}
const HORIZONTAL_CODE: &str = r#"ScrollArea { scroll_x: true, scroll_y: false, fade_edges: true,
    Group { wrap: false, gap: SpacingToken::Sm,
        for region in ["eu-central", "eu-west", "eu-south", "us-east", "us-west", "ap-south", "ap-east", "sa-east"] {
            Badge { label: region.to_string(), pill: true, dot: true, variant: BadgeVariant::Success }
        }
    }
}"#;

#[component]
fn MaxHeight() -> Element {
    rsx! {
        ScrollArea { max_height: Some("10rem".to_string()), scroll_type: ScrollType::Always,
            Stack { gap: SpacingToken::Sm,
                Text { size: TextSize::Sm, "1. La chiave di sessione dura 24 ore." }
                Text { size: TextSize::Sm, "2. Ogni nodo ruota le chiavi in modo indipendente." }
                Text { size: TextSize::Sm, "3. Il registro conserva gli ultimi 30 giorni." }
                Text { size: TextSize::Sm, "4. Gli audit sono firmati con Ed25519." }
                Text { size: TextSize::Sm, "5. La revoca si propaga in meno di 2 secondi." }
                Text { size: TextSize::Sm, "6. I backup sono cifrati prima di lasciare il nodo." }
            }
        }
    }
}
const MAX_HEIGHT_CODE: &str = r#"ScrollArea { max_height: Some("10rem".to_string()), scroll_type: ScrollType::Always,
    Stack { gap: SpacingToken::Sm,
        Text { size: TextSize::Sm, "1. La chiave di sessione dura 24 ore." }
        Text { size: TextSize::Sm, "2. Ogni nodo ruota le chiavi in modo indipendente." }
        Text { size: TextSize::Sm, "3. Il registro conserva gli ultimi 30 giorni." }
        Text { size: TextSize::Sm, "4. Gli audit sono firmati con Ed25519." }
        Text { size: TextSize::Sm, "5. La revoca si propaga in meno di 2 secondi." }
        Text { size: TextSize::Sm, "6. I backup sono cifrati prima di lasciare il nodo." }
    }
}"#;

#[component]
fn InContext() -> Element {
    rsx! {
        Card {
            title: Some("Registro eventi".to_string()),
            subtitle: Some("edge-07 · ultime 2 ore".to_string()),
            buttons: Some(rsx! {
                Badge { label: "Live".to_string(), variant: BadgeVariant::Success, dot: true, pill: true }
            }),
            ScrollArea { height: Some("14rem".to_string()), fade_edges: true, offset_scrollbars: true,
                List { spacing: ListSpacing::Sm,
                    for (time , text , level) in [
                        ("09:41", "Rotazione chiavi completata", BadgeVariant::Success),
                        ("09:38", "Latenza sopra soglia su edge-03", BadgeVariant::Warning),
                        ("09:30", "Nuovo peer: edge-22", BadgeVariant::Primary),
                        ("09:12", "Backup cifrato caricato", BadgeVariant::Success),
                        ("08:57", "Handshake fallito da 10.4.0.9", BadgeVariant::Danger),
                        ("08:44", "Aggiornamento firmware 2.3.1", BadgeVariant::Primary),
                        ("08:20", "Rotazione chiavi avviata", BadgeVariant::Default),
                        ("08:02", "Sessione operatore aperta", BadgeVariant::Default),
                    ]
                    {
                        ListItem { key: "{time}",
                            Group { gap: SpacingToken::Sm, wrap: false,
                                Badge { label: time.to_string(), variant: level }
                                Text { size: TextSize::Sm, "{text}" }
                            }
                        }
                    }
                }
            }
        }
    }
}
const IN_CONTEXT_CODE: &str = r#"Card {
    title: Some("Registro eventi".to_string()),
    subtitle: Some("edge-07 · ultime 2 ore".to_string()),
    buttons: Some(rsx! {
        Badge { label: "Live".to_string(), variant: BadgeVariant::Success, dot: true, pill: true }
    }),
    ScrollArea { height: Some("14rem".to_string()), fade_edges: true, offset_scrollbars: true,
        List { spacing: ListSpacing::Sm,
            for (time , text , level) in [
                ("09:41", "Rotazione chiavi completata", BadgeVariant::Success),
                ("09:38", "Latenza sopra soglia su edge-03", BadgeVariant::Warning),
                ("09:30", "Nuovo peer: edge-22", BadgeVariant::Primary),
                ("09:12", "Backup cifrato caricato", BadgeVariant::Success),
                ("08:57", "Handshake fallito da 10.4.0.9", BadgeVariant::Danger),
                ("08:44", "Aggiornamento firmware 2.3.1", BadgeVariant::Primary),
                ("08:20", "Rotazione chiavi avviata", BadgeVariant::Default),
                ("08:02", "Sessione operatore aperta", BadgeVariant::Default),
            ]
            {
                ListItem {
                    Group { gap: SpacingToken::Sm, wrap: false,
                        Badge { label: time.to_string(), variant: level }
                        Text { size: TextSize::Sm, "{text}" }
                    }
                }
            }
        }
    }
}"#;

const PROPS: &[PropDoc] = &[
    prop("scroll_type", "ScrollType", "Hover", "Barre visibili: Hover, Always, Auto (nativo), Never (nascoste, scroll attivo)."),
    prop("height / max_height", "Option<String>", "None", "Altezza fissa o massima, qualsiasi unità CSS (\"16rem\", \"50vh\")."),
    prop("width / max_width", "Option<String>", "None", "Larghezza fissa o massima."),
    prop("scroll_x", "bool", "false", "Scorrimento orizzontale."),
    prop("scroll_y", "bool", "true", "Scorrimento verticale."),
    prop("fade_edges", "bool", "false", "Dissolvenza ai bordi: segnala che c'è altro contenuto."),
    prop("offset_scrollbars", "bool", "false", "Padding tra contenuto e barra."),
    prop("class / viewport_class", "String", "\"\"", "Classi sul contenitore esterno e sul viewport."),
];

pub fn story() -> Story {
    Story {
        slug: "scroll-area",
        name: "ScrollArea",
        category: Category::Layout,
        features: &["scroll_area"],
        icon: IconKind::Layers,
        tagline: "Area scorrevole con micro-scrollbar del tema e dissolvenza ai bordi.",
        keywords: &["scroll", "scorrimento", "scrollbar", "overflow", "fade", "viewport"],
        status: Status::Stable,
        playground: Some(Playground {
            component: "ScrollArea",
            controls: vec![
                Control::choice("scroll_type", "Barre", TYPE_OPTS, 0),
                Control::text("height", "Altezza", "16rem").emit(Emit::Template("height: Some(\"{v}\".to_string())")),
                Control::bool("fade_edges", "Dissolvenza ai bordi", false),
                Control::bool("offset_scrollbars", "Distanzia la barra", false),
            ],
            render,
            prelude_code: None,
            fixed_code: &[],
            children_code: Some(CHILDREN),
            stage: Stage::Start,
        }),
        examples: vec![
            Example {
                id: "horizontal",
                title: "Orizzontale",
                description: "Una riga di regioni che scorre di lato.",
                code: HORIZONTAL_CODE,
                render: || rsx! { Horizontal {} },
                span: Span::Half,
            },
            Example {
                id: "max-height",
                title: "Altezza massima",
                description: "Cresce col contenuto fino a 10rem, poi scorre.",
                code: MAX_HEIGHT_CODE,
                render: || rsx! { MaxHeight {} },
                span: Span::Half,
            },
            Example {
                id: "context",
                title: "In contesto",
                description: "Registro eventi a scorrimento dentro una card.",
                code: IN_CONTEXT_CODE,
                render: || rsx! { InContext {} },
                span: Span::Full,
            },
        ],
        props: vec![PropGroup { component: "ScrollArea", rows: PROPS }],
        recipe: Some(Recipe {
            recipe: "Corpo scorrevole: @include sd-scrollbar (language.md §3.2)",
            depth: "Void: prende la superficie del contenitore che lo ospita",
            tokens: &["--sd-border-strong", "--sd-border-hover", "--sd-tint-2", "--sd-radius-full", "--sd-space-2"],
            notes: "Barra sottile a pillola, visibile solo al passaggio in modalità Hover; mai una barra nativa chiara.",
        }),
        a11y: &[
            "Lo scorrimento è quello nativo del browser: rotella, touch e tastiera continuano a funzionare.",
            "Con Never la barra è nascosta ma il contenuto resta scorrevole e leggibile per intero.",
            "fade_edges è solo una maschera visiva: nessun contenuto viene rimosso dal DOM.",
            "Il viewport non ha tabindex: se il contenuto non ha elementi focalizzabili, rendi focalizzabile un contenitore.",
        ],
        related: &["virtual-list", "lists", "cards", "chat"],
    }
}
