use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::components::{Counter, FeatureBanner, PageHeader, PropRow, ShowcaseTabs, Stage};

#[component]
pub fn AlertDialogPage() -> Element {
    let mut is_danger_open = use_signal(|| false);
    let mut is_info_open = use_signal(|| false);
    let mut feedback = use_signal(|| "In attesa di azioni utente...".to_string());

    let props = vec![
        PropRow {
            name: "opened",
            r#type: "Signal<bool>",
            default: "-",
            description: "Signal reattivo che controlla la visibilità della finestra di conferma",
        },
        PropRow {
            name: "title",
            r#type: "String",
            default: "-",
            description: "Titolo di allerta dell'operazione",
        },
        PropRow {
            name: "description",
            r#type: "String",
            default: "-",
            description: "Spiegazione chiara dell'azione e delle relative conseguenze",
        },
        PropRow {
            name: "confirm_label",
            r#type: "String",
            default: "\"Conferma\"",
            description: "Testo visualizzato sul pulsante primario di conferma",
        },
        PropRow {
            name: "cancel_label",
            r#type: "String",
            default: "\"Annulla\"",
            description: "Testo visualizzato sul pulsante secondario di annullamento",
        },
        PropRow {
            name: "is_destructive",
            r#type: "bool",
            default: "true",
            description: "Se true, applica la palette cremisi di pericolo e l'icona di allerta",
        },
        PropRow {
            name: "on_confirm",
            r#type: "Option<EventHandler<()>>",
            default: "None",
            description: "Callback scatenato alla pressione del pulsante di conferma",
        },
    ];

    let code_snippet = r#"let mut opened = use_signal(|| false);

Button {
    variant: ButtonVariant::Danger,
    onclick: move |_| opened.set(true),
    "Elimina Repository"
}

AlertDialog {
    opened: opened,
    title: "Sei assolutamente sicuro?".to_string(),
    description: "Questa azione non può essere annullata. La repository e tutti i branch associati verranno rimossi permanentemente dai server.".to_string(),
    confirm_label: "Elimina definitivamente".to_string(),
    cancel_label: "Annulla operazione".to_string(),
    is_destructive: true,
    on_confirm: move |_| println!("Repository eliminata!"),
}"#
    .to_string();

    rsx! {
        div { class: "sc-stack sc-stack--loose",
            PageHeader {
                title: "AlertDialog / ConfirmDialog".to_string(),
                description: "Finestra di dialogo bloccante con backdrop e focus trapping per conferme critiche e distruttive o prompt decisionali irreversibili.".to_string(),
            }

            FeatureBanner { feature_name: "alert_dialog" }

            ShowcaseTabs {
                preview: rsx! {
                    Stage {
                        p { class: "sc-muted-text",
                            "Scegli una delle azioni sottostanti per verificare il comportamento dell'AlertDialog in modalità distruttiva o informativa."
                        }

                        div { class: "sc-actions",
                            Button {
                                variant: ButtonVariant::Danger,
                                left_icon: rsx! { Icon { kind: IconKind::Trash2, size: 15 } },
                                onclick: move |_| is_danger_open.set(true),
                                "Azione Distruttiva (Danger)"
                            }
                            Button {
                                variant: ButtonVariant::Primary,
                                left_icon: rsx! { Icon { kind: IconKind::CheckCircle, size: 15 } },
                                onclick: move |_| is_info_open.set(true),
                                "Azione Ordinaria (Confirm)"
                            }
                        }

                        Counter { label: "Feedback".to_string(), value: feedback() }
                    }
                },
                code: code_snippet,
                props: props,
            }

            // Dialog Distruttivo
            AlertDialog {
                opened: is_danger_open,
                title: "Eliminare definitivamente la risorsa?".to_string(),
                description: "Questa azione è irreversibile. Tutti i dati associati, compresi i backup e le credenziali crittografiche, verranno cancellati in modo permanente dai nodi del cluster.".to_string(),
                confirm_label: "Elimina per sempre".to_string(),
                cancel_label: "Annulla".to_string(),
                is_destructive: true,
                on_confirm: move |_| feedback.set("Confermata eliminazione risorsa!".to_string()),
                on_cancel: move |_| feedback.set("Operazione di eliminazione annullata.".to_string()),
            }

            // Dialog di Conferma Standard
            ConfirmDialog {
                opened: is_info_open,
                title: "Pubblicare nuova release v0.5.0?".to_string(),
                description: "La nuova versione del pacchetto shadow-design verrà inviata a crates.io e la documentazione docs.rs verrà generata automaticamente.".to_string(),
                confirm_label: "Pubblica Release".to_string(),
                cancel_label: "Modifica ancora".to_string(),
                is_destructive: false,
                icon: Some(IconKind::CheckCircle),
                on_confirm: move |_| feedback.set("Release v0.5.0 pubblicata con successo!".to_string()),
                on_cancel: move |_| feedback.set("Pubblicazione rimandata.".to_string()),
            }
        }
    }
}
