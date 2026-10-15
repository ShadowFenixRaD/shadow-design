use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::components::{FeatureBanner, PropRow, ShowcaseTabs};

#[component]
pub fn PinInputPage() -> Element {
    let pin_4 = use_signal(|| "12".to_string());
    let otp_6 = use_signal(String::new);
    let masked_pin = use_signal(|| "4096".to_string());
    let alpha_code = use_signal(|| "A7".to_string());
    let mut complete_msg = use_signal(String::new);

    // Interactive playground controls
    let playground_val = use_signal(|| "13".to_string());
    let mut play_len = use_signal(|| 4usize);
    let mut play_mask = use_signal(|| false);
    let mut play_numeric = use_signal(|| true);
    let mut play_disabled = use_signal(|| false);
    let mut play_error = use_signal(|| false);
    let mut play_size = use_signal(|| ControlSize::Md);

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
        div { class: "max-w-4xl space-y-8",
            div {
                Heading { order: HeadingOrder::H2, "PinInput / InputOTP" }
                Text { dimmed: true, size: TextSize::Sm,
                    "Componente tattile per l'inserimento di codici PIN, token 2FA e OTP a caselle multiple. "
                    "Supporta avanzamento automatico del focus su digitazione, cancellazione con backspace, "
                    "distribuzione rapida di paste/autofill su più celle, mascheramento password e filtro numerico."
                }
            }

            FeatureBanner { feature_name: "pin-input" }

            ShowcaseTabs {
                props: props,
                code: code_snippet,
                preview: rsx! {
                    div { class: "space-y-8",
                        // Live Playground
                        Card {
                            div { class: "p-6 space-y-6",
                                Heading { order: HeadingOrder::H3, "Live Playground" }

                                div { class: "flex flex-wrap gap-4 items-center p-4 bg-zinc-950/60 rounded-lg border border-white/5",
                                    label { class: "flex items-center gap-2 text-sm text-zinc-300",
                                        "Lunghezza:"
                                        select {
                                            class: "bg-zinc-900 text-zinc-100 rounded px-2 py-1 text-sm border border-white/10",
                                            value: "{play_len}",
                                            onchange: move |e| {
                                                if let Ok(l) = e.value().parse::<usize>() {
                                                    play_len.set(l);
                                                }
                                            },
                                            option { value: "4", "4 cifre" }
                                            option { value: "6", "6 cifre" }
                                            option { value: "8", "8 cifre" }
                                        }
                                    }

                                    label { class: "flex items-center gap-2 text-sm text-zinc-300",
                                        "Dimensione:"
                                        select {
                                            class: "bg-zinc-900 text-zinc-100 rounded px-2 py-1 text-sm border border-white/10",
                                            value: match *play_size.read() {
                                                ControlSize::Sm => "sm",
                                                ControlSize::Md => "md",
                                                ControlSize::Lg => "lg",
                                            },
                                            onchange: move |e| {
                                                match e.value().as_str() {
                                                    "sm" => play_size.set(ControlSize::Sm),
                                                    "lg" => play_size.set(ControlSize::Lg),
                                                    _ => play_size.set(ControlSize::Md),
                                                }
                                            },
                                            option { value: "sm", "Small (36px)" }
                                            option { value: "md", "Medium (44px)" }
                                            option { value: "lg", "Large (54px)" }
                                        }
                                    }

                                    label { class: "flex items-center gap-2 text-sm text-zinc-300 cursor-pointer",
                                        input {
                                            r#type: "checkbox",
                                            checked: *play_mask.read(),
                                            onchange: move |e| play_mask.set(e.checked()),
                                        }
                                        "Maschera (password)"
                                    }

                                    label { class: "flex items-center gap-2 text-sm text-zinc-300 cursor-pointer",
                                        input {
                                            r#type: "checkbox",
                                            checked: *play_numeric.read(),
                                            onchange: move |e| play_numeric.set(e.checked()),
                                        }
                                        "Solo Numeri"
                                    }

                                    label { class: "flex items-center gap-2 text-sm text-zinc-300 cursor-pointer",
                                        input {
                                            r#type: "checkbox",
                                            checked: *play_error.read(),
                                            onchange: move |e| play_error.set(e.checked()),
                                        }
                                        "Errore"
                                    }

                                    label { class: "flex items-center gap-2 text-sm text-zinc-300 cursor-pointer",
                                        input {
                                            r#type: "checkbox",
                                            checked: *play_disabled.read(),
                                            onchange: move |e| play_disabled.set(e.checked()),
                                        }
                                        "Disabilitato"
                                    }
                                }

                                div { class: "flex flex-col items-center justify-center p-8 bg-black/40 rounded-xl border border-white/10 gap-4",
                                    PinInput {
                                        value: playground_val,
                                        length: *play_len.read(),
                                        mask: *play_mask.read(),
                                        numeric_only: *play_numeric.read(),
                                        disabled: *play_disabled.read(),
                                        error: *play_error.read(),
                                        size: *play_size.read(),
                                    }

                                    div { class: "text-xs text-zinc-400 font-mono mt-2",
                                        "Valore live: "
                                        span { class: "text-purple-400 font-bold", "\"{playground_val}\"" }
                                    }
                                }
                            }
                        }

                        // Standard Cases
                        div { class: "grid grid-cols-1 md:grid-cols-2 gap-6",
                            Card {
                                div { class: "p-6 space-y-4",
                                    Heading { order: HeadingOrder::H4, "Autenticazione 2FA (6 Cifre)" }
                                    Text { dimmed: true, size: TextSize::Sm,
                                        "Inserisci il codice di sicurezza a 6 cifre ricevuto sul tuo dispositivo authenticator."
                                    }
                                    div { class: "py-2",
                                        PinInput {
                                            value: otp_6,
                                            length: 6,
                                            numeric_only: true,
                                            on_complete: move |val: String| {
                                                complete_msg.set(format!("Codice {} validato con successo!", val));
                                            },
                                        }
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
                                div { class: "p-6 space-y-4",
                                    Heading { order: HeadingOrder::H4, "PIN Mascherato (4 Cifre)" }
                                    Text { dimmed: true, size: TextSize::Sm,
                                        "Ideale per codice bancomat o PIN di sblocco con mascheramento caratteri."
                                    }
                                    div { class: "py-2",
                                        PinInput {
                                            value: masked_pin,
                                            length: 4,
                                            mask: true,
                                            numeric_only: true,
                                            size: ControlSize::Lg,
                                        }
                                    }
                                    div { class: "text-xs text-zinc-500 font-mono",
                                        "Valore memorizzato: {masked_pin}"
                                    }
                                }
                            }

                            Card {
                                div { class: "p-6 space-y-4",
                                    Heading { order: HeadingOrder::H4, "Codice Voucher Alfanumerico" }
                                    Text { dimmed: true, size: TextSize::Sm,
                                        "Accetta sia lettere che numeri per coupon, codici attivazione e ticket."
                                    }
                                    div { class: "py-2",
                                        PinInput {
                                            value: alpha_code,
                                            length: 5,
                                            numeric_only: false,
                                            placeholder: Some("•".to_string()),
                                        }
                                    }
                                    div { class: "text-xs text-zinc-500 font-mono",
                                        "Valore: {alpha_code}"
                                    }
                                }
                            }

                            Card {
                                div { class: "p-6 space-y-4",
                                    Heading { order: HeadingOrder::H4, "Scala Dimensioni (Sm / Md / Lg)" }
                                    div { class: "space-y-4",
                                        div { class: "flex items-center gap-3",
                                            span { class: "w-8 text-xs text-zinc-400", "Sm" }
                                            PinInput {
                                                value: pin_4,
                                                length: 4,
                                                size: ControlSize::Sm,
                                            }
                                        }
                                        div { class: "flex items-center gap-3",
                                            span { class: "w-8 text-xs text-zinc-400", "Md" }
                                            PinInput {
                                                value: pin_4,
                                                length: 4,
                                                size: ControlSize::Md,
                                            }
                                        }
                                        div { class: "flex items-center gap-3",
                                            span { class: "w-8 text-xs text-zinc-400", "Lg" }
                                            PinInput {
                                                value: pin_4,
                                                length: 4,
                                                size: ControlSize::Lg,
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                },
            }
        }
    }
}
