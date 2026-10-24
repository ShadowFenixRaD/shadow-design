//! Story: Grid, GridCol, SimpleGrid (feature `grid`).
use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::story::schema::*;
use crate::story::sections::DemoCell;

const SPACINGS: [GridSpacing; 6] =
    [GridSpacing::None, GridSpacing::Xs, GridSpacing::Sm, GridSpacing::Md, GridSpacing::Lg, GridSpacing::Xl];
const GUTTER_OPTS: &[ChoiceOption] = &[
    opt("None", "GridSpacing::None"),
    opt("Xs", "GridSpacing::Xs"),
    opt("Sm", "GridSpacing::Sm"),
    opt("Md", "GridSpacing::Md"),
    opt("Lg", "GridSpacing::Lg"),
    opt("Xl", "GridSpacing::Xl"),
];
const ALIGNS: [GridAlign; 5] = [GridAlign::Start, GridAlign::Center, GridAlign::End, GridAlign::Stretch, GridAlign::Baseline];
const ALIGN_OPTS: &[ChoiceOption] = &[
    opt("Start", "GridAlign::Start"),
    opt("Center", "GridAlign::Center"),
    opt("End", "GridAlign::End"),
    opt("Stretch", "GridAlign::Stretch"),
    opt("Baseline", "GridAlign::Baseline"),
];
const JUSTIFIES: [GridJustify; 5] =
    [GridJustify::Start, GridJustify::Center, GridJustify::End, GridJustify::SpaceBetween, GridJustify::SpaceAround];
const JUSTIFY_OPTS: &[ChoiceOption] = &[
    opt("Start", "GridJustify::Start"),
    opt("Center", "GridJustify::Center"),
    opt("End", "GridJustify::End"),
    opt("Between", "GridJustify::SpaceBetween"),
    opt("Around", "GridJustify::SpaceAround"),
];

const CHILDREN: &str = r#"GridCol { span: 12, md: 8,
    DemoCell { label: Some("span 12 · md 8".to_string()), accent: true, tall: true }
}
GridCol { span: 12, md: 4,
    DemoCell { label: Some("span 12 · md 4".to_string()) }
}
GridCol { span: 6, md: 3,
    DemoCell { label: Some("6 · md 3".to_string()), short: true }
}
GridCol { span: 6, md: 3,
    DemoCell { label: Some("6 · md 3".to_string()), short: true }
}"#;

fn render(k: &Knobs) -> Element {
    rsx! {
        Grid {
            gutter: k.pick("gutter", &SPACINGS),
            align: k.pick("align", &ALIGNS),
            justify: k.pick("justify", &JUSTIFIES),
            GridCol { span: 12, md: 8,
                DemoCell { label: Some("span 12 · md 8".to_string()), accent: true, tall: true }
            }
            GridCol { span: 12, md: 4,
                DemoCell { label: Some("span 12 · md 4".to_string()) }
            }
            GridCol { span: 6, md: 3,
                DemoCell { label: Some("6 · md 3".to_string()), short: true }
            }
            GridCol { span: 6, md: 3,
                DemoCell { label: Some("6 · md 3".to_string()), short: true }
            }
        }
    }
}

#[component]
fn Offsets() -> Element {
    rsx! {
        Grid { gutter: GridSpacing::Sm,
            GridCol { span: 4,
                DemoCell { label: Some("span 4".to_string()) }
            }
            GridCol { span: 4, offset: 4,
                DemoCell { label: Some("span 4 · offset 4".to_string()), accent: true }
            }
            GridCol { span: 6, offset: 3,
                DemoCell { label: Some("span 6 · offset 3".to_string()) }
            }
        }
    }
}
const OFFSETS_CODE: &str = r#"Grid { gutter: GridSpacing::Sm,
    GridCol { span: 4,
        DemoCell { label: Some("span 4".to_string()) }
    }
    GridCol { span: 4, offset: 4,
        DemoCell { label: Some("span 4 · offset 4".to_string()), accent: true }
    }
    GridCol { span: 6, offset: 3,
        DemoCell { label: Some("span 6 · offset 3".to_string()) }
    }
}"#;

#[component]
fn Responsive() -> Element {
    rsx! {
        Grid { gutter: GridSpacing::Sm,
            for i in 1..=4 {
                GridCol { key: "{i}", span: 12, sm: 6, lg: 3,
                    DemoCell { index: Some(i), label: Some("12 · sm 6 · lg 3".to_string()) }
                }
            }
        }
    }
}
const RESPONSIVE_CODE: &str = r#"Grid { gutter: GridSpacing::Sm,
    for i in 1..=4 {
        GridCol { span: 12, sm: 6, lg: 3,
            DemoCell { index: Some(i), label: Some("12 · sm 6 · lg 3".to_string()) }
        }
    }
}"#;

