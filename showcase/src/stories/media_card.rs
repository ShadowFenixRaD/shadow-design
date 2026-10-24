//! Story: MediaCard and CardActionArea (feature `media_card`). The playground goes through
//! `MediaDemo { knobs }` because the click raises a toast. Images come from picsum.photos (seeded).
use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::story::schema::*;

const RATIOS: [f64; 4] = [16.0 / 9.0, 4.0 / 3.0, 1.0, 21.0 / 9.0];
const RATIO_OPTS: &[ChoiceOption] =
    &[opt("16:9", "16.0 / 9.0"), opt("4:3", "4.0 / 3.0"), opt("1:1", "1.0"), opt("21:9", "21.0 / 9.0")];

const PRELUDE: &str = "let mut toast = use_notification();";

#[component]
fn MediaDemo(knobs: Knobs) -> Element {
    let mut toast = use_notification();
    let click = knobs.bool("on_click").then(|| {
        EventHandler::new(move |_| {
            toast.info("Riproduzione avviata");
        })
    });
    rsx! {
        MediaCard {
            image_url: "https://picsum.photos/seed/shadowfenix/800/450".to_string(),
            aspect_ratio: knobs.pick("aspect_ratio", &RATIOS),
            title: Some(knobs.text("title")).filter(|s| !s.is_empty()),
            subtitle: Some(knobs.text("subtitle")).filter(|s| !s.is_empty()),
            badge: Some(knobs.text("badge")).filter(|s| !s.is_empty()),
            on_click: click,
            footer: knobs.bool("footer").then(|| rsx! {
                Group { justify: JustifyContent::SpaceBetween, align: AlignItems::Center,
                    Text { size: TextSize::Sm, dimmed: true, "1.284 visualizzazioni" }
                    Badge { label: "Nuovo".to_string(), variant: BadgeVariant::Primary, pill: true }
                }
            }),
        }
    }
}

fn render(k: &Knobs) -> Element {
    rsx! {
        MediaDemo { knobs: k.clone() }
    }
}

#[component]
fn Ratios() -> Element {
    rsx! {
        SimpleGrid { cols: 2, spacing: GridSpacing::Sm,
            MediaCard { image_url: "https://picsum.photos/seed/sd-wide/640/360".to_string(), title: Some("16:9".to_string()) }
            MediaCard { image_url: "https://picsum.photos/seed/sd-square/400/400".to_string(), aspect_ratio: 1.0, title: Some("1:1".to_string()) }
        }
    }
}
const RATIOS_CODE: &str = r#"SimpleGrid { cols: 2, spacing: GridSpacing::Sm,
    MediaCard { image_url: "https://picsum.photos/seed/sd-wide/640/360".to_string(), title: Some("16:9".to_string()) }
    MediaCard { image_url: "https://picsum.photos/seed/sd-square/400/400".to_string(), aspect_ratio: 1.0, title: Some("1:1".to_string()) }
}"#;

#[component]
fn ActionArea() -> Element {
    let mut toast = use_notification();
    rsx! {
        Card {
            CardActionArea {
                on_click: move |_| {
                    toast.info("Apro la registrazione");
                },
                Stack { gap: SpacingToken::Xs,
                    Text { "Workshop · Temi con un solo hex" }
                    Text { size: TextSize::Sm, dimmed: true, "48 min · tutta l'area è cliccabile" }
                }
            }
        }
    }
}
const ACTION_AREA_CODE: &str = r#"let mut toast = use_notification();

Card {
    CardActionArea {
        on_click: move |_| {
            toast.info("Apro la registrazione");
        },
        Stack { gap: SpacingToken::Xs,
            Text { "Workshop · Temi con un solo hex" }
            Text { size: TextSize::Sm, dimmed: true, "48 min · tutta l'area è cliccabile" }
        }
    }
}"#;

#[component]
fn InContext() -> Element {
    let mut toast = use_notification();
    let talks = [
        ("talk-mesh", "Una mesh post-quantum in produzione", "Giulia Ferri · 32 min", "Live"),
        ("talk-dioxus", "Dioxus 0.7 su web, desktop e mobile", "Marco Bellini · 41 min", "Nuovo"),
        ("talk-glass", "Lit Glass: profondità al posto dei colori", "Sara Conti · 27 min", "Design"),
    ];
    rsx! {
        Stack { gap: SpacingToken::Md,
            Heading { order: HeadingOrder::H4, "Registrazioni del meetup" }
            SimpleGrid { cols: 1, sm: Some(2), lg: Some(3), spacing: GridSpacing::Md,
                for (seed, title, meta, tag) in talks {
                    MediaCard {
                        image_url: format!("https://picsum.photos/seed/{seed}/640/360"),
                        title: Some(title.to_string()),
                        subtitle: Some(meta.to_string()),
                        badge: Some(tag.to_string()),
                        on_click: move |_| {
                            toast.info(format!("Riproduco: {title}"));
                        },
                    }
                }
            }
        }
    }
}
const IN_CONTEXT_CODE: &str = r#"let mut toast = use_notification();
let talks = [
    ("talk-mesh", "Una mesh post-quantum in produzione", "Giulia Ferri · 32 min", "Live"),
    ("talk-dioxus", "Dioxus 0.7 su web, desktop e mobile", "Marco Bellini · 41 min", "Nuovo"),
    ("talk-glass", "Lit Glass: profondità al posto dei colori", "Sara Conti · 27 min", "Design"),
];

