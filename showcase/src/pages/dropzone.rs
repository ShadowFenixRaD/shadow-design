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

    let mut last_selected_file = use_signal(|| None::<String>);

    rsx! {
        div { class: "max-w-4xl space-y-8",
            div {
                Heading { order: HeadingOrder::H2, "Dropzone" }
                Text { dimmed: true, size: TextSize::Sm, "Area interattiva di caricamento file tramite Drag-and-Drop o selezione da file manager, con feedback visivo di hover e reject." }
            }

            FeatureBanner { feature_name: "dropzone" }

            ShowcaseTabs {
                preview: rsx! {
                    div { class: "p-8 rounded-xl bg-[#060608] border border-white/[0.08] flex flex-col gap-6 shadow-inner",
                        Dropzone {
                            accept: vec![".png".into(), ".jpg".into(), ".pdf".into(), ".rs".into()],
                            max_size_bytes: Some(15 * 1024 * 1024),
                            multiple: true,
                            loading: *is_loading.read(),
                            on_drop: move |new_files: Vec<FileMetadata>| {
                                if let Some(f) = new_files.first() {
                                    last_selected_file.set(Some(f.name.clone()));
                                }
                                is_loading.set(true);
                                let mut list = uploaded_files.read().clone();
                                list.extend(new_files);
                                uploaded_files.set(list);
                                is_loading.set(false);
                            },
                        }

                        // Banner notifica ultimo file selezionato
                        if let Some(name) = last_selected_file.read().as_ref() {
                            div { class: "p-3.5 rounded-xl border border-purple-500/40 bg-purple-950/30 flex items-center justify-between text-xs font-mono shadow-[0_0_15px_rgba(168,85,247,0.15)]",
                                div { class: "flex items-center gap-2.5",
                                    Icon { kind: IconKind::CheckCircle, size: 16, class: "text-emerald-400 flex-shrink-0" }
                                    span { class: "text-zinc-400", "File selezionato:" }
                                    span { class: "text-purple-300 font-bold", "{name}" }
                                }
                                Badge { label: "Caricato", variant: BadgeVariant::Success, dot: true }
                            }
                        }

                        // Lista file caricati
                        div { class: "p-5 rounded-xl border border-white/[0.06] bg-zinc-950/40 space-y-3",
                            div { class: "flex items-center justify-between",
                                span { class: "text-xs font-mono text-zinc-300 font-semibold uppercase tracking-wider", "File Caricati ({uploaded_files.read().len()})" }
                                if !uploaded_files.read().is_empty() {
                                    button {
                                        r#type: "button",
                                        class: "text-[11px] font-mono text-zinc-500 hover:text-rose-400 transition-colors cursor-pointer",
                                        onclick: move |_| {
                                            uploaded_files.set(Vec::new());
                                            last_selected_file.set(None);
                                        },
                                        "Svuota Elenco"
                                    }
                                }
                            }
                            if uploaded_files.read().is_empty() {
                                p { class: "text-xs text-zinc-600 font-mono py-2", "Nessun file presente." }
                            } else {
                                div { class: "space-y-2",
                                    for (i, file) in uploaded_files.read().iter().enumerate() {
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
