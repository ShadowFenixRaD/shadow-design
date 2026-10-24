//! Story: TreeView and TreeNode (feature `tree`). The selection lives in a signal, so the
//! playground goes through `TreeDemo { knobs }`.
use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::story::schema::*;

fn mesh() -> Vec<TreeNode> {
    vec![
        TreeNode::new("mesh-eu", "Cluster Europa")
            .with_icon(IconKind::Radio.into())
            .with_sublabel("3 regioni · 6 nodi")
            .with_children(vec![
                TreeNode::new("relay-mi", "Relay Milano")
                    .with_icon(IconKind::Server.into())
                    .with_sublabel("8 ms")
                    .with_badge("online")
                    .with_children(vec![
                        TreeNode::new("edge-07", "edge-07").with_icon(IconKind::Wifi.into()).with_sublabel("18 ms"),
                        TreeNode::new("edge-22", "edge-22").with_icon(IconKind::Wifi.into()).with_sublabel("24 ms"),
                    ]),
                TreeNode::new("relay-fra", "Relay Francoforte")
                    .with_icon(IconKind::Server.into())
                    .with_sublabel("31 ms")
                    .with_badge("lento")
                    .with_children(vec![
                        TreeNode::new("edge-31", "edge-31").with_icon(IconKind::Bluetooth.into()).with_sublabel("P2P · 4 ms"),
                    ]),
                TreeNode::new("vault-01", "Vault 01").with_icon(IconKind::Lock.into()).with_sublabel("snapshot cifrato"),
            ]),
    ]
}

/// Printed before the `TreeView {` block: the same signal and data the live demo uses.
const PRELUDE: &str = r#"let selected = use_signal(|| Some("relay-mi".to_string()));
let mut toast = use_notification();
let nodes = vec![
    TreeNode::new("mesh-eu", "Cluster Europa")
        .with_icon(IconKind::Radio.into())
        .with_sublabel("3 regioni · 6 nodi")
        .with_children(vec![
            TreeNode::new("relay-mi", "Relay Milano")
                .with_icon(IconKind::Server.into())
                .with_sublabel("8 ms")
                .with_badge("online")
                .with_children(vec![
                    TreeNode::new("edge-07", "edge-07").with_icon(IconKind::Wifi.into()).with_sublabel("18 ms"),
                    TreeNode::new("edge-22", "edge-22").with_icon(IconKind::Wifi.into()).with_sublabel("24 ms"),
                ]),
            TreeNode::new("relay-fra", "Relay Francoforte")
                .with_icon(IconKind::Server.into())
                .with_sublabel("31 ms")
                .with_badge("lento")
                .with_children(vec![
                    TreeNode::new("edge-31", "edge-31").with_icon(IconKind::Bluetooth.into()).with_sublabel("P2P · 4 ms"),
                ]),
            TreeNode::new("vault-01", "Vault 01").with_icon(IconKind::Lock.into()).with_sublabel("snapshot cifrato"),
        ]),
];"#;

/// Stateful demo: owns the selection signal the playground cannot (render is a plain fn).
#[component]
fn TreeDemo(knobs: Knobs) -> Element {
    let selected = use_signal(|| Some("relay-mi".to_string()));
    let mut toast = use_notification();
    let notify = knobs.bool("on_select");
    rsx! {
        TreeView {
            nodes: mesh(),
            selected_id: knobs.bool("selected_id").then_some(selected),
            on_select: move |id: String| {
                if notify {
                    toast.info(format!("Nodo selezionato: {id}"));
                }
            },
        }
    }
}

fn render(k: &Knobs) -> Element {
    rsx! {
        TreeDemo { knobs: k.clone() }
    }
}

#[component]
fn Project() -> Element {
    let selected = use_signal(|| Some("tree-rs".to_string()));
    rsx! {
        TreeView {
            nodes: vec![
                TreeNode::new("crate", "crate")
                    .with_icon(IconKind::Box.into())
                    .with_children(vec![
                        TreeNode::new("src", "src")
                            .with_icon(IconKind::Layers.into())
                            .with_children(vec![
                                TreeNode::new("lib-rs", "lib.rs").with_icon(IconKind::Code.into()),
                                TreeNode::new("tree-rs", "tree.rs").with_icon(IconKind::Code.into()).with_badge("modificato"),
                            ]),
                        TreeNode::new("scss", "scss")
                            .with_icon(IconKind::Palette.into())
                            .with_expanded(false)
                            .with_children(vec![
                                TreeNode::new("main-scss", "main.scss").with_icon(IconKind::Code.into()),
                                TreeNode::new("tree-scss", "_tree.scss").with_icon(IconKind::Code.into()),
                            ]),
                        TreeNode::new("cargo", "Cargo.toml").with_icon(IconKind::Settings.into()),
                    ]),
            ],
            selected_id: Some(selected),
        }
    }
}
const PROJECT_CODE: &str = r#"let selected = use_signal(|| Some("tree-rs".to_string()));

