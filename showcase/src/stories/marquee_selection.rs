//! Story: MarqueeSelection (feature `marquee_selection`). The selected indices live in a signal,
//! so the playground goes through `MarqueeDemo { knobs }`.
//!
//! Items are plain `div`s carrying `data-marquee-item`: the library hit-tests them and styles the
//! covered ones with `sd-marquee-item--selected`. The live tiles add the showcase-only
//! `sc-demo-cell` class for their frame; the samples omit it.
use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::story::schema::*;

const FILES: [&str; 9] = [
    "main.rs", "lib.rs", "theme.rs", "Cargo.toml", "main.scss", "_tree.scss", "README.md", "CHANGELOG.md", "flake.nix",
];

fn item_class(selected: &[usize], i: usize) -> &'static str {
    if selected.contains(&i) { "sc-demo-cell sd-marquee-item--selected" } else { "sc-demo-cell" }
}

const PRELUDE: &str = r#"let mut selected = use_signal(Vec::<usize>::new);
const FILES: [&str; 9] = [
    "main.rs", "lib.rs", "theme.rs", "Cargo.toml", "main.scss", "_tree.scss", "README.md", "CHANGELOG.md", "flake.nix",
];"#;

const GRID_CODE: &str = r#"SimpleGrid { cols: 2, sm: Some(3), spacing: GridSpacing::Sm,
    for (i, name) in FILES.iter().enumerate() {
        div {
            "data-marquee-item": "{i}",
            class: if selected().contains(&i) { "sd-marquee-item--selected" } else { "" },
            Icon { kind: IconKind::Code, size: 14 }
            "{name}"
        }
    }
}"#;

#[component]
fn MarqueeDemo(knobs: Knobs) -> Element {
    let mut selected = use_signal(Vec::<usize>::new);
    rsx! {
        MarqueeSelection {
            enabled: knobs.bool("enabled"),
            on_select_indices: move |indices: Vec<usize>| selected.set(indices),
            SimpleGrid { cols: 2, sm: Some(3), spacing: GridSpacing::Sm,
                for (i, name) in FILES.iter().enumerate() {
                    div {
                        key: "{i}",
                        "data-marquee-item": "{i}",
                        class: item_class(&selected(), i),
                        Icon { kind: IconKind::Code, size: 14 }
                        "{name}"
                    }
                }
            }
        }
    }
}

fn render(k: &Knobs) -> Element {
    rsx! {
        MarqueeDemo { knobs: k.clone() }
    }
}

const NODES: [&str; 5] = ["edge-07 · Milano", "edge-22 · Milano", "edge-31 · Francoforte", "relay-mi · Milano", "vault-01 · Parigi"];

#[component]
fn Rows() -> Element {
    let mut selected = use_signal(Vec::<usize>::new);
    let mut toast = use_notification();
    rsx! {
        MarqueeSelection {
            on_select_indices: move |indices: Vec<usize>| {
                toast.info(format!("{} nodi selezionati", indices.len()));
                selected.set(indices);
            },
            Stack { gap: SpacingToken::Xs,
                for (i, node) in NODES.iter().enumerate() {
                    div {
                        key: "{i}",
                        "data-marquee-item": "{i}",
                        class: item_class(&selected(), i),
                        "{node}"
                    }
                }
            }
        }
    }
}
const ROWS_CODE: &str = r#"let mut selected = use_signal(Vec::<usize>::new);
let mut toast = use_notification();

MarqueeSelection {
    on_select_indices: move |indices: Vec<usize>| {
        toast.info(format!("{} nodi selezionati", indices.len()));
        selected.set(indices);
    },
    Stack { gap: SpacingToken::Xs,
        for (i, node) in NODES.iter().enumerate() {
            div {
                "data-marquee-item": "{i}",
                class: if selected().contains(&i) { "sd-marquee-item--selected" } else { "" },
                "{node}"
            }
        }
    }
}"#;

#[component]
fn InContext() -> Element {
    let mut selected = use_signal(Vec::<usize>::new);
    let mut toast = use_notification();
    let count = selected.read().len();
    rsx! {
        Stack { gap: SpacingToken::Md,
            Group { gap: SpacingToken::Sm, justify: JustifyContent::SpaceBetween,
                Badge {
                    label: format!("{count} di {} file", FILES.len()),
                    variant: if count > 0 { BadgeVariant::Primary } else { BadgeVariant::Default },
                    pill: true,
                }
                Group { gap: SpacingToken::Sm,
                    Button {
                        variant: ButtonVariant::Ghost,
                        disabled: count == 0,
                        onclick: move |_| selected.set(Vec::new()),
                        "Deseleziona"
                    }
                    Button {
                        variant: ButtonVariant::Danger,
                        disabled: count == 0,
                        left_icon: Some(rsx! {
                            Icon { kind: IconKind::Trash2, size: 15 }
                        }),
                        onclick: move |_| {
                            toast.warning(format!("{} file spostati nel cestino", selected.read().len()));
                            selected.set(Vec::new());
                        },
                        "Elimina"
                    }
                }
            }
            MarqueeSelection {
                on_select_indices: move |indices: Vec<usize>| selected.set(indices),
                SimpleGrid { cols: 2, sm: Some(3), md: Some(5), spacing: GridSpacing::Sm,
                    for (i, name) in FILES.iter().enumerate() {
                        div {
                            key: "{i}",
                            "data-marquee-item": "{i}",
                            class: item_class(&selected(), i),
                            Icon { kind: IconKind::Code, size: 14 }
                            "{name}"
                        }
                    }
                }
            }
        }
    }
}
const IN_CONTEXT_CODE: &str = r#"let mut selected = use_signal(Vec::<usize>::new);
let mut toast = use_notification();
let count = selected.read().len();

