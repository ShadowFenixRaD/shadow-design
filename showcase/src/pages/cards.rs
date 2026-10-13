use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::components::banner::FeatureBanner;
use crate::components::props_table::PropRow;
use crate::components::showcase_tabs::ShowcaseTabs;

#[component]
pub fn CardsPage() -> Element {
    let mut selected_variant = use_signal(|| CardVariant::Default);
    let mut selected_shadow = use_signal(|| CardShadow::Md);
    let with_avatar = use_signal(|| true);
    let with_title = use_signal(|| true);
    let with_subtitle = use_signal(|| true);
    let with_progress = use_signal(|| false);
    let with_buttons = use_signal(|| true);
    let with_image = use_signal(|| false);
    let enable_glow = use_signal(|| false);
    let with_max_height = use_signal(|| false);

    let props = vec![
        PropRow {
            name: "id",
            r#type: "Option<String>",
            default: "None",
            description: "Identificatore univoco HTML per il contenitore.",
        },
        PropRow {
            name: "title",
            r#type: "Option<String>",
            default: "None",
            description: "Titolo primario dell'intestazione (renderizzato in h3).",
        },
        PropRow {
            name: "subtitle",
            r#type: "Option<String>",
            default: "None",
            description: "Testo secondario sotto al titolo in grigio attenuato.",
        },
        PropRow {
            name: "avatar",
            r#type: "Option<Element>",
            default: "None",
            description: "Slot per icona o componente Avatar a sinistra dell'intestazione.",
        },
        PropRow {
            name: "variant",
            r#type: "CardVariant",
            default: "CardVariant::Default",
            description: "Variante visiva (Default, Elevated, Interactive, Metric).",
        },
        PropRow {
            name: "shadow",
            r#type: "CardShadow",
            default: "CardShadow::Md",
            description: "Livello di elevazione e profondità dell'ombra ottica (None, Sm, Md, Lg, Xl).",
        },
        PropRow {
            name: "hover_glow",
            r#type: "bool",
            default: "false",
            description: "Micro-glow coordinato con il colore del tema attivo al passaggio del mouse.",
        },
        PropRow {
            name: "glow",
            r#type: "Option<bool>",
            default: "None",
            description: "Controllo esplicito del bagliore ottico della scheda.",
        },
        PropRow {
            name: "onclick / on_click",
            r#type: "Option<EventHandler<MouseEvent>>",
            default: "None",
            description: "Callback di click per CardVariant::Interactive con role='button' e tastiera.",
        },
        PropRow {
            name: "href",
            r#type: "Option<String>",
            default: "None",
            description: "URL di destinazione per CardVariant::Interactive tramite link stretched accessibile.",
        },
        PropRow {
            name: "progress",
            r#type: "Option<f32>",
            default: "None",
            description: "Avanzamento percentuale (0.0 .. 100.0) con barra ProgressBar integrata.",
        },
        PropRow {
            name: "buttons",
            r#type: "Option<Element>",
            default: "None",
            description: "Slot inferiore per azioni rapide e pulsanti.",
        },
        PropRow {
            name: "image",
            r#type: "Option<String>",
            default: "None",
            description: "Immagine a tutta larghezza in basso con adattamento e ritaglio bordi.",
        },
        PropRow {
            name: "image_height",
            r#type: "Option<String>",
            default: "None",
            description: "Altezza personalizzata opzionale per l'immagine inferiore (es. '160px').",
        },
        PropRow {
            name: "footer",
            r#type: "Option<Element>",
            default: "None",
            description: "Slot per la sezione a piè di pagina con sfondo e divisore dedicato.",
        },
        PropRow {
            name: "overflow_hidden",
            r#type: "bool",
            default: "false",
            description: "Forza overflow hidden per ritagliare media senza occludere menu dropdown.",
        },
        PropRow {
            name: "class",
            r#type: "String",
            default: "\"\"",
            description: "Classi CSS aggiuntive applicate al contenitore.",
        },
    ];

    let code_snippet = r#"Card {
    title: Some("Mesh Node Cluster".to_string()),
    subtitle: Some("Topologia di rete crittografica P2P".to_string()),
    variant: CardVariant::Interactive,
    shadow: CardShadow::Lg,
    hover_glow: true,
    avatar: rsx! {
        Avatar { name: Some("Shadow Core".to_string()), size: AvatarSize::Md, status: PresenceStatus::Online }
    },
    progress: Some(82.0),
    buttons: rsx! {
        Button { size: ButtonSize::Sm, variant: ButtonVariant::Primary, "Connetti" }
        Button { size: ButtonSize::Sm, variant: ButtonVariant::Outline, "Dettagli" }
    },
    image: Some("https://images.unsplash.com/photo-1618005182384-a83a8bd57fbe?auto=format&fit=crop&w=600&q=80".to_string()),
    footer: rsx! {
        span { class: "text-zinc-500", "Ultima sincronizzazione 2 minuti fa" }
    },
    p { class: "text-xs text-zinc-300", "Nodo di sincronizzazione Kyber-1024 attivo." }
}"#
        .to_string();

    let max_h_class = if with_max_height() { "max-h-[420px]" } else { "" };

    rsx! {
        div { class: "max-w-4xl space-y-8",
            div {
                Heading { order: HeadingOrder::H2, "Card (Unificata)" }
                Text { dimmed: true, size: TextSize::Sm, "Superficie modulare avanzata: supporto nativo per avatar, titolo centrato o affiancato, sottotitolo, progress bar, pulsanti d'azione dedicati e cover image inferiore con ritaglio bordi e max-height." }
            }

            FeatureBanner { feature_name: "card" }

            ShowcaseTabs {
                preview: rsx! {
                    div { class: "grid grid-cols-1 lg:grid-cols-2 gap-8 items-start",
                        // Live Card Preview
                        div { class: "w-full flex justify-center",
                            Card {
                                class: "{max_h_class} w-full max-w-sm",
                                avatar: if with_avatar() {
                                    Some(rsx! {
                                        Avatar {
                                            name: Some("Shadow Core".to_string()),
                                            size: AvatarSize::Md,
                                            status: PresenceStatus::Online,
                                        }
                                    })
                                } else {
                                    None
                                },
                                title: if with_title() { Some("Cluster Mesh #07".to_string()) } else { None },
                                subtitle: if with_subtitle() { Some("Kyber-1024 Post-Quantum • 4 nodi attivi".to_string()) } else { None },
                                variant: selected_variant(),
                                shadow: selected_shadow(),
                                hover_glow: enable_glow(),
                                progress: if with_progress() { Some(68.0) } else { None },
                                buttons: if with_buttons() {
                                    Some(rsx! {
                                        Button { size: ButtonSize::Sm, variant: ButtonVariant::Primary, "Sincronizza" }
                                        Button { size: ButtonSize::Sm, variant: ButtonVariant::Ghost, "Ispeziona" }
                                    })
                                } else {
                                    None
                                },
                                image: if with_image() {
                                    Some("https://images.unsplash.com/photo-1618005182384-a83a8bd57fbe?auto=format&fit=crop&w=600&q=80".to_string())
                                } else {
                                    None
                                },
                                div { class: "space-y-2",
                                    p { class: "text-xs text-zinc-300", "Contenuto informativo della card con tipografia ad alto contrasto e bordi ottici calibrati." }
                                    Badge { label: "Zero-Knowledge Validated".to_string(), variant: BadgeVariant::Success, dot: true }
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
                                            key: "{v:?}",
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
                                            key: "{s:?}",
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

                            div { class: "pt-2 border-t border-white/[0.06] space-y-2.5",
                                span { class: "text-zinc-400 block uppercase tracking-wider text-[10px]", "Sezioni Attive:" }
                                div { class: "grid grid-cols-2 gap-2 text-zinc-300",
                                    Checkbox {
                                        checked: with_avatar,
                                        label: Some("Avatar".to_string()),
                                        size: CheckboxSize::Sm,
                                    }
                                    Checkbox {
                                        checked: with_title,
                                        label: Some("Titolo".to_string()),
                                        size: CheckboxSize::Sm,
                                    }
                                    Checkbox {
                                        checked: with_subtitle,
                                        label: Some("Sottotitolo".to_string()),
                                        size: CheckboxSize::Sm,
                                    }
                                    Checkbox {
                                        checked: with_progress,
                                        label: Some("Progress Bar".to_string()),
                                        size: CheckboxSize::Sm,
                                    }
                                    Checkbox {
                                        checked: with_buttons,
                                        label: Some("Bottoni in Basso".to_string()),
                                        size: CheckboxSize::Sm,
                                    }
                                    Checkbox {
                                        checked: with_image,
                                        label: Some("Immagine in Basso".to_string()),
                                        size: CheckboxSize::Sm,
                                    }
                                }
                            }

                            div { class: "pt-2 border-t border-white/[0.06] space-y-2",
                                Checkbox {
                                    checked: enable_glow,
                                    label: Some("Attiva Micro-Glow su Hover".to_string()),
                                    size: CheckboxSize::Sm,
                                }
                                Checkbox {
                                    checked: with_max_height,
                                    label: Some("Test Altezza Massima (Taglio Immagine)".to_string()),
                                    size: CheckboxSize::Sm,
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
                    onclick: move |_| {},
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
