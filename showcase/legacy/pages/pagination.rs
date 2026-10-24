use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::components::{CheckControl, ChoicePicker, Counter, Playground, FeatureBanner, PageHeader, PropRow, ShowcaseTabs};

#[component]
pub fn PaginationPage() -> Element {
    let page = use_signal(|| 1);
    let total_pages = use_signal(|| 12);
    let siblings = use_signal(|| 1);
    let boundaries = use_signal(|| 1);
    let size = use_signal(|| ControlSize::Md);
    let show_edges = use_signal(|| true);
    let show_controls = use_signal(|| true);
    let is_disabled = use_signal(|| false);

    let props = vec![
        PropRow {
            name: "page",
            r#type: "Signal<usize>",
            default: "use_signal(|| 1)",
            description:
                "Signal reattivo collegato bidirezionalmente alla pagina corrente (1-indexed)",
        },
        PropRow {
            name: "total",
            r#type: "usize",
            default: "-",
            description:
                "Numero totale di pagine calcolato (es. totale_elementi / elementi_per_pagina)",
        },
        PropRow {
            name: "siblings",
            r#type: "usize",
            default: "1",
            description:
                "Numero di pagine adiacenti alla pagina attiva mostrate prima dell'ellissi",
        },
        PropRow {
            name: "boundaries",
            r#type: "usize",
            default: "1",
            description: "Numero di pagine sempre visibili all'inizio e alla fine del range",
        },
        PropRow {
            name: "show_edges",
            r#type: "bool",
            default: "false",
            description: "Mostra i pulsanti rapidi Prima Pagina («) e Ultima Pagina (»)",
        },
        PropRow {
            name: "show_controls",
            r#type: "bool",
            default: "true",
            description: "Mostra i pulsanti di navigazione Precedente (‹) e Successivo (›)",
        },
        PropRow {
            name: "size",
            r#type: "ControlSize",
            default: "ControlSize::Md",
            description: "Dimensione dei controlli tattili: ControlSize::Sm, Md o Lg",
        },
        PropRow {
            name: "on_change",
            r#type: "Option<EventHandler<usize>>",
            default: "None",
            description: "Callback emessa ad ogni transizione di pagina",
        },
    ];

    let code_snippet = r#"let mut current_page = use_signal(|| 1);

Pagination {
    page: current_page,
    total: 12,
    siblings: 1,
    boundaries: 1,
    show_edges: true,
    size: ControlSize::Md,
}"#
    .to_string();

    // Dati fittizi per la demo interattiva basata sulla pagina attiva
    let current_p = page();
    let records = (1..=4)
        .map(|i| {
            let id = (current_p - 1) * 4 + i;
            (
                id,
                format!("TX-NODE-{:04}", id),
                format!("0x{:x}..{:x}", id * 1234, id * 9876),
                id % 2 == 0,
            )
        })
        .collect::<Vec<_>>();

    rsx! {
        div { class: "sc-stack sc-stack--loose",
            PageHeader {
                title: "Pagination".to_string(),
                description: "Barra di navigazione a pagine numerata con calcolo dei salti ellittici, controlli direzionali, salto rapido agli estremi e supporto responsive.".to_string(),
            }

            FeatureBanner { feature_name: "pagination" }

            ShowcaseTabs {
                preview: rsx! {
                    Playground {
                        stage_start: true,
                        stage: rsx! {
                            div { class: "sc-stack w-full",
                                div { class: "sc-title-row",
                                    span { class: "sc-control-label", "Tabella audit paginata (record real-time)" }
                                    Counter { label: "Pagina".to_string(), value: format!("{} di {}", page(), total_pages()) }
                                }
                                List { spacing: ListSpacing::Md,
                                    for (id, name, hash, verified) in records {
                                        ListItem { key: "{id}",
                                            span { class: "sc-accent-text sc-mono", "#{id:03} " }
                                            strong { "{name}" }
                                            span { class: "sc-subtle-text sc-mono", " Digest: {hash} " }
                                            Badge {
                                                label: if verified { "VERIFIED".to_string() } else { "PENDING".to_string() },
                                                variant: if verified { BadgeVariant::Success } else { BadgeVariant::Default },
                                            }
                                        }
                                    }
                                }
                                div { class: "sc-title-row",
                                    span { class: "sc-subtle-text", "Visualizzazione di 4 elementi per pagina" }
                                    Pagination {
                                        page: page,
                                        total: total_pages(),
                                        siblings: siblings(),
                                        boundaries: boundaries(),
                                        show_controls: show_controls(),
                                        show_edges: show_edges(),
                                        size: size(),
                                        disabled: is_disabled(),
                                    }
                                }
                            }
                        },
                        controls: rsx! {
                            ChoicePicker {
                                label: "Totale pagine".to_string(),
                                options: vec![
                                    (8, "8".to_string()),
                                    (12, "12".to_string()),
                                    (15, "15".to_string()),
                                    (30, "30".to_string()),
                                ],
                                value: total_pages,
                            }
                            ChoicePicker {
                                label: "Taglia controlli (ControlSize)".to_string(),
                                options: vec![
                                    (ControlSize::Sm, "Small".to_string()),
                                    (ControlSize::Md, "Medium".to_string()),
                                    (ControlSize::Lg, "Large".to_string()),
                                ],
                                value: size,
                            }
                            hr { class: "sc-control-divider" }
                            CheckControl { label: "Mostra estremi (« / »)".to_string(), checked: show_edges }
                            CheckControl { label: "Mostra frecce precedente/successivo".to_string(), checked: show_controls }
                            CheckControl { label: "Disabilita barra".to_string(), checked: is_disabled }
                        },
                    }
                },
                code: code_snippet,
                props: props,
            }
        }
    }
}
