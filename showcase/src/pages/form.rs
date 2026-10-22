use dioxus::prelude::*;
use shadow_design::prelude::*;
use std::collections::HashMap;

use crate::components::{FeatureBanner, PropRow, ShowcaseTabs};

#[derive(Clone, PartialEq, Debug, Default)]
struct UserRegistrationModel {
    username: String,
    email: String,
    role: String,
    bio: String,
}

#[component]
pub fn FormValidationPage() -> Element {
    let form = use_form(
        UserRegistrationModel {
            username: "".to_string(),
            email: "operatore@shadowfenix.net".to_string(),
            role: "Developer".to_string(),
            bio: "Sviluppatore Dioxus specializzato in Rust e architetture UI ad alte prestazioni.".to_string(),
        },
        Some(|vals: &UserRegistrationModel| {
            let mut errors = HashMap::new();
            if vals.username.trim().is_empty() {
                errors.insert("username".to_string(), "Username obbligatorio (almeno 3 caratteri)".to_string());
            } else if vals.username.trim().len() < 3 {
                errors.insert("username".to_string(), "Lo username deve contenere almeno 3 caratteri".to_string());
            }

            if vals.email.trim().is_empty() {
                errors.insert("email".to_string(), "L'indirizzo email non può essere vuoto".to_string());
            } else if !vals.email.contains('@') || !vals.email.contains('.') {
                errors.insert("email".to_string(), "Inserisci un indirizzo email valido (es. user@domain.com)".to_string());
            }

            errors
        }),
    );

    let mut last_submission = use_signal(|| Option::<UserRegistrationModel>::None);

    let props = vec![
        PropRow {
            name: "initial_values",
            r#type: "T: Clone + 'static",
            default: "-",
            description: "Modello dati Rust contenente i campi del form con valori iniziali",
        },
        PropRow {
            name: "validate",
            r#type: "Option<Fn(&T) -> HashMap<String, String>>",
            default: "None",
            description: "Funzione di validazione dichiarativa sincrona o asincrona per campo e modello",
        },
        PropRow {
            name: "form.values",
            r#type: "Signal<T>",
            default: "-",
            description: "Signal reattivo che contiene lo stato corrente dei campi",
        },
        PropRow {
            name: "form.errors",
            r#type: "Signal<HashMap<String, String>>",
            default: "HashMap::new()",
            description: "Mappa reattiva degli errori di validazione per nome campo",
        },
        PropRow {
            name: "form.is_dirty",
            r#type: "Signal<bool>",
            default: "false",
            description: "Indica se almeno un campo è stato modificato rispetto allo stato iniziale",
        },
        PropRow {
            name: "form.submit",
            r#type: "fn(on_valid: FnOnce(T)) -> bool",
            default: "-",
            description: "Esegue prima la validazione: se valida invoca il callback, altrimenti blocca e popola gli errori",
        },
    ];

    let code_snippet = r#"let mut form = use_form(
    UserModel { username: "".to_string(), email: "".to_string() },
    Some(|values: &UserModel| {
        let mut errs = HashMap::new();
        if values.username.is_empty() {
            errs.insert("username".to_string(), "Campo obbligatorio".to_string());
        }
        errs
    }),
);

