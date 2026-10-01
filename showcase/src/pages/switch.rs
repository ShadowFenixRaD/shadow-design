use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::components::{FeatureBanner, PropRow, ShowcaseTabs};

#[component]
pub fn SwitchPage() -> Element {
    let dev_mode = use_signal(|| true);
    let push_alerts = use_signal(|| false);
    let telemetry = use_signal(|| true);

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
        div { class: "max-w-4xl space-y-8",
            div {
                Heading { order: HeadingOrder::H2, "Switch / Toggle Component" }
                Text { dimmed: true, size: TextSize::Sm, "Interruttori a scatto tattili ad alta precisione con animazioni fluide di traslazione e coordinamento dinamico con la palette primaria del tema." }
            }

            FeatureBanner { feature_name: "switch" }

            ShowcaseTabs {
                preview: rsx! {
                    div { class: "grid grid-cols-1 md:grid-cols-2 gap-8 items-start",
                        div { class: "space-y-6",
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
                                checked: use_signal(|| false),
                                label: "Opzione Bloccata (Disabled)".to_string(),
                                description: "Non modificabile con i privilegi utente correnti".to_string(),
                                disabled: true,
                            }
                        }

                        div { class: "p-5 rounded-xl bg-[#060608] border border-white/[0.08] space-y-3 font-mono text-xs shadow-inner",
                            p { class: "text-zinc-400 font-semibold uppercase text-[10px] tracking-wider", "Stato Reattivo Switch:" }
                            div { class: "p-3 rounded-lg bg-black/60 border border-white/[0.04] space-y-1.5",
                                div { class: "text-purple-300",
                                    span { class: "text-zinc-500", "dev_mode = " }
                                    "{dev_mode}"
                                }
                                div { class: "text-cyan-300",
                                    span { class: "text-zinc-500", "push_alerts = " }
                                    "{push_alerts}"
                                }
                                div { class: "text-emerald-300",
                                    span { class: "text-zinc-500", "telemetry = " }
                                    "{telemetry}"
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
