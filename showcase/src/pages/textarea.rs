use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::components::{FeatureBanner, PropRow, ShowcaseTabs};

#[component]
pub fn TextareaPage() -> Element {
    let bio = use_signal(|| "Ingegnere di sistemi distribuiti ed estimatore di architetture zero-runtime.".to_string());
    let auto_note = use_signal(|| "Prima riga di appunti...\nSeconda riga espansa automaticamente!\n".to_string());
    let commit_msg = use_signal(|| "feat(core): implement zero-cost abstractions".to_string());
    let feedback = use_signal(String::new);

    let props = vec![
        PropRow {
            name: "value",
            r#type: "Signal<String>",
            default: "-",
            description: "Signal reattivo bidirezionale Dioxus 0.7 per il testo multilinea",
        },
        PropRow {
            name: "label",
            r#type: "Option<String>",
            default: "None",
            description: "Etichetta descrittiva posizionata sopra al campo",
        },
        PropRow {
            name: "placeholder",
            r#type: "Option<String>",
            default: "None",
            description: "Testo segnaposto informativo visibile quando il campo è vuoto",
        },
        PropRow {
            name: "min_rows",
            r#type: "usize",
            default: "3",
            description: "Numero minimo di righe verticali visibili",
        },
        PropRow {
            name: "max_rows",
            r#type: "Option<usize>",
            default: "None",
            description: "Numero massimo di righe per l'espansione automatica autosize",
        },
        PropRow {
            name: "autosize",
            r#type: "bool",
            default: "false",
            description: "Abilita l'adattamento verticale dinamico proporzionale al contenuto digitato",
        },
        PropRow {
            name: "show_count",
            r#type: "bool",
            default: "false",
            description: "Mostra il contatore di caratteri in basso a destra",
        },
        PropRow {
            name: "max_length",
            r#type: "Option<usize>",
            default: "None",
            description: "Limite massimo di caratteri ammessi nel campo",
        },
        PropRow {
            name: "clearable",
            r#type: "bool",
            default: "false",
            description: "Mostra un pulsante tattile rapido (X) per azzerare il contenuto",
        },
        PropRow {
            name: "error",
            r#type: "Option<String>",
            default: "None",
            description: "Messaggio di errore che colora il bordo di rosso fenice e mostra la descrizione",
        },
        PropRow {
            name: "help_text",
            r#type: "Option<String>",
            default: "None",
            description: "Istruzione o nota di supporto posizionata sotto al campo",
        },
    ];

    let code_snippet = r#"let mut bio = use_signal(|| "Testo iniziale...".to_string());
let mut log_message = use_signal(String::new);

// 1. Textarea standard con contatore caratteri e clearable
Textarea {
    value: bio,
    label: Some("Biografia Utente".to_string()),
    placeholder: Some("Descrivi la tua attività...".to_string()),
    show_count: true,
    max_length: Some(250),
    clearable: true,
}

// 2. AutoResizeTextarea a espansione verticale dinamica
AutoResizeTextarea {
    value: log_message,
    label: Some("Log Note di Sistema".to_string()),
    placeholder: Some("Digita per vedere l'espansione automatica delle righe...".to_string()),
    min_rows: 2,
    max_rows: Some(6),
    clearable: true,
}"#
    .to_string();

    let bio_chars = bio().chars().count();
    let auto_lines = auto_note().matches('\n').count() + 1;

    rsx! {
        div { class: "max-w-4xl space-y-8",
            div {
                Heading { order: HeadingOrder::H2, "Textarea" }
                Text { dimmed: true, size: TextSize::Sm, "Aree di testo multilinea ad alta precisione con espansione verticale dinamica proporzionale al contenuto (AutoResizeTextarea), conteggio caratteri, cancellazione rapida (clearable) e supporto al tema Mantine." }
            }

            FeatureBanner { feature_name: "textarea" }

            ShowcaseTabs {
                preview: rsx! {
                    div { class: "grid grid-cols-1 lg:grid-cols-2 gap-8 items-start",
                        // Live Controls
                        div { class: "space-y-6",
                            // 1. Standard Textarea con contatore e clearable
                            Textarea {
                                value: bio,
                                label: Some("Biografia Utente".to_string()),
                                placeholder: Some("Scrivi una breve introduzione...".to_string()),
                                help_text: Some("Massimo 200 caratteri ammessi".to_string()),
                                show_count: true,
                                max_length: Some(200),
                                clearable: true,
                            }

                            // 2. AutoResizeTextarea dinamica
                            AutoResizeTextarea {
                                value: auto_note,
                                label: Some("Appunti Live (AutoResizeTextarea)".to_string()),
                                placeholder: Some("Aggiungi righe per vedere l'espansione automatica dell'altezza...".to_string()),
                                help_text: Some("Minimo 2 righe, espansione fino a 6 righe".to_string()),
                                min_rows: 2,
                                max_rows: Some(6),
                                clearable: true,
                            }

                            // 3. Textarea per messaggio commit
                            Textarea {
                                value: commit_msg,
                                label: Some("Messaggio di Commit / Release".to_string()),
                                min_rows: 2,
                                show_count: true,
                                max_length: Some(72),
                            }
                        }

                        // State Telemetry
                        div { class: "p-5 rounded-xl bg-[#060608] border border-white/[0.08] space-y-4 font-mono text-xs shadow-inner",
                            p { class: "text-zinc-400 font-semibold uppercase text-[10px] tracking-wider", "Stato Reattivo Signal:" }
                            div { class: "p-3 rounded-lg bg-black/60 border border-white/[0.04] space-y-2.5",
                                div { class: "text-purple-300",
                                    span { class: "text-zinc-500", "bio.len = " }
                                    "{bio_chars} / 200"
                                }
                                div { class: "text-cyan-300",
                                    span { class: "text-zinc-500", "auto_note.lines = " }
                                    "{auto_lines} righe calcolate"
                                }
                                div { class: "text-emerald-300",
                                    span { class: "text-zinc-500", "commit_msg = " }
                                    "\"{commit_msg}\""
                                }
                            }
                            span { class: "text-[11px] text-zinc-500 block leading-normal",
                                "La digitazione aggiorna istantaneamente lo stato reattivo e ricalcola le righe visibili in tempo reale senza flickering."
                            }
                        }
                    }
                },
                code: code_snippet,
                props: props,
            }

            // Varianti ed Error States
            Card {
                title: "Stati di Validazione ed Eccezioni".to_string(),
                subtitle: "Comportamento visivo con stato disabilitato, sola lettura ed errore di validazione".to_string(),
                div { class: "grid grid-cols-1 md:grid-cols-3 gap-6 pt-2",
                    Textarea {
                        value: use_signal(|| "Questo campo è in sola lettura e non può essere modificato.".to_string()),
                        label: Some("Sola Lettura (Readonly)".to_string()),
                        readonly: true,
                        min_rows: 3,
                    }

                    Textarea {
                        value: use_signal(|| "Contenuto bloccato dal sistema.".to_string()),
                        label: Some("Disabilitato (Disabled)".to_string()),
                        disabled: true,
                        min_rows: 3,
                    }

                    Textarea {
                        value: feedback,
                        label: Some("Segnalazione Bug".to_string()),
                        placeholder: Some("Descrivi il problema riscontrato...".to_string()),
                        error: if feedback().is_empty() { Some("La descrizione dell'anomalia è obbligatoria".to_string()) } else { None },
                        min_rows: 3,
                    }
                }
            }
        }
    }
}