#[component]
fn Simple() -> Element {
    rsx! {
        SimpleGrid { cols: 2, md: Some(3), spacing: GridSpacing::Sm,
            for i in 1..=6 {
                DemoCell { key: "{i}", index: Some(i), label: Some("1/3".to_string()) }
            }
        }
    }
}
const SIMPLE_CODE: &str = r#"SimpleGrid { cols: 2, md: Some(3), spacing: GridSpacing::Sm,
    for i in 1..=6 {
        DemoCell { index: Some(i), label: Some("1/3".to_string()) }
    }
}"#;

#[component]
fn RowGutter() -> Element {
    rsx! {
        Grid { gutter: GridSpacing::Xs, row_gutter: Some(GridSpacing::Xl),
            for i in 1..=6 {
                GridCol { key: "{i}", span: 4,
                    DemoCell { index: Some(i), short: true }
                }
            }
        }
    }
}
const ROW_GUTTER_CODE: &str = r#"Grid { gutter: GridSpacing::Xs, row_gutter: Some(GridSpacing::Xl),
    for i in 1..=6 {
        GridCol { span: 4,
            DemoCell { index: Some(i), short: true }
        }
    }
}"#;

#[component]
fn InContext() -> Element {
    rsx! {
        Grid { gutter: GridSpacing::Md,
            GridCol { span: 12, md: 8,
                Card {
                    title: Some("Traffico della mesh".to_string()),
                    subtitle: Some("Ultime 24 ore · 12 nodi".to_string()),
                    Stack { gap: SpacingToken::Sm,
                        ProgressBar { value: 72.0, show_label: true, label: Some("Banda in uscita".to_string()) }
                        ProgressBar { value: 41.0, show_label: true, label: Some("Banda in ingresso".to_string()) }
                        ProgressBar { value: 88.0, variant: ProgressVariant::Warning, show_label: true, label: Some("Disco edge-03".to_string()) }
                    }
                }
            }
            GridCol { span: 12, md: 4,
                Card { title: Some("Stato".to_string()), subtitle: Some("Aggiornato ora".to_string()),
                    Stack { gap: SpacingToken::Sm,
                        Group { justify: JustifyContent::SpaceBetween,
                            Text { size: TextSize::Sm, "Nodi online" }
                            Badge { label: "11/12".to_string(), variant: BadgeVariant::Success, dot: true }
                        }
                        Group { justify: JustifyContent::SpaceBetween,
                            Text { size: TextSize::Sm, "Latenza media" }
                            Badge { label: "18 ms".to_string() }
                        }
                        Group { justify: JustifyContent::SpaceBetween,
                            Text { size: TextSize::Sm, "Allarmi" }
                            Badge { label: "1".to_string(), variant: BadgeVariant::Warning }
                        }
                    }
                }
            }
        }
    }
}
const IN_CONTEXT_CODE: &str = r#"Grid { gutter: GridSpacing::Md,
    GridCol { span: 12, md: 8,
        Card {
            title: Some("Traffico della mesh".to_string()),
            subtitle: Some("Ultime 24 ore · 12 nodi".to_string()),
            Stack { gap: SpacingToken::Sm,
                ProgressBar { value: 72.0, show_label: true, label: Some("Banda in uscita".to_string()) }
                ProgressBar { value: 41.0, show_label: true, label: Some("Banda in ingresso".to_string()) }
                ProgressBar { value: 88.0, variant: ProgressVariant::Warning, show_label: true, label: Some("Disco edge-03".to_string()) }
            }
        }
    }
    GridCol { span: 12, md: 4,
        Card { title: Some("Stato".to_string()), subtitle: Some("Aggiornato ora".to_string()),
            Stack { gap: SpacingToken::Sm,
                Group { justify: JustifyContent::SpaceBetween,
                    Text { size: TextSize::Sm, "Nodi online" }
                    Badge { label: "11/12".to_string(), variant: BadgeVariant::Success, dot: true }
                }
                Group { justify: JustifyContent::SpaceBetween,
                    Text { size: TextSize::Sm, "Latenza media" }
                    Badge { label: "18 ms".to_string() }
                }
                Group { justify: JustifyContent::SpaceBetween,
                    Text { size: TextSize::Sm, "Allarmi" }
                    Badge { label: "1".to_string(), variant: BadgeVariant::Warning }
                }
            }
        }
    }
}"#;

