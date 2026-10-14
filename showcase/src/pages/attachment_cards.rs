use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::components::{FeatureBanner, PropRow, ShowcaseTabs};

#[component]
pub fn AttachmentCardsPage() -> Element {
    let mut is_file_mode = use_signal(|| true);
    let with_progress = use_signal(|| true);
    let mut progress_val = use_signal(|| 74.0f32);
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
        div { class: "max-w-4xl space-y-8",
            div {
                Heading { order: HeadingOrder::H2, "AttachmentCard" }
                Text { dimmed: true, size: TextSize::Sm, "Scheda per allegati multimediali, documenti e profili in chat con avatar circolare a sinistra, intestazione a destra, e area azioni/descrizione inferiore." }
            }

            FeatureBanner { feature_name: "attachment_card" }

            ShowcaseTabs {
                preview: rsx! {
                    div { class: "grid grid-cols-1 lg:grid-cols-2 gap-8 items-center",
                        // Live Preview
                        div { class: "p-8 rounded-xl bg-[#060608] border border-white/[0.08] flex flex-col items-center justify-center min-h-[220px] shadow-inner",
                            if is_file_mode() {
                                AttachmentCard {
                                    title: "the_mind_genesis_block.enc".to_string(),
                                    subtitle: Some("14.8 MB • Cifratura PQ".to_string()),
                                    avatar_icon: Some(IconKind::Lock.into()),
                                    progress: if with_progress() { Some(progress_val()) } else { None },
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
                        }

                        // Configurator
                        div { class: "space-y-4 text-xs font-mono",
                            div {
                                span { class: "text-zinc-400 block mb-1.5 uppercase tracking-wider text-[10px]", "Modalità Scheda:" }
                                div { class: "flex gap-2",
                                    button {
                                        r#type: "button",
                                        class: if is_file_mode() {
                                            "px-3 py-1.5 rounded bg-purple-950/80 text-purple-200 border border-purple-500/40 font-bold cursor-pointer"
                                        } else {
                                            "px-3 py-1.5 rounded bg-white/[0.03] text-zinc-400 border border-white/[0.06] cursor-pointer"
                                        },
                                        onclick: move |_| is_file_mode.set(true),
                                        "File / Documento"
                                    }
                                    button {
                                        r#type: "button",
                                        class: if !is_file_mode() {
                                            "px-3 py-1.5 rounded bg-purple-950/80 text-purple-200 border border-purple-500/40 font-bold cursor-pointer"
                                        } else {
                                            "px-3 py-1.5 rounded bg-white/[0.03] text-zinc-400 border border-white/[0.06] cursor-pointer"
                                        },
                                        onclick: move |_| is_file_mode.set(false),
                                        "Utente / Contatto"
                                    }
                                }
                            }

                            div { class: "pt-2 border-t border-white/[0.06] space-y-2.5",
                                Checkbox {
                                    checked: with_description,
                                    label: Some("Mostra Descrizione Inferiore".to_string()),
                                }
                                Checkbox {
                                    checked: with_actions,
                                    label: Some("Mostra Bottoni Azione (Scarica / Condividi)".to_string()),
                                }
                                Checkbox {
                                    checked: with_progress,
                                    label: Some("Integra ProgressBar di Trasferimento".to_string()),
                                }
                            }

                            if with_progress() {
                                div { class: "space-y-1.5 pt-1",
                                    div { class: "flex justify-between text-zinc-400 text-[10px]",
                                        span { "AVANZAMENTO FILE:" }
                                        span { "{progress_val():.0}%" }
                                    }
                                    input {
                                        r#type: "range",
                                        min: "0",
                                        max: "100",
                                        step: "1",
                                        class: "w-full accent-purple-500 cursor-pointer",
                                        value: "{progress_val}",
                                        oninput: move |e| {
                                            if let Ok(v) = e.value().parse::<f32>() {
                                                progress_val.set(v);
                                            }
                                        },
                                    }
                                }
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
