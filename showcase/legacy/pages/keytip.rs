use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::components::{FeatureBanner, PageHeader, Playground, ToggleControl, PropRow, ShowcaseTabs};

#[component]
pub fn KeytipPage() -> Element {
    let visible = use_signal(|| true);

    let props = vec![
        PropRow {
            name: "key_sequence",
            r#type: "String",
            default: "-",
            description: "Sequenza o combinazione di tasti da mostrare nel badge (es. 'Alt+F', 'Ctrl+S')",
        },
        PropRow {
            name: "visible",
            r#type: "Option<bool>",
            default: "None",
            description: "Controllo esplicito della visibilità del keytip (None = sempre visibile se non disabled)",
        },
        PropRow {
            name: "disabled",
            r#type: "bool",
            default: "false",
            description: "Se true, disattiva e nasconde il badge keytip",
        },
        PropRow {
            name: "children",
            r#type: "Element",
            default: "-",
            description: "Elemento interattivo associato alla scorciatoia",
        },
    ];

    let code_snippet = r#"Keytip {
    key_sequence: "Alt+S".to_string(),
    visible: Some(true),
    Button {
        variant: ButtonVariant::Primary,
        "Salva Progetto"
    }
}"#
    .to_string();

    rsx! {
        div { class: "sc-stack sc-stack--loose",
            PageHeader {
                title: "Keytip".to_string(),
                description: "Micro-etichette di scorciatoia da tastiera in sovrimpressione su comandi e pulsanti (stile Office Ribbon / IntelliJ).".to_string(),
            }

            FeatureBanner { feature_name: "keytip" }

            ShowcaseTabs {
                preview: rsx! {
                    Playground {
                        stage: rsx! {
                            div { class: "sc-actions",
                                Keytip { key_sequence: "Alt+F".to_string(), visible: Some(visible()),
                                    Button { variant: ButtonVariant::Secondary, "File" }
                                }
                                Keytip { key_sequence: "Alt+E".to_string(), visible: Some(visible()),
                                    Button { variant: ButtonVariant::Secondary, "Modifica" }
                                }
                                Keytip { key_sequence: "Ctrl+S".to_string(), visible: Some(visible()),
                                    Button { variant: ButtonVariant::Primary, "Salva" }
                                }
                                Keytip { key_sequence: "Ctrl+P".to_string(), visible: Some(visible()),
                                    Button { variant: ButtonVariant::Outline, "Cerca" }
                                }
                            }
                        },
                        controls: rsx! {
                            ToggleControl { label: "Mostra Keytips (simula l'attivazione della tastiera)".to_string(), checked: visible }
                        },
                    }
                },
                code: code_snippet,
                props: props,
            }
        }
    }
}
