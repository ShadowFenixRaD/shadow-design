//! Story: List and ListItem (feature `list`). Stateless: the playground renders straight from the
//! knobs, remounting the list when the icon changes (items read it once from the context).
use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::scenes::InboxPane;
use crate::story::schema::*;

const TYPES: [ListType; 2] = [ListType::Unordered, ListType::Ordered];
const TYPE_OPTS: &[ChoiceOption] = &[opt("Unordered", "ListType::Unordered"), opt("Ordered", "ListType::Ordered")];
const MARKERS: [Option<ListMarker>; 8] = [
    None,
    Some(ListMarker::Disc),
    Some(ListMarker::Circle),
    Some(ListMarker::Square),
    Some(ListMarker::Decimal),
    Some(ListMarker::LowerAlpha),
    Some(ListMarker::LowerRoman),
    Some(ListMarker::None),
];
const MARKER_OPTS: &[ChoiceOption] = &[
    opt("Automatico", "ListMarker::Disc"),
    opt("Disc", "ListMarker::Disc"),
    opt("Circle", "ListMarker::Circle"),
    opt("Square", "ListMarker::Square"),
    opt("Decimal", "ListMarker::Decimal"),
    opt("LowerAlpha", "ListMarker::LowerAlpha"),
    opt("LowerRoman", "ListMarker::LowerRoman"),
    opt("None", "ListMarker::None"),
];
const SPACINGS: [ListSpacing; 5] = [ListSpacing::Xs, ListSpacing::Sm, ListSpacing::Md, ListSpacing::Lg, ListSpacing::Xl];
const SPACING_OPTS: &[ChoiceOption] = &[
    opt("Xs", "ListSpacing::Xs"),
    opt("Sm", "ListSpacing::Sm"),
    opt("Md", "ListSpacing::Md"),
    opt("Lg", "ListSpacing::Lg"),
    opt("Xl", "ListSpacing::Xl"),
];
const SIZES: [ListSize; 5] = [ListSize::Xs, ListSize::Sm, ListSize::Md, ListSize::Lg, ListSize::Xl];
const SIZE_OPTS: &[ChoiceOption] =
    &[opt("Xs", "ListSize::Xs"), opt("Sm", "ListSize::Sm"), opt("Md", "ListSize::Md"), opt("Lg", "ListSize::Lg"), opt("Xl", "ListSize::Xl")];

const ITEMS_CODE: &str = r#"ListItem { "Aggiorna la versione in Cargo.toml" }
ListItem { "Rigenera il knowledge graph" }
ListItem { "Esegui clippy e i test del workspace" }
ListItem { "Pubblica su crates.io" }"#;

fn render(k: &Knobs) -> Element {
    let icon = k.bool("icon");
    let horizontal = k.bool("horizontal");
    rsx! {
        for key in [icon] {
            List {
                key: "{key}",
                list_type: k.pick("list_type", &TYPES),
                marker: k.pick("marker", &MARKERS),
                spacing: k.pick("spacing", &SPACINGS),
                size: k.pick("size", &SIZES),
                orientation: if horizontal { ListOrientation::Horizontal } else { ListOrientation::Vertical },
                center: k.bool("center"),
                icon: icon.then(|| rsx! {
                    Icon { kind: IconKind::CheckCircle, size: 16 }
                }),
                ListItem { "Aggiorna la versione in Cargo.toml" }
                ListItem { "Rigenera il knowledge graph" }
                ListItem { "Esegui clippy e i test del workspace" }
                ListItem { "Pubblica su crates.io" }
            }
        }
    }
}

#[component]
fn Nested() -> Element {
    rsx! {
        List {
            ListItem { "Cluster eu-west" }
            ListItem {
                "Nodi edge"
                List { marker: Some(ListMarker::Circle),
                    ListItem { "edge-07 · Milano" }
                    ListItem { "edge-12 · Francoforte" }
                    ListItem {
                        "Relay"
                        List { marker: Some(ListMarker::Square),
                            ListItem { "relay-03 · Parigi" }
                            ListItem { "relay-05 · Varsavia" }
                        }
                    }
                }
            }
            ListItem { "Nodo core-01" }
        }
    }
}
const NESTED_CODE: &str = r#"List {
    ListItem { "Cluster eu-west" }
    ListItem {
        "Nodi edge"
        List { marker: Some(ListMarker::Circle),
            ListItem { "edge-07 · Milano" }
            ListItem { "edge-12 · Francoforte" }
            ListItem {
                "Relay"
                List { marker: Some(ListMarker::Square),
                    ListItem { "relay-03 · Parigi" }
                    ListItem { "relay-05 · Varsavia" }
                }
            }
        }
    }
    ListItem { "Nodo core-01" }
}"#;

