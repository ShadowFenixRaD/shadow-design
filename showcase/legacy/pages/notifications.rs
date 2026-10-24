use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::components::{Stage, FeatureBanner, PageHeader, PropRow, ShowcaseTabs};

#[component]
pub fn NotificationsPage() -> Element {
    let mut notify = use_notification();

    let props = vec![
        PropRow {
            name: "NotificationProvider",
            r#type: "Component",
            default: "-",
            description: "Avvolge l'albero Dioxus e gestisce il rendering e la coda dei toast",
        },
        PropRow {
            name: "use_notification()",
            r#type: "Hook -> NotificationHandle",
            default: "-",
            description: "Fornisce l'handle per mostrare, chiudere o svuotare notifiche da qualsiasi componente",
        },
        PropRow {
            name: "show(msg, variant)",
            r#type: "Method -> usize",
            default: "-",
            description: "Crea una notifica toast con variante specificata e restituisce il suo ID univoco",
        },
        PropRow {
            name: "show_with_title(t, msg, variant)",
            r#type: "Method -> usize",
            default: "-",
            description: "Crea una notifica toast con titolo in evidenza",
        },
        PropRow {
            name: "dismiss(id)",
            r#type: "Method",
            default: "-",
            description: "Rimuove una specifica notifica dalla coda attiva",
        },
        PropRow {
            name: "clear()",
            r#type: "Method",
            default: "-",
            description: "Elimina istantaneamente tutte le notifiche visualizzate a schermo",
        },
    ];

    let code_snippet = r#"// Inizializza l'handle nel componente:
let mut notify = use_notification();

// Lancia notifiche reattive:
Button {
    variant: ButtonVariant::Success,
    onclick: move |_| {
        notify.success("Operazione completata con successo!");
    },
    "Mostra Notifica Success"
}

Button {
    variant: ButtonVariant::Primary,
    onclick: move |_| {
        notify.show_with_title(
            "Snapshot Creato",
            "Il backup del database è stato salvato sul cluster.",
            NotificationVariant::Info
        );
    },
    "Con Titolo"
}"#
    .to_string();

    rsx! {
        div { class: "sc-stack sc-stack--loose",
            PageHeader {
                title: "Notification / Toast System".to_string(),
                description: "Sistema globale di notifiche fluttuanti con animazioni di entrata, icone SVG contestuali, accenti hairline e chiusura manuale o programmata.".to_string(),
            }

            FeatureBanner { feature_name: "notification" }

            ShowcaseTabs {
                preview: rsx! {
                    Stage {
                        span { class: "sc-control-label", "Collauda le notifiche toast in tempo reale" }
                        div { class: "sc-actions",
                            Button {
                                size: ButtonSize::Sm,
                                variant: ButtonVariant::Success,
                                left_icon: rsx! { Icon { kind: IconKind::CheckCircle, size: 14 } },
                                onclick: move |_| {
                                    notify.success("Operazione registrata ed eseguita correttamente.");
                                },
                                "Toast Success"
                            }
                            Button {
                                size: ButtonSize::Sm,
                                variant: ButtonVariant::Primary,
                                left_icon: rsx! { Icon { kind: IconKind::Info, size: 14 } },
                                onclick: move |_| {
                                    notify.info("Nuovo aggiornamento del design system installato.");
                                },
                                "Toast Info"
                            }
                            Button {
                                size: ButtonSize::Sm,
                                variant: ButtonVariant::Outline,
                                left_icon: rsx! { Icon { kind: IconKind::AlertTriangle, size: 14 } },
                                onclick: move |_| {
                                    notify.warning("Attenzione: connessione debole verso il nodo.");
                                },
                                "Toast Warning"
                            }
                            Button {
                                size: ButtonSize::Sm,
                                variant: ButtonVariant::Danger,
                                left_icon: rsx! { Icon { kind: IconKind::XCircle, size: 14 } },
                                onclick: move |_| {
                                    notify.danger("Errore 500: impossibile completare la richiesta.");
                                },
                                "Toast Danger"
                            }
                        }
                        hr { class: "sc-control-divider" }
                        div { class: "sc-actions",
                            Button {
                                size: ButtonSize::Sm,
                                variant: ButtonVariant::Secondary,
                                left_icon: rsx! { Icon { kind: IconKind::Sparkles, size: 14 } },
                                onclick: move |_| {
                                    notify.show_with_title(
                                        "Compilazione Rust OK",
                                        "0 errori e 0 warning in 0.31s con profile release.",
                                        NotificationVariant::Success
                                    );
                                },
                                "Toast con Titolo Strutturato"
                            }
                            Button {
                                size: ButtonSize::Sm,
                                variant: ButtonVariant::Ghost,
                                onclick: move |_| notify.clear(),
                                "Svuota Tutte le Notifiche"
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
