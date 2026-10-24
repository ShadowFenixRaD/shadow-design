//! Story: MasonryGrid and ImageList (feature `masonry`).
use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::story::schema::*;
use crate::story::sections::DemoCell;

const COLS: [ResponsiveCols; 4] = [
    ResponsiveCols { base: 1, sm: 2, md: 3, lg: 4, xl: 4 },
    ResponsiveCols { base: 2, sm: 2, md: 2, lg: 2, xl: 2 },
    ResponsiveCols { base: 3, sm: 3, md: 3, lg: 3, xl: 3 },
    ResponsiveCols { base: 4, sm: 4, md: 4, lg: 4, xl: 4 },
];
const COLS_OPTS: &[ChoiceOption] = &[
    opt("Responsive", "ResponsiveCols::default()"),
    opt("2", "ResponsiveCols::fixed(2)"),
    opt("3", "ResponsiveCols::fixed(3)"),
    opt("4", "ResponsiveCols::fixed(4)"),
];
const GAPS: [ComponentSpacing; 6] = [
    ComponentSpacing::None,
    ComponentSpacing::Xs,
    ComponentSpacing::Sm,
    ComponentSpacing::Md,
    ComponentSpacing::Lg,
    ComponentSpacing::Xl,
];
const GAP_OPTS: &[ChoiceOption] = &[
    opt("None", "ComponentSpacing::None"),
    opt("Xs", "ComponentSpacing::Xs"),
    opt("Sm", "ComponentSpacing::Sm"),
    opt("Md", "ComponentSpacing::Md"),
    opt("Lg", "ComponentSpacing::Lg"),
    opt("Xl", "ComponentSpacing::Xl"),
];

const CHILDREN: &str = r#"for i in 1..=10 {
    DemoCell { index: Some(i), tall: i % 3 == 1, short: i % 3 == 2 }
}"#;

fn render(k: &Knobs) -> Element {
    rsx! {
        MasonryGrid { cols: k.pick("cols", &COLS), gap: k.pick("gap", &GAPS),
            for i in 1..=10 {
                DemoCell { key: "{i}", index: Some(i), tall: i % 3 == 1, short: i % 3 == 2 }
            }
        }
    }
}

#[component]
fn Gallery() -> Element {
    rsx! {
        ImageList {
            items: vec![
                MasonryItem::new("1", "https://picsum.photos/seed/shadow1/600/400").with_title("Sala server").with_subtitle("Francoforte"),
                MasonryItem::new("2", "https://picsum.photos/seed/shadow2/600/750").with_title("Rack edge-07").with_subtitle("Milano"),
                MasonryItem::new("3", "https://picsum.photos/seed/shadow3/600/500").with_title("Antenne mesh").with_subtitle("Torino"),
                MasonryItem::new("4", "https://picsum.photos/seed/shadow4/600/650").with_title("Laboratorio").with_subtitle("Bologna"),
                MasonryItem::new("5", "https://picsum.photos/seed/shadow5/600/450").with_title("Offsite").with_subtitle("Lago di Como"),
                MasonryItem::new("6", "https://picsum.photos/seed/shadow6/600/800").with_title("Cablaggio").with_subtitle("Roma"),
            ],
            cols: ResponsiveCols::responsive(1, 2, 3, 3, 3),
            gap: ComponentSpacing::Sm,
        }
    }
}
const GALLERY_CODE: &str = r#"ImageList {
    items: vec![
        MasonryItem::new("1", "https://picsum.photos/seed/shadow1/600/400").with_title("Sala server").with_subtitle("Francoforte"),
        MasonryItem::new("2", "https://picsum.photos/seed/shadow2/600/750").with_title("Rack edge-07").with_subtitle("Milano"),
        MasonryItem::new("3", "https://picsum.photos/seed/shadow3/600/500").with_title("Antenne mesh").with_subtitle("Torino"),
        MasonryItem::new("4", "https://picsum.photos/seed/shadow4/600/650").with_title("Laboratorio").with_subtitle("Bologna"),
        MasonryItem::new("5", "https://picsum.photos/seed/shadow5/600/450").with_title("Offsite").with_subtitle("Lago di Como"),
        MasonryItem::new("6", "https://picsum.photos/seed/shadow6/600/800").with_title("Cablaggio").with_subtitle("Roma"),
    ],
    cols: ResponsiveCols::responsive(1, 2, 3, 3, 3),
    gap: ComponentSpacing::Sm,
}"#;