#[component]
fn ItemIcons() -> Element {
    rsx! {
        List { spacing: ListSpacing::Md,
            ListItem {
                icon: Some(rsx! {
                    Icon { kind: IconKind::CheckCircle, size: 16 }
                }),
                "Build wasm32 completata"
            }
            ListItem {
                icon: Some(rsx! {
                    Icon { kind: IconKind::CheckCircle, size: 16 }
                }),
                "Test di integrazione: 214 passati"
            }
            ListItem {
                icon: Some(rsx! {
                    Icon { kind: IconKind::Clock, size: 16 }
                }),
                "Deploy canary in corso"
            }
            ListItem {
                icon: Some(rsx! {
                    Icon { kind: IconKind::XCircle, size: 16 }
                }),
                "Smoke test di Madrid non riuscito"
            }
        }
    }
}
const ITEM_ICONS_CODE: &str = r#"List { spacing: ListSpacing::Md,
    ListItem {
        icon: Some(rsx! {
            Icon { kind: IconKind::CheckCircle, size: 16 }
        }),
        "Build wasm32 completata"
    }
    ListItem {
        icon: Some(rsx! {
            Icon { kind: IconKind::CheckCircle, size: 16 }
        }),
        "Test di integrazione: 214 passati"
    }
    ListItem {
        icon: Some(rsx! {
            Icon { kind: IconKind::Clock, size: 16 }
        }),
        "Deploy canary in corso"
    }
    ListItem {
        icon: Some(rsx! {
            Icon { kind: IconKind::XCircle, size: 16 }
        }),
        "Smoke test di Madrid non riuscito"
    }
}"#;

#[component]
fn Horizontal() -> Element {
    rsx! {
        List { orientation: ListOrientation::Horizontal, marker: Some(ListMarker::None), spacing: ListSpacing::Lg, size: ListSize::Sm,
            ListItem { "Rust 1.85" }
            ListItem { "Dioxus 0.7" }
            ListItem { "MIT o Apache-2.0" }
            ListItem { "0 dipendenze JS" }
        }
    }
}
const HORIZONTAL_CODE: &str = r#"List { orientation: ListOrientation::Horizontal, marker: Some(ListMarker::None), spacing: ListSpacing::Lg, size: ListSize::Sm,
    ListItem { "Rust 1.85" }
    ListItem { "Dioxus 0.7" }
    ListItem { "MIT o Apache-2.0" }
    ListItem { "0 dipendenze JS" }
}"#;

#[component]
fn InContext() -> Element {
    rsx! {
        InboxPane {}
    }
}
const IN_CONTEXT_CODE: &str = r#"// scenes/inbox_pane.rs (estratto)
// MessageRow: ogni messaggio è un ListItem con l'avatar come icona.
ListItem {
    icon: Some(rsx! {
        Avatar { name: Some(m.from.to_string()), size: AvatarSize::Sm, status: Some(m.status) }
    }),
    // mittente, ora, oggetto, badge e anteprima
}

// InboxPane
List { spacing: ListSpacing::Sm,
    for m in shown {
        MessageRow { message: m }
    }
}"#;

