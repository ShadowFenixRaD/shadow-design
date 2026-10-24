use dioxus::prelude::*;
use shadow_design::dropzone::FileMetadata;
use shadow_design::prelude::*;

use crate::components::{FeatureBanner, PageHeader, PropRow, ShowcaseTabs};

#[component]
pub fn DropzonePage() -> Element {
    let mut uploaded_files = use_signal(|| {
        vec![
            FileMetadata {
                name: "progetto_alpha_architettura.pdf".to_string(),
                size_bytes: 1024 * 1024 * 2 + 1024 * 400,
                mime_type: Some("application/pdf".to_string()),
            },
            FileMetadata {
                name: "mockup_dark_theme.png".to_string(),
                size_bytes: 1024 * 840,
                mime_type: Some("image/png".to_string()),
            },
        ]
    });
    let is_loading = use_signal(|| false);

    let props = vec![
        PropRow {
            name: "on_drop",
            r#type: "Option<EventHandler<Vec<FileMetadata>>>",
            default: "None",
            description: "Handler richiamato al rilascio di file validi o selezione da finestra",
        },
        PropRow {
            name: "accept",
            r#type: "Vec<String>",
            default: "vec![]",
            description: "Lista di estensioni o tipi MIME ammessi (es. \".png\", \".pdf\")",
        },
        PropRow {
            name: "max_size_bytes",
            r#type: "Option<u64>",
            default: "None",
            description: "Dimensione massima consentita in byte per ciascun file",
        },
        PropRow {
            name: "loading",
            r#type: "bool",
            default: "false",
            description: "Attiva lo stato di caricamento con spinner e blocco delle interazioni",
        },
        PropRow {
            name: "multiple",
            r#type: "bool",
            default: "false",
            description: "Consente la selezione multipla contemporanea di file",
        },
    ];

    let code_snippet = r#"Dropzone {
    accept: vec![".png".into(), ".jpg".into(), ".pdf".into()],
    max_size_bytes: Some(10 * 1024 * 1024), // 10 MB
    multiple: true,
    on_drop: move |files: Vec<FileMetadata>| {
        for f in files {
            println!("Caricato: {} ({})", f.name, f.format_size());
        }
    },
}"#
    .to_string();

    let mut last_selected_file = use_signal(|| None::<String>);

    rsx! {
        div { class: "sc-stack sc-stack--loose",
            PageHeader {
                title: "Dropzone".to_string(),
                description: "Area interattiva di caricamento file tramite Drag-and-Drop o selezione da file manager, con feedback visivo di hover e reject.".to_string(),
            }

            FeatureBanner { feature_name: "dropzone" }

            ShowcaseTabs {
                preview: rsx! {
                    div { class: "sc-stack",
                        Dropzone {
                            accept: vec![".png".into(), ".jpg".into(), ".pdf".into(), ".rs".into()],
                            max_size_bytes: Some(15 * 1024 * 1024),
                            multiple: true,
                            loading: *is_loading.read(),
                            on_drop: move |new_files: Vec<FileMetadata>| {
                                if let Some(f) = new_files.first() {
                                    last_selected_file.set(Some(f.name.clone()));
                                }
                                let mut list = uploaded_files.read().clone();
                                list.extend(new_files);
                                uploaded_files.set(list);
                            },
                        }

                        if let Some(name) = last_selected_file.read().as_ref() {
                            Alert {
                                variant: AlertVariant::Success,
                                title: "File selezionato".to_string(),
                                "{name}"
                            }
                        }

                        Card { title: "File Caricati ({uploaded_files.read().len()})".to_string(),
                            div { class: "sc-stack",
                                if !uploaded_files.read().is_empty() {
                                    div { class: "sc-actions",
                                        Button {
                                            size: ButtonSize::Xs,
                                            variant: ButtonVariant::Ghost,
                                            onclick: move |_| {
                                                uploaded_files.set(Vec::new());
                                                last_selected_file.set(None);
                                            },
                                            "Svuota Elenco"
                                        }
                                    }
                                }
                                if uploaded_files.read().is_empty() {
                                    p { class: "sc-subtle-text sc-mono", "Nessun file presente." }
                                } else {
                                    div { class: "sc-list",
                                        for (i, file) in uploaded_files.read().iter().enumerate() {
                                            div { key: "{i}", class: "sc-title-row",
                                                span { class: "sc-actions",
                                                    Icon { kind: IconKind::Paperclip, size: 14 }
                                                    span { "{file.name}" }
                                                }
                                                span { class: "sc-actions",
                                                    span { class: "sc-subtle-text sc-mono", "{file.format_size()}" }
                                                    span { class: "sc-success-text", Icon { kind: IconKind::CheckCircle, size: 14 } }
                                                }
                                            }
                                        }
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