#[component]
fn InContext() -> Element {
    rsx! {
        MasonryGrid { cols: ResponsiveCols::responsive(1, 2, 3, 3, 3), gap: ComponentSpacing::Md,
            Card { title: Some("Incidente INC-142".to_string()), subtitle: Some("Risolto · 18 min".to_string()),
                Text { size: TextSize::Sm, dimmed: true, "Latenza sopra soglia su edge-03 dopo l'aggiornamento del firmware. Rollback automatico." }
            }
            Card { title: Some("Rilascio 0.5.2".to_string()),
                Stack { gap: SpacingToken::Xs,
                    Text { size: TextSize::Sm, "Nuovo motore dei temi" }
                    Text { size: TextSize::Sm, "Tipografia a tre ruoli" }
                    Text { size: TextSize::Sm, "Storie dello showcase" }
                }
            }
            Card { title: Some("Nota".to_string()),
                Text { size: TextSize::Sm, dimmed: true, "Ruotare le chiavi di policy.toml prima di venerdì." }
            }
            Card { title: Some("Capacità".to_string()), subtitle: Some("Disco dei nodi".to_string()),
                Stack { gap: SpacingToken::Sm,
                    ProgressBar { value: 42.0, show_label: true, label: Some("edge-01".to_string()) }
                    ProgressBar { value: 88.0, variant: ProgressVariant::Warning, show_label: true, label: Some("edge-03".to_string()) }
                    ProgressBar { value: 57.0, show_label: true, label: Some("edge-07".to_string()) }
                }
            }
            Card { title: Some("Turno di reperibilità".to_string()), subtitle: Some("Questa settimana".to_string()),
                Group { gap: SpacingToken::Sm,
                    Avatar { name: Some("Giulia Ferri".to_string()), size: AvatarSize::Sm }
                    Text { size: TextSize::Sm, "Giulia Ferri" }
                }
            }
        }
    }
}
const IN_CONTEXT_CODE: &str = r#"MasonryGrid { cols: ResponsiveCols::responsive(1, 2, 3, 3, 3), gap: ComponentSpacing::Md,
    Card { title: Some("Incidente INC-142".to_string()), subtitle: Some("Risolto · 18 min".to_string()),
        Text { size: TextSize::Sm, dimmed: true, "Latenza sopra soglia su edge-03 dopo l'aggiornamento del firmware. Rollback automatico." }
    }
    Card { title: Some("Rilascio 0.5.2".to_string()),
        Stack { gap: SpacingToken::Xs,
            Text { size: TextSize::Sm, "Nuovo motore dei temi" }
            Text { size: TextSize::Sm, "Tipografia a tre ruoli" }
            Text { size: TextSize::Sm, "Storie dello showcase" }
        }
    }
    Card { title: Some("Nota".to_string()),
        Text { size: TextSize::Sm, dimmed: true, "Ruotare le chiavi di policy.toml prima di venerdì." }
    }
    Card { title: Some("Capacità".to_string()), subtitle: Some("Disco dei nodi".to_string()),
        Stack { gap: SpacingToken::Sm,
            ProgressBar { value: 42.0, show_label: true, label: Some("edge-01".to_string()) }
            ProgressBar { value: 88.0, variant: ProgressVariant::Warning, show_label: true, label: Some("edge-03".to_string()) }
            ProgressBar { value: 57.0, show_label: true, label: Some("edge-07".to_string()) }
        }
    }
    Card { title: Some("Turno di reperibilità".to_string()), subtitle: Some("Questa settimana".to_string()),
        Group { gap: SpacingToken::Sm,
            Avatar { name: Some("Giulia Ferri".to_string()), size: AvatarSize::Sm }
            Text { size: TextSize::Sm, "Giulia Ferri" }
        }
    }
}"#;

