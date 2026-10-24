//! Scene: sign-in with a second factor (REVAMP §1.3). Fields, a PIN input, a labelled divider
//! and a danger alert when the code is wrong.
use dioxus::prelude::*;
use shadow_design::prelude::*;

/// The demo's valid one-time code.
const DEMO_CODE: &str = "424242";

#[component]
pub fn AuthCard(#[props(default)] compact: bool) -> Element {
    let email = use_signal(|| "giulia@shadowfenix.dev".to_string());
    let password = use_signal(|| "correct horse battery".to_string());
    let mut reveal = use_signal(|| false);
    let remember = use_signal(|| true);
    let code = use_signal(String::new);
    let mut wrong = use_signal(|| false);
    let mut toast = use_notification();
    let class = if compact { "sc-auth sc-auth--compact" } else { "sc-auth" };

    rsx! {
        div { class: "{class}",
            Card {
                title: Some("Accedi a Shadowfenix Mesh".to_string()),
                subtitle: Some("Console operatori · sessione cifrata".to_string()),
                Stack { gap: SpacingToken::Md,
                    TextInput {
                        value: email,
                        label: Some("Email".to_string()),
                        input_type: Some("email".to_string()),
                        left_icon: Some(SvgIcon::kind(IconKind::MessageSquare)),
                    }
                    TextInput {
                        value: password,
                        label: Some("Password".to_string()),
                        input_type: Some(if reveal() { "text" } else { "password" }.to_string()),
                        left_icon: Some(SvgIcon::kind(IconKind::Lock)),
                        right_icon: Some(SvgIcon::kind(if reveal() { IconKind::EyeOff } else { IconKind::Eye })),
                        right_icon_label: Some(if reveal() { "Nascondi la password" } else { "Mostra la password" }.to_string()),
                        on_right_icon_click: move |_| reveal.toggle(),
                    }
                    Checkbox { checked: remember, label: Some("Ricordami su questo dispositivo".to_string()) }
                    Divider { label: Some("Secondo fattore".to_string()) }
                    div { class: "sc-auth__pin",
                        span { class: "sc-eyebrow", "Codice dall'app di autenticazione" }
                        PinInput {
                            value: code,
                            length: 6,
                            numeric_only: true,
                            error: wrong(),
                            aria_label: Some("Codice a sei cifre".to_string()),
                            on_change: move |_| wrong.set(false),
                            on_complete: move |v: String| {
                                if v == DEMO_CODE {
                                    toast.success("Accesso verificato");
                                } else {
                                    wrong.set(true);
                                }
                            },
                        }
                        span { class: "sc-hint", "Prova 424242 per il codice corretto." }
                    }
                    if wrong() {
                        Alert { variant: AlertVariant::Danger, title: Some("Codice non valido".to_string()),
                            "Il codice è scaduto o errato. Ne arriva uno nuovo ogni 30 secondi."
                        }
                    }
                    Button {
                        full_width: true,
                        left_icon: Some(rsx! {
                            Icon { kind: IconKind::ShieldCheck, size: 15 }
                        }),
                        onclick: move |_| {
                            toast.info("Inserisci il codice a sei cifre");
                        },
                        "Continua"
                    }
                    Button {
                        variant: ButtonVariant::Secondary,
                        full_width: true,
                        left_icon: Some(rsx! {
                            Icon { kind: IconKind::Fingerprint, size: 15 }
                        }),
                        "Usa una passkey"
                    }
                }
            }
        }
    }
}