const GRID_PROPS: &[PropDoc] = &[
    prop("gutter", "GridSpacing", "Md", "Spazio tra colonne e righe: None, Xs, Sm, Md, Lg, Xl."),
    prop("row_gutter", "Option<GridSpacing>", "None", "Spazio verticale indipendente tra le righe."),
    prop("align", "GridAlign", "Stretch", "Allineamento verticale delle celle: Start, Center, End, Stretch, Baseline."),
    prop("justify", "GridJustify", "Start", "Distribuzione orizzontale: Start, Center, End, SpaceBetween, SpaceAround."),
    prop("class", "String", "\"\"", "Classi aggiuntive sul contenitore."),
];
const COL_PROPS: &[PropDoc] = &[
    prop("span", "u8", "12", "Colonne occupate su 12."),
    prop("offset", "u8", "0", "Colonne vuote prima della cella."),
    prop("sm / md / lg / xl", "Option<u8>", "None", "Span dai breakpoint 640, 768, 1024 e 1280px in su."),
    prop("class", "String", "\"\"", "Classi aggiuntive sulla cella."),
];
const SIMPLE_PROPS: &[PropDoc] = &[
    prop("cols", "u8", "1", "Colonne di uguale larghezza."),
    prop("sm / md / lg / xl", "Option<u8>", "None", "Colonne dai breakpoint in su."),
    prop("spacing", "GridSpacing", "Md", "Spazio tra le celle."),
    prop("vertical_spacing", "Option<GridSpacing>", "None", "Spazio verticale indipendente."),
    prop("class", "String", "\"\"", "Classi aggiuntive sul contenitore."),
];

pub fn story() -> Story {
    Story {
        slug: "grid",
        name: "Grid",
        category: Category::Layout,
        features: &["grid"],
        icon: IconKind::Layers,
        tagline: "Griglia a 12 colonne e SimpleGrid a colonne uguali, con spaziature a token e breakpoint.",
        keywords: &["grid", "griglia", "colonne", "columns", "layout", "responsive", "simplegrid"],
        status: Status::Stable,
        playground: Some(Playground {
            component: "Grid",
            controls: vec![
                Control::choice("gutter", "Gutter", GUTTER_OPTS, 3),
                Control::choice("align", "Allineamento", ALIGN_OPTS, 3),
                Control::choice("justify", "Distribuzione", JUSTIFY_OPTS, 0)
                    .hint("Visibile sull'ultima riga, che occupa 6 colonne su 12 da md in su."),
            ],
            render,
            prelude_code: None,
            fixed_code: &[],
            children_code: Some(CHILDREN),
            stage: Stage::Start,
        }),
        examples: vec![
            Example {
                id: "offset",
                title: "Offset",
                description: "Celle spostate a destra di n colonne.",
                code: OFFSETS_CODE,
                render: || rsx! { Offsets {} },
                span: Span::Half,
            },
            Example {
                id: "responsive",
                title: "Span responsive",
                description: "Una colonna su mobile, due da sm, quattro da lg.",
                code: RESPONSIVE_CODE,
                render: || rsx! { Responsive {} },
                span: Span::Half,
            },
            Example {
                id: "simple",
                title: "SimpleGrid",
                description: "Colonne uguali senza calcolare gli span.",
                code: SIMPLE_CODE,
                render: || rsx! { Simple {} },
                span: Span::Half,
            },
            Example {
                id: "row-gutter",
                title: "Gutter di riga",
                description: "Spazio verticale diverso da quello tra le colonne.",
                code: ROW_GUTTER_CODE,
                render: || rsx! { RowGutter {} },
                span: Span::Half,
            },
            Example {
                id: "context",
                title: "In contesto",
                description: "Pannello principale e colonna di stato in un cruscotto.",
                code: IN_CONTEXT_CODE,
                render: || rsx! { InContext {} },
                span: Span::Full,
            },
        ],
        props: vec![
            PropGroup { component: "Grid", rows: GRID_PROPS },
            PropGroup { component: "GridCol", rows: COL_PROPS },
            PropGroup { component: "SimpleGrid", rows: SIMPLE_PROPS },
        ],
        recipe: Some(Recipe {
            recipe: "Primitiva di layout (nessuna superficie, language.md §2 Void)",
            depth: "Void: la griglia non disegna nulla, ospita contenitori flat-lit",
            tokens: &["--sd-space-1", "--sd-space-2", "--sd-space-4", "--sd-space-6", "--sd-space-8"],
            notes: "Solo gap a token, mai margini: le card figlie portano la propria superficie.",
        }),
        a11y: &[
            "Contenitore div senza ruolo: l'ordine di lettura segue l'ordine nel sorgente.",
            "Offset e span cambiano solo la posizione visiva, mai l'ordine del DOM.",
            "Nessun elemento interattivo: il focus passa direttamente ai figli.",
        ],
        related: &["group", "stack", "masonry", "app-shell", "cards"],
    }
}
