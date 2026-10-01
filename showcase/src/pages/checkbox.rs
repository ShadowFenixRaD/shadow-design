use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::components::{FeatureBanner, PropRow, ShowcaseTabs};

#[component]
pub fn CheckboxPage() -> Element {
    let accept_terms = use_signal(|| true);
    let newsletter = use_signal(|| false);
    let diagnostics = use_signal(|| true);
    let indeterminate_state = use_signal(|| false);
    let error_state = use_signal(|| false);
    let mut custom_glow = use_signal(|| true);

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
        div { class: "max-w-4xl space-y-8",
            div {
                Heading { order: HeadingOrder::H2, "Checkbox Component" }
                Text { dimmed: true, size: TextSize::Sm, "Caselle di selezione tattili con animazione di feedback, supporto a tre dimensioni (Sm/Md/Lg), stato indeterminato, gestione errori e glow perimetrale personalizzabile." }
            }

            FeatureBanner { feature_name: "checkbox" }

            ShowcaseTabs {
                preview: rsx! {
                    div { class: "grid grid-cols-1 md:grid-cols-2 gap-8 items-start",
                        div { class: "space-y-6",
                            div { class: "space-y-4",
                                h4 { class: "text-xs font-semibold text-zinc-400 uppercase tracking-wider", "Dimensioni e Glow" }
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
                            }

                            div { class: "pt-4 border-t border-white/[0.06] space-y-4",
                                h4 { class: "text-xs font-semibold text-zinc-400 uppercase tracking-wider", "Stati Speciali" }

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
                                    checked: use_signal(|| true),
                                    disabled: true,
                                    label: "Checkbox Disabilitata (Attiva)".to_string(),
                                    description: "Opzione non modificabile con i tuoi permessi attuali".to_string(),
                                }
                            }
                        }

                        div { class: "space-y-4",
                            div { class: "p-4 rounded-xl bg-[#0e0e14] border border-white/[0.08] space-y-3",
                                h4 { class: "text-xs font-bold text-white uppercase tracking-wider", "Controlli Live" }
                                div { class: "flex items-center justify-between",
                                    span { class: "text-xs text-zinc-400", "Glow Perimetrale:" }
                                    button {
                                        r#type: "button",
                                        class: "px-3 py-1 rounded text-xs font-medium border border-white/10 bg-white/[0.04] hover:bg-white/[0.08] text-white cursor-pointer transition-colors",
                                        onclick: move |_| {
                                            let cur = *custom_glow.read();
                                            *custom_glow.write() = !cur;
                                        },
                                        if *custom_glow.read() { "Disattiva Glow" } else { "Attiva Glow" }
                                    }
                                }
                            }

                            div { class: "p-5 rounded-xl bg-[#060608] border border-white/[0.08] space-y-3 font-mono text-xs shadow-inner",
                                p { class: "text-zinc-400 font-semibold uppercase text-[10px] tracking-wider", "Stato Reattivo Checkbox:" }
                                div { class: "p-3 rounded-lg bg-black/60 border border-white/[0.04] space-y-1.5",
                                    div { class: "text-purple-300",
                                        span { class: "text-zinc-500", "accept_terms = " }
                                        "{accept_terms}"
                                    }
                                    div { class: "text-cyan-300",
                                        span { class: "text-zinc-500", "newsletter = " }
                                        "{newsletter}"
                                    }
                                    div { class: "text-emerald-300",
                                        span { class: "text-zinc-500", "diagnostics = " }
                                        "{diagnostics}"
                                    }
                                    div { class: "text-amber-300",
                                        span { class: "text-zinc-500", "indeterminate_state = " }
                                        "{indeterminate_state}"
                                    }
                                }
                            }
                        }
                    }
                },
                code: code_snippet,
                props: props
            }
        }
    }
}
