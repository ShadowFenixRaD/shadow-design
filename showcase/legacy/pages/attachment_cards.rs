use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::components::{
    CheckControl, ChoicePicker, ControlGroup, FeatureBanner, PageHeader, Playground, PropRow, ShowcaseTabs,
};

#[component]
pub fn AttachmentCardsPage() -> Element {
    let is_file_mode = use_signal(|| true);
    let with_progress = use_signal(|| true);
    let progress_val = use_signal(|| 74.0f64);
    let with_description = use_signal(|| true);
    let with_actions = use_signal(|| true);

    let props = vec![
        PropRow {
            name: "title",
            r#type: "String",
            default: "-",
            description: "Nome del file o dell'utente visualizzato nell'intestazione destra",
        },
        PropRow {
            name: "subtitle",
            r#type: "Option<String>",
            default: "None",
            description: "Dimensione file (es. '4.2 MB • Cifrato'), estensione o ruolo utente",
        },
        PropRow {
            name: "description",
            r#type: "Option<String>",
            default: "None",
            description: "Descrizione testuale opzionale collocata al di sotto dell'intestazione",
        },
        PropRow {
            name: "avatar",
            r#type: "Option<Element>",
            default: "None",
            description: "Avatar tondo sulla sinistra (supporta SvgIcon, immagine o Avatar nativo)",
        },
        PropRow {
            name: "progress",
            r#type: "Option<f32>",
            default: "None",
            description: "Avanzamento di trasferimento file (0.0..100.0) con ProgressBar integrata",
        },
        PropRow {
            name: "on_download",
            r#type: "Option<EventHandler<MouseEvent>>",
            default: "None",
            description: "Handler per il pulsante 'Scarica' con icona download integrata",
        },
        PropRow {
            name: "on_share",
            r#type: "Option<EventHandler<MouseEvent>>",
            default: "None",
            description: "Handler per il pulsante 'Condividi' con icona share integrata",
        },
    ];

    let code_snippet = r#"AttachmentCard {
    title: "the_mind_genesis_block.enc".to_string(),
    subtitle: Some("14.8 MB • Kyber-1024".to_string()),
    description: Some("Snapshot ledger cifrato con verifica di integrità SHA3.".to_string()),
    progress: Some(74.0),
    on_download: move |_| println!("Scaricamento avviato..."),
    on_share: move |_| println!("Condivisione avviata..."),
}"#
    .to_string();

    rsx! {
        div { class: "sc-stack sc-stack--loose",
            PageHeader {
                title: "AttachmentCard".to_string(),
                description: "Scheda per allegati multimediali, documenti e profili in chat con avatar circolare a sinistra, intestazione a destra, e area azioni/descrizione inferiore.".to_string(),
            }

            FeatureBanner { feature_name: "attachment_card" }

            ShowcaseTabs {
                preview: rsx! {
                    Playground {
                        stage: rsx! {
                            if is_file_mode() {
                                AttachmentCard {
                                    title: "the_mind_genesis_block.enc".to_string(),
                                    subtitle: Some("14.8 MB • Cifratura PQ".to_string()),
                                    avatar_icon: Some(IconKind::Lock.into()),
                                    progress: if with_progress() { Some(progress_val() as f32) } else { None },
                                    description: if with_description() { Some("Snapshot del ledger locale sincronizzato dai nodi subordinati.".to_string()) } else { None },
                                    on_download: if with_actions() { Some(EventHandler::new(|_| {})) } else { None },
                                    on_share: if with_actions() { Some(EventHandler::new(|_| {})) } else { None },
                                }
                            } else {
                                ContactCard {
                                    name: "Doctor Satoshi (Node #01)".to_string(),
                                    subtitle: Some("Nodo Creatore Mesh • 4ms".to_string()),
                                    avatar: Some(rsx! {
                                        Avatar {
                                            name: Some("Doctor Satoshi".to_string()),
                                            size: AvatarSize::Md,
                                            status: PresenceStatus::DirectP2P,
                                        }
                                    }),
                                    description: if with_description() { Some("Chiave pubblica post-quantum verificata sulla rete beacon.".to_string()) } else { None },
                                }
                            }
                        },
                        controls: rsx! {
                            ChoicePicker {
                                label: "Modalità Scheda".to_string(),
                                options: vec![
                                    (true, "File / Documento".to_string()),
                                    (false, "Utente / Contatto".to_string()),
                                ],
                                value: is_file_mode,
                            }
                            hr { class: "sc-control-divider" }
                            CheckControl { label: "Mostra Descrizione Inferiore".to_string(), checked: with_description }
                            CheckControl { label: "Mostra Bottoni Azione (Scarica / Condividi)".to_string(), checked: with_actions }
                            CheckControl { label: "Integra ProgressBar di Trasferimento".to_string(), checked: with_progress }
                            if with_progress() {
                                ControlGroup { label: "Avanzamento file: {progress_val():.0}%".to_string(),
                                    Slider { value: progress_val, min: 0.0, max: 100.0, step: 1.0 }
                                }
                            }
                        },
                    }
                },
                code: code_snippet,
                props: props,
            }
        }
    }
}
