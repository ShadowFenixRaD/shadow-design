use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::components::{Counter, FeatureBanner, PageHeader, Playground, PropRow, ShowcaseTabs};

#[component]
pub fn SwitchPage() -> Element {
    let dev_mode = use_signal(|| true);
    let push_alerts = use_signal(|| false);
    let telemetry = use_signal(|| true);
    let locked_state = use_signal(|| false);

    let props = vec![
        PropRow {
            name: "checked",
            r#type: "Signal<bool>",
            default: "-",
            description:
                "Signal reattivo che controlla e riflette lo stato attivo/inattivo dello switch",
        },
        PropRow {
            name: "label",
            r#type: "Option<String>",
            default: "None",
            description: "Etichetta testuale posizionata a fianco del cursore",
        },
        PropRow {
            name: "description",
            r#type: "Option<String>",
            default: "None",
            description: "Didascalia descrittiva aggiuntiva posizionata sotto l'etichetta",
        },
        PropRow {
            name: "disabled",
            r#type: "bool",
            default: "false",
            description: "Disabilita l'interruttore e ne impedisce il toggle al click",
        },
    ];

    let code_snippet = r#"let mut is_active = use_signal(|| true);

Switch {
    checked: is_active,
    label: "Modalità Telemetria Avanzata".to_string(),
    description: "Invia report analitici in tempo reale".to_string(),
}"#
    .to_string();

    rsx! {
        div { class: "sc-stack sc-stack--loose",
            PageHeader {
                title: "Switch".to_string(),
                description: "Interruttori a scatto tattili con animazioni fluide di traslazione e coordinamento con la palette primaria del tema.".to_string(),
            }

            FeatureBanner { feature_name: "switch" }

            ShowcaseTabs {
                preview: rsx! {
                    Playground {
                        stage_start: true,
                        stage: rsx! {
                            div { class: "sc-stack",
                                Switch {
                                    checked: dev_mode,
                                    label: "Modalità Sviluppatore".to_string(),
                                    description: "Abilita banner di debug e ispezione delle rotte in tempo reale".to_string(),
                                }
                                Switch {
                                    checked: push_alerts,
                                    label: "Notifiche Push di Sistema".to_string(),
                                    description: "Invia avvisi al completamento di deploy e pipeline CI/CD".to_string(),
                                }
                                Switch {
                                    checked: telemetry,
                                    label: "Report Telemetria Diagnostica".to_string(),
                                    description: "Condivisione anonima dei parametri di prestazione CPU/RAM".to_string(),
                                }
                                Switch {
                                    checked: locked_state,
                                    label: "Opzione Bloccata (Disabled)".to_string(),
                                    description: "Non modificabile con i privilegi utente correnti".to_string(),
                                    disabled: true,
                                }
                            }
                        },
                        controls: rsx! {
                            span { class: "sc-control-label", "Stato reattivo" }
                            div { class: "sc-stack sc-mono",
                                Counter { label: "dev_mode".to_string(), value: dev_mode().to_string() }
                                Counter { label: "push_alerts".to_string(), value: push_alerts().to_string() }
                                Counter { label: "telemetry".to_string(), value: telemetry().to_string() }
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