const GRID_PROPS: &[PropDoc] = &[
    prop("cols", "ResponsiveCols", "default() = 1/2/3/4/4", "Colonne per breakpoint: fixed(n) o responsive(base, sm, md, lg, xl)."),
    prop("gap", "ComponentSpacing", "Sm", "Spazio tra colonne e righe: None, Xs, Sm, Md, Lg, Xl."),
    prop("class", "String", "\"\"", "Classi aggiuntive sul contenitore."),
    prop("children", "Element", "-", "Elementi di altezza libera, distribuiti a cascata."),
];
const LIST_PROPS: &[PropDoc] = &[
    prop("items", "Vec<MasonryItem>", "-", "Immagini con id, src, titolo e didascalia."),
    prop("cols", "ResponsiveCols", "default()", "Come MasonryGrid."),
    prop("gap", "ComponentSpacing", "Sm", "Come MasonryGrid."),
    prop("with_lightbox", "bool", "true", "Apre l'immagine a tutto schermo con frecce e chiusura."),
    prop("class", "String", "\"\"", "Classi aggiuntive."),
];
const ITEM_PROPS: &[PropDoc] = &[
    prop("MasonryItem::new(id, src)", "fn", "-", "Costruttore."),
    prop(".with_title(..)", "fn", "-", "Titolo: didascalia e testo alternativo."),
    prop(".with_subtitle(..)", "fn", "-", "Riga secondaria della didascalia e del lightbox."),
    prop(".with_aspect_ratio(..)", "fn", "-", "Rapporto di forma noto in anticipo."),
];

pub fn story() -> Story {
    Story {
        slug: "masonry",
        name: "MasonryGrid",
        category: Category::Layout,
        features: &["masonry"],
        icon: IconKind::Layers,
        tagline: "Griglia a cascata per contenuti di altezza diversa e gallerie con lightbox.",
        keywords: &["masonry", "cascata", "galleria", "gallery", "pinterest", "immagini", "lightbox"],
        status: Status::Stable,
        playground: Some(Playground {
            component: "MasonryGrid",
            controls: vec![
                Control::choice("cols", "Colonne", COLS_OPTS, 0),
                Control::choice("gap", "Gap", GAP_OPTS, 2),
            ],
            render,
            prelude_code: None,
            fixed_code: &[],
            children_code: Some(CHILDREN),
            stage: Stage::Start,
        }),
        examples: vec![
            Example {
                id: "gallery",
                title: "ImageList con lightbox",
                description: "Clicca una foto: si apre a tutto schermo con frecce e Esc.",
                code: GALLERY_CODE,
                render: || rsx! { Gallery {} },
                span: Span::Full,
            },
            Example {
                id: "context",
                title: "In contesto",
                description: "Bacheca del team con card di altezza diversa.",
                code: IN_CONTEXT_CODE,
                render: || rsx! { InContext {} },
                span: Span::Full,
            },
        ],
        props: vec![
            PropGroup { component: "MasonryGrid", rows: GRID_PROPS },
            PropGroup { component: "ImageList", rows: LIST_PROPS },
            PropGroup { component: "MasonryItem", rows: ITEM_PROPS },
        ],
        recipe: Some(Recipe {
            recipe: "Primitiva di layout (§2 Void) + interactive container per le immagini (§3.1) + backdrop (§3.3)",
            depth: "Void (griglia) · Raised-max (lightbox)",
            tokens: &["--sd-space-2", "--sd-space-4", "--sd-radius-md", "--sd-scrim", "--sd-backdrop-blur", "--sd-border-hover"],
            notes: "Le colonne sono CSS columns guidate da variabili inline; il lightbox usa lo stesso velo di modali e drawer.",
        }),
        a11y: &[
            "Con with_lightbox ogni immagine ha role=button, tabindex 0 e si apre con Invio o Spazio.",
            "Il lightbox ha role=dialog, aria-modal e il titolo dell'immagine come aria-label; Esc lo chiude.",
            "Chiudi, Precedente e Successiva sono pulsanti nativi con etichette da Locale.",
            "Il testo alternativo di ogni immagine è il suo titolo: impostalo sempre.",
        ],
        related: &["grid", "aspect-ratio", "cards", "carousel"],
    }
}