TreeView {
    nodes: vec![
        TreeNode::new("crate", "crate")
            .with_icon(IconKind::Box.into())
            .with_children(vec![
                TreeNode::new("src", "src")
                    .with_icon(IconKind::Layers.into())
                    .with_children(vec![
                        TreeNode::new("lib-rs", "lib.rs").with_icon(IconKind::Code.into()),
                        TreeNode::new("tree-rs", "tree.rs").with_icon(IconKind::Code.into()).with_badge("modificato"),
                    ]),
                TreeNode::new("scss", "scss")
                    .with_icon(IconKind::Palette.into())
                    .with_expanded(false)
                    .with_children(vec![
                        TreeNode::new("main-scss", "main.scss").with_icon(IconKind::Code.into()),
                        TreeNode::new("tree-scss", "_tree.scss").with_icon(IconKind::Code.into()),
                    ]),
                TreeNode::new("cargo", "Cargo.toml").with_icon(IconKind::Settings.into()),
            ]),
    ],
    selected_id: Some(selected),
}"#;

#[component]
fn Plain() -> Element {
    rsx! {
        TreeView {
            nodes: vec![
                TreeNode::new("docs", "Documentazione").with_children(vec![
                    TreeNode::new("start", "Per iniziare"),
                    TreeNode::new("theme", "Tema").with_children(vec![
                        TreeNode::new("presets", "Preset"),
                        TreeNode::new("tokens", "Token"),
                    ]),
                    TreeNode::new("adr", "Decisioni (ADR)").with_expanded(false).with_children(vec![
                        TreeNode::new("adr-2", "ADR-0002 Lit Glass"),
                        TreeNode::new("adr-6", "ADR-0006 Showcase"),
                    ]),
                ]),
            ],
        }
    }
}
const PLAIN_CODE: &str = r#"TreeView {
    nodes: vec![
        TreeNode::new("docs", "Documentazione").with_children(vec![
            TreeNode::new("start", "Per iniziare"),
            TreeNode::new("theme", "Tema").with_children(vec![
                TreeNode::new("presets", "Preset"),
                TreeNode::new("tokens", "Token"),
            ]),
            TreeNode::new("adr", "Decisioni (ADR)").with_expanded(false).with_children(vec![
                TreeNode::new("adr-2", "ADR-0002 Lit Glass"),
                TreeNode::new("adr-6", "ADR-0006 Showcase"),
            ]),
        ]),
    ],
}"#;

/// Details of the nodes of `mesh()`: id, role, latency, state.
const NODE_INFO: [(&str, &str, &str, &str); 7] = [
    ("mesh-eu", "Cluster", "-", "Operativo"),
    ("relay-mi", "Relay", "8 ms", "Operativo"),
    ("edge-07", "Edge", "18 ms", "Operativo"),
    ("edge-22", "Edge", "24 ms", "Operativo"),
    ("relay-fra", "Relay", "31 ms", "Degradato"),
    ("edge-31", "Edge P2P", "4 ms", "Operativo"),
    ("vault-01", "Archivio", "12 ms", "Operativo"),
];

#[component]
fn InContext() -> Element {
    let selected = use_signal(|| Some("relay-fra".to_string()));
    let id = selected().unwrap_or_default();
    let (_, role, latency, state) = NODE_INFO.iter().copied().find(|n| n.0 == id).unwrap_or(NODE_INFO[0]);
    let degraded = state == "Degradato";
    rsx! {
        SimpleGrid { cols: 1, md: Some(2), spacing: GridSpacing::Md,
            TreeView { nodes: mesh(), selected_id: Some(selected) }
            Card {
                title: Some(id.clone()),
                subtitle: Some(format!("{role} · latenza {latency}")),
                footer: Some(rsx! {
                    Group { gap: SpacingToken::Sm, justify: JustifyContent::End,
                        Button { variant: ButtonVariant::Ghost, "Log" }
                        Button { variant: ButtonVariant::Secondary, disabled: !degraded, "Riavvia nodo" }
                    }
                }),
                Group { gap: SpacingToken::Sm,
                    Badge {
                        label: state.to_string(),
                        variant: if degraded { BadgeVariant::Warning } else { BadgeVariant::Success },
                        dot: true,
                    }
                    span { class: "sc-muted-text", "Seleziona un nodo dell'albero per vederne lo stato." }
                }
            }
        }
    }
}
const IN_CONTEXT_CODE: &str = r#"let selected = use_signal(|| Some("relay-fra".to_string()));
let id = selected().unwrap_or_default();
let (_, role, latency, state) = NODE_INFO.iter().copied().find(|n| n.0 == id).unwrap_or(NODE_INFO[0]);
let degraded = state == "Degradato";

