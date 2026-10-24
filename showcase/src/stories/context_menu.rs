//! Story: ContextMenu (feature `context_menu`). Compound component opened by a right click.
//! Its knobs shape the menu items, so they are printed through `children_code` placeholders.
use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::scenes::InboxPane;
use crate::story::schema::*;

const PRELUDE: &str = r#"let mut grid = use_signal(|| true);
let mut toast = use_notification();"#;

const CHILDREN_CODE: &str = r#"ContextMenuTrigger {
    Card { title: Some("deploy.toml".to_string()), subtitle: Some("Clic destro per le azioni".to_string()),
        Code { "region = \"eu-west\"" }
    }
}
ContextMenuContent {
    ContextMenuItem { label: "{label}".to_string(), icon: Some(IconKind::Copy), shortcut: Some("{shortcut}".to_string()), onclick: move |_| { toast.info("Copiato"); } }
    ContextMenuItem { label: "Incolla".to_string(), shortcut: Some("Ctrl+V".to_string()), disabled: {paste_disabled} }
    ContextMenuSeparator {}
    ContextMenuCheckboxItem { label: "Mostra griglia".to_string(), checked: grid(), onchange: move |v| grid.set(v) }
    ContextMenuSeparator {}
    ContextMenuItem { label: "Elimina".to_string(), icon: Some(IconKind::Trash2), shortcut: Some("Canc".to_string()), danger: {danger} }
}"#;

#[component]
fn ContextMenuDemo(knobs: Knobs) -> Element {
    let mut grid = use_signal(|| true);
    let mut toast = use_notification();
    rsx! {
        ContextMenu {
            ContextMenuTrigger {
                Card { title: Some("deploy.toml".to_string()), subtitle: Some("Clic destro per le azioni".to_string()),
                    Code { "region = \"eu-west\"" }
                }
            }
            ContextMenuContent {
                ContextMenuItem {
                    label: knobs.text("label"),
                    icon: Some(IconKind::Copy),
                    shortcut: Some(knobs.text("shortcut")),
                    onclick: move |_| {
                        toast.info("Copiato");
                    },
                }
                ContextMenuItem { label: "Incolla".to_string(), shortcut: Some("Ctrl+V".to_string()), disabled: knobs.bool("paste_disabled") }
                ContextMenuSeparator {}
                ContextMenuCheckboxItem { label: "Mostra griglia".to_string(), checked: grid(), onchange: move |v| grid.set(v) }
                ContextMenuSeparator {}
                ContextMenuItem { label: "Elimina".to_string(), icon: Some(IconKind::Trash2), shortcut: Some("Canc".to_string()), danger: knobs.bool("danger") }
            }
        }
    }
}

fn render(k: &Knobs) -> Element {
    rsx! {
        ContextMenuDemo { knobs: k.clone() }
    }
}

#[component]
fn Toggles() -> Element {
    let mut grid = use_signal(|| true);
    let mut minimap = use_signal(|| false);
    let grid_label = if grid() { "Griglia attiva" } else { "Griglia nascosta" };
    let minimap_label = if minimap() { "Minimappa attiva" } else { "Minimappa nascosta" };
    rsx! {
        ContextMenu {
            ContextMenuTrigger {
                Stack { gap: SpacingToken::Sm, align: AlignItems::Center,
                    Text { size: TextSize::Sm, "Editor della topologia" }
                    Group { gap: SpacingToken::Xs, justify: JustifyContent::Center,
                        Badge { label: grid_label.to_string(), pill: true }
                        Badge { label: minimap_label.to_string(), pill: true }
                    }
                    Text { size: TextSize::Xs, dimmed: true, "Clic destro per cambiare la vista" }
                }
            }
            ContextMenuContent {
                ContextMenuCheckboxItem { label: "Griglia".to_string(), checked: grid(), shortcut: Some("Ctrl+G".to_string()), onchange: move |v| grid.set(v) }
                ContextMenuCheckboxItem { label: "Minimappa".to_string(), checked: minimap(), shortcut: Some("Ctrl+M".to_string()), onchange: move |v| minimap.set(v) }
            }
        }
    }
}
const TOGGLES_CODE: &str = r#"let mut grid = use_signal(|| true);
let mut minimap = use_signal(|| false);
let grid_label = if grid() { "Griglia attiva" } else { "Griglia nascosta" };
let minimap_label = if minimap() { "Minimappa attiva" } else { "Minimappa nascosta" };

