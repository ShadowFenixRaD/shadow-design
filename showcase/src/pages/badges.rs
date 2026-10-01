use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::components::{FeatureBanner, PropRow, ShowcaseTabs};

#[component]
pub fn BadgesPage() -> Element {
    let mut selected_variant = use_signal(|| BadgeVariant::Primary);
    let mut is_pill = use_signal(|| true);
    let mut has_dot = use_signal(|| true);
    let badge_text = use_signal(|| "Online".to_string());

    let props = vec![
        PropRow {
            name: "label",
            r#type: "String",
            default: "-",
            description: "Testo visualizzato all'interno del badge",
        },
        PropRow {
            name: "variant",
            r#type: "BadgeVariant",
            default: "BadgeVariant::Default",
            description: "Variante semantica (Default, Primary, Success, Warning, Danger, Outline)",
        },
        PropRow {
            name: "pill",
            r#type: "bool",
            default: "false",
            description: "Rende i bordi completamente arrotondati (rounded-full)",
        },
        PropRow {
            name: "dot",
            r#type: "bool",
            default: "false",
            description: "Mostra un indicatore circolare con micro-animazione pulsante",
        },
    ];

    let code_snippet = r#"Badge {
    label: "Attivo".to_string(),
    variant: BadgeVariant::Success,
    pill: true,
    dot: true,
}"#
    .to_string();

    rsx! {
        div { class: "max-w-4xl space-y-8",
            div {
                Heading { order: HeadingOrder::H2, "Badge" }
                Text { dimmed: true, size: TextSize::Sm, "Etichette di stato e indicatori a pillola con supporto a micro-pulsazioni e forme rettangolari o arrotondate." }
            }

            FeatureBanner { feature_name: "badge" }

            ShowcaseTabs {
                preview: rsx! {
                    div { class: "grid grid-cols-1 lg:grid-cols-2 gap-8 items-center",
                        // Live Preview
                        div { class: "p-8 rounded-xl bg-[#060608] border border-white/[0.08] flex items-center justify-center min-h-[140px] shadow-inner",
                            Badge {
                                label: badge_text(),
                                variant: selected_variant(),
                                pill: is_pill(),
                                dot: has_dot(),
                            }
                        }

                        // Configurator
                        div { class: "space-y-4 text-xs font-mono",
                            div {
                                span { class: "text-zinc-400 block mb-1.5 uppercase tracking-wider text-[10px]", "Variante Semantica:" }
                                div { class: "flex flex-wrap gap-1.5",
                                    for v in [BadgeVariant::Default, BadgeVariant::Primary, BadgeVariant::Success, BadgeVariant::Warning, BadgeVariant::Danger, BadgeVariant::Outline] {
                                        button {
                                            r#type: "button",
                                            class: if selected_variant() == v {
                                                "px-2.5 py-1 rounded bg-purple-950/80 text-purple-200 border border-purple-500/40 font-bold cursor-pointer shadow-sm"
                                            } else {
                                                "px-2.5 py-1 rounded bg-white/[0.03] hover:bg-white/[0.08] text-zinc-400 hover:text-white border border-white/[0.06] cursor-pointer transition-colors"
                                            },
                                            onclick: move |_| selected_variant.set(v),
                                            "{v:?}"
                                        }
                                    }
                                }
                            }

                            div { class: "flex flex-wrap gap-4 pt-2 border-t border-white/[0.06]",
                                label { class: "flex items-center gap-1.5 cursor-pointer text-zinc-300",
                                    input {
                                        r#type: "checkbox",
                                        checked: "{is_pill}",
                                        onchange: move |e| is_pill.set(e.checked()),
                                    }
                                    "Forma a Pillola (Pill)"
                                }
                                label { class: "flex items-center gap-1.5 cursor-pointer text-zinc-300",
                                    input {
                                        r#type: "checkbox",
                                        checked: "{has_dot}",
                                        onchange: move |e| has_dot.set(e.checked()),
                                    }
                                    "Dot Pulsante (Live status)"
                                }
                            }
                        }
                    }
                },
                code: code_snippet,
                props: props,
            }

            // Visual Catalog
            Card {
                title: "Indicatori con Status Dot Animato".to_string(),
                subtitle: "Ideali per mostrare lo stato di servizi, pipeline o connessioni".to_string(),
                div { class: "flex flex-wrap items-center gap-3",
                    Badge { label: "Cluster Operativo".to_string(), variant: BadgeVariant::Success, dot: true, pill: true }
                    Badge { label: "Deployment In Corso".to_string(), variant: BadgeVariant::Primary, dot: true, pill: true }
                    Badge { label: "CPU Carico Alto".to_string(), variant: BadgeVariant::Warning, dot: true, pill: true }
                    Badge { label: "Disconnesso".to_string(), variant: BadgeVariant::Danger, dot: true, pill: true }
                    Badge { label: "Standby".to_string(), variant: BadgeVariant::Default, dot: true, pill: true }
                }
            }

            Card {
                title: "Varianti Rettangolari e Outline".to_string(),
                subtitle: "Stile sobrio per tag di codice, categorie e metadati".to_string(),
                div { class: "flex flex-wrap items-center gap-2.5",
                    Badge { label: "Rust 1.85".to_string(), variant: BadgeVariant::Default }
                    Badge { label: "Dioxus 0.7".to_string(), variant: BadgeVariant::Primary }
                    Badge { label: "Tailwind v4".to_string(), variant: BadgeVariant::Success }
                    Badge { label: "Zero Bloat".to_string(), variant: BadgeVariant::Outline }
                }
            }
        }
    }
}
