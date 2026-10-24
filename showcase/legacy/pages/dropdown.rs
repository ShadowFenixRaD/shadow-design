use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::components::{Counter, FeatureBanner, PageHeader, PropRow, ShowcaseTabs, Stage};

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
        div { class: "sc-stack sc-stack--loose",
            PageHeader {
                title: "Dropdown / Popover Component".to_string(),
                description: "Menu contestuali ancorati al trigger con z-index prioritario (z-[1000]), apertura bidirezionale (Top / Bottom) per non uscire dalla vista a fondo card, e chiusura automatica click-outside.".to_string(),
            }

            FeatureBanner { feature_name: "dropdown" }

            ShowcaseTabs {
                preview: rsx! {
                    div { class: "sc-stack",
                        Stage { wide: true,
                            // 1. Dropdown verso il basso (Bottom)
                            div { class: "sc-stack items-center",
                                span { class: "sc-control-label", "Apertura Verso il Basso:" }
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
                            div { class: "sc-stack items-center",
                                span { class: "sc-control-label", "Apertura Verso l'Alto (Fondo Card):" }
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

                        Counter { label: "Ultima azione eseguita".to_string(), value: selected_action() }
                    }
                },
                code: code_snippet,
                props: props,
            }
        }
    }
}