ContextMenu {
    ContextMenuTrigger {
        Stack { gap: SpacingToken::Sm, align: AlignItems::Center,
            Text { size: TextSize::Sm, "Editor della topologia" }
            Group { gap: SpacingToken::Xs, justify: JustifyContent::Center,
                Badge { label: grid_label.to_string(), pill: true }
                Badge { label: minimap_label.to_string(), pill: true }
            }
            Text { size: TextSize::Xs, dimmed: true, "Clic destro per cambiare la vista" }
        }
    }
    ContextMenuContent {
        ContextMenuCheckboxItem { label: "Griglia".to_string(), checked: grid(), shortcut: Some("Ctrl+G".to_string()), onchange: move |v| grid.set(v) }
        ContextMenuCheckboxItem { label: "Minimappa".to_string(), checked: minimap(), shortcut: Some("Ctrl+M".to_string()), onchange: move |v| minimap.set(v) }
    }
}"#;

#[component]
fn Rows() -> Element {
    let mut toast = use_notification();
    rsx! {
        Stack { gap: SpacingToken::Xs,
            for file in ["runbook.md", "topologia.svg", "chiavi.age"] {
                ContextMenu { key: "{file}",
                    ContextMenuTrigger {
                        Group { justify: JustifyContent::SpaceBetween,
                            Text { size: TextSize::Sm, "{file}" }
                            Text { size: TextSize::Xs, dimmed: true, "clic destro" }
                        }
                    }
                    ContextMenuContent {
                        ContextMenuItem {
                            label: "Rinomina".to_string(),
                            shortcut: Some("F2".to_string()),
                            onclick: move |_| {
                                toast.info(format!("Rinomina {file}"));
                            },
                        }
                        ContextMenuItem {
                            label: "Scarica".to_string(),
                            icon: Some(IconKind::Download),
                            onclick: move |_| {
                                toast.info(format!("Download di {file}"));
                            },
                        }
                    }
                }
            }
        }
    }
}
const ROWS_CODE: &str = r#"let mut toast = use_notification();

Stack { gap: SpacingToken::Xs,
    for file in ["runbook.md", "topologia.svg", "chiavi.age"] {
        ContextMenu {
            ContextMenuTrigger {
                Group { justify: JustifyContent::SpaceBetween,
                    Text { size: TextSize::Sm, "{file}" }
                    Text { size: TextSize::Xs, dimmed: true, "clic destro" }
                }
            }
            ContextMenuContent {
                ContextMenuItem {
                    label: "Rinomina".to_string(),
                    shortcut: Some("F2".to_string()),
                    onclick: move |_| {
                        toast.info(format!("Rinomina {file}"));
                    },
                }
                ContextMenuItem {
                    label: "Scarica".to_string(),
                    icon: Some(IconKind::Download),
                    onclick: move |_| {
                        toast.info(format!("Download di {file}"));
                    },
                }
            }
        }
    }
}"#;

#[component]
fn InContext() -> Element {
    rsx! {
        InboxPane {}
    }
}
const IN_CONTEXT_CODE: &str = r#"// scenes/inbox_pane.rs (estratto): tutta la lista dei messaggi è il trigger.
ContextMenu {
    ContextMenuTrigger {
        List { spacing: ListSpacing::Sm,
            for m in shown {
                MessageRow { message: m }
            }
        }
    }
    ContextMenuContent {
        ContextMenuItem { label: "Segna come letto".to_string(), icon: Some(IconKind::CheckCheck), shortcut: Some("R".to_string()) }
        ContextMenuItem { label: "Rispondi".to_string(), icon: Some(IconKind::Send), shortcut: Some("Invio".to_string()) }
        ContextMenuSeparator {}
        ContextMenuItem { label: "Elimina".to_string(), icon: Some(IconKind::Trash2), danger: true }
    }
}"#;

