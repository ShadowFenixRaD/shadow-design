use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::components::{
    ChoicePicker, FeatureBanner, PageHeader, Playground, PropRow, ShowcaseTabs, ToggleControl,
};

#[component]
pub fn FieldsetPage() -> Element {
    let variant = use_signal(|| FieldsetVariant::Default);
    let disabled = use_signal(|| false);
    let node_name = use_signal(|| "node-shadow-01".to_string());
    let proto = use_signal(|| "tls-v1.3-strict".to_string());

    let props = vec![
        PropRow {
            name: "legend",
            r#type: "String",
            default: "-",
            description: "Titolo primario inserito nell'elemento <legend>",
        },
        PropRow {
            name: "description",
            r#type: "Option<String>",
            default: "None",
            description: "Testo opzionale visualizzato direttamente sotto la legenda",
        },
        PropRow {
            name: "variant",
            r#type: "FieldsetVariant",
            default: "FieldsetVariant::Default",
            description: "Stile grafico del contenitore (Default, Filled, Ungrouped)",
        },
        PropRow {
            name: "disabled",
            r#type: "bool",
            default: "false",
            description: "Se true, disabilita il contenitore e a cascata tutti i controlli nativi contenuti",
        },
        PropRow {
            name: "children",
            r#type: "Element",
            default: "-",
            description: "Controlli o campi form contenuti nel gruppo logico",
        },
    ];

    let code_snippet = r#"Fieldset {
    legend: "Credenziali Crittografiche".to_string(),
    description: Some("Chiavi pubbliche e token di sicurezza.".to_string()),
    variant: FieldsetVariant::Default,
    disabled: false,
    div { class: "space-y-3",
        TextInput { placeholder: "Chiave SSH Ed25519" }
    }
}"#
    .to_string();

    rsx! {
        div { class: "sc-stack sc-stack--loose",
            PageHeader {
                title: "Fieldset".to_string(),
                description: "Raggruppa semanticamente controlli correlati all'interno di un modulo con legenda integrata e supporto allo stato disabled a cascata.".to_string(),
            }

            FeatureBanner { feature_name: "fieldset" }

            ShowcaseTabs {
                preview: rsx! {
                    Playground {
                        stage: rsx! {
                            div { class: "w-full max-w-md",
                                Fieldset {
                                    legend: "Cluster Security Profile".to_string(),
                                    description: Some("Definisci i parametri di cifratura per il nodo periferico.".to_string()),
                                    variant: variant(),
                                    disabled: disabled(),
                                    div { class: "sc-stack",
                                        TextInput {
                                            value: node_name,
                                            placeholder: "Hostname del nodo",
                                        }
                                        TextInput {
                                            value: proto,
                                            placeholder: "Protocollo",
                                        }
                                        Button {
                                            variant: ButtonVariant::Primary,
                                            size: ButtonSize::Sm,
                                            "Salva Configurazione"
                                        }
                                    }
                                }
                            }
                        },
                        controls: rsx! {
                            ChoicePicker {
                                label: "Variante".to_string(),
                                options: vec![
                                    (FieldsetVariant::Default, "Default".to_string()),
                                    (FieldsetVariant::Filled, "Filled".to_string()),
                                    (FieldsetVariant::Ungrouped, "Ungrouped".to_string()),
                                ],
                                value: variant,
                            }
                            ToggleControl { label: "Disabilitato".to_string(), checked: disabled }
                        },
                    }
                },
                code: code_snippet,
                props: props,
            }
        }
    }
}
