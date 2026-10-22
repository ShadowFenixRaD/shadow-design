use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::components::{FeatureBanner, PropRow, ShowcaseTabs};

#[component]
pub fn PopoverPage() -> Element {
    let opened = use_signal(|| false);
    let mut selected_pos = use_signal(|| PopoverPosition::Bottom);
    let with_arrow = use_signal(|| true);

    let props = vec![
        PropRow {
            name: "opened",
            r#type: "Signal<bool>",
            default: "-",
            description: "Signal reattivo che controlla l'apertura e chiusura del pannello fluttuante",
        },
        PropRow {
            name: "position",
            r#type: "PopoverPosition",
            default: "PopoverPosition::Bottom",
            description: "12 posizionamenti supportati (Top/Bottom/Left/Right con Start/Center/End)",
        },
        PropRow {
            name: "with_arrow",
            r#type: "bool",
            default: "true",
            description: "Mostra o nasconde la freccia indicatrice coordinata con il bordo specular",
        },
        PropRow {
            name: "close_on_escape",
            r#type: "bool",
            default: "true",
            description: "Chiude automaticamente il popover alla pressione del tasto Escape",
        },
        PropRow {
            name: "close_on_click_outside",
            r#type: "bool",
            default: "true",
            description: "Chiude il popover al click all'esterno del pannello",
        },
    ];

    let code_snippet = r#"let mut opened = use_signal(|| false);

Popover {
    opened: opened,
    position: PopoverPosition::Bottom,
    with_arrow: true,
    PopoverTrigger {
        Button {
            variant: ButtonVariant::Primary,
            "Apri Popover"
        }
    }
    PopoverDropdown {
        div { class: "space-y-3",
            h4 { class: "font-semibold text-sm text-white", "Configurazione Rapida" }
            p { class: "text-xs text-zinc-400", "Pannello fluttuante ancorato con freccia e dismiss su click esterno." }
            Button {
                variant: ButtonVariant::Secondary,
                size: ButtonSize::Sm,
                onclick: move |_| opened.set(false),
                "Salva e Chiudi"
            }
        }
    }
}"#
    .to_string();

    rsx! {
        div { class: "max-w-4xl space-y-8",
            div {
                Heading { order: HeadingOrder::H2, "Popover" }
                Text { dimmed: true, size: TextSize::Sm, "Pannello fluttuante contestuale ancorato a elementi trigger con 12 allineamenti, freccia ottica, z-index elevato e dismiss WAI-ARIA." }
            }

            FeatureBanner { feature_name: "popover" }

            ShowcaseTabs {
                preview: rsx! {
                    div { class: "p-12 rounded-xl bg-[#060608] border border-white/[0.08] flex flex-col items-center justify-center gap-6 shadow-inner",
                        div { class: "flex items-center gap-2 flex-wrap justify-center",
                            span { class: "text-xs text-zinc-400 mr-2 font-mono", "Posizione:" }
                            button {
                                r#type: "button",
                                class: format!("px-2.5 py-1 text-xs font-mono rounded-lg border transition-all cursor-pointer {}", if *selected_pos.read() == PopoverPosition::Top { "bg-purple-950/80 text-purple-200 border-purple-500/50" } else { "bg-white/[0.04] text-zinc-400 border-white/[0.06]" }),
                                onclick: move |_| selected_pos.set(PopoverPosition::Top),
                                "Top"
                            }
                            button {
                                r#type: "button",
                                class: format!("px-2.5 py-1 text-xs font-mono rounded-lg border transition-all cursor-pointer {}", if *selected_pos.read() == PopoverPosition::Bottom { "bg-purple-950/80 text-purple-200 border-purple-500/50" } else { "bg-white/[0.04] text-zinc-400 border-white/[0.06]" }),
                                onclick: move |_| selected_pos.set(PopoverPosition::Bottom),
                                "Bottom"
                            }
                            button {
                                r#type: "button",
                                class: format!("px-2.5 py-1 text-xs font-mono rounded-lg border transition-all cursor-pointer {}", if *selected_pos.read() == PopoverPosition::Left { "bg-purple-950/80 text-purple-200 border-purple-500/50" } else { "bg-white/[0.04] text-zinc-400 border-white/[0.06]" }),
                                onclick: move |_| selected_pos.set(PopoverPosition::Left),
                                "Left"
                            }
                            button {
                                r#type: "button",
                                class: format!("px-2.5 py-1 text-xs font-mono rounded-lg border transition-all cursor-pointer {}", if *selected_pos.read() == PopoverPosition::Right { "bg-purple-950/80 text-purple-200 border-purple-500/50" } else { "bg-white/[0.04] text-zinc-400 border-white/[0.06]" }),
                                onclick: move |_| selected_pos.set(PopoverPosition::Right),
                                "Right"
                            }
                        }

                        Popover {
                            opened: opened,
                            position: *selected_pos.read(),
                            with_arrow: *with_arrow.read(),
                            PopoverTrigger {
                                Button {
                                    variant: ButtonVariant::Primary,
                                    left_icon: rsx! { Icon { kind: IconKind::Layers, size: 15 } },
                                    "Trigger Popover"
                                }
                            }
                            PopoverDropdown {
                                div { class: "space-y-3 p-1",
                                    div { class: "flex items-center justify-between pb-2 border-b border-white/[0.08]",
                                        h4 { class: "font-semibold text-xs text-white uppercase tracking-wider font-mono", "Opzioni Account" }
                                        Badge { label: "Pro".to_string(), variant: BadgeVariant::Primary, pill: true }
                                    }
                                    p { class: "text-xs text-zinc-300 leading-relaxed",
                                        "Questo pannello fluttuante supporta contenuti ricchi, form interattivi ed è posizionato dinamicamente."
                                    }
                                    div { class: "flex justify-end gap-2 pt-2",
                                        Button {
                                            variant: ButtonVariant::Ghost,
                                            size: ButtonSize::Sm,
                                            onclick: {
                                                let mut op = opened;
                                                move |_| op.set(false)
                                            },
                                            "Chiudi"
                                        }
                                        Button {
                                            variant: ButtonVariant::Primary,
                                            size: ButtonSize::Sm,
                                            onclick: {
                                                let mut op = opened;
                                                move |_| op.set(false)
                                            },
                                            "Conferma"
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
