use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::components::{Counter, Playground, FeatureBanner, PageHeader, PropRow, ShowcaseTabs};

#[component]
pub fn TextareaPage() -> Element {
    let bio = use_signal(|| "Ingegnere di sistemi distribuiti ed estimatore di architetture zero-runtime.".to_string());
    let auto_note = use_signal(|| "Prima riga di appunti...\nSeconda riga espansa automaticamente!".to_string());
    let commit_msg = use_signal(|| "feat(core): implement zero-cost abstractions".to_string());
    let feedback = use_signal(String::new);
    let resize_demo_text = use_signal(|| "Trascina la maniglia in basso a destra per ridimensionare questo campo liberamente sia in larghezza che in altezza!".to_string());
    let height_only_text = use_signal(|| "Trascina l'angolo in basso a destra: questo campo consente di modificare solo l'altezza in verticale.".to_string());
    let readonly_text = use_signal(|| "Questo campo è in sola lettura e non può essere modificato.".to_string());
    let disabled_text = use_signal(|| "Contenuto bloccato dal sistema.".to_string());

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
            default: "3 (2 in AutoResizeTextarea)",
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
            name: "resize_width",
            r#type: "bool",
            default: "false",
            description: "Abilita la maniglia di ridimensionamento manuale per la larghezza in basso a destra (resize orizzontale o both)",
        },
        PropRow {
            name: "resize_height",
            r#type: "bool",
            default: "false",
            description: "Abilita la maniglia di ridimensionamento manuale per l'altezza in basso a destra (resize verticale o both)",
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
let mut note_resize = use_signal(String::new);

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
}

// 3. Textarea con ridimensionamento manuale di larghezza e altezza
Textarea {
    value: note_resize,
    label: Some("Note con Resize Manuale".to_string()),
    resize_width: true,
    resize_height: true,
    min_rows: 3,
}"#
    .to_string();

    let bio_chars = bio().chars().count();
    let auto_lines = auto_note().lines().count().max(1);

    rsx! {
        div { class: "sc-stack sc-stack--loose",
            PageHeader {
                title: "Textarea".to_string(),
                description: "Aree di testo multilinea ad alta precisione con espansione verticale dinamica proporzionale al contenuto (AutoResizeTextarea), ridimensionamento manuale configurabile (resize_width, resize_height), conteggio caratteri, cancellazione rapida (clearable) e supporto al tema Mantine.".to_string(),
            }

            FeatureBanner { feature_name: "textarea" }

            ShowcaseTabs {
                preview: rsx! {
                    Playground {
                        stage_start: true,
                        stage: rsx! {
                            div { class: "sc-stack w-full",
                                Textarea {
                                    value: bio,
                                    label: Some("Biografia Utente".to_string()),
                                    placeholder: Some("Scrivi una breve introduzione...".to_string()),
                                    help_text: Some("Massimo 200 caratteri ammessi".to_string()),
                                    show_count: true,
                                    max_length: Some(200),
                                    clearable: true,
                                }
                                AutoResizeTextarea {
                                    value: auto_note,
                                    label: Some("Appunti Live (AutoResizeTextarea)".to_string()),
                                    placeholder: Some("Aggiungi righe per vedere l'espansione automatica dell'altezza...".to_string()),
                                    help_text: Some("Minimo 2 righe, si adatta fedelmente al testo senza spazio morto".to_string()),
                                    min_rows: 2,
                                    max_rows: Some(6),
                                    clearable: true,
                                }
                                Textarea {
                                    value: commit_msg,
                                    label: Some("Messaggio di Commit / Release".to_string()),
                                    min_rows: 2,
                                    show_count: true,
                                    max_length: Some(72),
                                }
                            }
                        },
                        controls: rsx! {
                            span { class: "sc-control-label", "Stato reattivo Signal" }
                            div { class: "sc-stack sc-mono",
                                Counter { label: "bio.len".to_string(), value: format!("{bio_chars} / 200") }
                                Counter { label: "auto_note.lines".to_string(), value: format!("{auto_lines} righe calcolate") }
                                Counter { label: "commit_msg".to_string(), value: format!("\"{}\"", commit_msg()) }
                            }
                            span { class: "sc-page-desc",
                                "La digitazione aggiorna istantaneamente lo stato reattivo e ricalcola le righe visibili in tempo reale senza flickering."
                            }
                        },
                    }
                },
                code: code_snippet,
                props: props,
            }

            // Sezione Ridimensionamento Manuale
            Card {
                title: "Ridimensionamento Manuale (Resize Width & Height)".to_string(),
                subtitle: "Parametri opzionali 'resize_width' e 'resize_height' per abilitare il trascinamento manuale dall'angolo in basso a destra".to_string(),
                div { class: "sc-card-grid",
                    Textarea {
                        value: height_only_text,
                        label: Some("Resize Solo Altezza (resize_height: true)".to_string()),
                        resize_height: true,
                        min_rows: 3,
                    }

                    Textarea {
                        value: resize_demo_text,
                        label: Some("Resize Completo (resize_width: true, resize_height: true)".to_string()),
                        help_text: Some("Trascina l'angolo in basso a destra in orizzontale o in verticale".to_string()),
                        resize_width: true,
                        resize_height: true,
                        min_rows: 3,
                    }
                }
            }

            // Varianti ed Error States
            Card {
                title: "Stati di Validazione ed Eccezioni".to_string(),
                subtitle: "Comportamento visivo con stato disabilitato, sola lettura ed errore di validazione".to_string(),
                div { class: "sc-card-grid sc-card-grid--3",
                    Textarea {
                        value: readonly_text,
                        label: Some("Sola Lettura (Readonly)".to_string()),
                        readonly: true,
                        min_rows: 3,
                    }

                    Textarea {
                        value: disabled_text,
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
