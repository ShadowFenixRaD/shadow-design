//! Story: Breadcrumbs and BreadcrumbItem (feature `breadcrumbs`).
use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::story::schema::*;

const SEPARATORS: [Option<&str>; 3] = [None, Some("/"), Some("·")];
const SEPARATOR_OPTS: &[ChoiceOption] = &[
    opt("Chevron", "None"),
    opt("/", "Some(\"/\".to_string())"),
    opt("·", "Some(\"·\".to_string())"),
];

fn path() -> Vec<BreadcrumbData> {
    vec![
        BreadcrumbData::new("Home").with_href("#").with_icon(SvgIcon::kind(IconKind::Box)),
        BreadcrumbData::new("Organizzazione").with_href("#"),
        BreadcrumbData::new("Progetti").with_href("#"),
        BreadcrumbData::new("Mesh Europa").with_href("#"),
        BreadcrumbData::new("Nodi").with_href("#"),
        BreadcrumbData::new("edge-07"),
    ]
}

const PRELUDE: &str = r##"let items = vec![
    BreadcrumbData::new("Home").with_href("#").with_icon(SvgIcon::kind(IconKind::Box)),
    BreadcrumbData::new("Organizzazione").with_href("#"),
    BreadcrumbData::new("Progetti").with_href("#"),
    BreadcrumbData::new("Mesh Europa").with_href("#"),
    BreadcrumbData::new("Nodi").with_href("#"),
    BreadcrumbData::new("edge-07"),
];"##;

fn render(k: &Knobs) -> Element {
    let max = k.number("max_items") as usize;
    rsx! {
        Breadcrumbs {
            items: Some(path()),
            max_items: Some(max),
            separator_text: k.pick("separator_text", &SEPARATORS).map(str::to_string),
        }
    }
}

#[component]
fn Composed() -> Element {
    let mut toast = use_notification();
    rsx! {
        Breadcrumbs {
            BreadcrumbItem {
                icon: Some(SvgIcon::kind(IconKind::Server)),
                on_click: move |_| {
                    toast.info("Torna all'elenco dei nodi");
                },
                "Nodi"
            }
            li { class: "sd-breadcrumbs__separator", aria_hidden: "true", "/" }
            BreadcrumbItem { is_current: true, "edge-07" }
        }
    }
}
const COMPOSED_CODE: &str = r#"let mut toast = use_notification();

Breadcrumbs {
    BreadcrumbItem {
        icon: Some(SvgIcon::kind(IconKind::Server)),
        on_click: move |_| {
            toast.info("Torna all'elenco dei nodi");
        },
        "Nodi"
    }
    li { class: "sd-breadcrumbs__separator", aria_hidden: "true", "/" }
    BreadcrumbItem { is_current: true, "edge-07" }
}"#;

#[component]
fn Collapsed() -> Element {
    let expanded = use_signal(|| false);
    rsx! {
        Stack { gap: SpacingToken::Sm,
            Breadcrumbs {
                items: Some(vec![
                    BreadcrumbData::new("Archivio").with_href("#"),
                    BreadcrumbData::new("2026").with_href("#"),
                    BreadcrumbData::new("Ottobre").with_href("#"),
                    BreadcrumbData::new("Settimana 43").with_href("#"),
                    BreadcrumbData::new("Rapporto incidenti"),
                ]),
                max_items: Some(3),
                expanded: Some(expanded),
            }
            Text { size: TextSize::Xs, dimmed: true,
                if expanded() { "Percorso completo" } else { "Percorso compresso: clicca i puntini" }
            }
        }
    }
}
const COLLAPSED_CODE: &str = r##"let expanded = use_signal(|| false);

Stack { gap: SpacingToken::Sm,
    Breadcrumbs {
        items: Some(vec![
            BreadcrumbData::new("Archivio").with_href("#"),
            BreadcrumbData::new("2026").with_href("#"),
            BreadcrumbData::new("Ottobre").with_href("#"),
            BreadcrumbData::new("Settimana 43").with_href("#"),
            BreadcrumbData::new("Rapporto incidenti"),
        ]),
        max_items: Some(3),
        expanded: Some(expanded),
    }
    Text { size: TextSize::Xs, dimmed: true,
        if expanded() { "Percorso completo" } else { "Percorso compresso: clicca i puntini" }
    }
}"##;

#[component]
fn InContext() -> Element {
    let mut toast = use_notification();
    rsx! {
        Card {
            Stack { gap: SpacingToken::Sm,
                Breadcrumbs {
                    items: Some(vec![
                        BreadcrumbData::new("Progetti").with_href("#"),
                        BreadcrumbData::new("Mesh Europa").with_href("#"),
                        BreadcrumbData::new("edge-07"),
                    ]),
                }
                Group { justify: JustifyContent::SpaceBetween,
                    Group { gap: SpacingToken::Sm,
                        Heading { order: HeadingOrder::H3, "edge-07" }
                        Badge { label: "Online".to_string(), variant: BadgeVariant::Success, dot: true }
                    }
                    Group { gap: SpacingToken::Sm,
                        Button { variant: ButtonVariant::Secondary, size: ButtonSize::Sm, "Riavvia" }
                        Button {
                            size: ButtonSize::Sm,
                            onclick: move |_| {
                                toast.success("Configurazione distribuita su edge-07");
                            },
                            "Distribuisci"
                        }
                    }
                }
            }
        }
    }
}
const IN_CONTEXT_CODE: &str = r##"let mut toast = use_notification();