const LIST_PROPS: &[PropDoc] = &[
    prop("list_type", "ListType", "Unordered", "Unordered (<ul>) o Ordered (<ol>)."),
    prop("marker", "Option<ListMarker>", "None", "Disc, Circle, Square, Decimal, LowerAlpha, UpperAlpha, LowerRoman, UpperRoman, None, Custom; None sceglie in base al tipo."),
    prop("spacing", "ListSpacing", "Sm", "Spazio tra le voci, da Xs a Xl."),
    prop("size", "ListSize", "Md", "Corpo del testo, da Xs a Xl."),
    prop("orientation", "ListOrientation", "Vertical", "Vertical o Horizontal (voci in riga)."),
    prop("center", "bool", "false", "Centra le voci."),
    prop("with_padding", "Option<bool>", "None", "Rientro del marcatore; automatico se None."),
    prop("icon", "Option<Element>", "None", "Icona comune a tutte le voci al posto del marcatore."),
    prop("id / class", "Option<String> / String", "None / \"\"", "Id e classi aggiuntive."),
    prop("children", "Element", "-", "Le ListItem, anche con liste annidate."),
];
const ITEM_PROPS: &[PropDoc] = &[
    prop("icon", "Option<Element>", "None", "Icona di questa voce; sovrascrive quella della lista."),
    prop("id / class", "Option<String> / String", "None / \"\"", "Id e classi aggiuntive."),
    prop("children", "Element", "-", "Contenuto della voce."),
];

pub fn story() -> Story {
    Story {
        slug: "lists",
        name: "List",
        category: Category::Dati,
        features: &["list"],
        icon: IconKind::CheckCheck,
        tagline: "Elenchi ordinati e non ordinati, con marcatori, icone per voce, nidificazione e disposizione in riga.",
        keywords: &["list", "lista", "elenco", "ul", "ol", "bullet", "marcatore"],
        status: Status::Stable,
        playground: Some(Playground {
            component: "List",
            controls: vec![
                Control::choice("list_type", "Tipo", TYPE_OPTS, 0),
                Control::choice("marker", "Marcatore", MARKER_OPTS, 0).emit(Emit::Template("marker: Some({v})")),
                Control::choice("spacing", "Spaziatura", SPACING_OPTS, 1),
                Control::choice("size", "Dimensione", SIZE_OPTS, 2),
                Control::bool("icon", "Icona comune", false)
                    .emit(Emit::Template("icon: Some(rsx! { Icon { kind: IconKind::CheckCircle, size: 16 } })")),
                Control::bool("horizontal", "In riga", false).emit(Emit::Template("orientation: ListOrientation::Horizontal")),
                Control::bool("center", "Centrata", false),
            ],
            render,
            prelude_code: None,
            fixed_code: &[],
            children_code: Some(ITEMS_CODE),
            stage: Stage::Start,
        }),
        examples: vec![
            Example {
                id: "nested",
                title: "Annidate",
                description: "Ogni livello sceglie il proprio marcatore.",
                code: NESTED_CODE,
                render: || rsx! { Nested {} },
                span: Span::Half,
            },
            Example {
                id: "item-icons",
                title: "Icona per voce",
                description: "Lo stato di ogni passo al posto del marcatore.",
                code: ITEM_ICONS_CODE,
                render: || rsx! { ItemIcons {} },
                span: Span::Half,
            },
            Example {
                id: "horizontal",
                title: "In riga",
                description: "Metadati compatti senza marcatore.",
                code: HORIZONTAL_CODE,
                render: || rsx! { Horizontal {} },
                span: Span::Half,
            },
            Example {
                id: "context",
                title: "In contesto",
                description: "I messaggi dell'inbox operatori sono voci con avatar.",
                code: IN_CONTEXT_CODE,
                render: || rsx! { InContext {} },
                span: Span::Full,
            },
        ],
        props: vec![
            PropGroup { component: "List", rows: LIST_PROPS },
            PropGroup { component: "ListItem", rows: ITEM_PROPS },
        ],
        recipe: Some(Recipe {
            recipe: "Text and labels (language.md §3.12)",
            depth: "Nessuna superficie: testo statico sul contenitore",
            tokens: &["--sd-text", "--sd-text-muted", "--sd-text-subtle", "--sd-text-md", "--sd-space-2", "--sd-font-body"],
            notes: "La lista non ha superficie propria: eredita quella del contenitore e usa solo testo e spaziatura.",
        }),
        a11y: &[
            "Markup <ul>/<ol> e <li> nativo: gli screen reader annunciano il numero di voci.",
            "Le icone delle voci sono aria-hidden: il significato deve stare nel testo.",
            "Marker None e disposizione in riga non cambiano la semantica di lista.",
        ],
        related: &["timelines", "trees", "virtual-list", "tables"],
    }
}
