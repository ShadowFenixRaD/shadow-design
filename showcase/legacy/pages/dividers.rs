use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::components::{
    ChoicePicker, ControlGroup, FeatureBanner, PageHeader, Playground, PropRow, ShowcaseTabs, ToggleControl,
};

#[component]
pub fn DividersPage() -> Element {
    let orientation = use_signal(|| Orientation::Horizontal);
    let with_label = use_signal(|| true);
    let label_text = use_signal(|| "Oggi • Messaggi Crittografati".to_string());

    let props = vec![
        PropRow {
            name: "orientation",
            r#type: "Orientation",
            default: "Orientation::Horizontal",
            description: "Orientamento del separatore: Horizontal o Vertical",
        },
        PropRow {
            name: "label",
            r#type: "Option<String>",
            default: "None",
            description: "Etichetta testuale opzionale posizionata al centro del divisore (es. 'Oggi', 'Ieri')",
        },
    ];

    let code_snippet = r#"Divider {
    orientation: Orientation::Horizontal,
    label: Some("Oggi • 12 Ottobre".to_string()),
}"#
    .to_string();

    rsx! {
        div { class: "sc-stack sc-stack--loose",
            PageHeader {
                title: "Divider & Separatore".to_string(),
                description: "Separatore semitrasparente hairline per suddividere sezioni, blocchi temporali o date all'interno di chat e form.".to_string(),
            }

            FeatureBanner { feature_name: "divider" }

            ShowcaseTabs {
                preview: rsx! {
                    Playground {
                        stage_start: true,
                        stage: rsx! {
                            if orientation() == Orientation::Horizontal {
                                div { class: "sc-stack w-full",
                                    span { class: "sc-subtle-text sc-mono", "Messaggio precedente (Ieri 23:40)" }
                                    Divider {
                                        orientation: Orientation::Horizontal,
                                        label: if with_label() { Some(label_text()) } else { None },
                                    }
                                    span { class: "sc-subtle-text sc-mono", "Messaggio recente (Oggi 14:28)" }
                                }
                            } else {
                                div { class: "flex items-center h-28 gap-4",
                                    span { class: "sc-subtle-text sc-mono", "Pannello Sinistro" }
                                    Divider {
                                        orientation: Orientation::Vertical,
                                        label: if with_label() { Some("OR".to_string()) } else { None },
                                    }
                                    span { class: "sc-subtle-text sc-mono", "Pannello Destro" }
                                }
                            }
                        },
                        controls: rsx! {
                            ChoicePicker {
                                label: "Orientamento".to_string(),
                                options: vec![
                                    (Orientation::Horizontal, "Orizzontale (Horizontal)".to_string()),
                                    (Orientation::Vertical, "Verticale (Vertical)".to_string()),
                                ],
                                value: orientation,
                            }
                            hr { class: "sc-control-divider" }
                            ToggleControl { label: "Mostra Etichetta Testuale Centrata".to_string(), checked: with_label }
                            if with_label() {
                                ControlGroup { label: "Testo Etichetta".to_string(),
                                    TextInput { value: label_text }
                                }
                            }
                        },
                    }
                },
                code: code_snippet,
                props: props,
            }
        }
    }
}
