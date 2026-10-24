use dioxus::prelude::*;
use shadow_design::prelude::*;
use std::collections::HashMap;

use crate::components::{FeatureBanner, PageHeader, PropRow, ShowcaseTabs};

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

    // Segnali di campo sincronizzati con il modello del form.
    let mut username = use_signal(|| form.read().username.clone());
    let mut email = use_signal(|| form.read().email.clone());
    use_effect(move || {
        let v = username();
        if form.read().username != v {
            form.with_mut(|m| m.username = v);
            form.clear_error("username");
        }
    });
    use_effect(move || {
        let v = email();
        if form.read().email != v {
            form.with_mut(|m| m.email = v);
            form.clear_error("email");
        }
    });

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

    let err_username = form.get_error("username");
    let err_email = form.get_error("email");

    rsx! {
        div { class: "sc-stack sc-stack--loose",
            PageHeader {
                title: "Form & Validation Engine".to_string(),
                description: "Motore di gestione e validazione reattiva dei form ispirato a Mantine Form, con tracciamento dello stato is_dirty, is_touched, errori per campo e handler di submit typesafe.".to_string(),
            }

            FeatureBanner { feature_name: "form" }

            ShowcaseTabs {
                preview: rsx! {
                    div { class: "sc-stack",
                        div { class: "sc-actions",
                            Badge {
                                label: if is_dirty_val { "Modificato (Dirty)".to_string() } else { "Iniziale (Pristine)".to_string() },
                                variant: if is_dirty_val { BadgeVariant::Primary } else { BadgeVariant::Default },
                            }
                            Badge {
                                label: if is_valid_val { "Nessun Errore".to_string() } else { "Errori di Validazione".to_string() },
                                variant: if is_valid_val { BadgeVariant::Success } else { BadgeVariant::Danger },
                                dot: true,
                            }
                        }

                        Card { class: "w-full max-w-md",
                            div { class: "sc-stack",
                                TextInput {
                                    value: username,
                                    label: "Username *".to_string(),
                                    placeholder: "es. shadow_operator".to_string(),
                                    error: err_username.clone(),
                                }
                                TextInput {
                                    value: email,
                                    label: "Email Aziendale *".to_string(),
                                    placeholder: "es. user@shadowfenix.net".to_string(),
                                    input_type: "email".to_string(),
                                    error: err_email.clone(),
                                }
                                div { class: "sc-actions",
                                    Button {
                                        variant: ButtonVariant::Primary,
                                        onclick: move |_| {
                                            form.submit(|data| {
                                                last_submission.set(Some(data));
                                            });
                                        },
                                        "Invia Modulo (Submit)"
                                    }
                                    Button {
                                        variant: ButtonVariant::Secondary,
                                        onclick: move |_| {
                                            let _ = form.validate();
                                        },
                                        "Valida Campi"
                                    }
                                    Button {
                                        variant: ButtonVariant::Ghost,
                                        onclick: move |_| {
                                            form.reset();
                                            username.set(String::new());
                                            email.set("operatore@shadowfenix.net".to_string());
                                            last_submission.set(None);
                                        },
                                        "Reset"
                                    }
                                }
                            }
                        }

                        if let Some(ref submitted) = last_submission() {
                            Alert {
                                variant: AlertVariant::Success,
                                title: "Modulo Convalidato e Inviato con Successo!".to_string(),
                                "Username: {submitted.username} | Email: {submitted.email}"
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
