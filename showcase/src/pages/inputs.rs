use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::components::{FeatureBanner, PropRow, ShowcaseTabs};

#[component]
pub fn InputsPage() -> Element {
    let username = use_signal(|| "christian_shadow".to_string());
    let search_query = use_signal(|| "".to_string());
    let password = use_signal(|| "secret_password".to_string());
    let email = use_signal(|| "".to_string());
    let mut show_password = use_signal(|| false);

    let props = vec![
        PropRow {
            name: "value",
            r#type: "Signal<String>",
            default: "-",
            description: "Signal reattivo bidirezionale Dioxus 0.7 per il valore digitato",
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
            description: "Testo segnaposto informativo visibile a campo vuoto",
        },
        PropRow {
            name: "clearable",
            r#type: "bool",
            default: "false",
            description: "Mostra un pulsante per svuotare il campo con un click",
        },
        PropRow {
            name: "left_icon",
            r#type: "Option<SvgIcon>",
            default: "None",
            description: "Icona SVG a sinistra (modifica automaticamente il padding del testo a 42px evitando collisioni)",
        },
        PropRow {
            name: "right_icon",
            r#type: "Option<SvgIcon>",
            default: "None",
            description: "Icona SVG a destra (modifica automaticamente il padding del testo a 42px)",
        },
        PropRow {
            name: "on_left_icon_click",
            r#type: "Option<EventHandler<MouseEvent>>",
            default: "None",
            description: "Handler di click opzionale per rendere l'icona sinistra un pulsante tattile interattivo",
        },
        PropRow {
            name: "on_right_icon_click",
            r#type: "Option<EventHandler<MouseEvent>>",
            default: "None",
            description: "Handler di click opzionale per rendere l'icona destra un pulsante tattile interattivo (es. mostra/nascondi password)",
        },
        PropRow {
            name: "error",
            r#type: "Option<String>",
            default: "None",
            description: "Messaggio di errore che attiva il bordo rosso e avvisa l'utente",
        },
        PropRow {
            name: "help_text",
            r#type: "Option<String>",
            default: "None",
            description: "Nota o istruzione di supporto visualizzata sotto al campo",
        },
    ];

    let code_snippet = r#"let mut password = use_signal(|| "secret".to_string());
let mut show_password = use_signal(|| false);

TextInput {
    value: password,
    label: "Password".to_string(),
    input_type: if show_password() { "text".to_string() } else { "password".to_string() },
    right_icon: Some(if show_password() { IconKind::EyeOff.into() } else { IconKind::Eye.into() }),
    on_right_icon_click: move |_| show_password.set(!show_password()),
}"#
    .to_string();

    rsx! {
        div { class: "max-w-4xl space-y-8",
            div {
                Heading { order: HeadingOrder::H2, "TextInput" }
                Text { dimmed: true, size: TextSize::Sm, "Campi di testo ad alta precisione con binding reattivo Dioxus 0.7, icone vettoriali SvgIcon premibili (on_left_icon_click / on_right_icon_click) con adattamento spaziale automatico e clear button nativo." }
            }

            FeatureBanner { feature_name: "input" }

            ShowcaseTabs {
                preview: rsx! {
                    div { class: "grid grid-cols-1 lg:grid-cols-2 gap-8 items-start",
                        // Live Inputs
                        div { class: "space-y-4",
                            TextInput {
                                value: username,
                                label: "Nome Utente".to_string(),
                                placeholder: "Inserisci username...".to_string(),
                                clearable: true,
                                left_icon: Some(IconKind::Terminal.into()),
                                help_text: "Identificatore univoco per l'account".to_string(),
                            }

                            TextInput {
                                value: search_query,
                                label: "Ricerca Rapida".to_string(),
                                placeholder: "Filtra componenti...".to_string(),
                                clearable: true,
                                left_icon: Some(IconKind::Search.into()),
                            }

                            TextInput {
                                value: email,
                                label: "Indirizzo Email".to_string(),
                                placeholder: "dev@shadowfenix.org".to_string(),
                                error: if email().is_empty() { Some("L'email è un campo obbligatorio".to_string()) } else { None },
                            }
                        }

                        // State Telemetry
                        div { class: "p-5 rounded-xl bg-[#060608] border border-white/[0.08] space-y-3 font-mono text-xs shadow-inner",
                            p { class: "text-zinc-400 font-semibold uppercase text-[10px] tracking-wider", "Stato Reattivo Signal:" }
                            div { class: "p-3 rounded-lg bg-black/60 border border-white/[0.04] space-y-1.5",
                                div { class: "text-purple-300",
                                    span { class: "text-zinc-500", "username = " }
                                    "\"{username}\""
                                }
                                div { class: "text-cyan-300",
                                    span { class: "text-zinc-500", "search_query = " }
                                    "\"{search_query}\""
                                }
                                div { class: "text-emerald-300",
                                    span { class: "text-zinc-500", "email = " }
                                    "\"{email}\""
                                }
                                div { class: "text-yellow-300",
                                    span { class: "text-zinc-500", "show_password = " }
                                    "{show_password}"
                                }
                            }
                            span { class: "text-[11px] text-zinc-500 block leading-normal",
                                "Ogni modifica digitata e ogni click sull'icona della password aggiorna istantaneamente lo stato reattivo."
                            }
                        }
                    }
                },
                code: code_snippet,
                props: props,
            }

            // Password with toggle example
            Card {
                title: "Esempio Avanzato: Campo Password con Toggle Visibilità".to_string(),
                subtitle: "Supporto nativo a livello libreria per icone premibili reattive con on_right_icon_click".to_string(),
                div { class: "max-w-md space-y-3",
                    TextInput {
                        value: password,
                        label: "Password di Accesso".to_string(),
                        input_type: if show_password() { "text".to_string() } else { "password".to_string() },
                        right_icon: Some(if show_password() { IconKind::EyeOff.into() } else { IconKind::Eye.into() }),
                        on_right_icon_click: move |_| show_password.set(!show_password()),
                        help_text: "Clicca sull'icona dell'occhio a destra per mostrare o nascondere la password in chiaro".to_string(),
                    }
                }
            }
        }
    }
}
