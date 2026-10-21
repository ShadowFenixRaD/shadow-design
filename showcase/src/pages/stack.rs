use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::components::banner::FeatureBanner;
use crate::components::props_table::PropRow;
use crate::components::showcase_tabs::ShowcaseTabs;

#[component]
pub fn StackPage() -> Element {
    let mut selected_gap = use_signal(|| SpacingToken::Md);
    let mut selected_align = use_signal(|| AlignItems::Stretch);
    let mut selected_justify = use_signal(|| JustifyContent::Start);

    let props = vec![
        PropRow {
            name: "gap",
            r#type: "SpacingToken",
            default: "SpacingToken::Md",
            description: "Spaziatura verticale uniforme tra tutti i figli dello Stack (None, Xs, Sm, Md, Lg, Xl).",
        },
        PropRow {
            name: "align",
            r#type: "AlignItems",
            default: "AlignItems::Stretch",
            description: "Allineamento orizzontale trasversale dei figli (Start, Center, End, Stretch, Baseline).",
        },
        PropRow {
            name: "justify",
            r#type: "JustifyContent",
            default: "JustifyContent::Start",
            description: "Distribuzione verticale lungo l'asse principale (Start, Center, End, SpaceBetween, SpaceAround, SpaceEvenly).",
        },
        PropRow {
            name: "class",
            r#type: "String",
            default: "\"\"",
            description: "Classi CSS personalizzate applicate al contenitore Stack.",
        },
    ];

    let code_snippet = r#"Stack {
    gap: SpacingToken::Md,
    align: AlignItems::Stretch,
    Card {
        div { class: "p-3", "Primo blocco dati" }
    }
    Card {
        div { class: "p-3", "Secondo blocco dati" }
    }
    Button { variant: ButtonVariant::Primary, "Salva Configurazione" }
}"#
    .to_string();

    rsx! {
        div { class: "max-w-4xl space-y-8",
            div {
                Heading { order: HeadingOrder::H2, "Stack" }
                Text { dimmed: true, size: TextSize::Sm, "Primitiva Flexbox verticale a colonna per impilare componenti con spaziatura tokenizzata rigorosa, allineamento calibrato e zero margini esterni dispersivi." }
            }

            FeatureBanner { feature_name: "stack" }

            ShowcaseTabs {
                preview: rsx! {
                    div { class: "space-y-6 w-full",
                        // Live Stack Container
                        div { class: "p-6 rounded-2xl bg-zinc-950/80 border border-white/[0.08] min-h-[220px]",
                            Stack {
                                gap: selected_gap(),
                                align: selected_align(),
                                justify: selected_justify(),
                                class: "w-full",
                                div { class: "p-3.5 rounded-xl bg-purple-950/40 border border-purple-800/40 flex items-center justify-between",
                                    span { class: "text-xs font-mono text-purple-200 font-semibold", "Modulo Alpha (Security Core)" }
                                    Badge { label: "99.98% SLA".to_string(), variant: BadgeVariant::Success, dot: true }
                                }
                                div { class: "p-3.5 rounded-xl bg-zinc-900/70 border border-white/10 flex items-center justify-between",
                                    span { class: "text-xs font-mono text-zinc-300", "Servizio Router Mesh P2P" }
                                    Badge { label: "Attivo".to_string(), variant: BadgeVariant::Primary }
                                }
                                div { class: "p-3.5 rounded-xl bg-white/[0.02] border border-white/[0.06] flex items-center justify-between",
                                    span { class: "text-xs font-mono text-zinc-400", "Canale Telemetrico e Log" }
                                    span { class: "text-[11px] font-mono text-zinc-500", "4.2 MB/s" }
                                }
                            }
                        }

                        // Configurator Controls
                        div { class: "p-4 rounded-xl bg-white/[0.02] border border-white/[0.08] space-y-4",
                            span { class: "text-xs font-mono uppercase tracking-wider text-zinc-400 font-semibold block", "Controlli Interattivi Playground" }
                            div { class: "space-y-4",
                                div {
                                    span { class: "text-xs font-mono text-zinc-400 uppercase tracking-wider block mb-2", "Spaziatura Verticale (Gap)" }
                                    div { class: "flex flex-wrap gap-2",
                                        for s in [SpacingToken::None, SpacingToken::Xs, SpacingToken::Sm, SpacingToken::Md, SpacingToken::Lg, SpacingToken::Xl] {
                                            button {
                                                class: if selected_gap() == s {
                                                    "px-3 py-1 text-xs font-mono rounded-lg bg-purple-950 border border-purple-600 text-purple-200"
                                                } else {
                                                    "px-3 py-1 text-xs font-mono rounded-lg bg-zinc-900 border border-white/10 text-zinc-400 hover:text-white"
                                                },
                                                onclick: move |_| selected_gap.set(s),
                                                "{s:?}"
                                            }
                                        }
                                    }
                                }

                                div {
                                    span { class: "text-xs font-mono text-zinc-400 uppercase tracking-wider block mb-2", "Allineamento Orizzontale (AlignItems)" }
                                    div { class: "flex flex-wrap gap-2",
                                        for a in [AlignItems::Start, AlignItems::Center, AlignItems::End, AlignItems::Stretch] {
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
                                    span { class: "text-xs font-mono text-zinc-400 uppercase tracking-wider block mb-2", "Giustificazione Verticale (JustifyContent)" }
                                    div { class: "flex flex-wrap gap-2",
                                        for j in [JustifyContent::Start, JustifyContent::Center, JustifyContent::End, JustifyContent::SpaceBetween] {
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
