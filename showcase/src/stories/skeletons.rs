//! Story: Skeleton (feature `skeleton`). Stateless: the playground renders it directly.
use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::story::schema::*;

const SHAPES: [SkeletonShape; 3] = [SkeletonShape::Text, SkeletonShape::Circle, SkeletonShape::Rectangle];
const SHAPE_OPTS: &[ChoiceOption] = &[
    opt("Text", "SkeletonShape::Text"),
    opt("Circle", "SkeletonShape::Circle"),
    opt("Rectangle", "SkeletonShape::Rectangle"),
];

fn render(k: &Knobs) -> Element {
    let width = k.text("width");
    let height = k.text("height");
    rsx! {
        Skeleton {
            shape: k.pick("shape", &SHAPES),
            count: k.number("count") as usize,
            animate: k.bool("animate"),
            width: (!width.is_empty()).then_some(width),
            height: (!height.is_empty()).then_some(height),
        }
    }
}

#[component]
fn Shapes() -> Element {
    rsx! {
        Group { gap: SpacingToken::Md, align: AlignItems::Start, wrap: false,
            Skeleton { shape: SkeletonShape::Circle, width: Some("48px".to_string()), height: Some("48px".to_string()) }
            Stack { gap: SpacingToken::Sm, class: "flex-1",
                Skeleton { shape: SkeletonShape::Rectangle, height: Some("96px".to_string()) }
                Skeleton { shape: SkeletonShape::Text, count: 3 }
            }
        }
    }
}
const SHAPES_CODE: &str = r#"Group { gap: SpacingToken::Md, align: AlignItems::Start, wrap: false,
    Skeleton { shape: SkeletonShape::Circle, width: Some("48px".to_string()), height: Some("48px".to_string()) }
    Stack { gap: SpacingToken::Sm, class: "flex-1",
        Skeleton { shape: SkeletonShape::Rectangle, height: Some("96px".to_string()) }
        Skeleton { shape: SkeletonShape::Text, count: 3 }
    }
}"#;

#[component]
fn Static() -> Element {
    rsx! {
        Stack { gap: SpacingToken::Md,
            for _ in 0..3 {
                Group { gap: SpacingToken::Md, wrap: false,
                    Skeleton { shape: SkeletonShape::Rectangle, width: Some("40px".to_string()), height: Some("40px".to_string()), animate: false }
                    Stack { gap: SpacingToken::Xs, class: "flex-1",
                        Skeleton { width: Some("45%".to_string()), animate: false }
                        Skeleton { width: Some("80%".to_string()), animate: false }
                    }
                }
            }
        }
    }
}
const STATIC_CODE: &str = r#"Stack { gap: SpacingToken::Md,
    for _ in 0..3 {
        Group { gap: SpacingToken::Md, wrap: false,
            Skeleton { shape: SkeletonShape::Rectangle, width: Some("40px".to_string()), height: Some("40px".to_string()), animate: false }
            Stack { gap: SpacingToken::Xs, class: "flex-1",
                Skeleton { width: Some("45%".to_string()), animate: false }
                Skeleton { width: Some("80%".to_string()), animate: false }
            }
        }
    }
}"#;

#[component]
fn InContext() -> Element {
    let loaded = use_signal(|| false);
    rsx! {
        Card {
            title: Some("Operatore di turno".to_string()),
            subtitle: Some("Profilo e ultime attività".to_string()),
            buttons: Some(rsx! {
                Switch { checked: loaded, label: Some("Dati caricati".to_string()) }
            }),
            div { "aria-busy": "{!loaded()}",
                if loaded() {
                    Stack { gap: SpacingToken::Md,
                        Group { gap: SpacingToken::Md, wrap: false,
                            Avatar { name: Some("Giulia Ferri".to_string()), size: AvatarSize::Lg, status: Some(PresenceStatus::Online) }
                            Stack { gap: SpacingToken::Xs, class: "flex-1",
                                Text { "Giulia Ferri" }
                                Text { size: TextSize::Sm, dimmed: true, "SRE · turno 08-16 · Milano" }
                            }
                            Badge { label: "In turno".to_string(), variant: BadgeVariant::Success, pill: true, dot: true }
                        }
                        Text { size: TextSize::Sm, dimmed: true,
                            "Ha chiuso 3 incidenti oggi. Ultima azione: rollback di edge-21 alle 11:42, latenza tornata sotto i 40 ms."
                        }
                    }
                } else {
                    Stack { gap: SpacingToken::Md,
                        Group { gap: SpacingToken::Md, wrap: false,
                            Skeleton { shape: SkeletonShape::Circle, width: Some("48px".to_string()), height: Some("48px".to_string()) }
                            Stack { gap: SpacingToken::Xs, class: "flex-1",
                                Skeleton { width: Some("35%".to_string()) }
                                Skeleton { width: Some("60%".to_string()) }
                            }
                        }
                        Skeleton { count: 2 }
                    }
                }
            }
        }
    }
}
const IN_CONTEXT_CODE: &str = r#"let loaded = use_signal(|| false);

