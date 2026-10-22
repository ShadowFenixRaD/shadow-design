use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::components::{FeatureBanner, PropRow, ShowcaseTabs};

#[component]
pub fn HoverCardPage() -> Element {
    let props = vec![
        PropRow {
            name: "open_delay",
            r#type: "u32",
            default: "150",
            description: "Ritardo in millisecondi prima dell'apertura del card al passaggio del mouse",
        },
        PropRow {
            name: "close_delay",
            r#type: "u32",
            default: "200",
            description: "Ritardo in millisecondi prima della chiusura quando il cursore esce dall'area",
        },
        PropRow {
            name: "width",
            r#type: "Option<u32>",
            default: "None",
            description: "Larghezza personalizzata in pixel del contenitore di anteprima",
        },
        PropRow {
            name: "position",
            r#type: "HoverCardPosition",
            default: "HoverCardPosition::Bottom",
            description: "Posizione di apertura (Bottom, Top, Left, Right)",
        },
    ];

    let code_snippet = r#"HoverCard {
    open_delay: 150,
    close_delay: 200,
    HoverCardTrigger {
        span { class: "text-purple-400 font-semibold cursor-pointer underline underline-offset-4",
            "@shadowfenix"
        }
    }
    HoverCardDropdown {
        div { class: "flex items-start gap-3.5",
            Avatar {
                name: "Shadow Fenix".to_string(),
                size: AvatarSize::Md,
            }
            div { class: "space-y-1.5 flex-1 min-w-0",
                div { class: "flex items-center justify-between",
                    h4 { class: "font-bold text-sm text-white", "Shadowfenix RaD" }
                    Badge { label: "Core Team".to_string(), variant: BadgeVariant::Primary, pill: true }
                }
                p { class: "text-xs text-zinc-400", "Ecosistema di componenti ad alte prestazioni per Dioxus 0.7." }
            }
        }
    }
}"#
    .to_string();

    rsx! {
        div { class: "max-w-4xl space-y-8",
            div {
                Heading { order: HeadingOrder::H2, "HoverCard" }
                Text { dimmed: true, size: TextSize::Sm, "Scheda di preview contestuale ricca attivata al passaggio del cursore (hover) con ritardi configurabili e persistenza su interazione." }
            }

            FeatureBanner { feature_name: "hover_card" }

            ShowcaseTabs {
                preview: rsx! {
                    div { class: "p-8 py-20 rounded-xl bg-[#060608] border border-white/[0.08] flex flex-col items-center justify-center gap-6 text-center shadow-inner min-h-[400px]",
                        p { class: "text-xs text-zinc-400 max-w-md",
                            "Passa il cursore sulla menzione sottostante per visualizzare l'anteprima del profilo. Il card rimane interattivo consentendo di cliccare pulsanti o link al suo interno."
                        }

                        div { class: "p-4 rounded-xl bg-white/[0.02] border border-white/[0.06] flex items-center gap-2 text-sm text-zinc-300",
                            span { "Sviluppato con passione da" }
                            HoverCard {
                                open_delay: 100,
                                close_delay: 200,
                                HoverCardTrigger {
                                    span { class: "text-purple-400 font-bold hover:text-purple-300 underline underline-offset-4 decoration-purple-500/50 cursor-pointer transition-colors",
                                        "@shadowfenix"
                                    }
                                }
                                HoverCardDropdown {
                                    div { class: "flex items-start gap-3.5 p-1 text-left",
                                        Avatar {
                                            name: "Shadow Fenix".to_string(),
                                            size: AvatarSize::Md,
                                        }
                                        div { class: "space-y-1.5 flex-1 min-w-0",
                                            div { class: "flex items-center justify-between gap-2",
                                                h4 { class: "font-bold text-sm text-white", "Shadowfenix RaD" }
                                                Badge { label: "Verified".to_string(), variant: BadgeVariant::Primary, pill: true }
                                            }
                                            p { class: "text-xs text-zinc-400 leading-relaxed",
                                                "Laboratorio di ricerca e sviluppo specializzato in architetture Rust & Dioxus web/desktop."
                                            }
                                            div { class: "pt-2 flex items-center gap-3 text-[11px] font-mono text-zinc-500",
                                                span { strong { class: "text-zinc-200", "50+" } " componenti" }
                                                span { "•" }
                                                span { strong { class: "text-zinc-200", "100%" } " pure SCSS" }
                                            }
                                        }
                                    }
                                }
                            }
                            span { "per la community Rust." }
                        }
                    }
                },
                code: code_snippet,
                props: props,
            }
        }
    }
}
