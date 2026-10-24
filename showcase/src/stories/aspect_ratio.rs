//! Story: AspectRatio (feature `aspect_ratio`).
use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::story::schema::*;
use crate::story::sections::DemoCell;

const RATIOS: [f64; 5] = [16.0 / 9.0, 4.0 / 3.0, 1.0, 21.0 / 9.0, 3.0 / 4.0];
const RATIO_OPTS: &[ChoiceOption] = &[
    opt("16:9", "16.0 / 9.0"),
    opt("4:3", "4.0 / 3.0"),
    opt("1:1", "1.0"),
    opt("21:9", "21.0 / 9.0"),
    opt("3:4", "3.0 / 4.0"),
];

fn render(k: &Knobs) -> Element {
    rsx! {
        AspectRatio { ratio: k.pick("ratio", &RATIOS),
            DemoCell { label: Some("Media".to_string()), accent: true }
        }
    }
}

#[component]
fn Square() -> Element {
    rsx! {
        SimpleGrid { cols: 3, spacing: GridSpacing::Sm,
            for i in 1..=6 {
                AspectRatio { key: "{i}", ratio: 1.0,
                    DemoCell { index: Some(i) }
                }
            }
        }
    }
}
const SQUARE_CODE: &str = r#"SimpleGrid { cols: 3, spacing: GridSpacing::Sm,
    for i in 1..=6 {
        AspectRatio { ratio: 1.0,
            DemoCell { index: Some(i) }
        }
    }
}"#;

#[component]
fn Custom() -> Element {
    rsx! {
        AspectRatio { ratio: 2.39,
            DemoCell { label: Some("2.39:1 · anamorfico".to_string()) }
        }
    }
}
const CUSTOM_CODE: &str = r#"AspectRatio { ratio: 2.39,
    DemoCell { label: Some("2.39:1 · anamorfico".to_string()) }
}"#;

#[component]
fn InContext() -> Element {
    rsx! {
        SimpleGrid { cols: 1, md: Some(3), spacing: GridSpacing::Md,
            for (title , meta) in [
                ("Rilascio 0.5", "Registrazione · 42 min"),
                ("Audit Kyber", "Talk · 28 min"),
                ("Mesh in produzione", "Demo · 15 min"),
            ]
            {
                Stack { key: "{title}", gap: SpacingToken::Sm,
                    AspectRatio { ratio: 16.0 / 9.0,
                        Skeleton { shape: SkeletonShape::Rectangle }
                    }
                    Text { "{title}" }
                    Text { size: TextSize::Sm, dimmed: true, "{meta}" }
                }
            }
        }
    }
}
const IN_CONTEXT_CODE: &str = r#"SimpleGrid { cols: 1, md: Some(3), spacing: GridSpacing::Md,
    for (title , meta) in [
        ("Rilascio 0.5", "Registrazione · 42 min"),
        ("Audit Kyber", "Talk · 28 min"),
        ("Mesh in produzione", "Demo · 15 min"),
    ]
    {
        Stack { gap: SpacingToken::Sm,
            AspectRatio { ratio: 16.0 / 9.0,
                Skeleton { shape: SkeletonShape::Rectangle }
            }
            Text { "{title}" }
            Text { size: TextSize::Sm, dimmed: true, "{meta}" }
        }
    }
}"#;

const PROPS: &[PropDoc] = &[
    prop("ratio", "f64", "16.0 / 9.0", "Larghezza diviso altezza. 16:9, 4:3, 1:1 e 21:9 usano una classe, gli altri valori uno stile inline."),
    prop("class", "String", "\"\"", "Classi aggiuntive sul contenitore."),
    prop("children", "Element", "-", "Il media: ogni figlio riempie il riquadro (object-fit: cover)."),
];

pub fn story() -> Story {
    Story {
        slug: "aspect-ratio",
        name: "AspectRatio",
        category: Category::Layout,
        features: &["aspect_ratio"],
        icon: IconKind::Box,
        tagline: "Riserva lo spazio di immagini, video e mappe con un rapporto di forma fisso.",
        keywords: &["aspect ratio", "rapporto", "proporzioni", "media", "video", "immagine", "cls"],
        status: Status::Stable,
        playground: Some(Playground {
            component: "AspectRatio",
            controls: vec![Control::choice("ratio", "Rapporto", RATIO_OPTS, 0)],
            render,
            prelude_code: None,
            fixed_code: &[],
            children_code: Some("DemoCell { label: Some(\"Media\".to_string()), accent: true }"),
            stage: Stage::Form,
        }),
        examples: vec![
            Example {
                id: "square",
                title: "Galleria quadrata",
                description: "Sei miniature 1:1 in una SimpleGrid.",
                code: SQUARE_CODE,
                render: || rsx! { Square {} },
                span: Span::Half,
            },
            Example {
                id: "custom",
                title: "Rapporto libero",
                description: "Qualsiasi valore f64, qui il formato cinema 2.39.",
                code: CUSTOM_CODE,
                render: || rsx! { Custom {} },
                span: Span::Half,
            },
            Example {
                id: "context",
                title: "In contesto",
                description: "Libreria video in caricamento: lo spazio è già riservato, nessuno spostamento.",
                code: IN_CONTEXT_CODE,
                render: || rsx! { InContext {} },
                span: Span::Full,
            },
        ],
        props: vec![PropGroup { component: "AspectRatio", rows: PROPS }],
        recipe: Some(Recipe {
            recipe: "Primitiva di layout (nessuna superficie, language.md §2 Void)",
            depth: "Void: il riquadro prende la superficie del media che contiene",
            tokens: &["--sd-radius-md", "--sd-well", "--sd-space-2"],
            notes: "overflow hidden e figli assoluti: il media non sposta mai il layout mentre carica.",
        }),
        a11y: &[
            "Contenitore div senza ruolo: il testo alternativo va sul media figlio (alt, title, aria-label).",
            "Il riquadro mantiene l'altezza prima del caricamento: nessun salto per chi naviga col lettore.",
            "Nessun elemento interattivo aggiunto: i controlli del media restano raggiungibili da tastiera.",
        ],
        related: &["center", "masonry", "cards", "skeletons"],
    }
}
