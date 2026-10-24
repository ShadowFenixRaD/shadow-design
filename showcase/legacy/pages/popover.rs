use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::components::{ChoicePicker, Playground, ToggleControl, FeatureBanner, PageHeader, PropRow, ShowcaseTabs};

#[component]
pub fn PopoverPage() -> Element {
    let opened = use_signal(|| false);
    let selected_pos = use_signal(|| PopoverPosition::Bottom);
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
            strong { class: "sc-section-title", "Configurazione Rapida" }
            p { class: "sc-page-desc", "Pannello fluttuante ancorato con freccia e dismiss su click esterno." }
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
        div { class: "sc-stack sc-stack--loose",
            PageHeader {
                title: "Popover".to_string(),
                description: "Pannello fluttuante contestuale ancorato a elementi trigger con 12 allineamenti, freccia ottica, z-index elevato e dismiss WAI-ARIA.".to_string(),
            }

            FeatureBanner { feature_name: "popover" }

            ShowcaseTabs {
                preview: rsx! {
                    Playground {
                        stage: rsx! {
                            div { class: "py-24",
                                Popover {
                                    opened: opened,
                                    position: selected_pos(),
                                    with_arrow: with_arrow(),
                                    PopoverTrigger {
                                        Button {
                                            variant: ButtonVariant::Primary,
                                            left_icon: rsx! { Icon { kind: IconKind::Layers, size: 15 } },
                                            "Trigger Popover"
                                        }
                                    }
                                    PopoverDropdown {
                                        div { class: "sc-stack",
                                            div { class: "sc-title-row",
                                                strong { class: "sc-section-title", "Opzioni Account" }
                                                Badge { label: "Pro".to_string(), variant: BadgeVariant::Primary, pill: true }
                                            }
                                            p { class: "sc-page-desc",
                                                "Questo pannello fluttuante supporta contenuti ricchi, form interattivi ed è posizionato dinamicamente."
                                            }
                                            div { class: "sc-actions",
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
                        controls: rsx! {
                            ChoicePicker {
                                label: "Posizione".to_string(),
                                options: vec![
                                    (PopoverPosition::Top, "Top".to_string()),
                                    (PopoverPosition::Bottom, "Bottom".to_string()),
                                    (PopoverPosition::Left, "Left".to_string()),
                                    (PopoverPosition::Right, "Right".to_string()),
                                ],
                                value: selected_pos,
                            }
                            ToggleControl { label: "Freccia".to_string(), checked: with_arrow }
                        },
                    }
                },
                code: code_snippet,
                props: props,
            }
        }
    }
}
