use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::components::FeatureBanner;

#[component]
pub fn ModalsPage() -> Element {
    let mut is_modal_open = use_signal(|| false);

    rsx! {
        div { class: "max-w-4xl space-y-8",
            div {
                h2 { class: "text-2xl font-bold text-white", "Modal Component" }
                p { class: "text-zinc-400 text-sm mt-1", "Finestre di dialogo sovrapposte con backdrop oscurato e chiusura controllata da Signal." }
            }

            FeatureBanner { feature_name: "modal" }

            Card {
                title: "Test Apertura Modale".to_string(),
                shadow: CardShadow::Md,
                div { class: "space-y-4",
                    p { class: "text-sm text-zinc-300", "Clicca sul pulsante sottostante per aprire la finestra modale:" }
                    Button {
                        variant: ButtonVariant::Primary,
                        onclick: move |_| *is_modal_open.write() = true,
                        "Apri Finestra Modale"
                    }
                }
            }

            Modal {
                is_open: is_modal_open,
                title: "Esempio Modale Shadow Design".to_string(),
                div { class: "space-y-4 text-sm text-zinc-300",
                    p { "Questa finestra modale è completamente integrata con i Signal di Dioxus 0.7 e chiudibile sia dalla X sia da azioni personalizzate." }
                    div { class: "flex justify-end gap-3 pt-3",
                        Button {
                            variant: ButtonVariant::Secondary,
                            onclick: move |_| *is_modal_open.write() = false,
                            "Annulla"
                        }
                        Button {
                            variant: ButtonVariant::Primary,
                            onclick: move |_| *is_modal_open.write() = false,
                            "Conferma"
                        }
                    }
                }
            }
        }
    }
}
