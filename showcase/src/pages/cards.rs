use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::components::{FeatureBanner, PropRow, ShowcaseTabs};

#[component]
pub fn CardsPage() -> Element {
    let mut selected_variant = use_signal(|| CardVariant::Default);
    let mut selected_shadow = use_signal(|| CardShadow::Md);
    let mut enable_glow = use_signal(|| false);
    let mut with_avatar = use_signal(|| true);
    let mut with_title = use_signal(|| true);
    let mut with_subtitle = use_signal(|| true);
    let mut with_progress = use_signal(|| false);
    let mut with_buttons = use_signal(|| true);
    let mut with_image = use_signal(|| false);
    let mut with_max_height = use_signal(|| false);

    let props = vec![
        PropRow {
            name: "title",
            r#type: "Option<String>",
            default: "None",
            description: "Titolo primario dell'intestazione (centrato se senza avatar, affiancato se con avatar)",
        },
        PropRow {
            name: "subtitle",
            r#type: "Option<String>",
            default: "None",
            description: "Sottotitolo più piccolo e scuro coordinato con il titolo",
        },
        PropRow {
            name: "avatar",
            r#type: "Option<Element>",
            default: "None",
            description: "Elemento avatar o icona (centrato se senza titolo, al fianco se con titolo)",
        },
        PropRow {
            name: "buttons",
            r#type: "Option<Element>",
            default: "None",
            description: "Sezione bottoni ed azioni dedicata in basso alla card",
        },
        PropRow {
            name: "progress",
            r#type: "Option<f32>",
            default: "None",
            description: "Avanzamento percentuale (0.0 .. 100.0) con barra ProgressBar integrata",
        },
        PropRow {
            name: "image",
            r#type: "Option<String>",
            default: "None",
            description: "Immagine a tutta larghezza in basso con adattamento e ritaglio bordi",
        },
        PropRow {
            name: "image_height",
            r#type: "Option<String>",
            default: "None",
            description: "Altezza personalizzata opzionale per l'immagine inferiore (es. '160px')",
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
            description: "Micro-glow coordinato con il colore del tema attivo al passaggio del mouse",
        },
        PropRow {
            name: "footer",
            r#type: "Option<Element>",
            default: "None",
            description: "Slot per la sezione a piè di pagina con sfondo e divisore dedicato",
        },
    ];

    let code_snippet = r#"Card {
    avatar: rsx! {
        Avatar { name: Some("Shadow Core".to_string()), size: AvatarSize::Md, status: PresenceStatus::Online }
    },
    title: Some("Mesh Node Cluster".to_string()),
    subtitle: Some("Topologia di rete crittografica P2P".to_string()),
    progress: Some(82.0),
    buttons: rsx! {
        Button { size: ButtonSize::Sm, variant: ButtonVariant::Primary, "Connetti" }
        Button { size: ButtonSize::Sm, variant: ButtonVariant::Outline, "Dettagli" }
    },
    image: Some("https://images.unsplash.com/photo-1618005182384-a83a8bd57fbe?auto=format&fit=crop&w=600&q=80".to_string()),
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

                            div { class: "pt-2 border-t border-white/[0.06] space-y-2.5",
                                span { class: "text-zinc-400 block uppercase tracking-wider text-[10px]", "Sezioni Attive:" }
                                div { class: "grid grid-cols-2 gap-2 text-zinc-300",
                                    label { class: "flex items-center gap-1.5 cursor-pointer",
                                        input {
                                            r#type: "checkbox",
                                            checked: "{with_avatar}",
                                            onchange: move |e| with_avatar.set(e.checked()),
                                        }
                                        "Avatar"
                                    }
                                    label { class: "flex items-center gap-1.5 cursor-pointer",
                                        input {
                                            r#type: "checkbox",
                                            checked: "{with_title}",
                                            onchange: move |e| with_title.set(e.checked()),
                                        }
                                        "Titolo"
                                    }
                                    label { class: "flex items-center gap-1.5 cursor-pointer",
                                        input {
                                            r#type: "checkbox",
                                            checked: "{with_subtitle}",
                                            onchange: move |e| with_subtitle.set(e.checked()),
                                        }
                                        "Sottotitolo"
                                    }
                                    label { class: "flex items-center gap-1.5 cursor-pointer",
                                        input {
                                            r#type: "checkbox",
                                            checked: "{with_progress}",
                                            onchange: move |e| with_progress.set(e.checked()),
                                        }
                                        "Progress Bar"
                                    }
                                    label { class: "flex items-center gap-1.5 cursor-pointer",
                                        input {
                                            r#type: "checkbox",
                                            checked: "{with_buttons}",
                                            onchange: move |e| with_buttons.set(e.checked()),
                                        }
                                        "Bottoni in Basso"
                                    }
                                    label { class: "flex items-center gap-1.5 cursor-pointer",
                                        input {
                                            r#type: "checkbox",
                                            checked: "{with_image}",
                                            onchange: move |e| with_image.set(e.checked()),
                                        }
                                        "Immagine in Basso"
                                    }
                                }
                            }

                            div { class: "pt-2 border-t border-white/[0.06] space-y-2",
                                label { class: "flex items-center gap-1.5 cursor-pointer text-zinc-300",
                                    input {
                                        r#type: "checkbox",
                                        checked: "{enable_glow}",
                                        onchange: move |e| enable_glow.set(e.checked()),
                                    }
                                    "Attiva Micro-Glow su Hover"
                                }
                                label { class: "flex items-center gap-1.5 cursor-pointer text-amber-300",
                                    input {
                                        r#type: "checkbox",
                                        checked: "{with_max_height}",
                                        onchange: move |e| with_max_height.set(e.checked()),
                                    }
                                    "Test Altezza Massima (Taglio Immagine)"
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