SimpleGrid { cols: 1, md: Some(2), spacing: GridSpacing::Md,
    TreeView { nodes: mesh(), selected_id: Some(selected) }
    Card {
        title: Some(id.clone()),
        subtitle: Some(format!("{role} · latenza {latency}")),
        footer: Some(rsx! {
            Group { gap: SpacingToken::Sm, justify: JustifyContent::End,
                Button { variant: ButtonVariant::Ghost, "Log" }
                Button { variant: ButtonVariant::Secondary, disabled: !degraded, "Riavvia nodo" }
            }
        }),
        Group { gap: SpacingToken::Sm,
            Badge {
                label: state.to_string(),
                variant: if degraded { BadgeVariant::Warning } else { BadgeVariant::Success },
                dot: true,
            }
            span { "Seleziona un nodo dell'albero per vederne lo stato." }
        }
    }
}"#;

const TREE_PROPS: &[PropDoc] = &[
    prop("nodes", "Vec<TreeNode>", "-", "Nodi radice; ogni nodo porta i propri figli."),
    prop("selected_id", "Option<Signal<Option<String>>>", "None", "Id del nodo selezionato; il clic o Invio lo aggiornano. Senza signal nessun nodo appare selezionato."),
    prop("on_select", "Option<EventHandler<String>>", "None", "Chiamato con l'id del nodo attivato (clic, Invio o Spazio)."),
    prop("class", "String", "\"\"", "Classi aggiuntive sul contenitore."),
];
const NODE_PROPS: &[PropDoc] = &[
    prop("TreeNode::new(id, label)", "fn", "-", "Costruttore; il nodo parte espanso."),
    prop(".with_sublabel(..)", "fn", "-", "Testo secondario in mono accanto all'etichetta."),
    prop(".with_badge(..)", "fn", "-", "Piccolo badge in tinta accento a fine riga."),
    prop(".with_icon(..)", "fn(SvgIcon)", "-", "Icona prima dell'etichetta: IconKind::X.into() o un SVG personalizzato."),
    prop(".with_children(..)", "fn(Vec<TreeNode>)", "-", "Figli; aggiunge il chevron per espandere e chiudere."),
    prop(".with_expanded(..)", "fn(bool)", "true", "Stato iniziale del ramo."),
];

pub fn story() -> Story {
    Story {
        slug: "trees",
        name: "TreeView",
        category: Category::Dati,
        features: &["tree"],
        icon: IconKind::Layers,
        tagline: "Gerarchia di nodi espandibili con selezione singola, per topologie, cartelle e indici.",
        keywords: &["tree", "albero", "treeview", "gerarchia", "cartelle", "file", "nodi"],
        status: Status::Stable,
        playground: Some(Playground {
            component: "TreeView",
            controls: vec![
                Control::bool("selected_id", "Selezione", true).emit(Emit::Template("selected_id: Some(selected)")),
                Control::bool("on_select", "Notifica alla selezione", false)
                    .emit(Emit::Template("on_select: move |id: String| { toast.info(format!(\"Nodo selezionato: {id}\")); }"))
                    .hint("Clicca un nodo: on_select riceve il suo id."),
            ],
            render,
            prelude_code: Some(PRELUDE),
            fixed_code: &["nodes,"],
            children_code: None,
            stage: Stage::Start,
        }),
        examples: vec![
            Example {
                id: "project",
                title: "Struttura di progetto",
                description: "Icone, badge e un ramo chiuso all'avvio con with_expanded(false).",
                code: PROJECT_CODE,
                render: || rsx! { Project {} },
                span: Span::Half,
            },
            Example {
                id: "plain",
                title: "Solo etichette",
                description: "Senza icone né selezione: un indice da esplorare.",
                code: PLAIN_CODE,
                render: || rsx! { Plain {} },
                span: Span::Half,
            },
            Example {
                id: "context",
                title: "In contesto",
                description: "La topologia a sinistra, il dettaglio del nodo selezionato a destra.",
                code: IN_CONTEXT_CODE,
                render: || rsx! { InContext {} },
                span: Span::Full,
            },
        ],
        props: vec![
            PropGroup { component: "TreeView", rows: TREE_PROPS },
            PropGroup { component: "TreeNode", rows: NODE_PROPS },
        ],
        recipe: Some(Recipe {
            recipe: "Container (language.md §3.1) + items (§3.4) + selected rail (§3.7)",
            depth: "Flat-lit",
            tokens: &["--sd-surface-1", "--sd-border", "--sd-item-h", "--sd-tint-1", "--sd-accent-soft", "--sd-accent", "--sd-font-mono"],
            notes: "Ogni nodo è un sd-item; il selezionato prende il rail accento sul bordo d'attacco, i rami una guida tratteggiata.",
        }),
        a11y: &[
            "Contenitore con role=tree e nodi con role=treeitem, aria-selected e aria-expanded sui rami.",
            "Ogni riga è nel tab order: Invio e Spazio la selezionano.",
            "Il chevron è un <button> separato che espande o chiude senza cambiare la selezione.",
            "Non c'è navigazione con le frecce: per alberi molto lunghi affiancalo a un campo di ricerca.",
        ],
        related: &["lists", "accordion", "transfer-list", "virtual-list"],
    }
}
