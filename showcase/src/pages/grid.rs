use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::components::banner::FeatureBanner;
use crate::components::props_table::PropRow;
use crate::components::showcase_tabs::ShowcaseTabs;

#[component]
pub fn GridPage() -> Element {
    let mut selected_gutter = use_signal(|| GridSpacing::Md);
    let mut selected_align = use_signal(|| GridAlign::Stretch);
    let mut selected_justify = use_signal(|| GridJustify::Start);
    let mut simple_cols = use_signal(|| 3u8);

    let props = vec![
        PropRow {
            name: "gutter",
            r#type: "GridSpacing",
            default: "GridSpacing::Md",
            description: "Spaziatura orizzontale e verticale tra le celle (None, Xs, Sm, Md, Lg, Xl).",
        },
        PropRow {
            name: "row_gutter",
            r#type: "Option<GridSpacing>",
            default: "None",
            description: "Gutter verticale indipendente per separare le righe con passo diverso.",
        },
        PropRow {
            name: "align",
            r#type: "GridAlign",
            default: "GridAlign::Stretch",
            description: "Allineamento verticale delle celle (Start, Center, End, Stretch, Baseline).",
        },
        PropRow {
            name: "justify",
            r#type: "GridJustify",
            default: "GridJustify::Start",
            description: "Giustificazione orizzontale (Start, Center, End, SpaceBetween, SpaceAround).",
        },
        PropRow {
            name: "span (GridCol)",
            r#type: "u8",
            default: "12",
            description: "Numero di colonne occupate nella griglia a 12 colonne (1..=12).",
        },
        PropRow {
            name: "offset (GridCol)",
            r#type: "u8",
            default: "0",
            description: "Numero di colonne di offset iniziale per spaziatura o rientro a sinistra (0..=11).",
        },
        PropRow {
            name: "sm, md, lg, xl (GridCol)",
            r#type: "Option<u8>",
            default: "None",
            description: "Span responsive per i rispettivi breakpoint WAI-ARIA (640px, 768px, 1024px, 1280px).",
        },
        PropRow {
            name: "cols (SimpleGrid)",
            r#type: "u8",
            default: "1",
            description: "Numero di colonne a larghezza equivalente della SimpleGrid (1..=12).",
        },
        PropRow {
            name: "spacing (SimpleGrid)",
            r#type: "GridSpacing",
            default: "GridSpacing::Md",
            description: "Spaziatura uniforme orizzontale e verticale tra le celle di SimpleGrid.",
        },
    ];

    let code_snippet = r#"// --- Grid a 12 Colonne Responsive ---
Grid {
    gutter: GridSpacing::Md,
    align: GridAlign::Stretch,
    GridCol { span: 12, md: 8,
        div { class: "p-4 rounded-xl bg-purple-950/40 border border-purple-800/40 text-purple-200", "Colonna Principale (Span 8 su Desktop)" }
    }
    GridCol { span: 12, md: 4,
        div { class: "p-4 rounded-xl bg-zinc-900/60 border border-white/10 text-zinc-300", "Sidebar Laterale (Span 4 su Desktop)" }
    }
}

// --- SimpleGrid a Colonne Equivalenti ---
SimpleGrid {
    cols: 3,
    spacing: GridSpacing::Md,
    sm: 1, md: 2, lg: 3,
    div { class: "p-4 rounded-xl bg-zinc-900/60 border border-white/10", "Card A" }
    div { class: "p-4 rounded-xl bg-zinc-900/60 border border-white/10", "Card B" }
    div { class: "p-4 rounded-xl bg-zinc-900/60 border border-white/10", "Card C" }
}"#
    .to_string();

    rsx! {
        div { class: "max-w-4xl space-y-8",
            div {
                Heading { order: HeadingOrder::H2, "Grid & SimpleGrid" }
                Text { dimmed: true, size: TextSize::Sm, "Sistemi strutturali responsive a 12 colonne (Grid / GridCol) e a colonne equivalenti auto-bilanciate (SimpleGrid) con gutter tokenizzato e breakpoint standard." }
            }

            FeatureBanner { feature_name: "grid" }

            ShowcaseTabs {
                preview: rsx! {
                    div { class: "space-y-6 w-full",
                        // Sezione 1: Grid 12 Colonne
                        div { class: "space-y-3",
                            div { class: "flex items-center justify-between",
                                span { class: "text-xs font-mono uppercase tracking-wider text-purple-400 font-semibold", "1. Grid a 12 Colonne con Span e Offset" }
                                span { class: "text-[11px] font-mono text-zinc-500", "Gutter: {selected_gutter():?}" }
                            }

                            Grid {
                                gutter: selected_gutter(),
                                align: selected_align(),
                                justify: selected_justify(),
                                GridCol { span: 12, md: 8,
                                    div { class: "p-4 rounded-xl bg-purple-950/40 border border-purple-800/40 flex flex-col justify-center text-center",
                                        span { class: "text-xs font-mono text-purple-200 font-bold", "Span: 12 (Mobile) / 8 (Desktop)" }
                                        span { class: "text-[11px] text-purple-400/80", "Contenuto principale ad alta larghezza" }
                                    }
                                }
                                GridCol { span: 12, md: 4,
                                    div { class: "p-4 rounded-xl bg-zinc-900/60 border border-white/10 flex flex-col justify-center text-center",
                                        span { class: "text-xs font-mono text-zinc-300 font-bold", "Span: 12 / 4" }
                                        span { class: "text-[11px] text-zinc-500", "Aside contestuale" }
                                    }
                                }
                                GridCol { span: 6, md: 4,
                                    div { class: "p-3 rounded-lg bg-white/[0.03] border border-white/10 text-center text-xs font-mono text-zinc-400", "Colonna 1 (Span 4)" }
                                }
                                GridCol { span: 6, md: 4,
                                    div { class: "p-3 rounded-lg bg-white/[0.03] border border-white/10 text-center text-xs font-mono text-zinc-400", "Colonna 2 (Span 4)" }
                                }
                                GridCol { span: 12, md: 4,
                                    div { class: "p-3 rounded-lg bg-white/[0.03] border border-white/10 text-center text-xs font-mono text-zinc-400", "Colonna 3 (Span 4)" }
                                }
                                GridCol { span: 12, md: 6, offset: 3,
                                    div { class: "p-3 rounded-lg bg-purple-900/20 border border-purple-500/30 text-center text-xs font-mono text-purple-300", "Span 6 con Offset 3 (Centrato su 12 col)" }
                                }
                            }
                        }

                        // Sezione 2: SimpleGrid Auto-Layout
                        div { class: "space-y-3 pt-4 border-t border-white/[0.08]",
                            div { class: "flex items-center justify-between",
                                span { class: "text-xs font-mono uppercase tracking-wider text-purple-400 font-semibold", "2. SimpleGrid a Colonne Equivalenti" }
                                span { class: "text-[11px] font-mono text-zinc-500", "Cols attive: {simple_cols()}" }
                            }

                            SimpleGrid {
                                cols: simple_cols(),
                                spacing: selected_gutter(),
                                sm: 1,
                                md: simple_cols(),
                                for i in 1..=6 {
                                    div { class: "p-4 rounded-xl bg-zinc-900/60 border border-white/10 flex flex-col gap-1 items-center justify-center text-center shadow-sm",
                                        span { class: "w-7 h-7 rounded-lg bg-purple-500/10 border border-purple-500/20 text-purple-300 font-mono text-xs flex items-center justify-center font-bold", "{i}" }
                                        span { class: "text-xs font-semibold text-white", "Modulo #{i}" }
                                        span { class: "text-[10px] text-zinc-500 font-mono", "Equal 1/{simple_cols()} fr" }
                                    }
                                }
                            }
                        }
                        // Sezione 3: Configurator / Controlli Interattivi
                        div { class: "p-4 rounded-xl bg-white/[0.02] border border-white/[0.08] space-y-4",
                            span { class: "text-xs font-mono uppercase tracking-wider text-zinc-400 font-semibold block", "Controlli Interattivi Playground" }
                            div { class: "space-y-4",
                                div {
                                    span { class: "text-xs font-mono text-zinc-400 uppercase tracking-wider block mb-2", "Gutter Spaziatura" }
                                    div { class: "flex flex-wrap gap-2",
                                        for s in [GridSpacing::None, GridSpacing::Xs, GridSpacing::Sm, GridSpacing::Md, GridSpacing::Lg, GridSpacing::Xl] {
                                            button {
                                                class: if selected_gutter() == s {
                                                    "px-3 py-1 text-xs font-mono rounded-lg bg-purple-950 border border-purple-600 text-purple-200"
                                                } else {
                                                    "px-3 py-1 text-xs font-mono rounded-lg bg-zinc-900 border border-white/10 text-zinc-400 hover:text-white"
                                                },
                                                onclick: move |_| selected_gutter.set(s),
                                                "{s:?}"
                                            }
                                        }
                                    }
                                }

                                div {
                                    span { class: "text-xs font-mono text-zinc-400 uppercase tracking-wider block mb-2", "Allineamento Verticale" }
                                    div { class: "flex flex-wrap gap-2",
                                        for a in [GridAlign::Start, GridAlign::Center, GridAlign::End, GridAlign::Stretch] {
                                            button {
                                                class: if selected_align() == a {
                                                    "px-3 py-1 text-xs font-mono rounded-lg bg-purple-950 border border-purple-600 text-purple-200"
                                                } else {
                                                    "px-3 py-1 text-xs font-mono rounded-lg bg-zinc-900 border border-white/10 text-zinc-400 hover:text-white"
                                                },
                                                onclick: move |_| selected_align.set(a),
                                                "{a:?}"
                                            }
                                        }
                                    }
                                }

                                div {
                                    span { class: "text-xs font-mono text-zinc-400 uppercase tracking-wider block mb-2", "Giustificazione Orizzontale" }
                                    div { class: "flex flex-wrap gap-2",
                                        for j in [GridJustify::Start, GridJustify::Center, GridJustify::End, GridJustify::SpaceBetween, GridJustify::SpaceAround] {
                                            button {
                                                class: if selected_justify() == j {
                                                    "px-3 py-1 text-xs font-mono rounded-lg bg-purple-950 border border-purple-600 text-purple-200"
                                                } else {
                                                    "px-3 py-1 text-xs font-mono rounded-lg bg-zinc-900 border border-white/10 text-zinc-400 hover:text-white"
                                                },
                                                onclick: move |_| selected_justify.set(j),
                                                "{j:?}"
                                            }
                                        }
                                    }
                                }

                                div {
                                    span { class: "text-xs font-mono text-zinc-400 uppercase tracking-wider block mb-2", "Colonne SimpleGrid" }
                                    div { class: "flex flex-wrap gap-2",
                                        for c in [1u8, 2, 3, 4, 6] {
                                            button {
                                                class: if simple_cols() == c {
                                                    "px-3 py-1 text-xs font-mono rounded-lg bg-purple-950 border border-purple-600 text-purple-200"
                                                } else {
                                                    "px-3 py-1 text-xs font-mono rounded-lg bg-zinc-900 border border-white/10 text-zinc-400 hover:text-white"
                                                },
                                                onclick: move |_| simple_cols.set(c),
                                                "{c} Colonne"
                                            }
                                        }
                                    }
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
