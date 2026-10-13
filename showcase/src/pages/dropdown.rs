use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::components::{FeatureBanner, PropRow, ShowcaseTabs};

#[component]
pub fn DropdownPage() -> Element {
    let mut is_open = use_signal(|| false);
    let mut is_top_open = use_signal(|| false);
    let mut selected_action = use_signal(|| "Nessuna azione eseguita".to_string());

    let props = vec![
        PropRow {
            name: "is_open",
            r#type: "Signal<bool>",
            default: "-",
            description: "Signal reattivo che governa la visibilità del menu a tendina",
        },
        PropRow {
            name: "trigger",
            r#type: "Element",
            default: "-",
            description: "Elemento cliccabile che ancora e attiva il menu",
        },
        PropRow {
            name: "placement",
            r#type: "DropdownPlacement",
            default: "DropdownPlacement::BottomRight",
            description: "Direzione di ancoraggio: BottomRight, BottomLeft, TopRight, TopLeft (ideale vicino ai bordi della card)",
        },
        PropRow {
            name: "glow",
            r#type: "Option<bool>",
            default: "None",
            description: "Alone ottico perimetrale coordinato con il tema attivo",
        },
        PropRow {
            name: "DropdownItem",
            r#type: "Component",
            default: "-",
            description: "Singola voce di menu con supporto a icona, colore danger ed handler onclick",
        },
        PropRow {
            name: "DropdownDivider",
            r#type: "Component",
            default: "-",
            description: "Separatore orizzontale sottile tra gruppi di voci di menu",
        },
    ];

    let code_snippet = r#"let mut is_open = use_signal(|| false);
let mut is_top_open = use_signal(|| false);

// Apertura standard verso il basso
Dropdown {
    is_open: is_open,
    placement: DropdownPlacement::BottomRight,
    trigger: rsx! {
        Button {
            variant: ButtonVariant::Primary,
            right_icon: rsx! { Icon { kind: IconKind::ChevronDown, size: 14 } },
            "Menu Azioni"
        }
    },
    DropdownItem {
        icon: Some(IconKind::Terminal),
        onclick: move |_| { is_open.set(false); },
        "Console Terminale"
    }
}

// Apertura verso l'alto (quando vicino al fondo della card)
Dropdown {
    is_open: is_top_open,
    placement: DropdownPlacement::TopRight,
    trigger: rsx! {
        Button {
            variant: ButtonVariant::Secondary,
            right_icon: rsx! { Icon { kind: IconKind::ChevronUp, size: 14 } },
            "Menu Superiore"
        }
    },
    DropdownItem {
        onclick: move |_| { is_top_open.set(false); },
        "Azione..."
    }
}"#
    .to_string();

    rsx! {
        div { class: "max-w-4xl space-y-8",
            div {
                Heading { order: HeadingOrder::H2, "Dropdown / Popover Component" }
                Text { dimmed: true, size: TextSize::Sm, "Menu contestuali ancorati al trigger con z-index prioritario (z-[1000]), apertura bidirezionale (Top / Bottom) per non uscire dalla vista a fondo card, e chiusura automatica click-outside." }
            }

            FeatureBanner { feature_name: "dropdown" }

            ShowcaseTabs {
                preview: rsx! {
                    div { class: "space-y-6",
                        div { class: "p-8 rounded-xl bg-[#060608] border border-white/[0.08] shadow-inner flex flex-wrap items-center justify-around gap-8 min-h-[300px] overflow-visible relative",
                            // 1. Dropdown verso il basso (Bottom)
                            div { class: "flex flex-col items-center gap-2",
                                span { class: "text-xs text-zinc-400 font-medium", "Apertura Verso il Basso:" }
                                Dropdown {
                                    is_open: is_open,
                                    placement: DropdownPlacement::BottomRight,
                                    trigger: rsx! {
                                        Button {
                                            variant: ButtonVariant::Primary,
                                            right_icon: rsx! { Icon { kind: IconKind::ChevronDown, size: 14 } },
                                            "Azioni Rapide (Down)"
                                        }
                                    },
                                    DropdownItem {
                                        icon: Some(IconKind::Terminal),
                                        onclick: move |_| {
                                            selected_action.set("Aperta console di comando".to_string());
                                            is_open.set(false);
                                        },
                                        "Console Terminale"
                                    }
                                    DropdownItem {
                                        icon: Some(IconKind::Sparkles),
                                        onclick: move |_| {
                                            selected_action.set("Avviata ottimizzazione memoria".to_string());
                                            is_open.set(false);
                                        },
                                        "Ottimizza Runtime"
                                    }
                                    DropdownDivider {}
                                    DropdownItem {
                                        icon: Some(IconKind::Settings),
                                        onclick: move |_| {
                                            selected_action.set("Aperte impostazioni account".to_string());
                                            is_open.set(false);
                                        },
                                        "Impostazioni"
                                    }
                                    DropdownItem {
                                        icon: Some(IconKind::XCircle),
                                        danger: true,
                                        onclick: move |_| {
                                            selected_action.set("Richiesta disconnessione sessione".to_string());
                                            is_open.set(false);
                                        },
                                        "Disconnetti Account"
                                    }
                                }
                            }

                            // 2. Dropdown verso l'alto (Top - anti bottom clipping)
                            div { class: "flex flex-col items-center gap-2",
                                span { class: "text-xs text-zinc-400 font-medium", "Apertura Verso l'Alto (Fondo Card):" }
                                Dropdown {
                                    is_open: is_top_open,
                                    placement: DropdownPlacement::TopRight,
                                    trigger: rsx! {
                                        Button {
                                            variant: ButtonVariant::Secondary,
                                            right_icon: rsx! { Icon { kind: IconKind::ChevronUp, size: 14 } },
                                            "Opzioni Fondo (Up)"
                                        }
                                    },
                                    DropdownItem {
                                        icon: Some(IconKind::Layers),
                                        onclick: move |_| {
                                            selected_action.set("Attivato layer stacking avanzato".to_string());
                                            is_top_open.set(false);
                                        },
                                        "Attiva Layer Stacking"
                                    }
                                    DropdownItem {
                                        icon: Some(IconKind::Eye),
                                        onclick: move |_| {
                                            selected_action.set("Ispezionato overflow card".to_string());
                                            is_top_open.set(false);
                                        },
                                        "Ispeziona Bordi Card"
                                    }
                                    DropdownDivider {}
                                    DropdownItem {
                                        icon: Some(IconKind::CheckCircle),
                                        onclick: move |_| {
                                            selected_action.set("Verificato rendering privo di clipping".to_string());
                                            is_top_open.set(false);
                                        },
                                        "Verifica Z-Index 1000"
                                    }
                                }
                            }
                        }

                        div { class: "p-3 rounded-lg bg-[#0e0e14] border border-white/[0.06] flex items-center justify-between text-xs font-mono",
                            span { class: "text-zinc-500", "Ultima azione eseguita:" }
                            strong { class: "text-purple-300", "{selected_action}" }
                        }
                    }
                },
                code: code_snippet,
                props: props,
            }
        }
    }
}
