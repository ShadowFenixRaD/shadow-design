use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::components::{ChoicePicker, ControlGroup, ControlRow, Counter, Playground, ToggleControl, FeatureBanner, PageHeader, PropRow, ShowcaseTabs};

#[component]
pub fn StepperPage() -> Element {
    let mut active_step = use_signal(|| 0usize);
    let orientation = use_signal(|| StepperOrientation::Horizontal);
    let allow_click_completed = use_signal(|| true);
    let simulate_error = use_signal(|| false);

    // Form inputs state
    let node_id = use_signal(|| "node-01-rome".to_string());
    let pq_enabled = use_signal(|| true);

    let props = vec![
        PropRow {
            name: "active",
            r#type: "usize",
            default: "0",
            description: "Indice dello step correntemente attivo (0-based)",
        },
        PropRow {
            name: "orientation",
            r#type: "Option<StepperOrientation>",
            default: "StepperOrientation::Horizontal",
            description: "Disposizione dei passi: Horizontal (in riga con linee di connessione) o Vertical (procedurale)",
        },
        PropRow {
            name: "allow_click_completed",
            r#type: "bool",
            default: "true",
            description: "Consente all'utente di cliccare sui passi già completati per tornare indietro",
        },
        PropRow {
            name: "on_change",
            r#type: "Option<EventHandler<usize>>",
            default: "None",
            description: "Callback invocata al click su uno step abilitato",
        },
        PropRow {
            name: "children",
            r#type: "Element",
            default: "-",
            description: "Elenco dei componenti StepperStep ed eventuale StepperCompleted finale",
        },
    ];

    let code_snippet = r##"Stepper {
    active: active_step(),
    orientation: StepperOrientation::Horizontal,
    allow_click_completed: true,
    on_change: move |step| active_step.set(step),

    StepperStep {
        step: 0,
        label: "Identità".to_string(),
        description: Some("Chiavi HSM & ID".to_string()),
        icon: Some(SvgIcon::Kind(IconKind::Key)),
        Card {
            "Contenuto dello Step 1..."
        }
    }
    StepperStep {
        step: 1,
        label: "Rete Mesh".to_string(),
        description: Some("Topologia P2P".to_string()),
        icon: Some(SvgIcon::Kind(IconKind::Radio)),
        Card {
            "Contenuto dello Step 2..."
        }
    }
    StepperStep {
        step: 2,
        label: "Sicurezza".to_string(),
        description: Some("Post-Quantum ML-KEM".to_string()),
        icon: Some(SvgIcon::Kind(IconKind::ShieldCheck)),
        Card {
            "Contenuto dello Step 3..."
        }
    }
    StepperCompleted {
        Alert { variant: AlertVariant::Success,
            "Provisioning del nodo completato con successo!"
        }
    }
}"##
    .to_string();

    let total_steps = 3usize;

    rsx! {
        div { class: "sc-stack sc-stack--loose",
            PageHeader {
                title: "Stepper".to_string(),
                description: "Guida procedurale multi-step per wizard o flussi guidati, con supporto orizzontale e verticale, nodi numerati o a icone SVG, linee connettive reattive e gestione errori.".to_string(),
            }

            FeatureBanner { feature_name: "stepper" }

            ShowcaseTabs {
                preview: rsx! {
                    Playground {
                        stage_start: true,
                        stage: rsx! {
                            Stepper {
                                active: active_step(),
                                orientation: orientation(),
                                allow_click_completed: allow_click_completed(),
                                on_change: move |step| active_step.set(step),

                                StepperStep {
                                    step: 0,
                                    label: "Identità".to_string(),
                                    description: Some("Chiavi HSM & ID".to_string()),
                                    icon: Some(SvgIcon::Kind(IconKind::Key)),
                                    Card { title: "Passo 1: Definizione identità nodo".to_string(),
                                        div { class: "sc-stack",
                                            TextInput {
                                                value: node_id,
                                                label: Some("Nome Identificativo Nodo Mesh".to_string()),
                                                placeholder: "es. node-eu-alpha".to_string(),
                                            }
                                            p { class: "sc-page-desc", "L'identificatore univoco viene firmato con la chiave asimmetrica locale." }
                                        }
                                    }
                                }

                                StepperStep {
                                    step: 1,
                                    label: "Rete Mesh".to_string(),
                                    description: Some("Topologia P2P".to_string()),
                                    icon: Some(SvgIcon::Kind(IconKind::Radio)),
                                    state: if simulate_error() && active_step() == 1 { Some(StepState::Error) } else { None },
                                    Card { title: "Passo 2: Configurazione canale radio".to_string(),
                                        if simulate_error() {
                                            Alert {
                                                variant: AlertVariant::Danger,
                                                title: Some("Errore Sincronizzazione Mesh".to_string()),
                                                "Nessun peer rilevato sulla frequenza radio 868 MHz. Verifica l'antenna o reimposta il canale."
                                            }
                                        } else {
                                            div { class: "sc-stack",
                                                Counter { label: "Protocollo di trasporto".to_string(), value: "LoRaWAN & UDP Direct".to_string() }
                                                Counter { label: "Canale primario".to_string(), value: "Ch #07 (868.100 MHz)".to_string() }
                                            }
                                        }
                                    }
                                }

                                StepperStep {
                                    step: 2,
                                    label: "Sicurezza".to_string(),
                                    description: Some("Post-Quantum ML-KEM".to_string()),
                                    icon: Some(SvgIcon::Kind(IconKind::ShieldCheck)),
                                    Card { title: "Passo 3: Parametri di crittografia quantistica".to_string(),
                                        Switch {
                                            checked: pq_enabled,
                                            label: Some("Abilita Handshake ML-KEM 768 (NIST FIPS 203)".to_string()),
                                            description: Some("Protegge le comunicazioni mesh da intercettazioni future computate via calcolo quantistico.".to_string()),
                                        }
                                    }
                                }

                                if active_step() >= total_steps {
                                    StepperCompleted {
                                        Alert {
                                            variant: AlertVariant::Success,
                                            title: Some("Provisioning Completato!".to_string()),
                                            "Il nodo \"{node_id()}\" è stato configurato con successo ed è ora attivo nella rete mesh decentralizzata."
                                        }
                                        div { class: "sc-actions",
                                            Button {
                                                size: ButtonSize::Sm,
                                                variant: ButtonVariant::Primary,
                                                onclick: move |_| active_step.set(0),
                                                "Riavvia Configurazione Wizard"
                                            }
                                        }
                                    }
                                }
                            }
                        },
                        controls: rsx! {
                            ChoicePicker {
                                label: "Orientamento".to_string(),
                                options: vec![
                                    (StepperOrientation::Horizontal, "Orizzontale".to_string()),
                                    (StepperOrientation::Vertical, "Verticale".to_string()),
                                ],
                                value: orientation,
                            }
                            hr { class: "sc-control-divider" }
                            ControlRow {
                                ToggleControl { label: "Click su step completati".to_string(), checked: allow_click_completed }
                                ToggleControl { label: "Simula errore su step 2".to_string(), checked: simulate_error }
                            }
                            hr { class: "sc-control-divider" }
                            ControlGroup { label: "Avanzamento rapido".to_string(),
                                div { class: "sc-actions",
                                    Button {
                                        size: ButtonSize::Sm,
                                        variant: ButtonVariant::Outline,
                                        disabled: active_step() == 0,
                                        onclick: move |_| {
                                            if active_step() > 0 {
                                                active_step.set(active_step() - 1);
                                            }
                                        },
                                        "Indietro"
                                    }
                                    Button {
                                        size: ButtonSize::Sm,
                                        variant: ButtonVariant::Primary,
                                        disabled: active_step() >= total_steps,
                                        onclick: move |_| {
                                            if active_step() < total_steps {
                                                active_step.set(active_step() + 1);
                                            }
                                        },
                                        if active_step() == total_steps - 1 { "Concludi" } else { "Avanti" }
                                    }
                                    if active_step() >= total_steps {
                                        Button {
                                            size: ButtonSize::Sm,
                                            variant: ButtonVariant::Ghost,
                                            onclick: move |_| active_step.set(0),
                                            "Reset"
                                        }
                                    }
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