Stack { gap: SpacingToken::Md,
    Heading { order: HeadingOrder::H4, "Registrazioni del meetup" }
    SimpleGrid { cols: 1, sm: Some(2), lg: Some(3), spacing: GridSpacing::Md,
        for (seed, title, meta, tag) in talks {
            MediaCard {
                image_url: format!("https://picsum.photos/seed/{seed}/640/360"),
                title: Some(title.to_string()),
                subtitle: Some(meta.to_string()),
                badge: Some(tag.to_string()),
                on_click: move |_| {
                    toast.info(format!("Riproduco: {title}"));
                },
            }
        }
    }
}"#;

const MEDIA_PROPS: &[PropDoc] = &[
    prop("image_url", "String", "-", "Immagine di testa (alt = title)."),
    prop("video_url", "Option<String>", "None", "Video muto in loop al posto dell'immagine."),
    prop("aspect_ratio", "f64", "16.0 / 9.0", "Rapporto fisso dell'area media (AspectRatio)."),
    prop("title / subtitle", "Option<String>", "None", "Titolo e riga secondaria."),
    prop("badge", "Option<String>", "None", "Etichetta sopra l'immagine."),
    prop("footer", "Option<Element>", "None", "Piè di pagina della card."),
    prop("on_click", "Option<EventHandler<MouseEvent>>", "None", "Rende la card attivabile."),
    prop("children / class", "Element / String", "-", "Contenuto sotto il titolo e classi aggiuntive."),
];
const AREA_PROPS: &[PropDoc] = &[
    prop("children", "Element", "-", "Contenuto reso attivabile come un'unica area."),
    prop("on_click", "Option<EventHandler<MouseEvent>>", "None", "Click o Invio/Spazio."),
    prop("disabled", "bool", "false", "Area inattiva (tabindex -1)."),
    prop("class", "String", "\"\"", "Classi aggiuntive."),
];

pub fn story() -> Story {
    Story {
        slug: "media-card",
        name: "MediaCard",
        category: Category::Contenuti,
        features: &["media_card"],
        icon: IconKind::Eye,
        tagline: "Card con immagine o video a rapporto fisso, titolo, badge e area cliccabile.",
        keywords: &["media card", "immagine", "video", "galleria", "copertina", "card action area"],
        status: Status::Stable,
        playground: Some(Playground {
            component: "MediaCard",
            controls: vec![
                Control::choice("aspect_ratio", "Rapporto", RATIO_OPTS, 0),
                Control::text("title", "Titolo", "Una mesh post-quantum in produzione")
                    .emit(Emit::Template("title: Some(\"{v}\".to_string())")),
                Control::text("subtitle", "Sottotitolo", "Giulia Ferri · 32 min")
                    .emit(Emit::Template("subtitle: Some(\"{v}\".to_string())")),
                Control::text("badge", "Badge", "Live").emit(Emit::Template("badge: Some(\"{v}\".to_string())")),
                Control::bool("on_click", "Cliccabile", true)
                    .emit(Emit::Template("on_click: move |_| { toast.info(\"Riproduzione avviata\"); }")),
                Control::bool("footer", "Piè di pagina", false).emit(Emit::Template(
                    "footer: Some(rsx! { Group { justify: JustifyContent::SpaceBetween, align: AlignItems::Center, Text { size: TextSize::Sm, dimmed: true, \"1.284 visualizzazioni\" } Badge { label: \"Nuovo\".to_string(), variant: BadgeVariant::Primary, pill: true } } })",
                )),
            ],
            render,
            prelude_code: Some(PRELUDE),
            fixed_code: &["image_url: \"https://picsum.photos/seed/shadowfenix/800/450\".to_string(),"],
            children_code: None,
            stage: Stage::Form,
        }),
        examples: vec![
            Example {
                id: "ratios",
                title: "Rapporti",
                description: "16:9 di default, 1:1 per copertine quadrate.",
                code: RATIOS_CODE,
                render: || rsx! { Ratios {} },
                span: Span::Half,
            },
            Example {
                id: "action-area",
                title: "CardActionArea",
                description: "Rende attivabile un blocco qualsiasi dentro una Card.",
                code: ACTION_AREA_CODE,
                render: || rsx! { ActionArea {} },
                span: Span::Half,
            },
            Example {
                id: "context",
                title: "In contesto",
                description: "La galleria delle registrazioni di un meetup.",
                code: IN_CONTEXT_CODE,
                render: || rsx! { InContext {} },
                span: Span::Full,
            },
        ],
        props: vec![
            PropGroup { component: "MediaCard", rows: MEDIA_PROPS },
            PropGroup { component: "CardActionArea", rows: AREA_PROPS },
        ],
        recipe: Some(Recipe {
            recipe: "Container (§3.1), interactive container con on_click",
            depth: "Flat-lit",
            tokens: &["--sd-surface-1", "--sd-border", "--sd-border-hover", "--sd-elev-1", "--sd-elev-2", "--sd-radius-lg", "--sd-focus-ring"],
            notes: "Cliccabile, la card prende bordo hover ed elevazione 2, mai glow; il badge resta l'unico accento.",
        }),
        a11y: &[
            "L'immagine ha alt uguale al titolo.",
            "Con on_click la card ha role=button e tabindex=0 e si attiva con Invio o Spazio.",
            "CardActionArea ha role=button, tabindex e aria-disabled.",
            "Il video è muto e in loop, senza audio da annunciare.",
        ],
        related: &["cards", "document-card", "aspect-ratio", "carousel"],
    }
}
