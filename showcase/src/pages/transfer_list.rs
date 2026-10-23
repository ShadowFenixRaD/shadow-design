use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::components::{FeatureBanner, PropRow, ShowcaseTabs};

#[component]
pub fn TransferListPage() -> Element {
    let source_items = use_signal(|| vec![
        TransferItem::new("1", "Dioxus 0.7 Engine").with_description("Framework reattivo Rust"),
        TransferItem::new("2", "Tailwind Preflight Reset").with_description("Normalizzazione browser"),
        TransferItem::new("3", "Pure SCSS Theme Pipeline").with_description("Zero-tooling bundling asset"),
        TransferItem::new("4", "SVG Data Visualization").with_description("Grafici nativi senza JS"),
        TransferItem::new("5", "WAI-ARIA Accessibility").with_description("Focus trapping e ruoli APG"),
    ]);

    let target_items = use_signal(|| vec![
        TransferItem::new("6", "Zero Bloat Feature Flags").with_description("Modularità estrema Cargo"),
        TransferItem::new("7", "Geist Typography Hierarchy").with_description("Font stack nitido dark"),
    ]);

    let props = vec![
        PropRow {
            name: "source",
            r#type: "Signal<Vec<TransferItem>>",
            default: "-",
            description: "Segnale reattivo per la colonna degli elementi di origine",
        },
        PropRow {
            name: "target",
            r#type: "Signal<Vec<TransferItem>>",
            default: "-",
            description: "Segnale reattivo per la colonna degli elementi assegnati/destinazione",
        },
        PropRow {
            name: "searchable",
            r#type: "bool",
            default: "true",
            description: "Mostra campo di ricerca con filtro live in cima a ciascuna colonna",
        },
        PropRow {
            name: "titles",
            r#type: "(String, String)",
            default: "(\"Origine\", \"Destinazione\")",
            description: "Intestazioni testuali per le due colonne",
        },
        PropRow {
            name: "height",
            r#type: "String",
            default: "\"320px\"",
            description: "Altezza fissa o minima con scorrimento verticale interno",
        },
    ];

    let code_snippet = r#"let source = use_signal(|| vec![...]);
let target = use_signal(|| vec![...]);

TransferList {
    source: source,
    target: target,
    titles: ("Funzionalità Disponibili".to_string(), "Funzionalità Assegnate".to_string()),
    searchable: true,
}"#
    .to_string();

    rsx! {
        div { class: "max-w-4xl space-y-8",
            div {
                Heading { order: HeadingOrder::H2, "TransferList" }
                Text { dimmed: true, size: TextSize::Sm, "Interfaccia a due colonne affiancate in stile Material UI per trasferire elementi da un elenco di origine a un elenco di destinazione con ricerca e selezione multipla." }
            }

            FeatureBanner { feature_name: "transfer-list" }

            ShowcaseTabs {
                preview: rsx! {
                    div { class: "w-full p-4 rounded-2xl bg-zinc-950/70 border border-white/[0.08]",
                        TransferList {
                            source: source_items,
                            target: target_items,
                            titles: ("Componenti da Integrare".to_string(), "Componenti Attivi".to_string()),
                            height: "280px".to_string(),
                        }
                    }
                },
                code: code_snippet,
                props: props,
            }
        }
    }
}
