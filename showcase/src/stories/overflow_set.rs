//! Story: OverflowSet and ResponsiveToolbar (feature `overflow_set`). Stateful playground
//! (`ToolbarDemo { knobs }`): `on_action` shows a toast.
use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::story::schema::*;

fn actions() -> Vec<ToolbarAction> {
    vec![
        ToolbarAction::new("new", "Nuovo").with_icon(IconKind::Plus),
        ToolbarAction::new("copy", "Duplica").with_icon(IconKind::Copy),
        ToolbarAction::new("share", "Condividi").with_icon(IconKind::Share2),
        ToolbarAction::new("download", "Esporta").with_icon(IconKind::Download),
        ToolbarAction::new("lock", "Blocca").with_icon(IconKind::Lock),
        ToolbarAction::new("trash", "Elimina").with_icon(IconKind::Trash2).with_danger(true),
    ]
}

const PRELUDE: &str = r#"let mut toast = use_notification();
let actions = vec![
    ToolbarAction::new("new", "Nuovo").with_icon(IconKind::Plus),
    ToolbarAction::new("copy", "Duplica").with_icon(IconKind::Copy),
    ToolbarAction::new("share", "Condividi").with_icon(IconKind::Share2),
    ToolbarAction::new("download", "Esporta").with_icon(IconKind::Download),
    ToolbarAction::new("lock", "Blocca").with_icon(IconKind::Lock),
    ToolbarAction::new("trash", "Elimina").with_icon(IconKind::Trash2).with_danger(true),
];"#;

#[component]
fn ToolbarDemo(knobs: Knobs) -> Element {
    let mut toast = use_notification();
    rsx! {
        OverflowSet {
            items: actions(),
            on_action: move |id: String| {
                toast.info(format!("Azione: {id}"));
            },
            max_visible: Some(knobs.number("max_visible") as usize),
            overflow_button_label: knobs.text("overflow_button_label"),
        }
    }
}

fn render(k: &Knobs) -> Element {
    rsx! {
        ToolbarDemo { knobs: k.clone() }
    }
}

#[component]
fn Extra() -> Element {
    rsx! {
        OverflowSet {
            items: vec![
                ToolbarAction::new("bold", "Grassetto").with_shortcut("Ctrl+B"),
                ToolbarAction::new("italic", "Corsivo").with_shortcut("Ctrl+I"),
            ],
            overflow_items: vec![
                ToolbarAction::new("history", "Cronologia").with_icon(IconKind::Clock),
                ToolbarAction::new("settings", "Impostazioni").with_icon(IconKind::Settings),
            ],
            overflow_button_label: "Altri strumenti".to_string(),
        }
    }
}
const EXTRA_CODE: &str = r#"OverflowSet {
    items: vec![
        ToolbarAction::new("bold", "Grassetto").with_shortcut("Ctrl+B"),
        ToolbarAction::new("italic", "Corsivo").with_shortcut("Ctrl+I"),
    ],
    overflow_items: vec![
        ToolbarAction::new("history", "Cronologia").with_icon(IconKind::Clock),
        ToolbarAction::new("settings", "Impostazioni").with_icon(IconKind::Settings),
    ],
    overflow_button_label: "Altri strumenti".to_string(),
}"#;

#[component]
fn Disabled() -> Element {
    rsx! {
        OverflowSet {
            items: vec![
                ToolbarAction::new("publish", "Pubblica").with_icon(IconKind::Upload),
                ToolbarAction::new("revert", "Ripristina").with_disabled(true),
                ToolbarAction::new("delete", "Elimina").with_icon(IconKind::Trash2).with_danger(true),
            ],
        }
    }
}
const DISABLED_CODE: &str = r#"OverflowSet {
    items: vec![
        ToolbarAction::new("publish", "Pubblica").with_icon(IconKind::Upload),
        ToolbarAction::new("revert", "Ripristina").with_disabled(true),
        ToolbarAction::new("delete", "Elimina").with_icon(IconKind::Trash2).with_danger(true),
    ],
}"#;

#[component]
fn InContext() -> Element {
    let mut toast = use_notification();
    rsx! {
        Card {
            title: Some("rapporto-q3.pdf".to_string()),
            subtitle: Some("2,4 MB · modificato ieri da Marco Lenzi".to_string()),
            buttons: Some(rsx! {
                ResponsiveToolbar {
                    items: vec![
                        ToolbarAction::new("open", "Apri").with_icon(IconKind::ExternalLink),
                        ToolbarAction::new("download", "Scarica").with_icon(IconKind::Download),
                        ToolbarAction::new("share", "Condividi").with_icon(IconKind::Share2),
                        ToolbarAction::new("lock", "Proteggi").with_icon(IconKind::Lock),
                        ToolbarAction::new("delete", "Elimina").with_icon(IconKind::Trash2).with_danger(true),
                    ],
                    max_visible: Some(2),
                    overflow_button_label: "Altre azioni sul file".to_string(),
                    on_action: move |id: String| {
                        if id == "delete" {
                            toast.warning("rapporto-q3.pdf spostato nel cestino");
                        } else {
                            toast.info(format!("Azione {id} su rapporto-q3.pdf"));
                        }
                    },
                }
            }),
            Text { size: TextSize::Sm, dimmed: true, "Le prime due azioni restano visibili; le altre finiscono nel menu." }
        }
    }
}
const IN_CONTEXT_CODE: &str = r#"let mut toast = use_notification();

