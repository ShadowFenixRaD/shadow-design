use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::components::FeatureBanner;

#[component]
pub fn InputsPage() -> Element {
    let username = use_signal(|| "DioxusDeveloper".to_string());
    let email = use_signal(|| "".to_string());

    rsx! {
        div { class: "max-w-4xl space-y-8",
            div {
                h2 { class: "text-2xl font-bold text-white", "TextInput Component" }
                p { class: "text-zinc-400 text-sm mt-1", "Campi di testo reattivi collegati direttamente ai Signal di Dioxus 0.7." }
            }

            FeatureBanner { feature_name: "input" }

            Card {
                title: "Test Interattivo Reattivo".to_string(),
                shadow: CardShadow::Md,
                div { class: "grid grid-cols-1 md:grid-cols-2 gap-6",
                    div { class: "space-y-4",
                        TextInput {
                            value: username,
                            label: "Nome Utente".to_string(),
                            placeholder: "Inserisci username...".to_string(),
                            help_text: "Nome visibile nel profilo".to_string(),
                        }

                        TextInput {
                            value: email,
                            label: "Email".to_string(),
                            placeholder: "mario.rossi@example.com".to_string(),
                            error: if email().is_empty() { Some("L'email è obbligatoria".to_string()) } else { None },
                        }

                        TextInput {
                            value: use_signal(|| "password123".to_string()),
                            label: "Password Disabilitata".to_string(),
                            input_type: "password".to_string(),
                            disabled: true,
                        }
                    }

                    div { class: "p-4 rounded-xl bg-black border border-purple-950/70 text-sm space-y-2 shadow-inner",
                        p { class: "font-semibold text-zinc-300", "Stato Reattivo in tempo reale:" }
                        div { class: "font-mono text-xs text-purple-400", "username = \"{username}\"" }
                        div { class: "font-mono text-xs text-emerald-400", "email = \"{email}\"" }
                    }
                }
            }
        }
    }
}
