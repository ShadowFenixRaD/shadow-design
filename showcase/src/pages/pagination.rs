use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::components::{FeatureBanner, PropRow, ShowcaseTabs};

#[component]
pub fn PaginationPage() -> Element {
    let page = use_signal(|| 1);
    let mut total_pages = use_signal(|| 12);
    let siblings = use_signal(|| 1);
    let boundaries = use_signal(|| 1);
    let mut size = use_signal(|| ControlSize::Md);
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
        div { class: "max-w-4xl space-y-8",
            div {
                Heading { order: HeadingOrder::H2, "Pagination" }
                Text { dimmed: true, size: TextSize::Sm, "Barra di navigazione a pagine numerata con calcolo dei salti ellittici, controlli direzionali, salto rapido agli estremi e supporto responsive." }
            }

            FeatureBanner { feature_name: "pagination" }

            ShowcaseTabs {
                preview: rsx! {
                    div { class: "space-y-6",
                        // Toolbar di configurazione
                        div { class: "p-4 rounded-xl bg-[#060608] border border-white/[0.08] grid grid-cols-1 md:grid-cols-3 gap-4 text-xs font-mono",
                            div {
                                span { class: "text-zinc-400 block mb-1.5 uppercase tracking-wider text-[10px]", "Totale Pagine:" }
                                div { class: "flex gap-2",
                                    button {
                                        r#type: "button",
                                        class: if total_pages() == 8 {
                                            "px-2.5 py-1 rounded bg-purple-950/80 text-purple-200 border border-purple-500/40 font-bold cursor-pointer"
                                        } else {
                                            "px-2.5 py-1 rounded bg-white/[0.03] text-zinc-400 border border-white/[0.06] cursor-pointer"
                                        },
                                        onclick: move |_| total_pages.set(8),
                                        "8 Pagine"
                                    }
                                    button {
                                        r#type: "button",
                                        class: if total_pages() == 15 {
                                            "px-2.5 py-1 rounded bg-purple-950/80 text-purple-200 border border-purple-500/40 font-bold cursor-pointer"
                                        } else {
                                            "px-2.5 py-1 rounded bg-white/[0.03] text-zinc-400 border border-white/[0.06] cursor-pointer"
                                        },
                                        onclick: move |_| total_pages.set(15),
                                        "15 Pagine"
                                    }
                                    button {
                                        r#type: "button",
                                        class: if total_pages() == 30 {
                                            "px-2.5 py-1 rounded bg-purple-950/80 text-purple-200 border border-purple-500/40 font-bold cursor-pointer"
                                        } else {
                                            "px-2.5 py-1 rounded bg-white/[0.03] text-zinc-400 border border-white/[0.06] cursor-pointer"
                                        },
                                        onclick: move |_| total_pages.set(30),
                                        "30 Pagine"
                                    }
                                }
                            }

                            div {
                                span { class: "text-zinc-400 block mb-1.5 uppercase tracking-wider text-[10px]", "Taglia Controlli (ControlSize):" }
                                div { class: "flex gap-2",
                                    button {
                                        r#type: "button",
                                        class: if size() == ControlSize::Sm {
                                            "px-2.5 py-1 rounded bg-purple-950/80 text-purple-200 border border-purple-500/40 font-bold cursor-pointer"
                                        } else {
                                            "px-2.5 py-1 rounded bg-white/[0.03] text-zinc-400 border border-white/[0.06] cursor-pointer"
                                        },
                                        onclick: move |_| size.set(ControlSize::Sm),
                                        "Small"
                                    }
                                    button {
                                        r#type: "button",
                                        class: if size() == ControlSize::Md {
                                            "px-2.5 py-1 rounded bg-purple-950/80 text-purple-200 border border-purple-500/40 font-bold cursor-pointer"
                                        } else {
                                            "px-2.5 py-1 rounded bg-white/[0.03] text-zinc-400 border border-white/[0.06] cursor-pointer"
                                        },
                                        onclick: move |_| size.set(ControlSize::Md),
                                        "Medium"
                                    }
                                    button {
                                        r#type: "button",
                                        class: if size() == ControlSize::Lg {
                                            "px-2.5 py-1 rounded bg-purple-950/80 text-purple-200 border border-purple-500/40 font-bold cursor-pointer"
                                        } else {
                                            "px-2.5 py-1 rounded bg-white/[0.03] text-zinc-400 border border-white/[0.06] cursor-pointer"
                                        },
                                        onclick: move |_| size.set(ControlSize::Lg),
                                        "Large"
                                    }
                                }
                            }

                            div { class: "flex flex-col justify-center space-y-2 pt-2 md:pt-0 border-t md:border-t-0 md:border-l border-white/[0.06] md:pl-4",
                                Checkbox {
                                    checked: show_edges,
                                    label: Some("Mostra Estremi (« / »)".to_string()),
                                }
                                Checkbox {
                                    checked: show_controls,
                                    label: Some("Mostra Frecce Precedente/Successivo".to_string()),
                                }
                                Checkbox {
                                    checked: is_disabled,
                                    label: Some("Disabilita Barra".to_string()),
                                }
                            }
                        }

                        // Preview box interattiva con Tabella Mockup collegata al segnale
                        div { class: "p-6 rounded-xl bg-[#030304] border border-white/[0.08] shadow-2xl space-y-6",
                            div { class: "flex items-center justify-between text-xs font-mono text-zinc-400 pb-3 border-b border-white/[0.06]",
                                span { "TABELLA AUDIT PAGINATA (RECORD REAL-TIME)" }
                                span { class: "text-purple-400 font-bold", "Pagina {page()} di {total_pages()}" }
                            }

                            // Mock table rows
                            div { class: "space-y-2",
                                for (id, name, hash, verified) in records {
                                    div {
                                        key: "{id}",
                                        class: "p-3 rounded-lg bg-zinc-950/60 border border-white/[0.04] flex items-center justify-between hover:border-white/[0.1] transition-colors",
                                        div { class: "flex items-center gap-3",
                                            span { class: "text-xs font-mono text-purple-400 font-bold w-12", "#{id:03}" }
                                            div {
                                                div { class: "text-xs font-medium text-zinc-200", "{name}" }
                                                div { class: "text-[10px] font-mono text-zinc-500", "Digest crittografico: {hash}" }
                                            }
                                        }
                                        Badge {
                                            label: if verified { "VERIFIED".to_string() } else { "PENDING".to_string() },
                                            variant: if verified { BadgeVariant::Success } else { BadgeVariant::Default },
                                        }
                                    }
                                }
                            }

                            // Barra di paginazione centrata
                            div { class: "pt-4 border-t border-white/[0.06] flex items-center justify-between flex-wrap gap-4",
                                div { class: "text-[11px] font-mono text-zinc-500",
                                    "Visualizzazione di 4 elementi per pagina"
                                }
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
                    }
                },
                code: code_snippet,
                props: props,
            }
        }
    }
}
