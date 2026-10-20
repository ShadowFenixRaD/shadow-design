use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::components::{FeatureBanner, PropRow, ShowcaseTabs};

#[component]
pub fn StepperPage() -> Element {
    let mut active_step = use_signal(|| 0usize);
    let mut orientation = use_signal(|| StepperOrientation::Horizontal);
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
        div { class: "p-4 rounded-xl bg-white/[0.02] border border-white/[0.08]",
            "Contenuto dello Step 1..."
        }
    }
    StepperStep {
        step: 1,
        label: "Rete Mesh".to_string(),
        description: Some("Topologia P2P".to_string()),
        icon: Some(SvgIcon::Kind(IconKind::Radio)),
        div { class: "p-4 rounded-xl bg-white/[0.02] border border-white/[0.08]",
            "Contenuto dello Step 2..."
        }
    }
    StepperStep {
        step: 2,
        label: "Sicurezza".to_string(),
        description: Some("Post-Quantum ML-KEM".to_string()),
        icon: Some(SvgIcon::Kind(IconKind::ShieldCheck)),
        div { class: "p-4 rounded-xl bg-white/[0.02] border border-white/[0.08]",
            "Contenuto dello Step 3..."
        }
    }
    StepperCompleted {
        div { class: "p-6 rounded-xl bg-purple-950/20 border border-purple-500/30 text-center",
            "Provisioning del nodo completato con successo!"
        }
    }
}"##
    .to_string();

    let total_steps = 3usize;

    rsx! {
        div { class: "max-w-4xl space-y-8",
            div {
                Heading { order: HeadingOrder::H2, "Stepper" }
                Text { dimmed: true, size: TextSize::Sm, "Guida procedurale multi-step per wizard o flussi guidati, con supporto orizzontale e verticale, nodi numerati o a icone SVG, linee connettive reattive e gestione errori." }
            }

            FeatureBanner { feature_name: "stepper" }

            ShowcaseTabs {
                preview: rsx! {
                    div { class: "space-y-6",
                        // Toolbar interattiva
                        div { class: "p-4 rounded-xl bg-[#060608] border border-white/[0.08] grid grid-cols-1 md:grid-cols-3 gap-4 text-xs font-mono",
                            div {
                                span { class: "text-zinc-400 block mb-1.5 uppercase tracking-wider text-[10px]", "Orientamento:" }
                                div { class: "flex space-x-2",
                                    button {
                                        r#type: "button",
                                        class: if orientation() == StepperOrientation::Horizontal {
                                            "flex-1 px-2 py-1.5 rounded bg-purple-950/80 text-purple-200 border border-purple-500/40 font-bold cursor-pointer"
                                        } else {
                                            "flex-1 px-2 py-1.5 rounded bg-white/[0.03] text-zinc-400 border border-white/[0.06] cursor-pointer"
                                        },
                                        onclick: move |_| orientation.set(StepperOrientation::Horizontal),
                                        "Orizzontale"
                                    }
                                    button {
                                        r#type: "button",
                                        class: if orientation() == StepperOrientation::Vertical {
                                            "flex-1 px-2 py-1.5 rounded bg-purple-950/80 text-purple-200 border border-purple-500/40 font-bold cursor-pointer"
                                        } else {
                                            "flex-1 px-2 py-1.5 rounded bg-white/[0.03] text-zinc-400 border border-white/[0.06] cursor-pointer"
                                        },
                                        onclick: move |_| orientation.set(StepperOrientation::Vertical),
                                        "Verticale"
                                    }
                                }
                            }

                            div { class: "flex flex-col justify-center space-y-2 pt-2 md:pt-0 border-t md:border-l border-white/[0.06] md:pl-4",
                                Checkbox {
                                    checked: allow_click_completed,
                                    label: Some("Click su Step Completati".to_string()),
                                }
                                Checkbox {
                                    checked: simulate_error,
                                    label: Some("Simula Errore su Step 2".to_string()),
                                }
                            }

                            div { class: "flex flex-col justify-center pt-2 md:pt-0 border-t md:border-l border-white/[0.06] md:pl-4",
                                span { class: "text-zinc-400 block mb-1.5 uppercase tracking-wider text-[10px]", "Avanzamento Rapido:" }
                                div { class: "flex space-x-2",
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
                        }

                        // Preview Box: Stepper Interattivo
                        div { class: "p-6 rounded-xl bg-[#060608] border border-white/[0.08] shadow-2xl",
                            Stepper {
                                active: active_step(),
                                orientation: orientation(),
                                allow_click_completed: allow_click_completed(),
                                on_change: move |step| active_step.set(step),

                                // Step 0: Identità
                                StepperStep {
                                    step: 0,
                                    label: "Identità".to_string(),
                                    description: Some("Chiavi HSM & ID".to_string()),
                                    icon: Some(SvgIcon::Kind(IconKind::Key)),
                                    div { class: "p-6 rounded-xl bg-white/[0.02] border border-white/[0.06] space-y-4",
                                        div { class: "text-xs font-mono text-purple-400 font-bold uppercase", "Passo 1: Definizione Identità Nodo" }
                                        div { class: "max-w-md space-y-3",
                                            TextInput {
                                                value: node_id,
                                                label: Some("Nome Identificativo Nodo Mesh".to_string()),
                                                placeholder: "es. node-eu-alpha".to_string(),
                                            }
                                            div { class: "text-[11px] font-mono text-zinc-500", "L'identificatore univoco viene firmato con la chiave asimmetrica locale." }
                                        }
                                    }
                                }

                                // Step 1: Topologia Rete
                                StepperStep {
                                    step: 1,
                                    label: "Rete Mesh".to_string(),
                                    description: Some("Topologia P2P".to_string()),
                                    icon: Some(SvgIcon::Kind(IconKind::Radio)),
                                    state: if simulate_error() && active_step() == 1 { Some(StepState::Error) } else { None },
                                    div { class: "p-6 rounded-xl bg-white/[0.02] border border-white/[0.06] space-y-4",
                                        div { class: "text-xs font-mono text-purple-400 font-bold uppercase", "Passo 2: Configurazione Canale Radio" }
                                        if simulate_error() {
                                            Alert {
                                                variant: AlertVariant::Danger,
                                                title: Some("Errore Sincronizzazione Mesh".to_string()),
                                                "Nessun peer rilevato sulla frequenza radio 868 MHz. Verifica l'antenna o reimposta il canale."
                                            }
                                        } else {
                                            div { class: "grid grid-cols-1 md:grid-cols-2 gap-4 text-xs font-mono",
                                                div { class: "p-3 rounded bg-white/[0.03] border border-white/[0.04]",
                                                    div { class: "text-zinc-400", "Protocollo di Trasporto:" }
                                                    div { class: "text-zinc-100 font-bold mt-1", "LoRaWAN & UDP Direct" }
                                                }
                                                div { class: "p-3 rounded bg-white/[0.03] border border-white/[0.04]",
                                                    div { class: "text-zinc-400", "Canale Primario:" }
                                                    div { class: "text-zinc-100 font-bold mt-1", "Ch #07 (868.100 MHz)" }
                                                }
                                            }
                                        }
                                    }
                                }

                                // Step 2: Crittografia Post-Quantum
                                StepperStep {
                                    step: 2,
                                    label: "Sicurezza".to_string(),
                                    description: Some("Post-Quantum ML-KEM".to_string()),
                                    icon: Some(SvgIcon::Kind(IconKind::ShieldCheck)),
                                    div { class: "p-6 rounded-xl bg-white/[0.02] border border-white/[0.06] space-y-4",
                                        div { class: "text-xs font-mono text-purple-400 font-bold uppercase", "Passo 3: Parametri di Crittografia Quantistica" }
                                        div { class: "space-y-3",
                                            Switch {
                                                checked: pq_enabled,
                                                label: Some("Abilita Handshake ML-KEM 768 (NIST FIPS 203)".to_string()),
                                                description: Some("Protegge le comunicazioni mesh da intercettazioni future computate via calcolo quantistico.".to_string()),
                                            }
                                        }
                                    }
                                }

                                // Stato finale completato
                                if active_step() >= total_steps {
                                    StepperCompleted {
                                        div { class: "p-8 rounded-xl bg-purple-950/20 border border-purple-500/30 text-center space-y-4",
                                            div {
                                                class: "w-12 h-12 rounded-full bg-purple-900/60 border border-purple-400/40 text-purple-200 flex items-center justify-center mx-auto shadow-lg",
                                                style: "box-shadow: 0 0 20px rgba(168, 85, 247, 0.4);",
                                                Icon { kind: IconKind::Check, size: 24 }
                                            }
                                            div {
                                                Heading { order: HeadingOrder::H3, "Provisioning Completato!" }
                                                Text { dimmed: true, size: TextSize::Sm, "Il nodo \"{node_id()}\" è stato configurato con successo ed è ora attivo nella rete mesh decentralizzata." }
                                            }
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
                        }
                    }
                },
                code: code_snippet,
                props: props,
            }
        }
    }
}
