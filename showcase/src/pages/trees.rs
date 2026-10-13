use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::components::{FeatureBanner, PropRow, ShowcaseTabs};

#[component]
pub fn TreesPage() -> Element {
    let selected_node = use_signal(|| Some("node-0".to_string()));

    let props = vec![
        PropRow {
            name: "nodes",
            r#type: "Vec<TreeNode>",
            default: "-",
            description: "Gerarchia ricorsiva dei nodi con id, label, sublabel, icona e figli",
        },
        PropRow {
            name: "selected_id",
            r#type: "Option<Signal<Option<String>>>",
            default: "None",
            description: "Signal per tracciare e modificare l'id del nodo correntemente selezionato",
        },
        PropRow {
            name: "on_select",
            r#type: "Option<EventHandler<String>>",
            default: "None",
            description: "Callback scatenata quando l'utente clicca su un nodo dell'albero",
        },
    ];

    let code_snippet = r#"TreeView {
    nodes: vec![
        TreeNode::new("node-0", "Nodo Creatore #00")
            .with_icon(IconKind::Radio.into())
            .with_sublabel("Root Mesh • 0ms")
            .with_badge("CREATOR")
            .with_children(vec![
                TreeNode::new("node-1", "Router Mesh #01")
                    .with_icon(IconKind::Server.into())
                    .with_sublabel("8ms • 1 hop")
                    .with_badge("ONLINE")
                    .with_children(vec![
                        TreeNode::new("node-1-1", "Subordinato 1.1").with_sublabel("18ms • 2 hops"),
                        TreeNode::new("node-1-2", "Subordinato 1.2").with_sublabel("24ms • 2 hops"),
                    ]),
            ]),
    ],
    selected_id: Some(selected_node),
}"#
    .to_string();

    let mesh_nodes = vec![
        TreeNode::new("node-0", "Nodo Creatore #00")
            .with_icon(IconKind::Radio.into())
            .with_sublabel("Root Mesh • 0ms")
            .with_badge("CREATOR")
            .with_children(vec![
                TreeNode::new("node-1", "Router Mesh #01")
                    .with_icon(IconKind::Server.into())
                    .with_sublabel("8ms • 1 hop")
                    .with_badge("ONLINE")
                    .with_children(vec![
                        TreeNode::new("node-1-1", "Subordinato 1.1")
                            .with_icon(IconKind::Wifi.into())
                            .with_sublabel("18ms • 2 hops"),
                        TreeNode::new("node-1-2", "Subordinato 1.2")
                            .with_icon(IconKind::Wifi.into())
                            .with_sublabel("24ms • 2 hops"),
                    ]),
                TreeNode::new("node-2", "Peer Diretto #02")
                    .with_icon(IconKind::Bluetooth.into())
                    .with_sublabel("Direct P2P • 4ms")
                    .with_badge("P2P")
                    .with_children(vec![
                        TreeNode::new("node-2-1", "Storage Node 2.1")
                            .with_icon(IconKind::Lock.into())
                            .with_sublabel("Snapshot cifrato PQ"),
                    ]),
            ]),
    ];

    rsx! {
        div { class: "max-w-4xl space-y-8",
            div {
                Heading { order: HeadingOrder::H2, "TreeView & Nodi ad Albero" }
                Text { dimmed: true, size: TextSize::Sm, "Struttura gerarchica per topologie di rete mesh, mappe dei nodi subordinati e filesystem con guide di connessione e indicatori di latenza." }
            }

            FeatureBanner { feature_name: "tree" }

            ShowcaseTabs {
                preview: rsx! {
                    div { class: "grid grid-cols-1 lg:grid-cols-2 gap-8 items-start",
                        // Live Preview
                        div { class: "p-6 rounded-xl bg-[#060608] border border-white/[0.08] shadow-inner",
                            TreeView {
                                nodes: mesh_nodes,
                                selected_id: Some(selected_node),
                            }
                        }

                        // Info & Selected State
                        div { class: "space-y-4 text-xs font-mono",
                            div { class: "p-4 rounded-lg bg-white/[0.02] border border-white/[0.06] space-y-2",
                                span { class: "text-zinc-400 block text-[10px] uppercase tracking-wider", "Nodo Selezionato:" }
                                div { class: "text-sm font-bold text-purple-300",
                                    {
                                        let sel = selected_node().unwrap_or_else(|| "Nessuno".to_string());
                                        rsx! { "{sel}" }
                                    }
                                }
                                p { class: "text-zinc-400 text-xs leading-relaxed",
                                    "Fai clic sulle frecce chevron per espandere/collassare i rami dell'albero mesh e sui nodi per selezionarli."
                                }
                            }
                        }
                    }
                },
                code: code_snippet,
                props: props,
            }
        }
    }
}