Card {
    title: Some("rapporto-q3.pdf".to_string()),
    subtitle: Some("2,4 MB · modificato ieri da Marco Lenzi".to_string()),
    buttons: Some(rsx! {
        ResponsiveToolbar {
            items: vec![
                ToolbarAction::new("open", "Apri").with_icon(IconKind::ExternalLink),
                ToolbarAction::new("download", "Scarica").with_icon(IconKind::Download),
                ToolbarAction::new("share", "Condividi").with_icon(IconKind::Share2),
                ToolbarAction::new("lock", "Proteggi").with_icon(IconKind::Lock),
                ToolbarAction::new("delete", "Elimina").with_icon(IconKind::Trash2).with_danger(true),
            ],
            max_visible: Some(2),
            overflow_button_label: "Altre azioni sul file".to_string(),
            on_action: move |id: String| {
                if id == "delete" {
                    toast.warning("rapporto-q3.pdf spostato nel cestino");
                } else {
                    toast.info(format!("Azione {id} su rapporto-q3.pdf"));
                }
            },
        }
    }),
    Text { size: TextSize::Sm, dimmed: true, "Le prime due azioni restano visibili; le altre finiscono nel menu." }
}"#;

const SET_PROPS: &[PropDoc] = &[
    prop("items", "Vec<ToolbarAction>", "-", "Azioni in ordine di importanza."),
    prop("max_visible", "Option<usize>", "None (4)", "Azioni visibili prima del menu."),
    prop("overflow_items", "Vec<ToolbarAction>", "[]", "Azioni sempre nel menu, dopo quelle in eccesso."),
    prop("overflow_button_label", "String", "\"More actions\"", "Etichetta accessibile e tooltip del pulsante \"…\"."),
    prop("on_action", "Option<EventHandler<String>>", "None", "Id dell'azione scelta, visibile o nel menu."),
    prop("class", "String", "\"\"", "Classi aggiuntive."),
];
const ACTION_PROPS: &[PropDoc] = &[
    prop("ToolbarAction::new(id, label)", "fn", "-", "Costruttore."),
    prop(".with_icon(IconKind)", "fn", "-", "Icona prima dell'etichetta."),
    prop(".with_shortcut(..)", "fn", "-", "Scorciatoia associata (dato)."),
    prop(".with_disabled(bool)", "fn", "-", "Azione non attivabile."),
    prop(".with_danger(bool)", "fn", "-", "Stile danger (testo --sd-danger-strong)."),
];

pub fn story() -> Story {
    Story {
        slug: "overflow-set",
        name: "OverflowSet",
        category: Category::Navigazione,
        features: &["overflow_set"],
        icon: IconKind::MoreHorizontal,
        tagline: "Barra di azioni che mostra le più importanti e sposta le altre in un menu.",
        keywords: &["overflow", "toolbar", "barra strumenti", "azioni", "altro", "more actions"],
        status: Status::Stable,
        playground: Some(Playground {
            component: "OverflowSet",
            controls: vec![
                Control::number("max_visible", "Azioni visibili", 4.0, 1.0, 6.0, 1.0, 0)
                    .emit(Emit::Template("max_visible: Some({v})")),
                Control::text("overflow_button_label", "Etichetta del menu", "Altre azioni")
                    .emit(Emit::Template("overflow_button_label: \"{v}\".to_string()")),
            ],
            render,
            prelude_code: Some(PRELUDE),
            fixed_code: &["items: actions,", "on_action: move |id: String| { toast.info(format!(\"Azione: {id}\")); },"],
            children_code: None,
            stage: Stage::Start,
        }),
        examples: vec![
            Example {
                id: "extra",
                title: "Voci solo nel menu",
                description: "overflow_items finisce sempre dietro \"…\".",
                code: EXTRA_CODE,
                render: || rsx! { Extra {} },
                span: Span::Half,
            },
            Example {
                id: "disabled",
                title: "Azione disabilitata",
                description: "Nessun overflow: tutto entra nella barra.",
                code: DISABLED_CODE,
                render: || rsx! { Disabled {} },
                span: Span::Half,
            },
            Example {
                id: "context",
                title: "In contesto",
                description: "Azioni su un file, con eliminazione nel menu.",
                code: IN_CONTEXT_CODE,
                render: || rsx! { InContext {} },
                span: Span::Full,
            },
        ],
        props: vec![
            PropGroup { component: "OverflowSet / ResponsiveToolbar", rows: SET_PROPS },
            PropGroup { component: "ToolbarAction", rows: ACTION_PROPS },
        ],
        recipe: Some(Recipe {
            recipe: "Track di item (§3.4) + menu floating panel (§3.2)",
            depth: "Well (barra) · Raised (menu)",
            tokens: &["--sd-well", "--sd-border", "--sd-item-h", "--sd-tint-1", "--sd-danger-strong", "--sd-surface-2"],
            notes: "Le azioni sono item ghost in una track; il menu è il Dropdown della libreria.",
        }),
        a11y: &[
            "Ogni azione visibile è un <button> nativo, con disabled quando serve.",
            "Il pulsante \"…\" ha aria-label e title da overflow_button_label.",
            "Il menu di overflow è il Dropdown della libreria, con le sue voci da tastiera.",
            "Le azioni danger restano annunciate dal testo, non solo dal colore.",
        ],
        related: &["menubar", "dropdown", "buttons", "group"],
    }
}
