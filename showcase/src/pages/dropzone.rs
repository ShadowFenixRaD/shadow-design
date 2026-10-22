use dioxus::prelude::*;
use shadow_design::dropzone::FileMetadata;
use shadow_design::prelude::*;

use crate::components::{FeatureBanner, PropRow, ShowcaseTabs};

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
    let mut is_loading = use_signal(|| false);

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

    let files_list = uploaded_files.read().clone();

    rsx! {
        div { class: "max-w-4xl space-y-8",
            div {
                Heading { order: HeadingOrder::H2, "Dropzone" }
                Text { dimmed: true, size: TextSize::Sm, "Area interattiva di caricamento file tramite Drag-and-Drop o selezione da file manager, con feedback visivo di hover e reject." }
            }

            FeatureBanner { feature_name: "dropzone" }

            ShowcaseTabs {
                preview: rsx! {
                    div { class: "p-8 rounded-xl bg-[#060608] border border-white/[0.08] flex flex-col gap-8 shadow-inner",
                        Dropzone {
                            accept: vec![".png".into(), ".jpg".into(), ".pdf".into(), ".rs".into()],
                            max_size_bytes: Some(15 * 1024 * 1024),
                            multiple: true,
                            loading: *is_loading.read(),
                            on_drop: move |new_files: Vec<FileMetadata>| {
                                is_loading.set(true);
                                // Simula processing
                                let mut list = uploaded_files.read().clone();
                                list.extend(new_files);
                                uploaded_files.set(list);
                                is_loading.set(false);
                            },
                        }

                        // Lista file caricati
                        div { class: "p-5 rounded-xl border border-white/[0.06] bg-zinc-950/40 space-y-3",
                            div { class: "flex items-center justify-between",
                                span { class: "text-xs font-mono text-zinc-300 font-semibold uppercase tracking-wider", "File Caricati ({files_list.len()})" }
                                if !files_list.is_empty() {
                                    button {
                                        r#type: "button",
                                        class: "text-[11px] font-mono text-zinc-500 hover:text-rose-400 transition-colors cursor-pointer",
                                        onclick: move |_| uploaded_files.set(Vec::new()),
                                        "Svuota Elenco"
                                    }
                                }
                            }
                            if files_list.is_empty() {
                                p { class: "text-xs text-zinc-600 font-mono py-2", "Nessun file presente." }
                            } else {
                                div { class: "space-y-2",
                                    for (i, file) in files_list.iter().enumerate() {
                                        div {
                                            key: "{i}",
                                            class: "flex items-center justify-between p-3 rounded-lg bg-white/[0.02] border border-white/[0.04] text-xs font-mono",
                                            div { class: "flex items-center gap-3 min-width-0 truncate",
                                                Icon { kind: IconKind::Paperclip, size: 14, class: "text-purple-400 flex-shrink-0" }
                                                span { class: "text-zinc-200 truncate", "{file.name}" }
                                            }
                                            div { class: "flex items-center gap-4 flex-shrink-0",
                                                span { class: "text-zinc-500 text-[11px]", "{file.format_size()}" }
                                                Icon { kind: IconKind::CheckCircle, size: 14, class: "text-emerald-400" }
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
