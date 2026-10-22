use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::components::{FeatureBanner, PropRow, ShowcaseTabs};

#[component]
pub fn IndicatorPage() -> Element {
    let mut selected_pos = use_signal(|| IndicatorPosition::TopEnd);
    let mut selected_color = use_signal(|| IndicatorColor::Violet);
    let mut processing = use_signal(|| true);

    let props = vec![
        PropRow {
            name: "position",
            r#type: "IndicatorPosition",
            default: "IndicatorPosition::TopEnd",
            description: "Angolo di ancoraggio (TopEnd, TopStart, BottomEnd, BottomStart)",
        },
        PropRow {
            name: "color",
            r#type: "IndicatorColor",
            default: "IndicatorColor::Violet",
            description: "Colore e alone luminoso (Violet, Emerald, Rose, Amber, Cyan)",
        },
        PropRow {
            name: "size",
            r#type: "IndicatorSize",
            default: "IndicatorSize::Md",
            description: "Dimensione del dot o badge (Sm, Md, Lg)",
        },
        PropRow {
            name: "processing",
            r#type: "bool",
            default: "false",
            description: "Attiva l'animazione di onda radar a impulsi indicando elaborazione attiva",
        },
        PropRow {
            name: "label",
            r#type: "Option<String>",
            default: "None",
            description: "Conteggio o etichetta testuale nel badge (es. \"3\" o \"99+\")",
        },
    ];

    let code_snippet = r#"Indicator {
    position: IndicatorPosition::TopEnd,
    color: IndicatorColor::Violet,
    processing: true,
    Avatar {
        name: "Shadow Fenix".to_string(),
        size: AvatarSize::Md,
    }
}"#
    .to_string();

    rsx! {
        div { class: "max-w-4xl space-y-8",
            div {
                Heading { order: HeadingOrder::H2, "Indicator" }
                Text { dimmed: true, size: TextSize::Sm, "Punto di stato o badge contatore ancorato agli angoli di qualsiasi elemento (avatar, pulsanti, icone) con supporto per onda radar pulsante." }
            }

            FeatureBanner { feature_name: "indicator" }

            ShowcaseTabs {
                preview: rsx! {
                    div { class: "p-12 py-16 rounded-xl bg-[#060608] border border-white/[0.08] flex flex-col items-center justify-center gap-8 shadow-inner min-h-[340px]",
                        div { class: "flex items-center gap-4 flex-wrap justify-center",
                            div { class: "flex items-center gap-2",
                                span { class: "text-xs text-zinc-400 font-mono", "Posizione:" }
                                button {
                                    r#type: "button",
                                    class: format!("px-2.5 py-1 text-xs font-mono rounded-lg border transition-all cursor-pointer {}", if *selected_pos.read() == IndicatorPosition::TopEnd { "bg-purple-950/80 text-purple-200 border-purple-500/50" } else { "bg-white/[0.04] text-zinc-400 border-white/[0.06]" }),
                                    onclick: move |_| selected_pos.set(IndicatorPosition::TopEnd),
                                    "TopEnd"
                                }
                                button {
                                    r#type: "button",
                                    class: format!("px-2.5 py-1 text-xs font-mono rounded-lg border transition-all cursor-pointer {}", if *selected_pos.read() == IndicatorPosition::TopStart { "bg-purple-950/80 text-purple-200 border-purple-500/50" } else { "bg-white/[0.04] text-zinc-400 border-white/[0.06]" }),
                                    onclick: move |_| selected_pos.set(IndicatorPosition::TopStart),
                                    "TopStart"
                                }
                                button {
                                    r#type: "button",
                                    class: format!("px-2.5 py-1 text-xs font-mono rounded-lg border transition-all cursor-pointer {}", if *selected_pos.read() == IndicatorPosition::BottomEnd { "bg-purple-950/80 text-purple-200 border-purple-500/50" } else { "bg-white/[0.04] text-zinc-400 border-white/[0.06]" }),
                                    onclick: move |_| selected_pos.set(IndicatorPosition::BottomEnd),
                                    "BottomEnd"
                                }
                            }

                            div { class: "flex items-center gap-2",
                                span { class: "text-xs text-zinc-400 font-mono", "Colore:" }
                                button {
                                    r#type: "button",
                                    class: format!("px-2 py-1 text-xs font-mono rounded-lg border transition-all cursor-pointer {}", if *selected_color.read() == IndicatorColor::Violet { "bg-purple-950/80 text-purple-200 border-purple-500/50" } else { "bg-white/[0.04] text-zinc-400 border-white/[0.06]" }),
                                    onclick: move |_| selected_color.set(IndicatorColor::Violet),
                                    "Violet"
                                }
                                button {
                                    r#type: "button",
                                    class: format!("px-2 py-1 text-xs font-mono rounded-lg border transition-all cursor-pointer {}", if *selected_color.read() == IndicatorColor::Cyan { "bg-cyan-950/80 text-cyan-200 border-cyan-500/50" } else { "bg-white/[0.04] text-zinc-400 border-white/[0.06]" }),
                                    onclick: move |_| selected_color.set(IndicatorColor::Cyan),
                                    "Cyan"
                                }
                                button {
                                    r#type: "button",
                                    class: format!("px-2 py-1 text-xs font-mono rounded-lg border transition-all cursor-pointer {}", if *selected_color.read() == IndicatorColor::Amber { "bg-amber-950/80 text-amber-200 border-amber-500/50" } else { "bg-white/[0.04] text-zinc-400 border-white/[0.06]" }),
                                    onclick: move |_| selected_color.set(IndicatorColor::Amber),
                                    "Amber"
                                }
                            }

                            div { class: "flex items-center gap-2",
                                span { class: "text-xs text-zinc-400 font-mono", "Onda Radar:" }
                                button {
                                    r#type: "button",
                                    class: format!("px-2.5 py-1 text-xs font-mono rounded-lg border transition-all cursor-pointer {}", if *processing.read() { "bg-purple-950/80 text-purple-200 border-purple-500/50" } else { "bg-white/[0.04] text-zinc-400 border-white/[0.06]" }),
                                    onclick: move |_| processing.toggle(),
                                    if *processing.read() { "Attiva" } else { "Spenta" }
                                }
                            }
                        }

                        div { class: "flex items-center gap-8 flex-wrap justify-center p-6 rounded-2xl bg-white/[0.02] border border-white/[0.06]",
                            Indicator {
                                position: *selected_pos.read(),
                                color: IndicatorColor::Emerald,
                                processing: *processing.read(),
                                Avatar {
                                    name: "Christian Torbidone".to_string(),
                                    size: AvatarSize::Lg,
                                }
                            }

                            Indicator {
                                position: *selected_pos.read(),
                                color: IndicatorColor::Rose,
                                label: Some("3".to_string()),
                                Button {
                                    variant: ButtonVariant::Secondary,
                                    "Messaggi"
                                }
                            }

                            Indicator {
                                position: *selected_pos.read(),
                                color: *selected_color.read(),
                                processing: *processing.read(),
                                Button {
                                    variant: ButtonVariant::Primary,
                                    left_icon: rsx! { Icon { kind: IconKind::Info, size: 16 } },
                                    "Notifiche"
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