Stack { gap: SpacingToken::Md,
    Group { gap: SpacingToken::Sm, justify: JustifyContent::SpaceBetween,
        Badge {
            label: format!("{count} di {} file", FILES.len()),
            variant: if count > 0 { BadgeVariant::Primary } else { BadgeVariant::Default },
            pill: true,
        }
        Group { gap: SpacingToken::Sm,
            Button {
                variant: ButtonVariant::Ghost,
                disabled: count == 0,
                onclick: move |_| selected.set(Vec::new()),
                "Deseleziona"
            }
            Button {
                variant: ButtonVariant::Danger,
                disabled: count == 0,
                left_icon: Some(rsx! {
                    Icon { kind: IconKind::Trash2, size: 15 }
                }),
                onclick: move |_| {
                    toast.warning(format!("{} file spostati nel cestino", selected.read().len()));
                    selected.set(Vec::new());
                },
                "Elimina"
            }
        }
    }
    MarqueeSelection {
        on_select_indices: move |indices: Vec<usize>| selected.set(indices),
        SimpleGrid { cols: 2, sm: Some(3), md: Some(5), spacing: GridSpacing::Sm,
            for (i, name) in FILES.iter().enumerate() {
                div {
                    "data-marquee-item": "{i}",
                    class: if selected().contains(&i) { "sd-marquee-item--selected" } else { "" },
                    Icon { kind: IconKind::Code, size: 14 }
                    "{name}"
                }
            }
        }
    }
}"#;

const PROPS: &[PropDoc] = &[
    prop("children", "Element", "-", "Il contenuto; gli elementi selezionabili portano l'attributo data-marquee-item con il proprio indice."),
    prop("enabled", "bool", "true", "Attiva il trascinamento; false lascia il contenuto inerte."),
    prop("on_select_indices", "Option<EventHandler<Vec<usize>>>", "None", "Al rilascio: gli indici degli elementi toccati dal rettangolo."),
    prop("on_change", "Option<EventHandler<MarqueeRect>>", "None", "Al rilascio: il rettangolo (x, y, width, height) relativo al contenitore."),
    prop("class", "String", "\"\"", "Classi aggiuntive sul contenitore."),
];
const RECT_PROPS: &[PropDoc] = &[
    prop("x / y", "f64", "-", "Angolo in alto a sinistra, in px dal contenitore."),
    prop("width / height", "f64", "-", "Dimensioni in px; sotto i 3 px il trascinamento è un clic e non emette nulla."),
];

pub fn story() -> Story {
    Story {
        slug: "marquee-selection",
        name: "MarqueeSelection",
        category: Category::Utility,
        features: &["marquee_selection"],
        icon: IconKind::Layers,
        tagline: "Seleziona più elementi trascinando un rettangolo, come nella vista a icone di un file manager.",
        keywords: &["marquee", "lasso", "selezione multipla", "trascina", "rubber band", "drag select"],
        status: Status::Stable,
        playground: Some(Playground {
            component: "MarqueeSelection",
            controls: vec![Control::bool("enabled", "Attivo", true).hint("Trascina sopra i file per selezionarli.")],
            render,
            prelude_code: Some(PRELUDE),
            fixed_code: &["on_select_indices: move |indices: Vec<usize>| selected.set(indices),"],
            children_code: Some(GRID_CODE),
            stage: Stage::Start,
        }),
        examples: vec![
            Example {
                id: "rows",
                title: "Righe di un elenco",
                description: "Funziona su qualsiasi layout: qui una colonna di nodi.",
                code: ROWS_CODE,
                render: || rsx! { Rows {} },
                span: Span::Half,
            },
            Example {
                id: "context",
                title: "In contesto",
                description: "Una cartella di file: seleziona col rettangolo, poi agisci dalla barra.",
                code: IN_CONTEXT_CODE,
                render: || rsx! { InContext {} },
                span: Span::Full,
            },
        ],
        props: vec![
            PropGroup { component: "MarqueeSelection", rows: PROPS },
            PropGroup { component: "MarqueeRect", rows: RECT_PROPS },
        ],
        recipe: Some(Recipe {
            recipe: "Selected (language.md §3.7) + rettangolo in tinta accento",
            depth: "Velo sopra il contenuto",
            tokens: &["--sd-accent-soft", "--sd-accent-border", "--sd-accent-strong", "--sd-glow-sm", "--sd-radius-sm"],
            notes: "Il rettangolo è accent-soft con bordo tratteggiato; gli elementi coperti prendono sd-selected.",
        }),
        a11y: &[
            "Il rettangolo funziona solo con puntatore (mouse, penna, touch): offri sempre un'alternativa da tastiera.",
            "Il contenitore non ha ruolo: se gli elementi sono selezionabili, usa controlli con stato (Checkbox, aria-selected) per annunciarlo.",
            "Il rettangolo è decorativo e ha pointer-events: none; il contenuto resta leggibile e cliccabile.",
        ],
        related: &["transfer-list", "checkbox", "lists", "grid"],
    }
}
