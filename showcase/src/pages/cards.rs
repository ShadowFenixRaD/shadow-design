use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::components::{FeatureBanner, PropRow, ShowcaseTabs};

#[component]
pub fn CardsPage() -> Element {
    let mut selected_variant = use_signal(|| CardVariant::Default);
    let mut selected_shadow = use_signal(|| CardShadow::Md);
    let mut enable_glow = use_signal(|| false);

    let props = vec![
        PropRow {
            name: "title",
            r#type: "Option<String>",
            default: "None",
            description: "Titolo primario dell'intestazione della card",
        },
        PropRow {
            name: "subtitle",
            r#type: "Option<String>",
            default: "None",
            description: "Sottotitolo o didascalia esplicativa",
        },
        PropRow {
            name: "variant",
            r#type: "CardVariant",
            default: "CardVariant::Default",
            description: "Variante visiva (Default, Elevated, Interactive, Metric)",
        },
        PropRow {
            name: "shadow",
            r#type: "CardShadow",
            default: "CardShadow::Md",
            description: "Livello di elevazione e profondità dell'ombra ottica",
        },
        PropRow {
            name: "hover_glow",
            r#type: "bool",
            default: "false",
            description:
                "Attiva un micro-glow coordinato con il colore primario al passaggio del cursore",
        },
        PropRow {
            name: "footer",
            r#type: "Option<Element>",
            default: "None",
            description: "Slot per la sezione a piè di pagina con sfondo dedicato",
        },
    ];

    let code_snippet = r#"Card {
    title: "Metric Analytics".to_string(),
    subtitle: "Dati registrati nelle ultime 24 ore".to_string(),
    variant: CardVariant::Interactive,
    hover_glow: true,
    div { class: "text-2xl font-bold text-white", "99.98%" }
    footer: rsx! {
        span { class: "text-xs text-zinc-400", "Aggiornato in tempo reale" }
    }
}"#
    .to_string();

    rsx! {
        div { class: "max-w-4xl space-y-8",
            div {
                Heading { order: HeadingOrder::H2, "Card" }
                Text { dimmed: true, size: TextSize::Sm, "Superfici di contenimento a 4 livelli, rifinite con bordi hairline a 1px, top specular highlights e micro-glow selettivi." }
            }

            FeatureBanner { feature_name: "card" }

            ShowcaseTabs {
                preview: rsx! {
                    div { class: "grid grid-cols-1 lg:grid-cols-2 gap-8 items-start",
                        // Live Card Preview
                        div { class: "w-full",
                            Card {
                                title: "Pannello di Controllo".to_string(),
                                subtitle: "Stato del server e metriche".to_string(),
                                variant: selected_variant(),
                                shadow: selected_shadow(),
                                hover_glow: enable_glow(),
                                footer: rsx! {
                                    div { class: "flex items-center justify-between",
                                        span { class: "text-xs text-zinc-500", "Stato: Operativo" }
                                        Button { size: ButtonSize::Xs, variant: ButtonVariant::Ghost, "Espandi" }
                                    }
                                },
                                div { class: "space-y-2",
                                    p { class: "text-xs text-zinc-300", "Contenuto interno della card con tipografia ad alto contrasto e bordi ottici calibrati." }
                                    Badge { label: "Cluster Attivo".to_string(), variant: BadgeVariant::Success, dot: true }
                                }
                            }
                        }

                        // Configurator
                        div { class: "space-y-4 text-xs font-mono",
                            div {
                                span { class: "text-zinc-400 block mb-1.5 uppercase tracking-wider text-[10px]", "Variante Card:" }
                                div { class: "flex flex-wrap gap-1.5",
                                    for v in [CardVariant::Default, CardVariant::Elevated, CardVariant::Interactive, CardVariant::Metric] {
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

                            div {
                                span { class: "text-zinc-400 block mb-1.5 uppercase tracking-wider text-[10px]", "Livello Ombra:" }
                                div { class: "flex flex-wrap gap-1.5",
                                    for s in [CardShadow::None, CardShadow::Sm, CardShadow::Md, CardShadow::Lg, CardShadow::Xl] {
                                        button {
                                            r#type: "button",
                                            class: if selected_shadow() == s {
                                                "px-2.5 py-1 rounded bg-purple-950/80 text-purple-200 border border-purple-500/40 font-bold cursor-pointer shadow-sm"
                                            } else {
                                                "px-2.5 py-1 rounded bg-white/[0.03] hover:bg-white/[0.08] text-zinc-400 hover:text-white border border-white/[0.06] cursor-pointer transition-colors"
                                            },
                                            onclick: move |_| selected_shadow.set(s),
                                            "{s:?}"
                                        }
                                    }
                                }
                            }

                            div { class: "pt-2 border-t border-white/[0.06]",
                                label { class: "flex items-center gap-1.5 cursor-pointer text-zinc-300",
                                    input {
                                        r#type: "checkbox",
                                        checked: "{enable_glow}",
                                        onchange: move |e| enable_glow.set(e.checked()),
                                    }
                                    "Attiva Micro-Glow su Hover"
                                }
                            }
                        }
                    }
                },
                code: code_snippet,
                props: props,
            }

            // Visual Examples
            div { class: "grid grid-cols-1 md:grid-cols-3 gap-5",
                Card {
                    variant: CardVariant::Metric,
                    div { class: "flex items-start justify-between gap-3",
                        div { class: "space-y-1.5",
                            span { class: "text-[11px] font-mono uppercase tracking-wider text-zinc-400 block", "Latenza Media" }
                            div { class: "text-2xl font-extrabold text-white tracking-tight", "1.2 ms" }
                            span { class: "text-[11px] text-emerald-400 font-mono flex items-center gap-1",
                                Icon { kind: IconKind::Check, size: 12 }
                                "+14% performance"
                            }
                        }
                        span { class: "p-2.5 rounded-xl bg-emerald-500/10 border border-emerald-500/20 text-emerald-400 flex items-center justify-center shrink-0",
                            Icon { kind: IconKind::Sparkles, size: 18 }
                        }
                    }
                }
                Card {
                    variant: CardVariant::Interactive,
                    hover_glow: true,
                    div { class: "flex items-start justify-between gap-3",
                        div { class: "space-y-1.5",
                            span { class: "text-[11px] font-mono uppercase tracking-wider text-zinc-400 block", "Richieste Totali" }
                            div { class: "text-2xl font-extrabold text-white tracking-tight", "4.8M" }
                            span { class: "text-[11px] text-purple-300 font-mono flex items-center gap-1",
                                Icon { kind: IconKind::Layers, size: 12 }
                                "Hover per micro-glow"
                            }
                        }
                        span { class: "p-2.5 rounded-xl bg-purple-500/10 border border-purple-500/20 text-purple-300 flex items-center justify-center shrink-0",
                            Icon { kind: IconKind::Layers, size: 18 }
                        }
                    }
                }
                Card {
                    variant: CardVariant::Elevated,
                    div { class: "flex items-start justify-between gap-3",
                        div { class: "space-y-1.5",
                            span { class: "text-[11px] font-mono uppercase tracking-wider text-zinc-400 block", "Affidabilità" }
                            div { class: "text-2xl font-extrabold text-white tracking-tight", "99.99%" }
                            span { class: "text-[11px] text-zinc-400 font-mono flex items-center gap-1",
                                Icon { kind: IconKind::CheckCircle, size: 12 }
                                "Livello Surface 2"
                            }
                        }
                        span { class: "p-2.5 rounded-xl bg-zinc-800/60 border border-white/10 text-zinc-300 flex items-center justify-center shrink-0",
                            Icon { kind: IconKind::CheckCircle, size: 18 }
                        }
                    }
                }
            }
        }
    }
}
