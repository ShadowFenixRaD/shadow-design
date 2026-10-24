use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::components::{
    ChoicePicker, ControlRow, Counter, FeatureBanner, PageHeader, Playground, PropRow, ShowcaseTabs,
    ToggleControl,
};

#[component]
pub fn PinInputPage() -> Element {
    let pin_4 = use_signal(|| "12".to_string());
    let otp_6 = use_signal(String::new);
    let masked_pin = use_signal(|| "4096".to_string());
    let alpha_code = use_signal(|| "A7".to_string());
    let mut complete_msg = use_signal(String::new);

    // Interactive playground controls
    let playground_val = use_signal(|| "13".to_string());
    let play_len = use_signal(|| 4usize);
    let play_mask = use_signal(|| false);
    let play_numeric = use_signal(|| true);
    let play_disabled = use_signal(|| false);
    let play_error = use_signal(|| false);
    let play_size = use_signal(|| ControlSize::Md);

    let props = vec![
        PropRow {
            name: "value",
            r#type: "Signal<String>",
            default: "-",
            description: "Signal reattivo stringa associato al valore complessivo del codice PIN/OTP",
        },
        PropRow {
            name: "length",
            r#type: "usize",
            default: "4",
            description: "Numero di caselle numeriche/alfanumeriche individuali da renderizzare (es. 4 o 6)",
        },
        PropRow {
            name: "mask",
            r#type: "bool",
            default: "false",
            description: "Se true, maschera i caratteri digitati stile password per la massima riservatezza",
        },
        PropRow {
            name: "numeric_only",
            r#type: "bool",
            default: "false",
            description: "Filtra rigorosamente l'input accettando unicamente cifre numeriche (0-9)",
        },
        PropRow {
            name: "size",
            r#type: "ControlSize",
            default: "ControlSize::Md",
            description: "Dimensione geometrica delle singole celle: Sm (36px), Md (44px), Lg (54px)",
        },
        PropRow {
            name: "disabled",
            r#type: "bool",
            default: "false",
            description: "Disabilita l'interazione e la modifica delle celle",
        },
        PropRow {
            name: "error",
            r#type: "bool",
            default: "false",
            description: "Evidenzia lo stato di errore con bordo perimetrale cremisi e glow di allerta",
        },
        PropRow {
            name: "placeholder",
            r#type: "Option<String>",
            default: "Some(\"○\")",
            description: "Carattere segnaposto visualizzato nelle celle vuote",
        },
        PropRow {
            name: "gap",
            r#type: "Option<String>",
            default: "None",
            description: "Spaziatura CSS orizzontale personalizzata tra le singole caselle",
        },
        PropRow {
            name: "auto_focus",
            r#type: "bool",
            default: "false",
            description: "Attiva automaticamente il focus sulla prima casella al montaggio del componente",
        },
        PropRow {
            name: "on_complete",
            r#type: "Option<EventHandler<String>>",
            default: "None",
            description: "Callback invocato non appena tutte le caselle sono state completate",
        },
        PropRow {
            name: "on_change",
            r#type: "Option<EventHandler<String>>",
            default: "None",
            description: "Callback invocato ad ogni digitazione, cancellazione o modifica del valore",
        },
    ];

    let code_snippet = r#"let mut otp_code = use_signal(String::new);

PinInput {
    value: otp_code,
    length: 6,
    numeric_only: true,
    mask: false,
    size: ControlSize::Md,
    auto_focus: true,
    on_complete: move |code: String| {
        println!("Codice OTP completato: {}", code);
    },
}"#
    .to_string();

    rsx! {
        div { class: "sc-stack sc-stack--loose",
            PageHeader {
                title: "PinInput / InputOTP".to_string(),
                description: "Inserimento di codici PIN, token 2FA e OTP a caselle multiple, con avanzamento automatico del focus, backspace, paste/autofill su più celle, mascheramento e filtro numerico.".to_string(),
            }

            FeatureBanner { feature_name: "pin_input" }

            ShowcaseTabs {
                props: props,
                code: code_snippet,
                preview: rsx! {
                    div { class: "sc-stack sc-stack--loose",
                        Playground {
                            stage: rsx! {
                                div { class: "sc-stack items-center",
                                    PinInput {
                                        value: playground_val,
                                        length: play_len(),
                                        mask: play_mask(),
                                        numeric_only: play_numeric(),
                                        disabled: play_disabled(),
                                        error: play_error(),
                                        size: play_size(),
                                    }
                                    Counter { label: "Valore live".to_string(), value: format!("\"{}\"", playground_val()) }
                                }
                            },
                            controls: rsx! {
                                ChoicePicker {
                                    label: "Lunghezza".to_string(),
                                    options: vec![
                                        (4usize, "4 cifre".to_string()),
                                        (6usize, "6 cifre".to_string()),
                                        (8usize, "8 cifre".to_string()),
                                    ],
                                    value: play_len,
                                }
                                ChoicePicker {
                                    label: "Dimensione".to_string(),
                                    options: vec![
                                        (ControlSize::Sm, "Sm".to_string()),
                                        (ControlSize::Md, "Md".to_string()),
                                        (ControlSize::Lg, "Lg".to_string()),
                                    ],
                                    value: play_size,
                                }
                                hr { class: "sc-control-divider" }
                                ControlRow {
                                    ToggleControl { label: "Maschera (password)".to_string(), checked: play_mask }
                                    ToggleControl { label: "Solo numeri".to_string(), checked: play_numeric }
                                    ToggleControl { label: "Errore".to_string(), checked: play_error }
                                    ToggleControl { label: "Disabilitato".to_string(), checked: play_disabled }
                                }
                            },
                        }

                        div { class: "sc-card-grid",
                            Card {
                                title: "Autenticazione 2FA (6 cifre)".to_string(),
                                subtitle: "Inserisci il codice di sicurezza a 6 cifre ricevuto sul tuo dispositivo authenticator.".to_string(),
                                div { class: "sc-stack",
                                    PinInput {
                                        value: otp_6,
                                        length: 6,
                                        numeric_only: true,
                                        on_complete: move |val: String| {
                                            complete_msg.set(format!("Codice {} validato con successo!", val));
                                        },
                                    }
                                    if !complete_msg.read().is_empty() {
                                        Alert {
                                            variant: AlertVariant::Success,
                                            title: "Verifica Riuscita".to_string(),
                                            "{complete_msg}"
                                        }
                                    }
                                }
                            }

                            Card {
                                title: "PIN mascherato (4 cifre)".to_string(),
                                subtitle: "Ideale per codice bancomat o PIN di sblocco con mascheramento caratteri.".to_string(),
                                div { class: "sc-stack",
                                    PinInput {
                                        value: masked_pin,
                                        length: 4,
                                        mask: true,
                                        numeric_only: true,
                                        size: ControlSize::Lg,
                                    }
                                    Counter { label: "Valore memorizzato".to_string(), value: masked_pin() }
                                }
                            }

                            Card {
                                title: "Codice voucher alfanumerico".to_string(),
                                subtitle: "Accetta sia lettere che numeri per coupon, codici attivazione e ticket.".to_string(),
                                div { class: "sc-stack",
                                    PinInput {
                                        value: alpha_code,
                                        length: 5,
                                        numeric_only: false,
                                        placeholder: Some("•".to_string()),
                                    }
                                    Counter { label: "Valore".to_string(), value: alpha_code() }
                                }
                            }

                            Card { title: "Scala dimensioni (Sm / Md / Lg)".to_string(),
                                div { class: "sc-stack",
                                    PinInput { value: pin_4, length: 4, size: ControlSize::Sm }
                                    PinInput { value: pin_4, length: 4, size: ControlSize::Md }
                                    PinInput { value: pin_4, length: 4, size: ControlSize::Lg }
                                }
                            }
                        }
                    }
                },
            }
        }
    }
}
