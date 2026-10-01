use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::components::{FeatureBanner, PropRow, ShowcaseTabs};

#[component]
pub fn AlertsPage() -> Element {
    let mut selected_variant = use_signal(|| AlertVariant::Info);
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
        div { class: "max-w-4xl space-y-8",
            div {
                Heading { order: HeadingOrder::H2, "Alert" }
                Text { dimmed: true, size: TextSize::Sm, "Riquadri di avviso e notifica con icone SVG vettoriali native, bordo laterale di accento e supporto a chiusura dismissibile." }
            }

            FeatureBanner { feature_name: "alert" }

            ShowcaseTabs {
                preview: rsx! {
                    div { class: "space-y-4",
                        if alert_visible() {
                            Alert {
                                title: "Notifica di Sistema".to_string(),
                                variant: selected_variant(),
                                on_close: move |_| alert_visible.set(false),
                                "Tutti i moduli dell'ecosistema shadow-design sono configurati e operativi con feature flags indipendenti."
                            }
                        } else {
                            div { class: "p-4 rounded-xl bg-[#060608] border border-white/[0.08] flex items-center justify-between text-xs text-zinc-400 font-mono",
                                span { "L'alert è stato chiuso." }
                                Button {
                                    size: ButtonSize::Xs,
                                    variant: ButtonVariant::Secondary,
                                    onclick: move |_| alert_visible.set(true),
                                    "Ripristina Alert"
                                }
                            }
                        }

                        div { class: "flex flex-wrap gap-1.5 pt-2 border-t border-white/[0.06] text-xs font-mono items-center",
                            span { class: "text-zinc-400 mr-2 uppercase text-[10px]", "Cambia Variante:" }
                            for v in [AlertVariant::Info, AlertVariant::Success, AlertVariant::Warning, AlertVariant::Danger] {
                                button {
                                    r#type: "button",
                                    class: if selected_variant() == v {
                                        "px-2.5 py-1 rounded bg-purple-950/80 text-purple-200 border border-purple-500/40 font-bold cursor-pointer shadow-sm"
                                    } else {
                                        "px-2.5 py-1 rounded bg-white/[0.03] hover:bg-white/[0.08] text-zinc-400 hover:text-white border border-white/[0.06] cursor-pointer transition-colors"
                                    },
                                    onclick: move |_| selected_variant.set(v),
                                    "{v:?}"
                                }
                            }
                        }
                    }
                },
                code: code_snippet,
                props: props,
            }

            // Visual Catalog
            div { class: "space-y-3",
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
