use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::components::{FeatureBanner, PropRow, ShowcaseTabs};

#[component]
pub fn SpeedDialPage() -> Element {
    let mut selected_direction = use_signal(|| SpeedDialDirection::Up);
    let mut last_action = use_signal(|| "Nessuna azione selezionata".to_string());

    let props = vec![
        PropRow {
            name: "icon",
            r#type: "IconKind",
            default: "IconKind::Plus",
            description: "Icona principale visualizzata sul trigger circolare FAB",
        },
        PropRow {
            name: "active_icon",
            r#type: "Option<IconKind>",
            default: "None",
            description: "Icona opzionale quando il menu è aperto (in alternativa ruota l'icona primaria)",
        },
        PropRow {
            name: "direction",
            r#type: "SpeedDialDirection",
            default: "SpeedDialDirection::Up",
            description: "Direzione di estensione dei pulsanti secondari (Up, Down, Left, Right)",
        },
        PropRow {
            name: "fixed",
            r#type: "bool",
            default: "false",
            description: "Fissa il pulsante nell'angolo inferiore destro dello schermo",
        },
        PropRow {
            name: "SpeedDialAction",
            r#type: "Component",
            default: "-",
            description: "Azione secondaria con icona, etichetta fluttuante e callback onclick",
        },
        PropRow {
            name: "Fab",
            r#type: "Component",
            default: "-",
            description: "Pulsante d'azione fluttuante circolare autonomo",
        },
    ];

    let code_snippet = r#"SpeedDial {
    direction: SpeedDialDirection::Up,
    icon: IconKind::Plus,
    SpeedDialAction {
        icon: IconKind::Send,
        label: Some("Invia messaggio".to_string()),
        onclick: move |_| println!("Invia"),
    }
    SpeedDialAction {
        icon: IconKind::Paperclip,
        label: Some("Allega file".to_string()),
        onclick: move |_| println!("Allega"),
    }
    SpeedDialAction {
        icon: IconKind::Share2,
        label: Some("Condividi link".to_string()),
        onclick: move |_| println!("Condividi"),
    }
}"#
    .to_string();

    rsx! {
        div { class: "max-w-4xl space-y-8",
            div {
                Heading { order: HeadingOrder::H2, "SpeedDial & FloatingActionButton (FAB)" }
                Text { dimmed: true, size: TextSize::Sm, "Pulsante circolare di azione primario con rotazione ottica e animazione di espansione radiale/lineare per azioni secondarie correlate." }
            }

            FeatureBanner { feature_name: "speed_dial" }

            ShowcaseTabs {
                preview: rsx! {
                    div { class: "p-12 rounded-xl bg-[#060608] border border-white/[0.08] flex flex-col items-center justify-center gap-8 shadow-inner min-h-[360px]",
                        div { class: "flex items-center gap-2 flex-wrap justify-center",
                            span { class: "text-xs text-zinc-400 mr-2 font-mono", "Direzione:" }
                            button {
                                r#type: "button",
                                class: format!("px-2.5 py-1 text-xs font-mono rounded-lg border transition-all cursor-pointer {}", if *selected_direction.read() == SpeedDialDirection::Up { "bg-purple-950/80 text-purple-200 border-purple-500/50" } else { "bg-white/[0.04] text-zinc-400 border-white/[0.06]" }),
                                onclick: move |_| selected_direction.set(SpeedDialDirection::Up),
                                "Up"
                            }
                            button {
                                r#type: "button",
                                class: format!("px-2.5 py-1 text-xs font-mono rounded-lg border transition-all cursor-pointer {}", if *selected_direction.read() == SpeedDialDirection::Down { "bg-purple-950/80 text-purple-200 border-purple-500/50" } else { "bg-white/[0.04] text-zinc-400 border-white/[0.06]" }),
                                onclick: move |_| selected_direction.set(SpeedDialDirection::Down),
                                "Down"
                            }
                            button {
                                r#type: "button",
                                class: format!("px-2.5 py-1 text-xs font-mono rounded-lg border transition-all cursor-pointer {}", if *selected_direction.read() == SpeedDialDirection::Left { "bg-purple-950/80 text-purple-200 border-purple-500/50" } else { "bg-white/[0.04] text-zinc-400 border-white/[0.06]" }),
                                onclick: move |_| selected_direction.set(SpeedDialDirection::Left),
                                "Left"
                            }
                            button {
                                r#type: "button",
                                class: format!("px-2.5 py-1 text-xs font-mono rounded-lg border transition-all cursor-pointer {}", if *selected_direction.read() == SpeedDialDirection::Right { "bg-purple-950/80 text-purple-200 border-purple-500/50" } else { "bg-white/[0.04] text-zinc-400 border-white/[0.06]" }),
                                onclick: move |_| selected_direction.set(SpeedDialDirection::Right),
                                "Right"
                            }
                        }

                        div { class: "relative p-8 rounded-xl bg-white/[0.02] border border-white/[0.06] flex items-center justify-center min-w-[280px] min-h-[220px]",
                            SpeedDial {
                                direction: *selected_direction.read(),
                                icon: IconKind::Plus,
                                SpeedDialAction {
                                    icon: IconKind::Send,
                                    label: Some("Nuovo messaggio".to_string()),
                                    onclick: move |_| last_action.set("Azione: Nuovo messaggio".to_string()),
                                }
                                SpeedDialAction {
                                    icon: IconKind::Paperclip,
                                    label: Some("Allega documento".to_string()),
                                    onclick: move |_| last_action.set("Azione: Allega documento".to_string()),
                                }
                                SpeedDialAction {
                                    icon: IconKind::Share2,
                                    label: Some("Condividi risorsa".to_string()),
                                    onclick: move |_| last_action.set("Azione: Condividi risorsa".to_string()),
                                }
                            }
                        }

                        div { class: "p-3 rounded-lg bg-white/[0.02] border border-white/[0.06] flex items-center gap-2 text-xs font-mono text-zinc-400",
                            Icon { kind: IconKind::Sparkles, size: 14 }
                            span { "Ultima azione eseguita: " }
                            span { class: "text-purple-300 font-semibold", "{last_action}" }
                        }
                    }
                },
                code: code_snippet,
                props: props,
            }
        }
    }
}
