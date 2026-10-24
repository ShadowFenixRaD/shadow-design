use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::components::{
    ChoicePicker, FeatureBanner, PageHeader, Playground, PropRow, ShowcaseTabs,
};

#[component]
pub fn AlertsPage() -> Element {
    let selected_variant = use_signal(|| AlertVariant::Info);
    let mut alert_visible = use_signal(|| true);

    let props = vec![
        PropRow {
            name: "title",
            r#type: "Option<String>",
            default: "None",
            description: "Intestazione in grassetto evidenziata con il colore della variante",
        },
        PropRow {
            name: "variant",
            r#type: "AlertVariant",
            default: "AlertVariant::Info",
            description: "Variante semantica (Info, Success, Warning, Danger)",
        },
        PropRow {
            name: "on_close",
            r#type: "Option<EventHandler<MouseEvent>>",
            default: "None",
            description: "Se specificato, mostra un pulsante X per chiudere o nascondere l'alert",
        },
    ];

    let code_snippet = r#"Alert {
    title: "Aggiornamento Disponibile".to_string(),
    variant: AlertVariant::Info,
    on_close: move |_| println!("Alert chiuso!"),
    "La versione 0.7.1 di Dioxus è ora supportata."
}"#
    .to_string();

    rsx! {
        div { class: "sc-stack sc-stack--loose",
            PageHeader {
                title: "Alert".to_string(),
                description: "Riquadri di avviso e notifica con icone SVG vettoriali native, bordo laterale di accento e supporto a chiusura dismissibile.".to_string(),
            }

            FeatureBanner { feature_name: "alert" }

            ShowcaseTabs {
                preview: rsx! {
                    Playground {
                        stage_start: true,
                        stage: rsx! {
                            if alert_visible() {
                                Alert {
                                    title: "Notifica di Sistema".to_string(),
                                    variant: selected_variant(),
                                    on_close: move |_| alert_visible.set(false),
                                    "Tutti i moduli dell'ecosistema shadow-design sono configurati e operativi con feature flags indipendenti."
                                }
                            } else {
                                div { class: "sc-title-row w-full",
                                    span { class: "sc-muted-text", "L'alert è stato chiuso." }
                                    Button {
                                        size: ButtonSize::Xs,
                                        variant: ButtonVariant::Secondary,
                                        onclick: move |_| alert_visible.set(true),
                                        "Ripristina Alert"
                                    }
                                }
                            }
                        },
                        controls: rsx! {
                            ChoicePicker {
                                label: "Variante".to_string(),
                                options: vec![
                                    (AlertVariant::Info, "Info".to_string()),
                                    (AlertVariant::Success, "Success".to_string()),
                                    (AlertVariant::Warning, "Warning".to_string()),
                                    (AlertVariant::Danger, "Danger".to_string()),
                                ],
                                value: selected_variant,
                            }
                        },
                    }
                },
                code: code_snippet,
                props: props,
            }

            // Visual Catalog
            div { class: "sc-stack",
                Alert {
                    title: "Informazione di Sistema (Info)".to_string(),
                    variant: AlertVariant::Info,
                    "La suite include componenti nativi senza alcuna dipendenza CSS pesante o icone esterne."
                }
                Alert {
                    title: "Operazione Riuscita (Success)".to_string(),
                    variant: AlertVariant::Success,
                    "Compilazione workspace completata con zero errori in 0.84s."
                }
                Alert {
                    title: "Avviso Importante (Warning)".to_string(),
                    variant: AlertVariant::Warning,
                    "Assicurati di non trattenere lock generazionali attraverso chiamate .await asincrone."
                }
                Alert {
                    title: "Errore di Rete (Danger)".to_string(),
                    variant: AlertVariant::Danger,
                    "Impossibile raggiungere l'endpoint specificato. Riprova tra alcuni secondi."
                }
            }
        }
    }
}
