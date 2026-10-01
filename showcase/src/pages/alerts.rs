use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::components::FeatureBanner;

#[component]
pub fn AlertsPage() -> Element {
    rsx! {
        div { class: "max-w-4xl space-y-8",
            div {
                h2 { class: "text-2xl font-bold text-white", "Alert Component" }
                p { class: "text-zinc-400 text-sm mt-1", "Messaggi di notifica in evidenza per comunicazioni critiche o informative." }
            }

            FeatureBanner { feature_name: "alert" }

            div { class: "space-y-4",
                Alert {
                    title: "Informazione di Sistema".to_string(),
                    variant: AlertVariant::Info,
                    "Il modulo UI è configurato come workspace cargo indipendente."
                }
                Alert {
                    title: "Operazione Riuscita".to_string(),
                    variant: AlertVariant::Success,
                    "Tutti i componenti sono pronti per essere importati nei siti web."
                }
                Alert {
                    title: "Attenzione".to_string(),
                    variant: AlertVariant::Warning,
                    "Assicurati di abilitare solo le feature strettamente necessarie per minimizzare il bundle."
                }
                Alert {
                    title: "Errore Connessione".to_string(),
                    variant: AlertVariant::Danger,
                    "Impossibile sincronizzare la risorsa remota."
                }
            }
        }
    }
}
