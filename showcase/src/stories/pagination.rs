//! Story: Pagination (feature `pagination`). Stateful playground (`PaginationDemo { knobs }`).
use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::story::schema::*;

const SIZES: [ControlSize; 3] = [ControlSize::Sm, ControlSize::Md, ControlSize::Lg];
const SIZE_OPTS: &[ChoiceOption] = &[opt("Sm", "ControlSize::Sm"), opt("Md", "ControlSize::Md"), opt("Lg", "ControlSize::Lg")];

const PRELUDE: &str = "let page = use_signal(|| 7usize);";

#[component]
fn PaginationDemo(knobs: Knobs) -> Element {
    let page = use_signal(|| 7usize);
    rsx! {
        Pagination {
            page,
            total: knobs.number("total") as usize,
            siblings: knobs.number("siblings") as usize,
            boundaries: knobs.number("boundaries") as usize,
            size: knobs.pick("size", &SIZES),
            show_controls: knobs.bool("show_controls"),
            show_edges: knobs.bool("show_edges"),
            disabled: knobs.bool("disabled"),
        }
    }
}

fn render(k: &Knobs) -> Element {
    rsx! {
        PaginationDemo { knobs: k.clone() }
    }
}

#[component]
fn Sizes() -> Element {
    let small = use_signal(|| 2usize);
    let large = use_signal(|| 2usize);
    rsx! {
        Stack { gap: SpacingToken::Md,
            Pagination { page: small, total: 5, size: ControlSize::Sm }
            Pagination { page: large, total: 5, size: ControlSize::Lg }
        }
    }
}
const SIZES_CODE: &str = r#"let small = use_signal(|| 2usize);
let large = use_signal(|| 2usize);

Stack { gap: SpacingToken::Md,
    Pagination { page: small, total: 5, size: ControlSize::Sm }
    Pagination { page: large, total: 5, size: ControlSize::Lg }
}"#;

#[component]
fn Edges() -> Element {
    let page = use_signal(|| 48usize);
    rsx! {
        Pagination { page, total: 120, show_edges: true, siblings: 0 }
    }
}
const EDGES_CODE: &str = r#"let page = use_signal(|| 48usize);

Pagination { page, total: 120, show_edges: true, siblings: 0 }"#;

#[component]
fn InContext() -> Element {
    rsx! {
        crate::scenes::InboxPane { compact: true }
    }
}
const IN_CONTEXT_CODE: &str = r#"// scenes/inbox_pane.rs (estratto): la paginazione nel piè della lista messaggi.
let page = use_signal(|| 1usize);

div {
    span { "Tasto destro su un messaggio per le azioni rapide" }
    Pagination { page, total: 4, size: ControlSize::Sm }
}"#;

const PROPS: &[PropDoc] = &[
    prop("page", "Signal<usize>", "-", "Pagina corrente, da 1."),
    prop("total", "usize", "-", "Numero di pagine."),
    prop("siblings", "usize", "1", "Pagine mostrate a fianco di quella corrente."),
    prop("boundaries", "usize", "1", "Pagine sempre visibili all'inizio e alla fine."),
    prop("show_controls", "bool", "true", "Frecce precedente e successiva."),
    prop("show_edges", "bool", "false", "Pulsanti prima e ultima pagina."),
    prop("size", "ControlSize", "Md", "Sm, Md o Lg."),
    prop("disabled", "bool", "false", "Disabilita tutti i controlli."),
    prop("on_change", "Option<EventHandler<usize>>", "None", "Nuova pagina scelta."),
    prop("class", "String", "\"\"", "Classi aggiuntive."),
];

pub fn story() -> Story {
    Story {
        slug: "pagination",
        name: "Pagination",
        category: Category::Navigazione,
        features: &["pagination"],
        icon: IconKind::MoreHorizontal,
        tagline: "Navigazione tra pagine numerate con salti ellittici per elenchi lunghi.",
        keywords: &["pagination", "paginazione", "pagine", "pager", "pagine numerate", "elenchi"],
        status: Status::Stable,
        playground: Some(Playground {
            component: "Pagination",
            controls: vec![
                Control::number("total", "Pagine totali", 20.0, 1.0, 50.0, 1.0, 0).emit(Emit::Always),
                Control::number("siblings", "Vicine", 1.0, 0.0, 3.0, 1.0, 0),
                Control::number("boundaries", "Estremi fissi", 1.0, 0.0, 3.0, 1.0, 0),
                Control::choice("size", "Dimensione", SIZE_OPTS, 1),
                Control::bool("show_controls", "Frecce", true),
                Control::bool("show_edges", "Prima e ultima", false),
                Control::bool("disabled", "Disabilitata", false),
            ],
            render,
            prelude_code: Some(PRELUDE),
            fixed_code: &["page,"],
            children_code: None,
            stage: Stage::Center,
        }),
        examples: vec![
            Example {
                id: "sizes",
                title: "Dimensioni",
                description: "Sm per i piè di tabella, Lg per le pagine di risultati.",
                code: SIZES_CODE,
                render: || rsx! { Sizes {} },
                span: Span::Half,
            },
            Example {
                id: "edges",
                title: "Elenchi lunghi",
                description: "120 pagine con salti alla prima e all'ultima.",
                code: EDGES_CODE,
                render: || rsx! { Edges {} },
                span: Span::Half,
            },
            Example {
                id: "context",
                title: "In contesto",
                description: "Il piè della casella operatori.",
                code: IN_CONTEXT_CODE,
                render: || rsx! { InContext {} },
                span: Span::Full,
            },
        ],
        props: vec![PropGroup { component: "Pagination", rows: PROPS }],
        recipe: Some(Recipe {
            recipe: "Track di item (§3.4) con pagina corrente sd-selected (§3.7)",
            depth: "Well (track) · item",
            tokens: &["--sd-well", "--sd-border", "--sd-item-h", "--sd-radius-sm", "--sd-accent-soft", "--sd-accent-strong", "--sd-glow-sm"],
            notes: "Numeri tabulari; una sola pagina selezionata; le frecce si disabilitano agli estremi.",
        }),
        a11y: &[
            "Rende un <nav> con aria-label \"Paginazione\" dal Locale.",
            "La pagina corrente ha aria-current=page; ogni numero ha aria-label \"Pagina n\".",
            "Frecce ed estremi hanno etichette da Locale (Pagina precedente, Pagina successiva…).",
            "Le ellissi sono aria-hidden.",
        ],
        related: &["tables", "breadcrumbs", "stepper", "virtual-list"],
    }
}