// Nel pulsante submit
Button {
    variant: ButtonVariant::Primary,
    on_click: move |_| {
        form.submit(|data| {
            println!("Form valido: {:?}", data);
        });
    },
    "Invia Form"
}"#
    .to_string();

    let is_dirty_val = *form.is_dirty.read();
    let is_valid_val = form.is_valid();
    let current_values = form.read();

    let err_username = form.get_error("username");
    let err_email = form.get_error("email");

    rsx! {
        div { class: "max-w-4xl space-y-8",
            div {
                Heading { order: HeadingOrder::H2, "Form & Validation Engine" }
                Text { dimmed: true, size: TextSize::Sm, "Motore di gestione e validazione reattiva dei form ispirato a Mantine Form, con tracciamento dello stato is_dirty, is_touched, errori per campo e handler di submit typesafe." }
            }

            FeatureBanner { feature_name: "form" }

            ShowcaseTabs {
                preview: rsx! {
                    div { class: "p-8 rounded-xl bg-[#060608] border border-white/[0.08] flex flex-col gap-6 shadow-inner",
                        // Telemetria di stato del form
                        div { class: "flex items-center justify-between text-xs font-mono text-zinc-400 p-3 rounded-lg bg-white/[0.02] border border-white/[0.06] flex-wrap gap-3",
                            div { class: "flex items-center gap-3",
                                span { "Stato Form:" }
                                span {
                                    class: if is_dirty_val { "text-purple-300 font-semibold" } else { "text-zinc-500" },
                                    if is_dirty_val { "Modificato (Dirty)" } else { "Iniziale (Pristine)" }
                                }
                            }
                            div { class: "flex items-center gap-3",
                                span { "Validità:" }
                                span {
                                    class: if is_valid_val { "text-emerald-400 font-semibold" } else { "text-rose-400 font-semibold" },
                                    if is_valid_val { "Nessun Errore" } else { "Errori di Validazione" }
                                }
                            }
                        }

                        // Form body
                        div { class: "max-w-md mx-auto w-full space-y-5",
                            div { class: "space-y-1.5",
                                label { class: "text-xs font-mono text-zinc-400 flex justify-between",
                                    span { "Username *" }
                                    if let Some(ref err) = err_username {
                                        span { class: "text-rose-400 text-[11px]", "{err}" }
                                    }
                                }
                                input {
                                    r#type: "text",
                                    value: "{current_values.username}",
                                    placeholder: "es. shadow_operator",
                                    class: format!("w-full px-3 py-2 text-xs font-mono rounded-lg border bg-zinc-950/80 text-zinc-200 transition-all focus:outline-none focus:ring-1 {}", if err_username.is_some() { "border-rose-500/80 focus:ring-rose-400" } else { "border-white/[0.1] focus:ring-purple-400" }),
                                    oninput: move |e| {
                                        let val = e.value();
                                        form.with_mut(|m| m.username = val);
                                        form.clear_error("username");
                                    },
                                }
                            }

                            div { class: "space-y-1.5",
                                label { class: "text-xs font-mono text-zinc-400 flex justify-between",
                                    span { "Email Aziendale *" }
                                    if let Some(ref err) = err_email {
                                        span { class: "text-rose-400 text-[11px]", "{err}" }
                                    }
                                }
                                input {
                                    r#type: "email",
                                    value: "{current_values.email}",
                                    placeholder: "es. user@shadowfenix.net",
                                    class: format!("w-full px-3 py-2 text-xs font-mono rounded-lg border bg-zinc-950/80 text-zinc-200 transition-all focus:outline-none focus:ring-1 {}", if err_email.is_some() { "border-rose-500/80 focus:ring-rose-400" } else { "border-white/[0.1] focus:ring-purple-400" }),
                                    oninput: move |e| {
                                        let val = e.value();
                                        form.with_mut(|m| m.email = val);
                                        form.clear_error("email");
                                    },
                                }
                            }

                            div { class: "flex items-center gap-3 pt-2",
                                button {
                                    r#type: "button",
                                    class: "px-4 py-2 text-xs font-mono font-semibold rounded-lg bg-purple-950/80 text-purple-200 border border-purple-500/50 hover:bg-purple-900 transition-all cursor-pointer shadow-[0_0_12px_rgba(168,85,247,0.25)]",
                                    onclick: move |_| {
                                        form.submit(|data| {
                                            last_submission.set(Some(data));
                                        });
                                    },
                                    "Invia Modulo (Submit)"
                                }

                                button {
                                    r#type: "button",
                                    class: "px-3 py-2 text-xs font-mono rounded-lg bg-white/[0.04] text-zinc-400 border border-white/[0.06] hover:bg-white/[0.08] transition-all cursor-pointer",
                                    onclick: move |_| {
                                        let _ = form.validate();
                                    },
                                    "Valida Campi"
                                }

                                button {
                                    r#type: "button",
                                    class: "px-3 py-2 text-xs font-mono rounded-lg bg-white/[0.04] text-zinc-400 border border-white/[0.06] hover:bg-white/[0.08] transition-all cursor-pointer ml-auto",
                                    onclick: move |_| {
                                        form.reset();
                                        last_submission.set(None);
                                    },
                                    "Reset"
                                }
                            }

                            if let Some(ref submitted) = last_submission() {
                                div { class: "p-4 rounded-lg border border-emerald-500/50 bg-emerald-950/30 text-emerald-200 text-xs font-mono space-y-1 shadow-[0_0_14px_rgba(16,185,129,0.2)] animate-fade-in",
                                    div { class: "font-semibold text-emerald-300 flex items-center gap-2",
                                        Icon { kind: IconKind::CheckCircle, size: 14 }
                                        "Modulo Convalidato e Inviato con Successo!"
                                    }
                                    div { class: "text-zinc-400 text-[11px]", "Username: {submitted.username} | Email: {submitted.email}" }
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
