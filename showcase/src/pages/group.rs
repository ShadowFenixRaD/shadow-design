use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::components::banner::FeatureBanner;
use crate::components::props_table::PropRow;
use crate::components::showcase_tabs::ShowcaseTabs;

#[component]
pub fn GroupPage() -> Element {
    let mut selected_gap = use_signal(|| SpacingToken::Md);
    let mut selected_align = use_signal(|| AlignItems::Center);
    let mut selected_justify = use_signal(|| JustifyContent::Start);
    let wrap = use_signal(|| true);
    let grow = use_signal(|| false);

    let props = vec![
        PropRow {
            name: "gap",
            r#type: "SpacingToken",
            default: "SpacingToken::Md",
            description: "Spaziatura orizzontale e verticale tra gli elementi (None, Xs, Sm, Md, Lg, Xl).",
        },
        PropRow {
            name: "align",
            r#type: "AlignItems",
            default: "AlignItems::Center",
            description: "Allineamento trasversale (cross-axis: Start, Center, End, Stretch, Baseline).",
        },
        PropRow {
            name: "justify",
            r#type: "JustifyContent",
            default: "JustifyContent::Start",
            description: "Giustificazione lungo l'asse orizzontale (Start, Center, End, SpaceBetween, SpaceAround, SpaceEvenly).",
        },
        PropRow {
            name: "wrap",
            r#type: "bool",
            default: "true",
            description: "Abilita o disabilita il ritorno a capo automatico (flex-wrap) quando lo spazio si esaurisce.",
        },
        PropRow {
            name: "grow",
            r#type: "bool",
            default: "false",
            description: "Se abilitato, forza tutti gli elementi figli ad espandersi equamente per riempire lo spazio residuo.",
        },
        PropRow {
            name: "class",
            r#type: "String",
            default: "\"\"",
            description: "Classi CSS aggiuntive applicate al contenitore Group.",
        },
    ];

    let code_snippet = r#"Group {
    gap: SpacingToken::Md,
    align: AlignItems::Center,
    justify: JustifyContent::SpaceBetween,
    wrap: true,
    grow: false,
    Button { variant: ButtonVariant::Primary, "Azione Primaria" }
    Button { variant: ButtonVariant::Outline, "Azione Secondaria" }
    Badge { label: "Stato Attivo".to_string(), variant: BadgeVariant::Success }
}"#
    .to_string();

    rsx! {
        div { class: "max-w-4xl space-y-8",
            div {
                Heading { order: HeadingOrder::H2, "Group" }
                Text { dimmed: true, size: TextSize::Sm, "Primitiva Flexbox orizzontale ad alta ergonomia. Gestisce allineamento, giustificazione, spaziatura tokenizzata uniforme e ritorno a capo automatico senza codice CSS boilerplate." }
            }

            FeatureBanner { feature_name: "group" }

            ShowcaseTabs {
                preview: rsx! {
                    div { class: "space-y-6 w-full",
                        // Live Group Container
                        div { class: "p-6 rounded-2xl bg-zinc-950/80 border border-white/[0.08] min-h-[140px] flex items-center justify-center",
                            Group {
                                gap: selected_gap(),
                                align: selected_align(),
                                justify: selected_justify(),
                                wrap: wrap(),
                                grow: grow(),
                                class: "w-full",
                                Button { variant: ButtonVariant::Primary, size: ButtonSize::Sm, "Primario" }
                                Button { variant: ButtonVariant::Outline, size: ButtonSize::Sm, "Secondario" }
                                Button { variant: ButtonVariant::Ghost, size: ButtonSize::Sm, "Ghost" }
                                Badge { label: "Mesh Online".to_string(), variant: BadgeVariant::Success, dot: true }
                                Badge { label: "Kyber-1024".to_string(), variant: BadgeVariant::Primary }
                                div { class: "px-3 py-1.5 rounded-lg bg-zinc-900 border border-white/10 text-xs font-mono text-zinc-300", "Custom Box" }
                            }
                        }

                        // Configurator Controls
                        div { class: "p-4 rounded-xl bg-white/[0.02] border border-white/[0.08] space-y-4",
                            span { class: "text-xs font-mono uppercase tracking-wider text-zinc-400 font-semibold block", "Controlli Interattivi Playground" }
                            div { class: "space-y-4",
                                div {
                                    span { class: "text-xs font-mono text-zinc-400 uppercase tracking-wider block mb-2", "Spaziatura (Gap)" }
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
                                    span { class: "text-xs font-mono text-zinc-400 uppercase tracking-wider block mb-2", "Allineamento (AlignItems)" }
                                    div { class: "flex flex-wrap gap-2",
                                        for a in [AlignItems::Start, AlignItems::Center, AlignItems::End, AlignItems::Stretch, AlignItems::Baseline] {
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
                                    span { class: "text-xs font-mono text-zinc-400 uppercase tracking-wider block mb-2", "Giustificazione (JustifyContent)" }
                                    div { class: "flex flex-wrap gap-2",
                                        for j in [JustifyContent::Start, JustifyContent::Center, JustifyContent::End, JustifyContent::SpaceBetween, JustifyContent::SpaceAround, JustifyContent::SpaceEvenly] {
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

                                div { class: "flex items-center gap-6 pt-2 border-t border-white/[0.06]",
                                    Checkbox {
                                        checked: wrap,
                                        label: "Wrap (Ritorno a capo automatico)".to_string(),
                                        size: CheckboxSize::Sm,
                                    }
                                    Checkbox {
                                        checked: grow,
                                        label: "Grow (Espansione uniforme elementi)".to_string(),
                                        size: CheckboxSize::Sm,
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