const MENU_PROPS: &[PropDoc] = &[
    prop("children", "Element", "-", "Un ContextMenuTrigger e un ContextMenuContent."),
    prop("class", "String", "\"\"", "Classi aggiuntive sul wrapper."),
];
const TRIGGER_PROPS: &[PropDoc] = &[
    prop("children", "Element", "-", "Area che apre il menu sull'evento contextmenu, alle coordinate del puntatore."),
    prop("class", "String", "\"\"", "Classi aggiuntive sull'area."),
];
const CONTENT_PROPS: &[PropDoc] = &[
    prop("children", "Element", "-", "Voci, voci checkbox e separatori."),
    prop("glow", "Option<bool>", "None", "Glow opzionale sul pannello."),
    prop("class", "String", "\"\"", "Classi aggiuntive sul pannello."),
];
const ITEM_PROPS: &[PropDoc] = &[
    prop("label", "String", "-", "Testo della voce."),
    prop("icon", "Option<IconKind>", "None", "Icona iniziale."),
    prop("shortcut", "Option<String>", "None", "Scorciatoia mostrata a destra (solo testo, non registra il tasto)."),
    prop("disabled / danger", "bool", "false", "Voce non attivabile; voce distruttiva."),
    prop("onclick", "Option<EventHandler<()>>", "None", "Azione; il menu si chiude dopo il clic."),
    prop("class", "String", "\"\"", "Classi aggiuntive."),
];
const CHECK_PROPS: &[PropDoc] = &[
    prop("checked", "bool", "-", "Stato attuale, annunciato con aria-checked."),
    prop("label", "String", "-", "Testo della voce."),
    prop("shortcut", "Option<String>", "None", "Scorciatoia mostrata a destra."),
    prop("onchange", "Option<EventHandler<bool>>", "None", "Riceve il nuovo valore; il menu si chiude."),
    prop("class", "String", "\"\"", "Classi aggiuntive."),
];

pub fn story() -> Story {
    Story {
        slug: "context-menu",
        name: "ContextMenu",
        category: Category::Overlay,
        features: &["context_menu"],
        icon: IconKind::MoreHorizontal,
        tagline: "Menu di azioni sull'elemento sotto il puntatore, aperto con il clic destro.",
        keywords: &["context menu", "menu contestuale", "tasto destro", "right click", "scorciatoie", "checkbox"],
        status: Status::Stable,
        playground: Some(Playground {
            component: "ContextMenu",
            controls: vec![
                Control::text("label", "Prima voce", "Copia percorso").emit(Emit::Skip),
                Control::text("shortcut", "Scorciatoia", "Ctrl+C").emit(Emit::Skip),
                Control::bool("paste_disabled", "Incolla disabilitato", true).emit(Emit::Skip),
                Control::bool("danger", "Elimina come danger", true).emit(Emit::Skip),
            ],
            render,
            prelude_code: Some(PRELUDE),
            fixed_code: &[],
            children_code: Some(CHILDREN_CODE),
            stage: Stage::Form,
        }),
        examples: vec![
            Example {
                id: "toggles",
                title: "Voci checkbox",
                description: "Opzioni di vista attivate dal menu, con scorciatoie.",
                code: TOGGLES_CODE,
                render: || rsx! { Toggles {} },
                span: Span::Half,
            },
            Example {
                id: "rows",
                title: "Un menu per riga",
                description: "Ogni riga ha il proprio menu e agisce sul proprio file.",
                code: ROWS_CODE,
                render: || rsx! { Rows {} },
                span: Span::Half,
            },
            Example {
                id: "context",
                title: "In contesto",
                description: "Azioni rapide sui messaggi dell'inbox operatori.",
                code: IN_CONTEXT_CODE,
                render: || rsx! { InContext {} },
                span: Span::Full,
            },
        ],
        props: vec![
            PropGroup { component: "ContextMenu", rows: MENU_PROPS },
            PropGroup { component: "ContextMenuTrigger", rows: TRIGGER_PROPS },
            PropGroup { component: "ContextMenuContent", rows: CONTENT_PROPS },
            PropGroup { component: "ContextMenuItem", rows: ITEM_PROPS },
            PropGroup { component: "ContextMenuCheckboxItem", rows: CHECK_PROPS },
        ],
        recipe: Some(Recipe {
            recipe: "Floating panel (language.md §3.2) + items (§3.4)",
            depth: "Raised",
            tokens: &["--sd-surface-2", "--sd-border-strong", "--sd-elev-2", "--sd-radius-lg", "--sd-item-h", "--sd-tint-1", "--sd-danger-strong"],
            notes: "Stesse voci del Dropdown; il pannello compare alle coordinate del puntatore.",
        }),
        a11y: &[
            "Il pannello ha role=menu; le voci sono pulsanti con role=menuitem o menuitemcheckbox con aria-checked.",
            "Si apre con l'evento contextmenu: tasto destro, oppure Maiusc+F10 o il tasto Menu su un elemento focalizzato nel trigger.",
            "Esc, un clic fuori o la scelta di una voce chiudono il menu.",
            "Le scorciatoie sono solo etichette: registra i tasti a parte (per esempio con use_hotkey).",
        ],
        related: &["dropdown", "menubar", "keytip", "lists"],
    }
}