Card {
    title: Some("Operatore di turno".to_string()),
    subtitle: Some("Profilo e ultime attività".to_string()),
    buttons: Some(rsx! {
        Switch { checked: loaded, label: Some("Dati caricati".to_string()) }
    }),
    div { "aria-busy": "{!loaded()}",
        if loaded() {
            Stack { gap: SpacingToken::Md,
                Group { gap: SpacingToken::Md, wrap: false,
                    Avatar { name: Some("Giulia Ferri".to_string()), size: AvatarSize::Lg, status: Some(PresenceStatus::Online) }
                    Stack { gap: SpacingToken::Xs, class: "flex-1",
                        Text { "Giulia Ferri" }
                        Text { size: TextSize::Sm, dimmed: true, "SRE · turno 08-16 · Milano" }
                    }
                    Badge { label: "In turno".to_string(), variant: BadgeVariant::Success, pill: true, dot: true }
                }
                Text { size: TextSize::Sm, dimmed: true,
                    "Ha chiuso 3 incidenti oggi. Ultima azione: rollback di edge-21 alle 11:42, latenza tornata sotto i 40 ms."
                }
            }
        } else {
            Stack { gap: SpacingToken::Md,
                Group { gap: SpacingToken::Md, wrap: false,
                    Skeleton { shape: SkeletonShape::Circle, width: Some("48px".to_string()), height: Some("48px".to_string()) }
                    Stack { gap: SpacingToken::Xs, class: "flex-1",
                        Skeleton { width: Some("35%".to_string()) }
                        Skeleton { width: Some("60%".to_string()) }
                    }
                }
                Skeleton { count: 2 }
            }
        }
    }
}"#;

const PROPS: &[PropDoc] = &[
    prop("shape", "SkeletonShape", "Text", "Text (riga), Circle (avatar), Rectangle (immagini e card)."),
    prop("count", "usize", "1", "Solo per Text: numero di righe; l'ultima è al 70% se width è assente."),
    prop("animate", "bool", "true", "Riflesso shimmer; rimosso con prefers-reduced-motion."),
    prop("width / height", "Option<String>", "None", "Dimensioni CSS esplicite (\"48px\", \"60%\")."),
    prop("class", "String", "\"\"", "Classi aggiuntive."),
];

pub fn story() -> Story {
    Story {
        slug: "skeletons",
        name: "Skeleton",
        category: Category::Dati,
        features: &["skeleton"],
        icon: IconKind::Box,
        tagline: "Segnaposto con la forma del contenuto, mostrato finché i dati non arrivano.",
        keywords: &["skeleton", "placeholder", "segnaposto", "caricamento", "loading", "shimmer"],
        status: Status::Stable,
        playground: Some(Playground {
            component: "Skeleton",
            controls: vec![
                Control::choice("shape", "Forma", SHAPE_OPTS, 0),
                Control::number("count", "Righe", 3.0, 1.0, 6.0, 1.0, 0).emit(Emit::Always).hint("Solo per la forma Text."),
                Control::text("width", "Larghezza", "").emit(Emit::Template("width: Some(\"{v}\".to_string())")),
                Control::text("height", "Altezza", "").emit(Emit::Template("height: Some(\"{v}\".to_string())")),
                Control::bool("animate", "Shimmer", true),
            ],
            render,
            prelude_code: None,
            fixed_code: &[],
            children_code: None,
            stage: Stage::Form,
        }),
        examples: vec![
            Example {
                id: "shapes",
                title: "Forme",
                description: "Cerchio per l'avatar, rettangolo per l'immagine, righe per il testo.",
                code: SHAPES_CODE,
                render: || rsx! { Shapes {} },
                span: Span::Half,
            },
            Example {
                id: "static",
                title: "Senza animazione",
                description: "animate: false per elenchi lunghi in cui il movimento distrae.",
                code: STATIC_CODE,
                render: || rsx! { Static {} },
                span: Span::Half,
            },
            Example {
                id: "context",
                title: "In contesto",
                description: "Lo scheletro ricalca la card caricata, così il layout non salta.",
                code: IN_CONTEXT_CODE,
                render: || rsx! { InContext {} },
                span: Span::Full,
            },
        ],
        props: vec![PropGroup { component: "Skeleton", rows: PROPS }],
        recipe: Some(Recipe {
            recipe: "Value fill (language.md §3.11, traccia) + motion decorativa (§5)",
            depth: "Well",
            tokens: &["--sd-well", "--sd-border-subtle", "--sd-tint-1", "--sd-radius-sm", "--sd-radius-full", "--sd-ease"],
            notes: "Riempimento incassato come le tracce; lo shimmer è un loop decorativo, quindi sparisce con reduced motion.",
        }),
        a11y: &[
            "Ogni segnaposto è aria-hidden: gli screen reader non leggono forme vuote.",
            "Segnala il caricamento sul contenitore con aria-busy, come nell'esempio In contesto.",
            "Lo shimmer usa sd-animate e viene rimosso con prefers-reduced-motion.",
        ],
        related: &["loading-overlay", "progress", "lists", "cards"],
    }
}
