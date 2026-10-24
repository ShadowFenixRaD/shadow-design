use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::components::{
    Counter, FeatureBanner, PageHeader, Playground, PropRow, ShowcaseTabs, ToggleControl,
};

#[component]
pub fn CheckboxPage() -> Element {
    let accept_terms = use_signal(|| true);
    let newsletter = use_signal(|| false);
    let diagnostics = use_signal(|| true);
    let indeterminate_state = use_signal(|| false);
    let error_state = use_signal(|| false);
    let disabled_active = use_signal(|| true);
    let custom_glow = use_signal(|| true);

    let props = vec![
        PropRow {
            name: "checked",
            r#type: "Signal<bool>",
            default: "-",
            description: "Signal reattivo che governa e riflette lo stato booleano della checkbox",
        },
        PropRow {
            name: "size",
            r#type: "CheckboxSize",
            default: "CheckboxSize::Md",
            description: "Dimensione visiva del riquadro: Sm (16px), Md (20px), Lg (24px)",
        },
        PropRow {
            name: "label",
            r#type: "Option<String>",
            default: "None",
            description: "Etichetta testuale posizionata a destra della casella",
        },
        PropRow {
            name: "description",
            r#type: "Option<String>",
            default: "None",
            description: "Didascalia descrittiva secondaria collocata sotto l'etichetta",
        },
        PropRow {
            name: "indeterminate",
            r#type: "bool",
            default: "false",
            description: "Rappresenta uno stato parziale con una barra orizzontale anziché la spunta",
        },
        PropRow {
            name: "error",
            r#type: "Option<String>",
            default: "None",
            description: "Messaggio di errore che evidenzia la casella in rosso ed espone una didascalia di allerta",
        },
        PropRow {
            name: "disabled",
            r#type: "bool",
            default: "false",
            description: "Disabilita l'interazione e sfuma l'opacità del componente",
        },
        PropRow {
            name: "glow",
            r#type: "Option<bool>",
            default: "None",
            description: "Override opzionale dell'alone di luce perimetrale coordinato con il tema",
        },
    ];

    let code_snippet = r#"let mut is_accepted = use_signal(|| true);

Checkbox {
    checked: is_accepted,
    size: CheckboxSize::Md,
    label: "Accetta termini e condizioni di servizio".to_string(),
    description: "Necessario per l'attivazione dell'account di produzione".to_string(),
    glow: Some(true),
}"#
    .to_string();

    rsx! {
        div { class: "sc-stack sc-stack--loose",
            PageHeader {
                title: "Checkbox Component".to_string(),
                description: "Caselle di selezione tattili con animazione di feedback, supporto a tre dimensioni (Sm/Md/Lg), stato indeterminato, gestione errori e glow perimetrale personalizzabile.".to_string(),
            }

            FeatureBanner { feature_name: "checkbox" }

            ShowcaseTabs {
                preview: rsx! {
                    Playground {
                        stage_start: true,
                        stage: rsx! {
                            div { class: "sc-stack",
                                span { class: "sc-control-label", "Dimensioni e Glow" }
                                Checkbox {
                                    checked: accept_terms,
                                    size: CheckboxSize::Sm,
                                    label: "Taglia Sm (16px) — Termini di Licenza".to_string(),
                                    glow: *custom_glow.read(),
                                }
                                Checkbox {
                                    checked: newsletter,
                                    size: CheckboxSize::Md,
                                    label: "Taglia Md (20px) — Iscrizione Bollettino".to_string(),
                                    description: "Ricevi aggiornamenti sulle release e patch di sicurezza".to_string(),
                                    glow: *custom_glow.read(),
                                }
                                Checkbox {
                                    checked: diagnostics,
                                    size: CheckboxSize::Lg,
                                    label: "Taglia Lg (24px) — Telemetria Avanzata".to_string(),
                                    description: "Monitoraggio runtime delle metriche di latenza di rete".to_string(),
                                    glow: *custom_glow.read(),
                                }

                                hr { class: "sc-control-divider" }
                                span { class: "sc-control-label", "Stati Speciali" }
                                Checkbox {
                                    checked: indeterminate_state,
                                    indeterminate: true,
                                    label: "Stato Indeterminato (Parziale)".to_string(),
                                    description: "Visualizza una barra di selezione parziale per nodi con figli misti".to_string(),
                                }
                                Checkbox {
                                    checked: error_state,
                                    error: "Campo obbligatorio non soddisfatto".to_string(),
                                    label: "Stato con Errore".to_string(),
                                    description: "Bordo evidenziato in rosso pericolo per campi obbligatori omessi".to_string(),
                                }
                                Checkbox {
                                    checked: disabled_active,
                                    disabled: true,
                                    label: "Checkbox Disabilitata (Attiva)".to_string(),
                                    description: "Opzione non modificabile con i tuoi permessi attuali".to_string(),
                                }
                            }
                        },
                        controls: rsx! {
                            ToggleControl { label: "Glow Perimetrale".to_string(), checked: custom_glow }
                            hr { class: "sc-control-divider" }
                            span { class: "sc-control-label", "Stato Reattivo Checkbox" }
                            Counter { label: "accept_terms".to_string(), value: accept_terms().to_string() }
                            Counter { label: "newsletter".to_string(), value: newsletter().to_string() }
                            Counter { label: "diagnostics".to_string(), value: diagnostics().to_string() }
                            Counter { label: "indeterminate_state".to_string(), value: indeterminate_state().to_string() }
                        },
                    }
                },
                code: code_snippet,
                props: props
            }
        }
    }
}
