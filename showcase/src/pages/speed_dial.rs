use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::components::{ChoicePicker, Counter, Playground, FeatureBanner, PageHeader, PropRow, ShowcaseTabs};

#[component]
pub fn SpeedDialPage() -> Element {
    let selected_direction = use_signal(|| SpeedDialDirection::Up);
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
        div { class: "sc-stack sc-stack--loose",
            PageHeader {
                title: "SpeedDial & FloatingActionButton (FAB)".to_string(),
                description: "Pulsante circolare di azione primario con rotazione ottica e animazione di espansione radiale/lineare per azioni secondarie correlate.".to_string(),
            }

            FeatureBanner { feature_name: "speed_dial" }

            ShowcaseTabs {
                preview: rsx! {
                    Playground {
                        stage: rsx! {
                            div { class: "relative flex items-center justify-center min-w-[280px] min-h-[240px]",
                                SpeedDial {
                                    direction: selected_direction(),
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
                        },
                        controls: rsx! {
                            ChoicePicker {
                                label: "Direzione".to_string(),
                                options: vec![
                                    (SpeedDialDirection::Up, "Up".to_string()),
                                    (SpeedDialDirection::Down, "Down".to_string()),
                                    (SpeedDialDirection::Left, "Left".to_string()),
                                    (SpeedDialDirection::Right, "Right".to_string()),
                                ],
                                value: selected_direction,
                            }
                            Counter { label: "Ultima azione eseguita".to_string(), value: last_action() }
                        },
                    }
                },
                code: code_snippet,
                props: props,
            }

            // SpeedDial fluttuante fisso di prova posizionato in basso a destra nello schermo
            SpeedDial {
                fixed: true,
                direction: SpeedDialDirection::Up,
                icon: IconKind::Plus,
                SpeedDialAction {
                    icon: IconKind::Send,
                    label: Some("Invia messaggio".to_string()),
                    onclick: move |_| last_action.set("FAB Fluttuante: Invia messaggio".to_string()),
                }
                SpeedDialAction {
                    icon: IconKind::Paperclip,
                    label: Some("Allega documento".to_string()),
                    onclick: move |_| last_action.set("FAB Fluttuante: Allega documento".to_string()),
                }
                SpeedDialAction {
                    icon: IconKind::Share2,
                    label: Some("Condividi risorsa".to_string()),
                    onclick: move |_| last_action.set("FAB Fluttuante: Condividi risorsa".to_string()),
                }
            }
        }
    }
}