Card {
    Stack { gap: SpacingToken::Sm,
        Breadcrumbs {
            items: Some(vec![
                BreadcrumbData::new("Progetti").with_href("#"),
                BreadcrumbData::new("Mesh Europa").with_href("#"),
                BreadcrumbData::new("edge-07"),
            ]),
        }
        Group { justify: JustifyContent::SpaceBetween,
            Group { gap: SpacingToken::Sm,
                Heading { order: HeadingOrder::H3, "edge-07" }
                Badge { label: "Online".to_string(), variant: BadgeVariant::Success, dot: true }
            }
            Group { gap: SpacingToken::Sm,
                Button { variant: ButtonVariant::Secondary, size: ButtonSize::Sm, "Riavvia" }
                Button {
                    size: ButtonSize::Sm,
                    onclick: move |_| {
                        toast.success("Configurazione distribuita su edge-07");
                    },
                    "Distribuisci"
                }
            }
        }
    }
}"##;

const BREADCRUMBS_PROPS: &[PropDoc] = &[
    prop("items", "Option<Vec<BreadcrumbData>>", "None", "Percorso come dati; l'ultima voce è la pagina corrente."),
    prop("max_items", "Option<usize>", "None", "Voci visibili prima di comprimere con \"…\"."),
    prop("items_before_collapse / items_after_collapse", "usize", "1", "Voci tenute prima e dopo i puntini."),
    prop("expanded", "Option<Signal<bool>>", "None", "Stato espanso controllato."),
    prop("on_toggle_expand", "Option<EventHandler<bool>>", "None", "Espansione o compressione."),
    prop("separator", "Option<Element>", "None", "Separatore personalizzato."),
    prop("separator_text", "Option<String>", "None", "Separatore testuale al posto del chevron."),
    prop("children", "Element", "-", "Composizione manuale con BreadcrumbItem, se items è None."),
    prop("class", "String", "\"\"", "Classi aggiuntive."),
];
const ITEM_PROPS: &[PropDoc] = &[
    prop("href", "Option<String>", "None", "Link della voce."),
    prop("on_click", "Option<EventHandler<MouseEvent>>", "None", "Senza href la voce diventa un <button>."),
    prop("is_current", "bool", "false", "Pagina corrente: aria-current=page, non cliccabile."),
    prop("icon", "Option<SvgIcon>", "None", "Icona prima dell'etichetta."),
    prop("class", "String", "\"\"", "Classi aggiuntive."),
];
const DATA_PROPS: &[PropDoc] = &[
    prop("BreadcrumbData::new(label)", "fn", "-", "Costruttore."),
    prop(".with_href(..)", "fn", "-", "Destinazione."),
    prop(".with_icon(SvgIcon)", "fn", "-", "Icona della voce."),
];

pub fn story() -> Story {
    Story {
        slug: "breadcrumbs",
        name: "Breadcrumbs",
        category: Category::Navigazione,
        features: &["breadcrumbs"],
        icon: IconKind::ChevronRight,
        tagline: "Mostra dove si trova la pagina nella gerarchia e permette di risalire.",
        keywords: &["breadcrumbs", "briciole", "percorso", "path", "gerarchia", "wayfinding"],
        status: Status::Stable,
        playground: Some(Playground {
            component: "Breadcrumbs",
            controls: vec![
                Control::number("max_items", "Voci visibili", 0.0, 0.0, 6.0, 1.0, 0)
                    .emit(Emit::Template("max_items: Some({v})"))
                    .hint("0 = nessuna compressione."),
                Control::choice("separator_text", "Separatore", SEPARATOR_OPTS, 0),
            ],
            render,
            prelude_code: Some(PRELUDE),
            fixed_code: &["items: Some(items),"],
            children_code: None,
            stage: Stage::Start,
        }),
        examples: vec![
            Example {
                id: "composed",
                title: "Composizione manuale",
                description: "BreadcrumbItem con on_click al posto di href.",
                code: COMPOSED_CODE,
                render: || rsx! { Composed {} },
                span: Span::Half,
            },
            Example {
                id: "collapsed",
                title: "Percorso compresso",
                description: "max_items 3: i puntini espandono il percorso.",
                code: COLLAPSED_CODE,
                render: || rsx! { Collapsed {} },
                span: Span::Half,
            },
            Example {
                id: "context",
                title: "In contesto",
                description: "Testata della pagina di un nodo, sopra titolo e azioni.",
                code: IN_CONTEXT_CODE,
                render: || rsx! { InContext {} },
                span: Span::Full,
            },
        ],
        props: vec![
            PropGroup { component: "Breadcrumbs", rows: BREADCRUMBS_PROPS },
            PropGroup { component: "BreadcrumbItem", rows: ITEM_PROPS },
            PropGroup { component: "BreadcrumbData", rows: DATA_PROPS },
        ],
        recipe: Some(Recipe {
            recipe: "Testo e link (language.md §3.12); separatori in --sd-text-subtle",
            depth: "Void: nessuna superficie",
            tokens: &["--sd-text", "--sd-text-muted", "--sd-text-subtle", "--sd-accent-strong", "--sd-text-sm", "--sd-focus-ring"],
            notes: "La voce corrente è --sd-text e non cliccabile; i link sono muted e prendono l'accento solo al passaggio.",
        }),
        a11y: &[
            "Rende un <nav> con aria-label da Locale e una lista ordinata <ol>.",
            "La voce corrente ha aria-current=page.",
            "I separatori sono aria-hidden: il lettore annuncia solo le voci.",
            "Il pulsante \"…\" ha un title da Locale (Mostra percorso completo / Comprimi percorso).",
        ],
        related: &["nav-link", "pagination", "navbar", "stepper"],
    }
}
